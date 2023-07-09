use yew::prelude::*;

mod components;
use crate::components::header::Header;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class={C!("flex")}>
            <Header />
        </div>
    }
}
