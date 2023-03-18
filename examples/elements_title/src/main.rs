use yew::prelude::*;
use yew_and_bulma::elements::{
    block::Block,
    title::{Size, Subtitle, Title},
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <Title>{"Title"}</Title>
        <Subtitle>{"Subtitle"}</Subtitle>

        <hr />

        <Block>
            <Title size={Size::One}>{"Title 1"}</Title>
            <Title size={Size::Two}>{"Title 2"}</Title>
            <Title size={Size::Three}>{"Title 3"}</Title>
            <Title size={Size::Four}>{"Title 4"}</Title>
            <Title size={Size::Five}>{"Title 5"}</Title>
            <Title size={Size::Six}>{"Title 6"}</Title>
        </Block>
        <Block>
            <Subtitle size={Size::One}>{"Subtitle 1"}</Subtitle>
            <Subtitle size={Size::Two}>{"Subtitle 2"}</Subtitle>
            <Subtitle size={Size::Three}>{"Subtitle 3"}</Subtitle>
            <Subtitle size={Size::Four}>{"Subtitle 4"}</Subtitle>
            <Subtitle size={Size::Five}>{"Subtitle 5"}</Subtitle>
            <Subtitle size={Size::Six}>{"Subtitle 6"}</Subtitle>
        </Block>

        <hr />

        <Block>
            <Title size={Size::One}>{"Title 1"}</Title>
            <Subtitle size={Size::One}>{"Subtitle 1"}</Subtitle>
        </Block>
        <Block>
            <Title size={Size::Two}>{"Title 2"}</Title>
            <Subtitle size={Size::Two}>{"Subtitle 2"}</Subtitle>
        </Block>
        <Block>
            <Title size={Size::Three}>{"Title 3"}</Title>
            <Subtitle size={Size::Three}>{"Subtitle 3"}</Subtitle>
        </Block>

        <hr />

        <Block>
            <Title size={Size::One} spaced={true}>{"Title 1"}</Title>
            <Subtitle size={Size::Three}>{"Subtitle 3"}</Subtitle>
        </Block>

        <Block>
            <Title size={Size::Two} spaced={true}>{"Title 2"}</Title>
            <Subtitle size={Size::Four}>{"Subtitle 4"}</Subtitle>
        </Block>

        <Block>
            <Title size={Size::Three} spaced={true}>{"Title 3"}</Title>
            <Subtitle size={Size::Five}>{"Subtitle 5"}</Subtitle>
        </Block>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
