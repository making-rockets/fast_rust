use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class={C!("m-24 bg-blue-900")}>
            {"header"}
        </div>
    }
}
