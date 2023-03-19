use yew::prelude::*;
use yew_and_bulma::elements::icon::Icon;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <Icon class="fas fas-home" />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
