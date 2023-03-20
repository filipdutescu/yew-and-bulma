use yew::prelude::*;
use yew_and_bulma::{elements::delete::Delete, utils::size::Size};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <Delete size={Size::Small} />
        <Delete />
        <Delete size={Size::Medium} />
        <Delete size={Size::Large} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
