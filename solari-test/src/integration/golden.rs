use serde::{Deserialize, Serialize};
use time::UtcDateTime;

use solari::api::{LatLng, response::SolariResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Golden {
    pub from_location: LatLng,
    pub to_location: LatLng,
    pub start_time: UtcDateTime,
    pub route: Option<SolariResponse>,
}
