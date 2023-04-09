use yew::prelude::*;
use yew_and_bulma::{layout::section::Section, utils::size::Size};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Section>
                {"This is some text in a box inside a section."}
            </Section>

            <Section size={Size::Medium}>
                {"This is some text in a box inside a medium section."}
            </Section>

            <Section size={Size::Large}>
                {"This is some text in a box inside a large section."}
            </Section>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
