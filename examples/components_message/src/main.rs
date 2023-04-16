use yew::prelude::*;
use yew_and_bulma::{
    components::message::{Message, MessageBody, MessageHeader},
    elements::block::Block,
    helpers::color::Color,
    layout::{container::Container, section::Section},
    utils::size::Size,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <Section>
                <Message>
                    <MessageHeader>{"Hello!"}</MessageHeader>

                    <MessageBody>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing
                        elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
                        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex
                        ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit
                        esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </MessageBody>
                </Message>
                <Block>{"The above is a normal pagination component, without anything special."}</Block>
            </Section>

            <Section>
                <Message color={Color::Primary}>
                    <MessageHeader>{"Hello!"}</MessageHeader>

                    <MessageBody>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing
                        elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
                        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex
                        ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit
                        esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </MessageBody>
                </Message>
                <Message color={Color::Info}>
                    <MessageHeader>{"Hello!"}</MessageHeader>

                    <MessageBody>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing
                        elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
                        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex
                        ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit
                        esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </MessageBody>
                </Message>
                <Message color={Color::Warning}>
                    <MessageHeader delete=false>{"Hello!"}</MessageHeader>

                    <MessageBody>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing
                        elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
                        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex
                        ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit
                        esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </MessageBody>
                </Message>
                <Message color={Color::Danger}>
                    <MessageBody>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing
                        elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
                        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex
                        ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit
                        esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </MessageBody>
                </Message>

                <Block>{"The above are pagination with various styles."}</Block>
            </Section>

            <Section>
                <Message size={Size::Small}>
                    <MessageHeader>{"Hello!"}</MessageHeader>

                    <MessageBody>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing
                        elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
                        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex
                        ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit
                        esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </MessageBody>
                </Message>
                <Message size={Size::Medium}>
                    <MessageHeader>{"Hello!"}</MessageHeader>

                    <MessageBody>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing
                        elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
                        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex
                        ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit
                        esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </MessageBody>
                </Message>
                <Message size={Size::Large}>
                    <MessageHeader>{"Hello!"}</MessageHeader>

                    <MessageBody>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing
                        elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
                        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex
                        ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit
                        esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </MessageBody>
                </Message>

                <Block>{"The above are pagination with different sizes."}</Block>
            </Section>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
