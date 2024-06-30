use leaflet::LatLng;
use log::info;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    window, Coordinates, Geolocation, Position, PositionError, PositionOptions,
};

#[derive(Default, Clone, Copy, Debug)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}

impl From<Coord> for LatLng {
    fn from(coord: Coord) -> Self {
        LatLng::new(coord.lat, coord.lon)
    }
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
fn init_geolocation() {
    // Attempt to access the Geolocation API from the browser's window object.
    let geolocation = window()
        .navigator()
        .geolocation()
        .expect("Unable to get geolocation.");

    // Clone the application and message mapper from the orders for use in the callback closures.
    // let (app, msg_mapper) = (orders.clone_app(), orders.msg_mapper());

    // Define a success callback that extracts the latitude and longitude from the Position object,
    // logs them, and sends a message to update the application state with the new coordinates.
    let success_callback = Closure::wrap(Box::new(move |position: Position| {
        let coords: Coordinates = position.coords();
        let latitude = coords.latitude();
        let longitude = coords.longitude();
        info!("Latitude: {}", latitude);
        info!("Longitude: {}", longitude);
        // app.update(msg_mapper(Msg::Position(Coord {
        //     lat: coords.latitude(),
        //     lon: coords.longitude(),
        // })));
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
