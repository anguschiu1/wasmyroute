pub mod geo;
pub mod map;
pub mod model;

use geo::Coord;
use log::{info, Level};
use model::Model;
use seed::{prelude::*, *};
use web_sys::{Coordinates, Geolocation, Position, PositionError, PositionOptions};

fn main() {
    let _ = console_log::init_with_level(Level::Info);
    info!("Hello, world! from log::macros"); // Print log in web console in Info level
    web_sys::console::log_1(&"Hello, world! from web_sys".into()); // Print log using browser-provided API through web_sys

    App::start("app", init, update, view); // Mount the `app` to the element with the `id` "app".
}

/// Initializes the geolocation feature of the web application.
///
/// This function attempts to access the user's current geolocation using the browser's Geolocation API.
/// On success, it updates the application state with the user's current latitude and longitude.
/// On failure, it logs an error message to the console.
///
/// # Parameters
/// - `orders`: A mutable reference to the application's `Orders` object, which allows for sending messages
///   and commands within the application.
///
/// # Panics
/// This function panics if it fails to access the Geolocation API or if the browser denies the geolocation request.
fn init_geolocation(orders: &mut impl Orders<Msg>) {
    // Attempt to access the Geolocation API from the browser's window object.
    let geolocation: Geolocation = window()
        .navigator()
        .geolocation()
        .expect("Unable to get geolocation.");

    // Clone the application and message mapper from the orders for use in the callback closures.
    let (app, msg_mapper) = (orders.clone_app(), orders.msg_mapper());

    // Define a success callback that extracts the latitude and longitude from the Position object,
    // logs them, and sends a message to update the application state with the new coordinates.
    let success_callback = Closure::wrap(Box::new(move |position: Position| {
        let coords: Coordinates = position.coords();
        let latitude = coords.latitude();
        let longitude = coords.longitude();
        info!("Latitude: {}", latitude);
        info!("Longitude: {}", longitude);
        app.update(msg_mapper(Msg::Position(Coord {
            lat: coords.latitude(),
            lon: coords.longitude(),
        })));
    }) as Box<dyn FnMut(Position)>);

    // Define an error callback that logs any errors encountered while attempting to get the geolocation.
    let error_callback = Closure::wrap(Box::new(move |error: PositionError| {
        info!("Error getting geolocation: {:?}", error);
    }) as Box<dyn FnMut(PositionError)>);

    // Configure geolocation options, enabling high accuracy.
    let mut options = PositionOptions::new();
    options.enable_high_accuracy(true);

    // Request the current position, providing the success and error callbacks, along with the options.
    geolocation
        .get_current_position_with_error_callback_and_options(
            success_callback.as_ref().unchecked_ref(),
            Some(error_callback.as_ref().unchecked_ref()),
            &options,
        )
        .expect("Unable to get position.");

    // Prevent the callbacks from being garbage-collected prematurely.
    success_callback.forget();
    error_callback.forget();
}

fn update_position(position: Coord, model: &mut model::Model, _orders: &mut impl Orders<Msg>) {
    info!("update position...");
    map::pan_to_position(model, position);
    model.position = position;
}

// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment, // Increment counter
    Decrement, // Decrement counter
    Position(Coord),
}

/// Initializes the application state and geolocation.
///
/// # Parameters
/// - `_`: An unused parameter of type `Url`, which could be used for routing purposes.
/// - `orders`: A mutable reference to the application's `Orders` object, allowing for
///   command and message passing within the application.
///
/// # Returns
/// Returns the initial model state of the application, which includes the setup of the map
/// and default values for other model fields.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    init_geolocation(orders); // Initialize geolocation features.
    map::init() // Initialize the map and return the initial model state.
}

/// Updates the application model based on the received message.
///
/// # Parameters
/// - `msg`: The message that indicates what action should be performed. It is an enum that
///   encapsulates the different types of actions that can occur.
/// - `model`: A mutable reference to the application's model. This model holds the state of the
///   application, and this function updates the model based on the received message.
/// - `orders`: A mutable reference to the application's `Orders` object, which allows for sending
///   messages and commands within the application. This parameter is used when the action requires
///   performing additional tasks that involve message passing or command execution.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => {
            model.zoom_in();
            update_position(model.position, model, orders);
        }
        Msg::Decrement => {
            model.zoom_out();
            update_position(model.position, model, orders);
        }
        Msg::Position(position) => {
            update_position(position, model, orders);
        }
    }
}

/// Renders the application's view.
///
/// This function constructs the HTML structure of the application's user interface. It displays
/// the current zoom level and the geographical coordinates (latitude and longitude) of the user's
/// position. It also provides buttons to increment and decrement the zoom level.
///
/// # Parameters
/// - `model`: A reference to the application's model, which contains the current state, including
///   the zoom level and the user's geographical position.
///
/// # Returns
/// Returns a `Node<Msg>` that represents the HTML structure of the application's view. This structure
/// is used by the Seed framework to render the view in the browser. The `Node<Msg>` type is a virtual
/// DOM node that encapsulates the elements and event listeners, facilitating efficient updates to the
/// real DOM in response to state changes.
fn view(model: &Model) -> Node<Msg> {
    div![
        "Zoom level: ",
        button!["-", ev(Ev::Click, |_| Msg::Decrement),],
        label![format!("{}", model.zoomlevel)],
        button!["+", ev(Ev::Click, |_| Msg::Increment),],
        label![format!(
            "lat:{}, lon:{}",
            model.position.lat, model.position.lon
        )]
    ]
}
