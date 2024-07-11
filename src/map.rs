use std::ops::Deref;

use leaflet::{LatLng, LayerGroup, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use log::info;
use rand::prelude::*;
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
    info!("Rendering MainMap with position {:?}", props.pos);
    let mut model = Model::default();
    use_effect_with((), move |_| {
        // use_effect_with hook with empty dependencies ensure this effect runs only once.
        // FnOnce, init map for the MainMap component.
        {
            info!("Initializing map...");
            let options = MapOptions::default();
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
            // Add a tile layer and a polyline to the map.
            add_tile_layer(&map);
            model.map = Some(map);
        }
        // TeardownFn
        || {}
    });

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
    info!("Position: {},{}", position.lat, position.lon);
    if model.map.is_some() {
        info!("Model has a Map");
        let zoom = model.zoomlevel.into();
        let map = model.map.as_ref().unwrap(); // Get a reference to the map.
        map.set_view(&position.into(), zoom); // Pass a reference to LatLng
    }
}

// pub fn draw_gpx_route(model: &Model) {
//     info!("draw_gpx_route...");
//     if let (Some(map), Some(gpx_lg)) = (&model.map, &model.gpx_lg) {
//         gpx_lg.clear_layers();
//         info!("gpx layer group cleared");

//         if let Some(model_gpx) = &model.gpx {
//             model_gpx.tracks.iter().for_each(|track| {
//                 track.segments.iter().for_each(|segment| {
//                     let latlngs = segment.points.iter().fold(Array::new(), |acc, point| {
//                         acc.push(&LatLng::new(point.point().y(), point.point().x()));
//                         acc
//                     });
//                     let gpx_route =
//                         &Polyline::new_with_options(&latlngs, &PolylineOptions::default());
//                     gpx_lg.add_layer(gpx_route);
//                     map.fit_bounds(&gpx_route.get_bounds());
//                     gpx_lg.add_to(map);
//                 });
//             });
//         } else {
//             info!("draw_gpx_route: map or gpx_lg is None");
//         }
//     }
// }
