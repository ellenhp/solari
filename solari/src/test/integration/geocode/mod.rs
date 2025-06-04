use anyhow::{bail, Result};
use reqwest::{Client, Url};
use s2::latlng::LatLng;
use serde::Deserialize;

/// Look up the provided text on the maps.earth pelias instance and return the response as a LatLng.
pub async fn geocode_address(text: &str) -> Result<LatLng> {
    // Define the base URL
    const BASE_URL: &str = "https://maps.earth/pelias/v1/search";

    let url = Url::parse_with_params(BASE_URL, &[("text", text)])?;

    // Create an HTTP client
    let client = Client::new();

    // Make the GET request
    let response = client.get(url).send().await?;

    // Check if the response was successful
    if !response.status().is_success() {
        bail!("Failed to geocode address: {}", response.status());
    }

    // Parse the response as JSON
    let response_data: GeocodeResponse = serde_json::from_str(&response.text().await?)?;

    // Extract the first result (if any)
    if let Some(first_result) = response_data.features.first() {
        // Extract the coordinates
        if let Some(center) = first_result
            .geometry
            .as_ref()
            .and_then(|geom| geom.coordinates.as_ref())
        {
            // Create a LatLng from the coordinates
            let latlng = LatLng::from_degrees(center[1], center[0]);
            return Ok(latlng);
        }
    }

    // If no results were found, return an error
    bail!("No results found for the provided text");
}

/// The structure of the geocoding response
#[derive(Deserialize, Debug)]
struct GeocodeResponse {
    features: Vec<Feature>,
}

/// A feature in the geocoding response
#[derive(Deserialize, Debug)]
struct Feature {
    geometry: Option<Geometry>,
}

/// The geometry of a feature
#[derive(Deserialize, Debug)]
struct Geometry {
    coordinates: Option<Vec<f64>>,
}
