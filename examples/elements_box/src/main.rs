use yew::prelude::*;
use yew_and_bulma::{elements::r#box::Box, layout::container::Container};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <Box>{"This is some text in a box."}</Box>
            <Box>{"This is some more text in a different box. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean efficitur sit amet massa fringilla egestas. Nullam condimentum luctus turpis."}</Box>
            <Box>{"This is some text in a third box."}</Box>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
