use gpx::Gpx;
use leaflet::{LatLng, LayerGroup, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use log::info;
use web_sys::js_sys::Array;
use yew::prelude::*;

use crate::geo::Coord;
use crate::Model;

#[derive(Properties, PartialEq)]
pub struct MainMapProps {
    pub pos: Coord,
    pub gpx: Gpx,
}

#[function_component(MainMap)]
pub fn main_map(props: &MainMapProps) -> Html {
    info!("1 Rendering MainMap, props.pos {:?}", props.pos);
    let model_state = use_state(Model::default);
    {
        let model = model_state.clone();
        let pos = props.pos;
        // use_effect_with hook with empty dependencies ensure this effect runs only once.
        use_effect_with((), move |_| {
            // FnOnce, init map for the MainMap component.
            info!("2 use_effect_with() - Initializing map...");
            let options = MapOptions::default();
            options.set_track_resize(true); // map automatically handles browser window resize to update itself.
            options.set_center(pos.into());
            options.set_zoom(1.0);
            let map = Map::new("map", &options);

            let gpx_lg = LayerGroup::new();
            gpx_lg.add_to(&map);
            let position_lg = LayerGroup::new();
            position_lg.add_to(&map);

            add_tile_layer(&map);

            let mut new_model = (*model).clone();
            new_model.map = Some(map);
            new_model.position_lg = Some(position_lg);
            new_model.gpx_lg = Some(gpx_lg);
            let zoom: u8 = 18;
            new_model.zoomlevel = zoom;
            model.set(new_model);
            // TeardownFn
            || {}
        });
    }
    {
        let pos = props.pos;
        let model = model_state.clone();
        let new_gpx = props.gpx.clone();
        use_effect_with((pos, new_gpx.clone()), move |_| {
            info!("5 use_effect - borrowing Map...");
            let mut new_model = (*model).clone();
            new_model.gpx = Some(new_gpx);
            info!("6. check gpx: {:?}", new_model.clone().gpx.unwrap());
            pan_to_position(&model, pos);
            draw_gpx_route(&new_model);
            // model.set(new_model);
            || {}
        });
    }
    html! {
    <>
        <div id="map"></div>
        <p>{ format!("pos: {:?}", props.pos) }</p>
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
        // map.set_view(&position.into(), zoom.into());
        map.fly_to(&position.into(), zoom.into());
    } else {
        info!("pan_to_position: Map is not in model");
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
