use yew::prelude::*;
use yew_and_bulma::{
    elements::title::Title,
    layout::{
        container::Container,
        level::{Level, LevelItem, LevelLeft, LevelRight},
    },
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <Title>{"Left and right aligned level items"}</Title>
            <Level>
                <LevelLeft>
                    <LevelItem>{"This is some text in a left level."}</LevelItem>
                </LevelLeft>

                <LevelRight>
                    <LevelItem>{"This is some text in a right level."}</LevelItem>
                </LevelRight>
            </Level>

            <Title>{"Center aligned level items"}</Title>
            <Level>
                <LevelItem>{"This is some text in a level item."}</LevelItem>
                <LevelItem>{"This is some text in a level item."}</LevelItem>
                <LevelItem>{"This is some text in a level item."}</LevelItem>
                <LevelItem>{"This is some text in a level item."}</LevelItem>
            </Level>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
