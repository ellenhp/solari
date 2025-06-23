use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use solari_spatial::SphereIndexVec;
use solari_transfers::{TransferGraph, fast_paths::FastGraphVec};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    valhalla_tiles: PathBuf,
    #[arg(long)]
    output: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    tracing::subscriber::set_global_default(FmtSubscriber::new())
        .expect("setting tracing default failed");

    let args = Args::parse();
    let database = Arc::new(redb::Database::create(
        args.output.join("graph_metadata.db"),
    )?);
    let transfer_graph =
        TransferGraph::<FastGraphVec, SphereIndexVec<usize>>::new(&args.valhalla_tiles, database)?;
    transfer_graph.save_to_dir(args.output)?;
    Ok(())
}
