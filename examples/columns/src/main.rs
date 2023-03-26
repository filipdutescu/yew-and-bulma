use yew::prelude::*;
use yew_and_bulma::{
    columns::{Column, Columns, Size},
    elements::block::Block,
    elements::r#box::Box,
    helpers::{
        color::{BackgroundColor, TextColor},
        typography::{TextAlignment, TextSize, TextWeight},
        visibility::Viewport,
    },
    utils::class::ClassBuilder,
};

#[function_component(App)]
fn app() -> Html {
    let class = ClassBuilder::default()
        .with_background_color(Some(BackgroundColor::Primary))
        .with_text_color(Some(TextColor::White))
        .with_text_size(Some(TextSize::Five))
        .with_text_alignment(Some(TextAlignment::Centered))
        .with_text_weight(Some(TextWeight::Bold))
        .build();

    html! {
        <>
        <Block>
            <Columns>
                <Column>
                    <Box class={class.clone()}>{"First column"}</Box>
                </Column>

                <Column>
                    <Box class={class.clone()}>{"Second column"}</Box>
                </Column>

                <Column>
                    <Box class={class.clone()}>{"Third column"}</Box>
                </Column>

                <Column>
                    <Box class={class.clone()}>{"Fourth column"}</Box>
                </Column>
            </Columns>
        </Block>

        <Block>
            <Columns multiline=true gapless=true viewport={Viewport::Mobile}>
                <Column size={Size::OneThird}>
                    <Box class={class.clone()}>{"First column"}</Box>
                </Column>

                <Column size={Size::OneThird}>
                    <Box class={class.clone()}>{"Second column"}</Box>
                </Column>

                <Column size={Size::OneThird}>
                    <Box class={class.clone()}>{"Third column"}</Box>
                </Column>

                <Column size={Size::OneThird}>
                    <Box class={class.clone()}>{"Fourth column"}</Box>
                </Column>

                <Column size={Size::OneThird}>
                    <Box class={class.clone()}>{"Fifth column"}</Box>
                </Column>

                <Column size={Size::OneThird}>
                    <Box class={class.clone()}>{"Sixth column"}</Box>
                </Column>

                <Column size={Size::OneThird}>
                    <Box class={class.clone()}>{"Seventh column"}</Box>
                </Column>

                <Column size={Size::OneThird}>
                    <Box class={class.clone()}>{"Eighth column"}</Box>
                </Column>

                <Column size={Size::OneThird}>
                    <Box class={class.clone()}>{"Ninth column"}</Box>
                </Column>
            </Columns>
        </Block>

        <Block>
            <Columns centered=true multiline=true center_vertically=true>
                <Column size={Size::Half}>
                    <Box class={class.clone()}>{"Short column"}</Box>
                </Column>

                <Column size={Size::Half}>
                    <Box class={class.clone()}>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}</Box>
                </Column>

                <Column size={Size::TwoFifths}>
                    <Box class={class.clone()}>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}</Box>
                </Column>

                <Column size={Size::TwoFifths}>
                    <Box class={class.clone()}>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}</Box>
                </Column>
            </Columns>
        </Block>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
