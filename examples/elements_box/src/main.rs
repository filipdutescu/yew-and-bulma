use yew::prelude::*;
use yew_and_bulma::elements::r#box::Box;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <Box>{"This is some text in a box."}</Box>
        <Box>{"This is some more text in a different box. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean efficitur sit amet massa fringilla egestas. Nullam condimentum luctus turpis."}</Box>
        <Box>{"This is some text in a third box."}</Box>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
