use crate::map;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    use_effect(|| {
        let _model = map::init();
        || {}
    });
    html! {
        <main>
            <div id="map"></div>
        </main>
    }
}
