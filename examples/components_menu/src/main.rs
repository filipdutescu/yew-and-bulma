use yew::prelude::*;
use yew_and_bulma::{
    components::menu::{Menu, MenuLabel, MenuList},
    elements::block::Block,
    layout::container::Container,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <Menu>
                <MenuLabel>{"General"}</MenuLabel>
                <MenuList>
                    <a class="is-active">{"Dashboard"}</a>
                    <a>{"About"}</a>
                </MenuList>

                <MenuLabel>{"Project"}</MenuLabel>
                <MenuList>
                    <>
                        <a>{"Team"}</a>
                        <ul>
                            <li><a>{"Members"}</a></li>
                            <li><a>{"Governance"}</a></li>
                        </ul>
                    </>
                    <a>{"Technologies"}</a>
                    <a>{"Blog"}</a>
                </MenuList>
            </Menu>
            <Block>{"The above is a normal menu component."}</Block>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
