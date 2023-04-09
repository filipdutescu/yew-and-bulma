use web_sys::window;
use yew::prelude::*;
use yew_and_bulma::{
    elements::{
        block::Block,
        button::{Button, Buttons},
        icon::{Icon, IconText},
    },
    helpers::color::Color,
    layout::container::Container,
    utils::size::Size,
};

#[function_component(App)]
fn app() -> Html {
    let onclick = yew::Callback::from(move |_| {
        _ = window().unwrap().alert_with_message("hello");
    });
    html! {
        <Container>
            <Block>
                <Button {onclick}>{"Button"}</Button>
            </Block>

            <Block>
                <Buttons>
                    <Button color={Color::Primary}>{"Normal primary"}</Button>
                    <Button color={Color::Danger}>{"Normal danger"}</Button>
                    <Button color={Color::Link}>{"Normal link"}</Button>
                </Buttons>

                <Buttons>
                    <Button light=true color={Color::Primary}>{"Light primary"}</Button>
                    <Button light=true color={Color::Danger}>{"Light danger"}</Button>
                    <Button light=true color={Color::Link}>{"Light link"}</Button>
                </Buttons>

                <Buttons>
                    <Button disabled=true color={Color::Primary}>{"Disabled primary"}</Button>
                    <Button disabled=true color={Color::Danger}>{"Disabled danger"}</Button>
                    <Button disabled=true color={Color::Link}>{"Disabled link"}</Button>
                </Buttons>
            </Block>

            <Block>
                <Buttons>
                    <Button size={Size::Small}>{"Small"}</Button>
                    <Button size={Size::Normal}>{"Normal"}</Button>
                    <Button size={Size::Medium}>{"Medium"}</Button>
                    <Button size={Size::Large}>{"Large"}</Button>
                </Buttons>
            </Block>

            <Block>
                <Buttons>
                    <Button>
                        <IconText>
                            <Icon
                                text="Like"
                                icon={html!{
                                    <span class="material-symbols-outlined">{"thumb_up"}</span>
                                }} />
                        </IconText>
                    </Button>
                    <Button>
                        <IconText>
                            <Icon
                                text="Share"
                                icon={html!{
                                    <span class="material-symbols-outlined">{"share"}</span>
                                }} />
                        </IconText>
                    </Button>
                </Buttons>
            </Block>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
