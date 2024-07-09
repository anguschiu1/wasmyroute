use gloo_utils::window;
use leaflet::LatLng;
use log::info;
use std::ops::Deref;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    Coordinates, Geolocation, Position, PositionError, PositionOptions,
};
use yew::prelude::*;

use crate::{map, model::Model};
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

#[function_component(InitGeolocation)]
pub fn init_geolocation() -> Html {
    let model_state = use_state(Model::default);

    // Attempt to access the Geolocation API from the browser's window object.
    let geolocation: Geolocation = window()
        .navigator()
        .geolocation()
        .expect("Unable to get geolocation.");

    // Define a success callback that extracts the latitude and longitude from the Position object,
    let model_state_clone = model_state.clone();
    let success_callback = Closure::wrap(Box::new(move |position: Position| {
        let position = Coord {
            lat: position.coords().latitude(),
            lon: position.coords().longitude(),
        };
        info!(
            "Geolocation API callback success\nlat:{}, log:{}",
            position.lat, position.lon
        );
        let mut model = model_state_clone.deref().clone();
        model.position = position;
        model_state_clone.set(model);
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

    let pos = (*model_state).clone().position;
    // // pan map to current position
    // map::pan_to_position(&model_state, pos);

    html! {
        <p>{"Position:"} {pos.lat}{","}{pos.lon}</p>
    }
}
