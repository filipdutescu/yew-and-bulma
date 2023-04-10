use yew::prelude::*;
use yew_and_bulma::{
    elements::{content::Content, delete::Delete},
    layout::{
        container::Container,
        media::{Media, MediaContent, MediaLeft, MediaRight},
    },
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <Media>
                <MediaLeft>
                    <Content>{"Avatar should go here"}</Content>
                </MediaLeft>

                <MediaContent>
                    <Content>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing
                        elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
                        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex
                        ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit
                        esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </Content>
                </MediaContent>

                <MediaRight>
                    <Delete />
                </MediaRight>
            </Media>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
