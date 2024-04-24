use leaflet::{LatLng, Map, MapOptions, Polyline, PolylineOptions, TileLayer};
use log::{error, info, Level};
use seed::{prelude::*, *};
use web_sys::js_sys::Array;

mod bindings;
mod geo;
mod map;
mod model;
mod osm;

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
enum Msg {
    Increment,
    OsmMapFetched(fetch::Result<String>),
}
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    let options = MapOptions::default();
    let map = Map::new("map", &options);
    map.set_view(&LatLng::new(51.5160977, -0.1091519), 10.0);

    add_tile_layer(&map);
    add_polyline(&map);

    orders.perform_cmd(async { Msg::OsmMapFetched(send_osm_request().await) });

    Model::default()
}

async fn send_osm_request() -> fetch::Result<String> {
    fetch(get_osm_request_url())
        .await?
        .check_status()?
        .text()
        .await
}
fn get_osm_request_url() -> &'static str {
    "https://www.openstreetmap.org/api/0.6/map?
     bbox=10.2%2C63.4%2C10.3%2C63.4"
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
        Msg::OsmFetched(Ok(response_data)) => {
            model.osm = quick_xml::de::from_str(&response_data)
                .expect("Unable to deserialize the OSM data");
            map::render_topology_and_position(&model);
        }
        Msg::OsmFetched(Err(fetch_error)) => {
            error!("Fetching OSM data failed: {:#?}", fetch_error);
        }
    }
}
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        button![model, ev(Ev::Click, |_| Msg::Increment),]
    ]
}
