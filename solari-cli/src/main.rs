mod build_timetable;
mod download_feeds;

use crate::build_timetable::{BuildArgs, run_build_timetable};
use crate::download_feeds::{DownloadFeedsArgs, run_download_feeds};
use clap::{Parser, Subcommand};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(name = "solari-cli")]
#[command(about = "A CLI tool for the Solari public transport routing system.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build(BuildArgs),
    DownloadFeeds(DownloadFeedsArgs),
}

#[tokio::main(worker_threads = 64)]
async fn main() -> Result<(), anyhow::Error> {
    tracing::subscriber::set_global_default(FmtSubscriber::new())
        .expect("setting tracing default failed");
    let cli = Cli::parse();

    match cli.command {
        Commands::Build(args) => run_build_timetable(args).await,
        Commands::DownloadFeeds(args) => run_download_feeds(args).await,
    }
}
