use yew::prelude::*;
use yew_and_bulma::{elements::delete::Delete, layout::container::Container, utils::size::Size};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <Delete size={Size::Small} />
            <Delete />
            <Delete size={Size::Medium} />
            <Delete size={Size::Large} />
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
