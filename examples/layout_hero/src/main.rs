use yew::prelude::*;
use yew_and_bulma::{
    elements::title::{Subtitle, Title},
    helpers::color::Color,
    layout::hero::{Hero, HeroBody, HeroFoot, HeroHead, Size},
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Hero size={Size::Medium} color={Color::Primary}>
            <HeroHead>
                <Title>{"Great header content"}</Title>
            </HeroHead>

            <HeroBody>
                <Title>{"Epic title"}</Title>
                <Subtitle>{"Some awesome subtitle"}</Subtitle>
            </HeroBody>

            <HeroFoot>
                <p>{"Some small text in the footer."}</p>
            </HeroFoot>
        </Hero>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
