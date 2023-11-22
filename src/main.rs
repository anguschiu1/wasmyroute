use log::{info, Level};
use web_sys::{
    js_sys,
    wasm_bindgen::{closure::Closure, JsCast, JsValue},
};

fn main() {
    let _ = console_log::init_with_level(Level::Debug);

    info!("Hello, world!");
    // web_sys::console::log_1(&"Hello, world!".into());

    fn geo_callback(position: JsValue) {
        let s = js_sys::JSON::stringify(&position).expect("Unable to stringify JSON");
        info!("Geo callback: {:?}", s);
    }
    let geolocation = web_sys::window()
        .expect("Unable to get browser window.")
        .navigator()
        .geolocation()
        .expect("Unable to get geolocation.");
    let geo_callback_function =
        Closure::wrap(Box::new(|pos| geo_callback(pos)) as Box<dyn Fn(JsValue)>);
    geolocation
        .get_current_position(&geo_callback_function.as_ref().unchecked_ref())
        .expect("Unable to get position");
    geo_callback_function.forget();
}
