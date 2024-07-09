use std::ops::Deref;

use leaflet::{LatLng, LayerGroup, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use log::info;
use rand::prelude::*;
use web_sys::js_sys::Array;
use yew::prelude::*;

use crate::geo::Coord;
use crate::Model;

#[function_component(MainMap)]
pub fn main_map() -> Html {
    let model_state = use_state(Model::default);
    let model_state_clone = model_state.clone();
    use_effect(move || {
        // FnOnce, init map for the MainMap component.
        if model_state.map.is_none() {
            let _model: Model = init();
            model_state.set(_model);
        }
        // TeardownFn
        || {}
    });

    html! {
    <>
      <div id="map"></div>
      <p>{"Zoom: "}{&model_state_clone.zoomlevel}</p>
      <p>{"Map in model: "}{&model_state_clone.map.is_some()}</p>
    </>

    }
}

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
        position: Some(position),
        gpx_lg: Some(gpx_lg),
        position_lg: Some(position_lg),
        ..Default::default() // Initialize counter if needed.
    };
    if model.map.is_some() {
        info!("Map is Some");
    }
    // Add a tile layer and a polyline to the map.
    add_tile_layer(model.map.as_ref().unwrap());

    model
}
fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}

pub fn pan_to_position(model: &Model, position: Coord) {
    info!("pan_to_position...");
    info!("Position: {},{}", position.lat, position.lon);
    if let Some(map) = &model.map {
        info!("Map is Some");
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
            model_gpx.tracks.iter().for_each(|track| {
                track.segments.iter().for_each(|segment| {
                    let latlngs = segment.points.iter().fold(Array::new(), |acc, point| {
                        acc.push(&LatLng::new(point.point().y(), point.point().x()));
                        acc
                    });
                    let gpx_route =
                        &Polyline::new_with_options(&latlngs, &PolylineOptions::default());
                    gpx_lg.add_layer(gpx_route);
                    map.fit_bounds(&gpx_route.get_bounds());
                    gpx_lg.add_to(map);
                });
            });
        } else {
            info!("draw_gpx_route: map or gpx_lg is None");
        }
    }
}
