use anyhow::bail;
use build_timetable::load_timetable_params;
use chrono::Days;
use chrono_tz::Tz;
use geocode::geocode_address;
use log::{debug, error, info};
use s2::latlng::LatLng;
use serde_json;
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use time::UtcDateTime;

use tempdir::TempDir;

use crate::api::response::SolariResponse;
use crate::raptor::timetable::mmap::MmapTimetable;
use crate::raptor::timetable::Time;
use crate::route::Router;
use crate::test::integration::golden::Golden;

mod build_timetable;
pub mod fix_golden;
mod geocode;
mod golden;

/// Convert an s2::latlng::LatLng to an api::LatLng
fn s2_latlng_to_api_latlng(latlng: &LatLng) -> crate::api::LatLng {
    crate::api::LatLng {
        lat: latlng.lat.deg(),
        lon: latlng.lng.deg(),
        stop: None,
    }
}

/// Convert an s2::latlng::LatLng to an api::LatLng
pub fn api_latlng_to_s2_latlng(latlng: &crate::api::LatLng) -> LatLng {
    LatLng::from_degrees(latlng.lat, latlng.lon)
}

fn print_golden_diff(old: &SolariResponse, new: &SolariResponse) {
    let old = serde_json::to_string_pretty(old).unwrap();
    let new = serde_json::to_string_pretty(new).unwrap();
    let diff = TextDiff::from_lines(&old, &new);

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        print!("{}{}", sign, change);
    }
}

async fn test_golden<'a>(
    router: &Router<'a, MmapTimetable<'a>>,
    golden: &Golden,
    golden_filename: &Path,
) -> anyhow::Result<()> {
    if golden.route.is_none() {
        bail!("Golden {:?} is missing a route.", golden_filename);
    }

    let from = LatLng::from_degrees(golden.from_location.lat, golden.from_location.lon);
    let to = LatLng::from_degrees(golden.to_location.lat, golden.to_location.lon);

    let start_time = Time::from_epoch_seconds(golden.start_time.unix_timestamp() as u32);

    let route = router
        .route(
            start_time,
            from,
            to,
            Some(1500f64),
            Some(1000),
            Some(4),
            Some(2),
        )
        .await;

    if golden.route != Some(route.clone()) {
        print_golden_diff(golden.route.as_ref().unwrap(), &route);
        bail!(format!("Golden {:?} failed diff", golden_filename))
    }
    Ok(())
}

pub async fn run_test_suite(goldens_dir: PathBuf) -> anyhow::Result<()> {
    let test_dir = TempDir::new("solari-golden")?;

    debug!("Building timetable at {:?}", test_dir.path());

    crate::test::integration::build_timetable::build_timetable(test_dir.path(), &goldens_dir)
        .await?;

    let timetable = MmapTimetable::open(&test_dir.path().join("timetable"))?;

    debug!("Built timetable");

    let router = Router::new(timetable, goldens_dir.join("valhalla_tiles"))?;

    let mut all_ok = true;
    // Read and parse all .json files in $goldens_dir/cases
    for entry in fs::read_dir(&goldens_dir.join("cases"))? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            let file = File::open(&path)?;
            let reader = BufReader::new(file);
            let golden: Golden = serde_json::from_reader(reader)?;

            match test_golden(&router, &golden, &path).await {
                Ok(_) => {
                    info!("Passed: {:?}", path)
                }
                Err(err) => {
                    all_ok = false;
                    error!("Failed: {:?}", err)
                }
            }
        }
    }

    if all_ok {
        Ok(())
    } else {
        bail!("Tests failed")
    }
}

pub async fn expand_test_suite(goldens_dir: PathBuf) -> anyhow::Result<()> {
    let test_dir = TempDir::new("solari-golden")?;

    debug!("Building timetable at {:?}", test_dir.path());

    crate::test::integration::build_timetable::build_timetable(test_dir.path(), &goldens_dir)
        .await?;

    let timetable = MmapTimetable::open(&test_dir.path().join("timetable"))?;

    debug!("Built timetable");

    let router = Router::new(timetable, goldens_dir.join("valhalla_tiles"))?;

    // Load parameters from params.json
    let params = load_timetable_params(&goldens_dir)?;

    loop {
        let name = prompt("Enter a name for this test case, or empty to exit: ")?;
        if name.is_empty() {
            break;
        }
        let mut from_location: Option<LatLng> = None;
        while from_location.is_none() {
            let text = prompt("From address: ")?;
            if text.is_empty() {
                break;
            }
            from_location = geocode_address(&text).await.ok();
        }
        let mut to_location: Option<LatLng> = None;
        while to_location.is_none() {
            let text = prompt("To address: ")?;
            if text.is_empty() {
                break;
            }
            to_location = geocode_address(&text).await.ok();
        }
        if to_location.is_none() {
            break;
        }

        let start_time = UtcDateTime::from_unix_timestamp(
            params
                .start_date
                .checked_add_days(Days::new(1))
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
                .and_local_timezone(Tz::UTC)
                .unwrap()
                .timestamp(),
        )
        .unwrap();

        let route = router
            .route(
                Time::from_epoch_seconds(start_time.unix_timestamp() as u32),
                from_location.unwrap(),
                to_location.unwrap(),
                Some(1500f64),
                Some(1000),
                Some(4),
                Some(2),
            )
            .await;

        // Create a Golden struct and write it to a file
        let golden = Golden {
            from_location: s2_latlng_to_api_latlng(&from_location.unwrap()),
            to_location: s2_latlng_to_api_latlng(&to_location.unwrap()),
            start_time,
            route: Some(route),
        };

        // Write the golden to a file
        let golden_path = goldens_dir.join("cases").join(format!("{}.json", name));
        let golden_file = File::create(golden_path)?;
        serde_json::to_writer_pretty(golden_file, &golden)?;
    }

    Ok(())
}

fn prompt(text: &str) -> anyhow::Result<String> {
    print!("{} ", text);
    std::io::stdout().flush()?;

    let mut response = String::new();
    std::io::stdin().read_line(&mut response)?;

    Ok(response.trim_end().to_string())
}
