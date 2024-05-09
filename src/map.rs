use leaflet::{LatLng, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use rand::prelude::*;
use web_sys::{js_sys::Array, wasm_bindgen::JsValue};

use crate::{Coord, Model};

pub fn init() -> Model {
    let options = MapOptions::default();
    let map = Map::new("map", &options);
    // Create a random start location, so we get to init the map even if geolocation isn't available.
    let mut rng = thread_rng();
    let position = Coord {
        lat: rng.gen_range(-90.0..90.0),
        lon: rng.gen_range(-180.0..180.0),
    };

    map.set_view(&LatLng::new(position.lat, position.lon), 10.0); // set default &Map for tiles and polylines

    // map.set_view(&LatLng::new(51.5160977, -0.1091519), 10.0); // set default &Map for tiles and polylines

    add_tile_layer(&map);
    add_polyline(&map);

    Model {
        map: Some(map),
        zoomlevel: 5,
        ..Default::default() //initialize counter
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
