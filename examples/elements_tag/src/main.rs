use yew::prelude::*;
use yew_and_bulma::{
    elements::{
        block::Block,
        tag::{Tag, Tags},
    },
    helpers::color::Color,
    utils::size::Size,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <Block>
            <Tags>
                <Tag>{"Tag label"}</Tag>
                <Tag color={Color::Primary}>{"Tag label"}</Tag>
                <Tag light=true color={Color::Primary}>{"Tag label"}</Tag>
            </Tags>
        </Block>

        <Block>
            <Tags>
                <Tag size={Size::Normal} color={Color::Primary}>{"Tag label"}</Tag>
                <Tag size={Size::Medium} color={Color::Link}>{"Tag label"}</Tag>
                <Tag size={Size::Large} color={Color::Danger}>{"Tag label"}</Tag>
            </Tags>
        </Block>

        <Block>
            <Tags>
                <Tag rounded=true size={Size::Normal} color={Color::Primary}>{"Tag label"}</Tag>
                <Tag rounded=true size={Size::Medium} color={Color::Link}>{"Tag label"}</Tag>
                <Tag rounded=true size={Size::Large} color={Color::Danger}>{"Tag label"}</Tag>
            </Tags>
        </Block>

        <Block>
            <Tags>
                <Tag delete=true size={Size::Normal} color={Color::Primary} />
                <Tag delete=true size={Size::Medium} color={Color::Link} />
                <Tag delete=true size={Size::Large} color={Color::Danger} />
            </Tags>
        </Block>

        <Block>
            <Tags addons=true>
                <Tag>{"Hello"}</Tag>
                <Tag color={Color::Link}>{"world!"}</Tag>
            </Tags>
        </Block>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
