use yew::prelude::*;
use yew_and_bulma::{
    elements::{
        block::Block,
        icon::{Icon, IconText},
    },
    helpers::color::TextColor,
    utils::size::Size,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <Block>
            <Icon
                icon={html! {
                    <span class="material-symbols-outlined">{"home"}</span>
                }} />
        </Block>

        <Block>
            <IconText>
                <Icon
                    icon={html! {
                        <span class="material-symbols-outlined">{"home"}</span>
                    }}
                    text="Home" />
            </IconText>
        </Block>

        <Block>
            <IconText>
                <Icon
                    icon={html! {
                        <span class="material-symbols-outlined">{"home"}</span>
                    }}
                    text="Home" />
                <Icon
                    icon={html! {
                        <span class="material-symbols-outlined">{"arrow_forward"}</span>
                    }} />
                <Icon
                    icon={html! {
                        <span class="material-symbols-outlined">{"book"}</span>
                    }}
                    text="Blog" />
                <Icon
                    icon={html! {
                        <span class="material-symbols-outlined">{"arrow_forward"}</span>
                    }} />
                <Icon
                    icon={html! {
                        <span class="material-symbols-outlined">{"subject"}</span>
                    }}
                    text="Article" />
            </IconText>
        </Block>

        <Block>
            <p>{"You call "}
                <IconText>
                    <Icon
                        icon={html! {
                            <span class="material-symbols-outlined">{"home"}</span>
                        }}
                        text="home" />
                </IconText>
            {" the place where the "}
                <IconText>
                    <Icon
                        icon={html! {
                            <span class="material-symbols-outlined">{"favorite"}</span>
                        }}
                        text="heart"/>
                </IconText>
            {" is."}
            </p>
        </Block>

        <Block>
            <IconText flex=true>
                <Icon
                    color={TextColor::Info}
                    icon={html! {
                        <span class="material-symbols-outlined">{"info"}</span>
                    }}
                    text="Information" />
            </IconText>
            {"Some very informative text underneath."}

            <IconText flex=true>
                <Icon
                    color={TextColor::Primary}
                    icon={html! {
                        <span class="material-symbols-outlined">{"check"}</span>
                    }}
                    text="Primary"/>
            </IconText>
            {"Some very informative text underneath."}
        </Block>

        <Block>
            <IconText color={TextColor::Info}>
                <Icon
                    color={TextColor::Info}
                    icon={html! {
                        <span class="material-symbols-outlined">{"info"}</span>
                    }}
                    text="Information" />
            </IconText>
            <Block>{"Some very informative text underneath."}</Block>

            <IconText color={TextColor::Primary}>
                <Icon
                    color={TextColor::Primary}
                    icon={html! {
                        <span class="material-symbols-outlined">{"check"}</span>
                    }}
                    text="Primary"/>
            </IconText>
            <Block>{"Some very informative text underneath."}</Block>
        </Block>

        <Block>
            <Icon
                color={TextColor::Info}
                size={Size::Small}
                icon={html! {
                    <span class="material-symbols-outlined">{"info"}</span>
                }}
                text="Small" />
            <Icon
                color={TextColor::Primary}
                size={Size::Small}
                icon={html! {
                    <span class="material-symbols-outlined">{"check"}</span>
                }}
                text="Small"/>
            <Icon
                color={TextColor::Info}
                size={Size::Large}
                icon={html! {
                    <span class="material-symbols-outlined">{"info"}</span>
                }}
                text="Large" />
            <Icon
                color={TextColor::Primary}
                size={Size::Large}
                icon={html! {
                    <span class="material-symbols-outlined">{"check"}</span>
                }}
                text="Large"/>
        </Block>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
