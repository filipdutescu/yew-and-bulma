use yew::prelude::*;
use yew_and_bulma::{elements::progress::ProgressBar, helpers::color::Color, utils::size::Size};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <ProgressBar value={15.0} />
        <ProgressBar value={25.0} color={Color::Primary} />
        <ProgressBar value={35.0} color={Color::Link} />
        <ProgressBar value={15.0} color={Color::Info} size={Size::Small} />
        <ProgressBar value={15.0} color={Color::Info} size={Size::Normal} />
        <ProgressBar value={15.0} color={Color::Info} size={Size::Medium} />
        <ProgressBar value={15.0} color={Color::Info} size={Size::Large} />
        <ProgressBar color={Color::Danger} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
