use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use leaflet::{LatLng, LayerGroup, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use log::info;
use rand::prelude::*;
use web_sys::console::info;
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
    let model = Rc::new(RefCell::new(Model::default()));
    {
        let model = Rc::clone(&model);
        use_effect_with((), move |_| {
            // use_effect_with hook with empty dependencies ensure this effect runs only once.
            // FnOnce, init map for the MainMap component.
            info!("2 use_effect_with() - Initializing map...");
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
            add_tile_layer(&map);
            map.set_view(&LatLng::new(position.lat, position.lon), 12.0);
            model.borrow_mut().map = Some(map);
            // TeardownFn
            || {}
        });
    }

    // Use effect_with_deps hook to trigger re-rendering when props.pos changes.
    let model_clone = Rc::clone(&model);
    info!(
        "4 print strong count of strong ref cycles: {}",
        Rc::strong_count(&model_clone)
    );
    if let Some(_map) = model.borrow().map.as_ref() {
        info!("4.1 Map is in model.");
    } else {
        info!("4.2 No Map, skipping update.");
    }
    let pos = props.pos;
    info!("pos is {:?}", pos);
    //FIXME: Find out why map is not being added to the model.
    use_effect(move || {
        info!("5 use_effect - borrowing Map...");
        info!(
            "6 print strong count of strong ref cycles: {}",
            Rc::strong_count(&model_clone)
        );
        if let Some(map) = model_clone.borrow().map.as_ref() {
            info!("use_effect - Updating map view...");
            map.set_view(&LatLng::new(pos.lat, pos.lon), 10.0);
        } else {
            info!("use_effect - No Map, skipping update.");
        }
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
