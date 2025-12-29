use crate::types::FlatpakExtError;
use clap::Parser;
use run_temp::run_no_install;

pub mod run_temp;
pub mod run_temp_tools;
pub mod types;
pub mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
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

fn main() -> Result<(), FlatpakExtError> {
    let cli = Cli::parse();
    if cli.verbose {
        simple_logger::init_with_level(log::Level::Trace).unwrap();
    }
    log::info!("Starting flatrun!");
    run_no_install(cli.file, cli.dep, cli.app_id, cli.remote, cli.clean)?;
    Ok(())
}
