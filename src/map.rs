use leaflet::{LatLng, LayerGroup, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use log::info;
use web_sys::js_sys::Array;
use yew::prelude::*;

use crate::geo::Coord;
use crate::Model;

#[derive(Properties, PartialEq)]
pub struct MainMapProps {
    pub pos: Coord,
}

#[function_component(MainMap)]
pub fn main_map(props: &MainMapProps) -> Html {
    info!("1 Rendering MainMap, props.pos {:?}", props.pos);
    let model_state = use_state(Model::default);
    {
        let model = model_state.clone();
        // use_effect_with hook with empty dependencies ensure this effect runs only once.
        use_effect_with((), move |_| {
            // FnOnce, init map for the MainMap component.
            info!("2 use_effect_with() - Initializing map...");
            let options = MapOptions::default();
            let map = Map::new("map", &options);
            let gpx_lg = LayerGroup::new();
            gpx_lg.add_to(&map);
            let position_lg = LayerGroup::new();
            position_lg.add_to(&map);
            let zoom: u8 = 18;

            // Generate a random start location to initialize the map if geolocation isn't available.
            // let mut rng = thread_rng();
            // let position = Coord {
            //     lat: rng.gen_range(-90.0..90.0),
            //     lon: rng.gen_range(-180.0..180.0),
            // };
            // Set the map view to the random position with a default zoom level.
            add_tile_layer(&map);
            pan_to_position(&model, Coord::default());
            let mut new_model = (*model).clone();
            new_model.map = Some(map);
            new_model.position_lg = Some(position_lg);
            new_model.gpx_lg = Some(gpx_lg);
            new_model.zoomlevel = zoom;
            model.set(new_model);
            // TeardownFn
            || {}
        });
    }

    let model_clone = model_state.clone();
    if let Some(_map) = model_clone.map.as_ref() {
        info!("4.1 Map is in model.");
    } else {
        info!("4.2 No Map, skipping update.");
    }
    let pos = props.pos;
    info!("4.3 pos is {:?}", pos);
    {
        let model = model_state.clone();
        use_effect(move || {
            info!("5 use_effect - borrowing Map...");
            pan_to_position(&model, pos);
            || {}
        });
    }

    html! {
    <>
    <p>{ format!("pos: {:?}", props.pos) }</p>
    <div id="map"></div>
    </>
    }
}

fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}

pub fn pan_to_position(model: &Model, position: Coord) {
    info!("pan_to_position...");
    if model.map.is_some() {
        let zoom: u8 = model.zoomlevel;
        let map = model.map.as_ref().unwrap();
        map.set_view(&position.into(), zoom.into());
    } else {
        info!("pan_to_position: Map is not in model");
    }
}

pub fn _draw_gpx_route(model: &Model) {
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
