use log::{info, Level};
use seed::{prelude::*, *};

fn main() {
    let _ = console_log::init_with_level(Level::Debug);

    info!("Hello, world!");
    web_sys::console::log_1(&"Hello, world!".into());
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
type Model = i32;
#[derive(Copy, Clone)]
enum Msg {
    Increment,
}
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
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
        button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}
