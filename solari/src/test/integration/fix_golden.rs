use crate::raptor::timetable::mmap::MmapTimetable;
use crate::raptor::timetable::Time;
use crate::route::Router;
use crate::test::integration::api_latlng_to_s2_latlng;
use crate::test::integration::build_timetable::build_timetable;
use crate::test::integration::golden::Golden;
use crate::test::integration::test_golden;
use anyhow::Result;
use log::{error, info};
use serde_json;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use tempdir::TempDir;

/// Fix all golden test cases by updating their responses.
pub async fn fix_golden_test_cases(goldens_dir: PathBuf) -> Result<()> {
    // Create a temporary directory for building the timetable
    let test_dir = TempDir::new("solari-golden")?;

    // Build the timetable
    info!("Building timetable at {:?}", test_dir.path());
    build_timetable(test_dir.path(), &goldens_dir).await?;

    // Open the timetable
    let timetable = MmapTimetable::open(&test_dir.path().join("timetable"))?;

    // Create a router
    let router = Router::new(timetable, goldens_dir.join("valhalla_tiles"))?;

    // Read and process all .json files in $goldens_dir/cases
    for entry in fs::read_dir(&goldens_dir.join("cases"))? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            // Read the golden test case
            let file = File::open(&path)?;
            let reader = BufReader::new(file);
            let mut golden: Golden = serde_json::from_reader(reader)?;

            // Test the golden and update it if needed
            match test_golden(&router, &golden, &path).await {
                Ok(_) => {
                    info!("Passed: {:?}", path);
                }
                Err(err) => {
                    error!("Failed: {:?}", err);

                    // Update the golden with the new response
                    let route = router
                        .route(
                            Time::from_epoch_seconds(golden.start_time.unix_timestamp() as u32),
                            api_latlng_to_s2_latlng(&golden.from_location),
                            api_latlng_to_s2_latlng(&golden.to_location),
                            Some(1500f64),
                            Some(1000),
                            Some(4),
                            Some(2),
                        )
                        .await;

                    golden.route = Some(route);

                    // Write the updated golden to disk
                    let golden_file = File::create(&path)?;
                    serde_json::to_writer_pretty(golden_file, &golden)?;

                    info!("Updated golden: {:?}", path);
                }
            }
        }
    }

    Ok(())
}
