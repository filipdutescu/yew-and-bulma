use yew::prelude::*;
use yew_and_bulma::{elements::r#box::Box, layout::container::Container};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <Box>{"This is some text in a box inside a container."}</Box>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
