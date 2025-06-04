use crate::raptor::timetable::mmap::MmapTimetable;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TimetableParams {
    pub(crate) start_date: NaiveDate,
    pub(crate) num_days: u16,
}

pub(crate) fn load_timetable_params(
    goldens_dir: impl AsRef<Path>,
) -> anyhow::Result<TimetableParams> {
    let params_path = goldens_dir.as_ref().join("params.json");
    let params_content = fs::read_to_string(params_path)?;
    let params: TimetableParams = serde_json::from_str(&params_content)?;
    Ok(params)
}

pub(crate) async fn build_timetable<'a>(
    test_dir: impl AsRef<Path>,
    goldens_dir: impl AsRef<Path>,
) -> anyhow::Result<MmapTimetable<'a>> {
    let mut zip_files = vec![];

    for entry in std::fs::read_dir(goldens_dir.as_ref())? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("zip") {
            zip_files.push(path);
        }
    }
    let timetable_path = test_dir.as_ref().join("timetable");
    let valhalla_path = goldens_dir.as_ref().join("valhalla_tiles");

    // Load parameters from params.json
    let params = load_timetable_params(goldens_dir)?;

    crate::build_timetable::timetable_from_feeds(
        &zip_files,
        &timetable_path,
        &valhalla_path,
        Some(params.start_date),
        Some(params.num_days),
    )
    .await
}
