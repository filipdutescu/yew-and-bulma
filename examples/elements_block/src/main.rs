use yew::prelude::*;
use yew_and_bulma::{elements::block::Block, layout::container::Container};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <Block>{"This is some text in a block."}</Block>
            <Block>{"This is some more text in a different block. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean efficitur sit amet massa fringilla egestas. Nullam condimentum luctus turpis."}</Block>
            <Block>{"This is some text in a third block."}</Block>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
