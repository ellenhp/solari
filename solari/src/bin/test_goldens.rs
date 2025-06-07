use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    tests_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    let mut all_ok = true;
    // Iterate through each subdirectory in tests_dir
    for entry in fs::read_dir(&args.tests_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Call run_test_suite for each subdirectory
            if let Err(e) = solari::test::integration::run_test_suite(path.clone()).await {
                eprintln!("Error running test suite for {}: {}", path.display(), e);
                all_ok = false;
            } else {
                println!("Successfully ran test suite for {}", path.display())
            }
        }
    }
    if !all_ok {
        bail!("Tests failed");
    }

    Ok(())
}
