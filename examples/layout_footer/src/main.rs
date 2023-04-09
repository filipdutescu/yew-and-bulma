use yew::prelude::*;
use yew_and_bulma::layout::footer::Footer;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Footer>
            {"This is some text inside a footer."}
        </Footer>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
