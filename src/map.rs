use leaflet::{LatLng, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use rand::prelude::*;
use web_sys::{js_sys::Array, wasm_bindgen::JsValue};

use crate::{Coord, Model};

/// Initializes the map model with a random location and default settings.
///
/// This function creates a new map instance with default options, sets a random
/// starting location if geolocation is not available, adds a tile layer and a polyline
/// to the map, and returns a `Model` instance containing the map and its settings.
///
/// # Returns
/// A `Model` instance containing:
/// - An `Option<Map>` holding the initialized map.
/// - The zoom level set to 5.
/// - The randomly generated starting position.
pub fn init() -> Model {
    // Set default map options.
    let options = MapOptions::default();
    // Create a new map instance with the specified element id and options.
    let map = Map::new("map", &options);

    // Generate a random start location to initialize the map if geolocation isn't available.
    let mut rng = thread_rng();
    let position = Coord {
        lat: rng.gen_range(-90.0..90.0),
        lon: rng.gen_range(-180.0..180.0),
    };

    // Set the map view to the random position with a default zoom level.
    map.set_view(&LatLng::new(position.lat, position.lon), 10.0);

    // Add a tile layer and a polyline to the map.
    add_tile_layer(&map);
    add_polyline(&map);

    // Return a Model instance with the map, zoom level, and position.
    Model {
        map: Some(map),
        zoomlevel: 10,
        position,
        // ..Default::default() // Initialize counter if needed.
    }
}

// Add tiles to coordinates and zoom level specified by &Map
fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}

// Draw lines on &Map
fn add_polyline(map: &Map) {
    let options = PolylineOptions::default();
    Polyline::new_with_options(
        &[
            LatLng::new(51.447148, -0.369531),
            LatLng::new(51.846775, 0.026800),
            LatLng::new(51.315329, 0.039838),
        ]
        .iter()
        .map(JsValue::from)
        .collect::<Array>(),
        &options,
    )
    .add_to(map);
}

pub fn pan_to_position(model: &Model, position: Coord) {
    if let Some(map) = &model.map {
        let zoom = model.zoomlevel.into();
        map.set_view(&LatLng::new(position.lat, position.lon), zoom);
    }
}
