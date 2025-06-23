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

impl PartialEq for LatLng {
    fn eq(&self, other: &Self) -> bool {
        let angle_epsiolon = 0.00000001;
        f64::abs(self.lat - other.lat) < angle_epsiolon
            && f64::abs(self.lon - other.lon) < angle_epsiolon
            && self.stop == other.stop
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SolariLeg {
    #[serde(rename = "transit")]
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
    #[serde(rename = "transfer")]
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
        route_shape: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SolariItinerary {
    pub start_location: LatLng,
    pub end_location: LatLng,
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
    pub legs: Vec<SolariLeg>,
}

impl PartialEq for SolariItinerary {
    fn eq(&self, other: &Self) -> bool {
        let meta_eq = self.start_location == other.start_location
            && self.end_location == other.end_location
            && self.start_time == other.start_time
            && self.end_time == other.end_time
            && self.legs.len() == other.legs.len();

        if !meta_eq {
            return false;
        }

        // The core problem we're trying to solve here is that it doesn't matter for the purposes of integration testing whether I get onto a bus at stop 1 or stop 2, as long as it's the same bus.
        for (leg1, leg2) in self.legs.iter().zip(other.legs.iter()) {
            let legs_eq = match (leg1, leg2) {
                (
                    SolariLeg::Transit {
                        start_time,
                        end_time,
                        start_location,
                        end_location,
                        route_shape,
                        transit_route,
                        transit_agency,
                    },
                    SolariLeg::Transit {
                        start_time: other_start_time,
                        end_time: other_end_time,
                        start_location: other_start_location,
                        end_location: other_end_location,
                        route_shape: other_route_shape,
                        transit_route: other_transit_route,
                        transit_agency: other_transit_agency,
                    },
                ) => {
                    transit_route == other_transit_route
                        && transit_agency == other_transit_agency
                        && (start_location != other_start_location
                            || start_time == other_start_time)
                        && (end_location != other_end_location || end_time == other_end_time)
                        && (start_location != other_start_location
                            || end_location != other_end_location
                            || route_shape == other_route_shape)
                        && (start_location == other_start_location
                            || end_location == other_end_location)
                }
                (SolariLeg::Transit { .. }, SolariLeg::Transfer { .. }) => false,
                (SolariLeg::Transfer { .. }, SolariLeg::Transit { .. }) => false,
                (
                    SolariLeg::Transfer {
                        start_time,
                        end_time,
                        start_location,
                        end_location,
                        route_shape,
                    },
                    SolariLeg::Transfer {
                        start_time: other_start_time,
                        end_time: other_end_time,
                        start_location: other_start_location,
                        end_location: other_end_location,
                        route_shape: other_route_shape,
                    },
                ) => {
                    (start_location == other_start_location || end_location == other_end_location)
                        && (start_time == other_start_time || end_time == other_end_time)
                        && (start_location != other_start_location
                            || end_location != other_end_location
                            || route_shape == other_route_shape)
                }
            };
            if !legs_eq {
                return false;
            }
        }
        false
    }
}
