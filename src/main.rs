use leaflet::{LatLng, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use log::{info, Level};
use seed::{prelude::*, *};
use web_sys::js_sys::Array;

fn main() {
    let _ = console_log::init_with_level(Level::Debug);

    info!("Hello, world!");
    web_sys::console::log_1(&"Hello, world!".into());
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}

fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}

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
type Model = i32;
#[derive(Copy, Clone)]
enum Msg {
    Increment,
}
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let options = MapOptions::default();
    let map = Map::new("map", &options);
    map.set_view(&LatLng::new(51.5160977, -0.1091519), 10.0);

    add_tile_layer(&map);
    add_polyline(&map);

    Model::default()
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        button![model, ev(Ev::Click, |_| Msg::Increment),]
    ]
}
