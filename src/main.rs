use leaflet::{LatLng, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use log::{info, Level};
use rand::prelude::*;
use seed::{prelude::*, *};
use web_sys::js_sys::Array;
use web_sys::{Coordinates, Geolocation, Position, PositionError};

pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}

fn main() {
    let _ = console_log::init_with_level(Level::Info);
    info!("Hello, world! from log::macros"); // Print log in web console in Info level
    web_sys::console::log_1(&"Hello, world! from web_sys".into()); // Print log using browser-provided API through web_sys

    get_user_location();

    App::start("app", init, update, view); // Mount the `app` to the element with the `id` "app".
}

pub fn get_user_location() {
    let geolocation: Geolocation = window()
        .navigator()
        .geolocation()
        .expect("Unable to get geolocation.");

    let success_callback = Closure::wrap(Box::new(move |position: Position| {
        let coords: Coordinates = position.coords();
        let latitude = coords.latitude();
        let longitude = coords.longitude();
        info!("Latitude: {}", latitude);
        info!("Longitude: {}", longitude);
    }) as Box<dyn FnMut(Position)>);

    let error_callback = Closure::wrap(Box::new(move |error: PositionError| {
        info!("Error getting geolocation: {:?}", error);
    }) as Box<dyn FnMut(PositionError)>);

    geolocation
        .get_current_position_with_error_callback(
            success_callback.as_ref().unchecked_ref(),
            Some(error_callback.as_ref().unchecked_ref()),
        )
        .unwrap();

    success_callback.forget();
    error_callback.forget();
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
// `Model` describes our app state.
type Model = i32;
#[derive(Copy, Clone)]

// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment, // Increment counter
}

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
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

    Model::default() //initialize counter
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        button![model, ev(Ev::Click, |_| Msg::Increment),]
    ]
}
