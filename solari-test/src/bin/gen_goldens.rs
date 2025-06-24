use std::path::PathBuf;

use clap::Parser;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    tests_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::new())
        .expect("setting tracing default failed");
    let args = Args::parse();

    // Iterate through each subdirectory in tests_dir
    let entries = std::fs::read_dir(&args.tests_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // Call expand_test_suite for each subdirectory
            if let Err(e) = solari_test::integration::expand_test_suite(path.clone()).await {
                eprintln!("Error expanding test suite for {}: {}", path.display(), e);
            } else {
                println!("Successfully expanded test suite for {}", path.display())
            }
        }
    }
    Ok(())
}
