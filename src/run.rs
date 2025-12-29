use indicatif::ProgressBar;
use libflatpak::{
    glib::{KeyFile, KeyFileFlags},
    prelude::RemoteExt,
    prelude::{
        BundleRefExt, FileExt, InstallationExt, InstallationExtManual, InstanceExt, RefExt,
        RemoteRefExt, TransactionExt,
    },
    LaunchFlags,
};
use rustix::process::{Pid, WaitOptions};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::thread;

use crate::types::{get_installation, Flatpak, FlatpakOut, FlatrunError, Remote, Repo};

/// Runs a flatpak
pub fn run(
    install_at: Repo,
    app: Flatpak,
    deps_at: Option<Repo>,
    runtime: Option<Flatpak>,
    remote_uri: Option<String>,
    pb: ProgressBar,
) -> Result<(), FlatrunError> {
    log::debug!("Get the flatpak installations, error out if they don't exist or some other error occurs...");
    let deps_repo: libflatpak::Installation =
        get_installation(&deps_at.as_ref().unwrap_or(&Repo::default()))?;
    let install_repo: libflatpak::Installation = get_installation(&install_at)?;
    log::debug!("Add remote for installations");
    let remote = remote_uri.map_or(Remote::default(), |x| Remote::new(x));
    let default_branch = remote.clone().default_branch;
    let remote = libflatpak::Remote::try_from(remote)?;
    remote.set_default_branch(&default_branch);
    if let Err(e) = deps_repo.add_remote(
        &remote,
        true,
        libflatpak::gio::Cancellable::current().as_ref(),
    ) {
        log::error!("There was a problem adding the remote: {}", e);
    }
    if let Err(e) = install_repo.add_remote(
        &remote,
        true,
        libflatpak::gio::Cancellable::current().as_ref(),
    ) {
        log::error!("There was a problem adding the remote: {}", e);
    }
    log::debug!("Get the flatpak");
    let app = app.convert_to_flatpak_out(&install_repo, &remote, &default_branch, false)?;
    log::debug!("Get the runtime");
    let runtime = runtime.map_or(
        {
            match app {
                FlatpakOut::Bundle(ref bundle) => {
                    let config = KeyFile::new();
                    config.load_from_bytes(&bundle.metadata().unwrap(), KeyFileFlags::NONE)?;
                    let runtime_str = config.string("Application", "runtime").unwrap().to_string();
                    let mut info = runtime_str.split("/");
                    let app_id = info.next().unwrap().to_string();
                    let _ = info.next().unwrap().to_string();
                    let branch = info.next().unwrap().to_string();
                    Ok::<FlatpakOut, FlatrunError>(
                        Flatpak::Download(app_id)
                            .convert_to_flatpak_out(&deps_repo, &remote, &branch, true)?,
                    )
                }
                FlatpakOut::Download(ref download) => {
                    let config = KeyFile::new();
                    config.load_from_bytes(&download.metadata().unwrap(), KeyFileFlags::NONE)?;
                    let runtime_str = config.string("Application", "runtime").unwrap().to_string();
                    let mut info = runtime_str.split("/");
                    let app_id = info.next().unwrap().to_string();
                    let _ = info.next().unwrap().to_string();
                    let branch = info.next().unwrap().to_string();
                    Ok::<FlatpakOut, FlatrunError>(
                        Flatpak::Download(app_id)
                            .convert_to_flatpak_out(&deps_repo, &remote, &branch, true)?,
                    )
                }
            }
        },
        |x| {
            Ok::<FlatpakOut, FlatrunError>(x.convert_to_flatpak_out(
                &deps_repo,
                &remote,
                &default_branch,
                true,
            )?)
        },
    )?;
    log::debug!("Create transactions");
    let deps_transaction = libflatpak::Transaction::for_installation(
        &deps_repo,
        libflatpak::gio::Cancellable::current().as_ref(),
    )?;
    let install_transaction = libflatpak::Transaction::for_installation(
        &install_repo,
        libflatpak::gio::Cancellable::current().as_ref(),
    )?;
    log::debug!("Connect operations to callback");
    {
        let pb = pb.clone();
        deps_transaction.connect_new_operation(move |_, transaction, progress| {
            let op_type = transaction.operation_type().clone();
            let app_ref = transaction.get_ref().unwrap().to_string();
            pb.set_position(0);
            pb.set_message(format!(
                "{} {}",
                app_ref.clone(),
                op_type.to_str().unwrap_or_default().as_str()
            ));
            let pb = pb.clone();
            progress.connect_changed(move |progress| {
                pb.set_position(progress.progress() as u64);
            });
        });
    }
    {
        let pb = pb.clone();
        install_transaction.connect_new_operation(move |_, transaction, progress| {
            let op_type = transaction.operation_type().clone();
            let app_ref = transaction.get_ref().unwrap().to_string();
            pb.set_position(0);
            pb.set_message(format!(
                "{} {}",
                app_ref.clone(),
                op_type.to_str().unwrap_or_default().as_str()
            ));
            let pb = pb.clone();
            progress.connect_changed(move |progress| {
                pb.set_position(progress.progress() as u64);
            });
        });
    }
    log::debug!("Add installation command to dependency transaction");
    if let Err(e) = match runtime {
        FlatpakOut::Bundle(ref bundle) => deps_transaction
            .add_install_bundle(&bundle.file().unwrap(), None)
            .map_err(|e| FlatrunError::from(e)),
        FlatpakOut::Download(ref download) => deps_transaction
            .add_install(
                &download.remote_name().unwrap(),
                &download.format_ref().unwrap(),
                &[],
            )
            .map_err(|e| FlatrunError::from(e)),
    } {
        log::warn!("Could not install dependency: {:?}", e);
    }
    log::debug!("Run dependency transaction.");
    deps_transaction.run(libflatpak::gio::Cancellable::current().as_ref())?;
    log::debug!("Set up sideload repo...");
    install_transaction
        .add_sideload_repo(&deps_repo.path().unwrap().path().unwrap().to_string_lossy());
    log::debug!("Add installation command to install transaction");
    if let Err(e) = match app {
        FlatpakOut::Bundle(ref bundle) => install_transaction
            .add_install_bundle(&bundle.file().unwrap(), None)
            .map_err(|e| FlatrunError::from(e)),
        FlatpakOut::Download(ref download) => install_transaction
            .add_install(
                &download.remote_name().unwrap(),
                &download.format_ref().unwrap(),
                &[],
            )
            .map_err(|e| FlatrunError::from(e)),
    } {
        log::error!("Could not install app: {:?}", e);
        panic!()
    }
    log::debug!("Add deps as dependency source");
    install_transaction.add_dependency_source(&deps_repo);
    log::debug!("Run install transaction");
    install_transaction.run(libflatpak::gio::Cancellable::current().as_ref())?;
    log::debug!("Run instance");

    let app_id = app.app_id();
    pb.finish_with_message(format!("Running {}", app_id));

    let inst = match app {
        FlatpakOut::Bundle(bundle) => install_repo.launch_full(
            LaunchFlags::DO_NOT_REAP,
            &bundle.name().unwrap(),
            bundle.arch().as_deref(),
            bundle.branch().as_deref(),
            None,
            libflatpak::gio::Cancellable::current().as_ref(),
        )?,
        FlatpakOut::Download(download) => install_repo.launch_full(
            LaunchFlags::DO_NOT_REAP,
            &download.name().unwrap(),
            download.arch().as_deref(),
            download.branch().as_deref(),
            None,
            libflatpak::gio::Cancellable::current().as_ref(),
        )?,
    };
    // Track instance
    let pid = Pid::from_raw(inst.pid()).unwrap();
    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            log::info!("Received signal {:?}", sig);
            let _ = rustix::process::kill_process(
                pid,
                rustix::process::Signal::from_named_raw(sig).unwrap(),
            );
        }
    });

    log::debug!("Waiting on instance to close...");
    while !rustix::process::waitpid(Some(pid), WaitOptions::empty())
        .is_ok_and(|x| x.is_some_and(|(_, y)| y.exited() || y.signaled()))
    {}
    log::debug!("Drop repos");
    drop(install_repo);
    drop(deps_at);
    Ok(())
}
