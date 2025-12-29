use std::{env, fs};

use crate::types::{Flatpak, FlatrunError, Repo};
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use indicatif_log_bridge::LogWrapper;

mod run;
use crate::run::run;
pub mod types;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true, arg_required_else_help = true)]
/// flatrun - Run a flatpak without installing it
struct Cli {
    /// Flatpak to run from file
    #[arg(short, long)]
    file: Option<String>,
    /// Dependency file (leave out to download dependencies automatically)
    #[arg(short, long)]
    dep: Option<String>,
    /// Flatpak appid to download
    #[arg(short, long)]
    app_id: Option<String>,
    /// Flatpak remote to use to download any flatpaks (defaults to flathub)
    #[arg(short, long)]
    remote: Option<String>,
    /// Clean out the temp repo directory
    #[arg(short, long)]
    clean: bool,
    /// Verbose
    #[arg(long)]
    verbose: bool,
}

fn main() -> Result<(), FlatrunError> {
    let cli = Cli::parse();
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).build();
    let level = logger.filter();
    let multi = MultiProgress::new();

    let pb = multi.add(ProgressBar::new(100));

    pb.set_style(
        ProgressStyle::with_template(
            "{wide_msg}\n[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}",
        )
        .unwrap()
        .progress_chars("=> "),
    );

    LogWrapper::new(multi.clone(), logger).try_init().unwrap();
    log::set_max_level(level);
    log::info!("Starting flatrun!");
    if cli.clean {
        let _ = fs::remove_dir_all(env::temp_dir().join("flatrun"));
        log::trace!("Cleared directory: {:?}", env::temp_dir().join("flatrun"));
    }
    for e in env::vars().map(|(x, y)| format!("{}={}", x, y)) {
        log::trace!("{e}");
    }

    match cli.file {
        Some(path) => {
            match run(
                Repo::temp(),
                Flatpak::new_from_uri(path),
                Some(Repo::default()),
                cli.dep.map(|x| Flatpak::new_from_uri(x)),
                cli.remote,
                pb,
            ) {
                Ok(_) => Ok(()),
                Err(e) => {
                    log::error!("{:?}", e);
                    Err(e)
                }
            }
        }
        None => {
            if let Some(app_id) = cli.app_id {
                match run(
                    Repo::temp_in(env::temp_dir().join("flatrun")),
                    Flatpak::Download(app_id),
                    Some(Repo::default()),
                    cli.dep.map(|x| Flatpak::new_from_uri(x)),
                    cli.remote,
                    pb,
                ) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        log::error!("{:?}", e);
                        Err(e)
                    }
                }
            } else {
                Ok(())
            }
        }
    }
}
