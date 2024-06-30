mod geo;
mod map;
mod model;
mod route;

mod app;

use app::App;
use geo::Coord;
use log::{info, Level};
use model::Model;

fn main() {
    let _ = console_log::init_with_level(Level::Info);
    info!("Hello, world! from log::macros"); // Print log in web console in Info level
    web_sys::console::log_1(&"Hello, world! from web_sys".into()); // Print log using browser-provided API through web_sys

    yew::Renderer::<App>::new().render();
}
