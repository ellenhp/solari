use std::{
    fs,
    hash::{DefaultHasher, Hasher},
    path::PathBuf,
};

use anyhow::bail;
use clap::Parser;
use gtfs_structures::GtfsReader;
use log::debug;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use solari::raptor::timetable::{in_memory::InMemoryTimetableBuilder, mmap::MmapTimetable};

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

fn process_gtfs<'a>(
    path: &PathBuf,
    base_path: &PathBuf,
) -> Result<MmapTimetable<'a>, anyhow::Error> {
    let feed = if let Ok(feed) = GtfsReader::default().read_from_path(path.to_str().unwrap()) {
        feed
    } else {
        bail!(format!("Failed to load feed: {:?}", path));
    };
    debug!("Processing feed: {:?}", path);
    let in_memory_timetable_builder = InMemoryTimetableBuilder::new(&feed)?;
    let hash = {
        let mut hasher = DefaultHasher::new();
        hasher.write(path.to_str().unwrap().as_bytes());
        format!("{:x}", hasher.finish())
    };

    let timetable_dir = base_path.join(hash);
    fs::create_dir_all(&timetable_dir).unwrap();
    Ok(MmapTimetable::from_in_memory(
        &in_memory_timetable_builder,
        &timetable_dir,
    )?)
}

async fn concat_timetables<'a>(
    paths: &[PathBuf],
    base_path: &PathBuf,
    valhalla_tile_path: &PathBuf,
) -> Result<MmapTimetable<'a>, anyhow::Error> {
    let paths = paths.to_vec();

    let timetables: Vec<MmapTimetable<'_>> = paths
        .par_iter()
        .filter_map(|path| MmapTimetable::open(path).ok())
        .collect();

    // Combine all timetables into one.
    let timetable = MmapTimetable::concatenate(&timetables, base_path, valhalla_tile_path).await;
    Ok(timetable)
}

async fn timetable_from_feeds<'a>(
    paths: &[PathBuf],
    base_path: &PathBuf,
    valhalla_tile_path: &PathBuf,
) -> Result<MmapTimetable<'a>, anyhow::Error> {
    let paths = paths.to_vec();

    let timetables: Vec<MmapTimetable<'_>> = paths
        .par_iter()
        .filter(|path| path.extension().map(|ext| ext == "zip") == Some(true))
        .filter_map(|path| {
            process_gtfs(&path, base_path)
                .map_err(|err| {
                    log::error!("Failed to process GTFS feed: {}", err);
                    err
                })
                .ok()
        })
        .collect();

    // Combine all timetables into one.
    let timetable = MmapTimetable::concatenate(&timetables, base_path, valhalla_tile_path).await;
    Ok(timetable)
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

        let _timetable = timetable_from_feeds(&paths, &args.base_path.into(), &args.valhalla_tiles)
            .await
            .unwrap();
    } else {
        let _timetable = timetable_from_feeds(
            &[args.gtfs_path.into()],
            &args.base_path.into(),
            &args.valhalla_tiles,
        )
        .await
        .unwrap();
    }
}
