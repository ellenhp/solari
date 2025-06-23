use clap::Parser;
use solari::timetable::build::{concat_timetables, timetable_from_feeds};
use std::{fs, path::PathBuf};

#[derive(Parser)]
pub struct BuildArgs {
    #[arg(long)]
    pub base_path: PathBuf,
    #[arg(long)]
    pub gtfs_path: PathBuf,
    #[arg(long)]
    pub valhalla_tiles: PathBuf,
    #[arg(short, long, default_value_t = 1)]
    pub num_threads: usize,
    #[arg(long, default_value_t = false)]
    pub concat_only: bool,
}

pub async fn run_build_timetable(args: BuildArgs) -> Result<(), anyhow::Error> {
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
    Ok(())
}
