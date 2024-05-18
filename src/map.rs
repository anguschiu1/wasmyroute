use leaflet::{LatLng, LayerGroup, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use log::info;
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

    let gpx_lg = LayerGroup::new();
    gpx_lg.add_to(&map);

    let position_lg = LayerGroup::new();
    position_lg.add_to(&map);

    // Generate a random start location to initialize the map if geolocation isn't available.
    let mut rng = thread_rng();
    let position = Coord {
        lat: rng.gen_range(-90.0..90.0),
        lon: rng.gen_range(-180.0..180.0),
    };

    // Set the map view to the random position with a default zoom level.
    map.set_view(&LatLng::new(position.lat, position.lon), 10.0);

    let model = Model {
        map: Some(map),
        zoomlevel: 10,
        position,
        gpx_lg: Some(gpx_lg),
        position_lg: Some(position_lg),
        ..Default::default() // Initialize counter if needed.
    };
    // Add a tile layer and a polyline to the map.
    add_tile_layer(model.map.as_ref().unwrap());
    add_polyline(
        &model,
        Coord {
            lat: 51.447148,
            lon: -0.369531,
        },
        Coord {
            lat: 51.846775,
            lon: 0.026800,
        },
    );
    model
}

// Draw lines on &Map
/// Adds a tile layer to the specified map.
///
/// This function adds a tile layer to the given map using the OpenStreetMap tile server.
///
/// # Arguments
///
/// * `map` - A reference to the `Map` instance to which the tile layer will be added.
fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}

/// Draws a polyline on the specified map between two coordinates.
///
/// This function creates a polyline connecting the `from` and `to` coordinates and adds it to the given map.
///
/// # Arguments
///
/// * `map` - A reference to the `Map` instance on which the polyline will be drawn.
/// * `from` - The starting coordinate of the polyline.
/// * `to` - The ending coordinate of the polyline.
fn add_polyline(model: &Model, from: Coord, to: Coord) {
    info!("add_polyline...");
    let options = PolylineOptions::default();
    Polyline::new_with_options(
        &[LatLng::new(from.lat, from.lon), LatLng::new(to.lat, to.lon)]
            .iter()
            .map(JsValue::from)
            .collect::<Array>(),
        &options,
    )
    .add_to(model.map.as_ref().unwrap());
}

/// Pans the map to the specified position with the current zoom level.
///
/// This function updates the map view to center on the given position, using the zoom level
/// stored in the model. If the map is not initialized, the function does nothing.
///
/// # Arguments
///
/// * `model` - A reference to the `Model` instance containing the map and its settings.
/// * `position` - The `Coord` struct representing the new position to pan to.
pub fn pan_to_position(model: &Model, position: Coord) {
    if let Some(map) = &model.map {
        let zoom = model.zoomlevel.into();
        map.set_view(&position.into(), zoom); // Pass a reference to LatLng
    }
}

pub fn draw_gpx_route(model: &Model) {
    info!("draw_gpx_route...");
    if let Some(map) = &model.map {
        if let Some(model_gpx) = &model.gpx {
            for track in model_gpx.tracks.iter() {
                for segment in track.segments.iter() {
                    for point in segment.points.iter() {
                        let coord = Coord {
                            lat: point.point().x(),
                            lon: point.point().y(),
                        };
                        // info!("drawing line between these two points: {:?}", coord);
                        add_polyline(model, coord, coord);
                    }
                }
            }
        } else {
            info!("draw_gpx_route: model_gpx is None");
        }
    } else {
        info!("draw_gpx_route: map is None");
    }
    // TODO: Pan to gpx route
}
