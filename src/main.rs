use log::{info, Level};

fn main() {
    let _ = console_log::init_with_level(Level::Debug);

    info!("Hello, world!");
    web_sys::console::log_1(&"Hello, world!".into());
}
