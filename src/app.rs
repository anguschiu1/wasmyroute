use crate::{geo, map};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    use_effect(|| {
        let _model = map::init();
        geo::init_geolocation();
        || {}
    });
    html! {
        <main>
            <div id="map"></div>
        </main>
    }
}
