use std::fs;
use std::path::PathBuf;

use anyhow::Result;
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

    // Iterate through each subdirectory in tests_dir
    for entry in fs::read_dir(&args.tests_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Err(e) =
                solari::test::integration::fix_golden::fix_golden_test_cases(path.clone()).await
            {
                eprintln!("Error fixing test suite for {}: {}", path.display(), e);
            } else {
                println!("Successfully fixed test suite for {}", path.display())
            }
        }
    }

    Ok(())
}
