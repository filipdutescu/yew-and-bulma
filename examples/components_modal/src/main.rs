use yew::prelude::*;
use yew_and_bulma::{
    components::modal::{
        Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
        ModalCardTitle, ModalClose, ModalContent,
    },
    elements::{block::Block, button::Button, delete::Delete, r#box::Box},
    helpers::color::Color,
    layout::{container::Container, section::Section},
    utils::size::Size,
};

#[function_component(App)]
fn app() -> Html {
    let modals = use_state(|| vec![false, false, false]);
    let no_modals = modals.len();
    let onclick: Vec<_> = (0..no_modals)
        .map(|i| {
            let (open_modals, close_modals) = (modals.clone(), modals.clone());
            let open = Callback::from(move |_| {
                let mut new_modals = Vec::with_capacity(no_modals);
                for j in 0..no_modals {
                    new_modals.push(
                        open_modals
                            .get(j)
                            .map(|m| if j == i { true } else { *m })
                            .unwrap_or(false),
                    );
                }

                open_modals.set(new_modals);
            });
            let close = Callback::from(move |_| {
                let mut new_modals = Vec::with_capacity(no_modals);
                for j in 0..no_modals {
                    new_modals.push(
                        close_modals
                            .get(j)
                            .map(|m| if j == i { false } else { *m })
                            .unwrap_or(false),
                    );
                }

                close_modals.set(new_modals);
            });

            (open, close)
        })
        .collect();

    html! {
        <Container>
            <Section>
                <Button onclick={&onclick[0].0}>{"Launch modal"}</Button>
                <Modal active={(*modals).first().unwrap_or(&false)}>
                    <ModalBackground onclick={&onclick[0].1} />
                    <ModalContent>
                        <Box><p>{"Hello, I'm a nice modal you can use!"}</p></Box>
                    </ModalContent>
                    <ModalClose size={Size::Large} onclick={&onclick[0].1} />
                </Modal>
                <Block>{"The above is a modal component."}</Block>
            </Section>

            <Section>
                <Button onclick={&onclick[1].0}>{"Launch modal"}</Button>
                <Modal active={(*modals).get(1).unwrap_or(&false)}>
                    <ModalBackground onclick={&onclick[1].1} />
                    <ModalContent>
                        <Box><p>{"Hello, I'm a nice modal you can use!"}</p></Box>
                    </ModalContent>
                </Modal>
                <Block>{"The above is a modal component without the close button."}</Block>
            </Section>

            <Section>
                <Button onclick={&onclick[2].0}>{"Launch modal"}</Button>
                <Modal active={(*modals).last().unwrap_or(&false)}>
                    <ModalBackground onclick={&onclick[2].1} />
                    <ModalCard>
                        <ModalCardHead>
                            <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
                            <Delete onclick={&onclick[2].1} />
                        </ModalCardHead>

                        <ModalCardBody>
                            <p>{"Hello, I'm a nice modal you can use!"}</p>
                        </ModalCardBody>

                        <ModalCardFoot>
                            <Button color={Color::Success} onclick={&onclick[2].1}>{"Greet back"}</Button>
                            <Button onclick={&onclick[2].1}>{"Ignore"}</Button>
                        </ModalCardFoot>
                    </ModalCard>
                </Modal>

                <Block>{"The above is a modal card component."}</Block>
            </Section>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
