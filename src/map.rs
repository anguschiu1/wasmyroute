use leaflet::{LatLng, LayerGroup, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use log::info;
use rand::prelude::*;
use web_sys::js_sys::Array;

use crate::{Coord, Model};

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

    model
}
fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}

pub fn pan_to_position(model: &Model, position: Coord) {
    if let Some(map) = &model.map {
        let zoom = model.zoomlevel.into();
        map.set_view(&position.into(), zoom); // Pass a reference to LatLng
    }
}

pub fn draw_gpx_route(model: &Model) {
    info!("draw_gpx_route...");
    if let (Some(map), Some(gpx_lg)) = (&model.map, &model.gpx_lg) {
        gpx_lg.clear_layers();
        info!("gpx layer group cleared");

        if let Some(model_gpx) = &model.gpx {
            for track in model_gpx.tracks.iter() {
                for segment in track.segments.iter() {
                    let latlngs = Array::new();
                    for point in segment.points.iter() {
                        latlngs.push(&LatLng::new(point.point().y(), point.point().x()));
                        info!(
                            "drawing line between these two points: {:?}",
                            latlngs.get(latlngs.length() - 1)
                        );
                    }
                    let options = PolylineOptions::default();
                    gpx_lg.add_layer(&Polyline::new_with_options(&latlngs, &options));
                }
            }
            gpx_lg.add_to(map);
        } else {
            info!("draw_gpx_route: model_gpx is None");
        }
    }
    // TODO: Pan to gpx route
}
