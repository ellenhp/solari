use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub mod request;
pub mod response;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LatLng {
    pub lat: f64,
    pub lon: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FareboxLeg {
    #[serde(alias = "transit")]
    Transit {
        #[serde(
            serialize_with = "time::serde::timestamp::milliseconds::serialize",
            deserialize_with = "time::serde::timestamp::milliseconds::deserialize"
        )]
        start_time: OffsetDateTime,
        #[serde(
            serialize_with = "time::serde::timestamp::milliseconds::serialize",
            deserialize_with = "time::serde::timestamp::milliseconds::deserialize"
        )]
        end_time: OffsetDateTime,
        start_location: LatLng,
        end_location: LatLng,
        #[serde(skip_serializing_if = "Option::is_none")]
        route_shape: Option<String>,
        transit_route: Option<String>,
        transit_agency: Option<String>,
    },
    #[serde(alias = "transfer")]
    Transfer {
        #[serde(
            serialize_with = "time::serde::timestamp::milliseconds::serialize",
            deserialize_with = "time::serde::timestamp::milliseconds::deserialize"
        )]
        start_time: OffsetDateTime,
        #[serde(
            serialize_with = "time::serde::timestamp::milliseconds::serialize",
            deserialize_with = "time::serde::timestamp::milliseconds::deserialize"
        )]
        end_time: OffsetDateTime,
        start_location: LatLng,
        end_location: LatLng,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FareboxItinerary {
    #[serde(
        serialize_with = "time::serde::timestamp::milliseconds::serialize",
        deserialize_with = "time::serde::timestamp::milliseconds::deserialize"
    )]
    pub start_time: OffsetDateTime,
    #[serde(
        serialize_with = "time::serde::timestamp::milliseconds::serialize",
        deserialize_with = "time::serde::timestamp::milliseconds::deserialize"
    )]
    pub end_time: OffsetDateTime,
    pub legs: Vec<FareboxLeg>,
}
