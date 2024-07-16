use crate::{geo::Coord, map::MainMap, route::GpxFile};

use gloo_utils::window;
use gpx::Gpx;
use log::info;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    Geolocation, Position, PositionError, PositionOptions,
};
use yew::prelude::*;
#[function_component(App)]
pub fn app() -> Html {
    let pos = use_state(Coord::default); // Use state hook trigger re-rendering when state changes.
    let gpx_state = use_state(Gpx::default); // Use state hook trigger re-rendering when state changes.

    {
        let pos = pos.clone();
        use_effect_with((), move |_| {
            // Use effect_with hook with empty dependencies ensures this effect runs only once.
            let pos = pos.clone();
            // Attempt to access the Geolocation API from the browser's window object.
            let geolocation: Geolocation = window()
                .navigator()
                .geolocation()
                .expect("Unable to get geolocation.");

            // Define a success callback that extracts the latitude and longitude from the Position object,
            let success_callback = Closure::wrap(Box::new(move |position: Position| {
                let position = Coord {
                    lat: position.coords().latitude(),
                    lon: position.coords().longitude(),
                };
                pos.set(position);
                info!("2. pos: UseStateHandle<Coord> {:?}", *pos.clone());
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
            || ()
        });
    }

    let gpx_state_clone = gpx_state.clone();
    let on_gpx_update = Callback::from(move |gpx: Option<Gpx>| {
        info!("GpxFile on_gpx_update: {:?}", gpx_state_clone);
        // TODO: Parse and display the GPX data in the MainMap component.
        if let Some(gpx) = gpx {
            gpx_state_clone.set(gpx);
        }
    });

    html! {
        <main>
            <MainMap pos={*pos} gpx={(*gpx_state).clone()}/>
            <GpxFile on_gpx_update={on_gpx_update}/>
        </main>
    }
}
