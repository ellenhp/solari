use std::{fs, path::PathBuf};

use clap::Parser;
use solari::build_timetable::{concat_timetables, timetable_from_feeds};

extern crate solari;

#[derive(Parser)]
struct BuildArgs {
    #[arg(long)]
    base_path: PathBuf,
    #[arg(long)]
    gtfs_path: PathBuf,
    #[arg(long)]
    valhalla_tiles: PathBuf,
    #[arg(short, long, default_value_t = 1)]
    num_threads: usize,
    #[arg(long, default_value_t = false)]
    concat_only: bool,
}

#[tokio::main(worker_threads = 64)]
async fn main() {
    env_logger::init();
    let args = BuildArgs::parse();
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.num_threads)
        .build_global()
        .unwrap();
    if args.concat_only {
        let paths: Vec<PathBuf> = fs::read_dir(&args.base_path)
            .unwrap()
            .map(|p| p.unwrap().path())
            .collect();

        let _timetable = concat_timetables(&paths, &args.base_path.into(), &args.valhalla_tiles)
            .await
            .unwrap();
    } else if fs::metadata(&args.gtfs_path).unwrap().is_dir() {
        let paths: Vec<PathBuf> = fs::read_dir(&args.gtfs_path)
            .unwrap()
            .map(|p| p.unwrap().path())
            .collect();

        let _timetable = timetable_from_feeds(
            &paths,
            &args.base_path.into(),
            &args.valhalla_tiles,
            None,
            None,
        )
        .await
        .unwrap();
    } else {
        let _timetable = timetable_from_feeds(
            &[args.gtfs_path.into()],
            &args.base_path.into(),
            &args.valhalla_tiles,
            None,
            None,
        )
        .await
        .unwrap();
    }
}
