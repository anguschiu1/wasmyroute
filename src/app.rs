use crate::geo::InitGeolocation;
use crate::map::MainMap;
use crate::model::Model;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <MainMap />
            <InitGeolocation />
        </main>
    }
}
