use yew::prelude::*;
use yew_and_bulma::{
    components::dropdown::{
        Align, Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu,
        DropdownTrigger,
    },
    elements::{block::Block, button::Button, icon::Icon},
    layout::{container::Container, section::Section},
};

#[function_component(App)]
fn app() -> Html {
    let actives = use_state(|| vec![false, false, false]);
    let no_actives = actives.len();
    let onclicks: Vec<_> = (0..no_actives)
        .map(|i| {
            let actives = actives.clone();

            Callback::from(move |_| {
                let mut new_actives = Vec::with_capacity(no_actives);
                for j in 0..no_actives {
                    new_actives.push(
                        actives
                            .get(j)
                            .map(|a| if j == i { !*a } else { *a })
                            .unwrap_or(false),
                    );
                }

                actives.set(new_actives);
            })
        })
        .collect();

    html! {
        <Container>
            <Section>
                <Dropdown active={actives[0]} onclick={onclicks[0].clone()}>
                    <DropdownTrigger>
                        <Button>
                            <span>{"Dropdown toggle"}</span>
                            <Icon icon={html!{
                                <span class="material-symbols-outlined">{"expand_more"}</span>
                            }} />
                        </Button>
                    </DropdownTrigger>

                    <DropdownMenu>
                        <DropdownContent>
                            <DropdownItem>{"One dropdown item"}</DropdownItem>
                            <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
                            <DropdownItem>{"Another dropdown item"}</DropdownItem>

                            <DropdownDivider />

                            <DropdownItem>{"A separate item"}</DropdownItem>
                        </DropdownContent>
                    </DropdownMenu>
                </Dropdown>
                <Block>{"The above is a normal pagination component, without anything special."}</Block>
            </Section>

            <Section>
                <Block>
                    <Dropdown hoverable=true>
                        <DropdownTrigger>
                            <Button>
                                <span>{"Hoverable dropdown toggle"}</span>
                                <Icon icon={html!{
                                    <span class="material-symbols-outlined">{"expand_more"}</span>
                                }} />
                            </Button>
                        </DropdownTrigger>

                        <DropdownMenu>
                            <DropdownContent>
                                <DropdownItem>{"One dropdown item"}</DropdownItem>
                                <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
                                <DropdownItem>{"Another dropdown item"}</DropdownItem>

                                <DropdownDivider />

                                <DropdownItem>{"A separate item"}</DropdownItem>
                            </DropdownContent>
                        </DropdownMenu>
                    </Dropdown>
                </Block>
                <Block>
                    <Dropdown active={actives[1]} up=true onclick={onclicks[1].clone()}>
                        <DropdownTrigger>
                            <Button>
                                <span>{"Dropup toggle"}</span>
                                <Icon icon={html!{
                                    <span class="material-symbols-outlined">{"expand_more"}</span>
                                }} />
                            </Button>
                        </DropdownTrigger>

                        <DropdownMenu>
                            <DropdownContent>
                                <DropdownItem>{"One dropdown item"}</DropdownItem>
                                <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
                                <DropdownItem>{"Another dropdown item"}</DropdownItem>

                                <DropdownDivider />

                                <DropdownItem>{"A separate item"}</DropdownItem>
                            </DropdownContent>
                        </DropdownMenu>
                    </Dropdown>
                </Block>
                <Block>
                    <Dropdown active={actives[2]} align={Align::Right} onclick={onclicks[2].clone()}>
                        <DropdownTrigger>
                            <Button>
                                <span>{"Right-aligned dropdown toggle"}</span>
                                <Icon icon={html!{
                                    <span class="material-symbols-outlined">{"expand_more"}</span>
                                }} />
                            </Button>
                        </DropdownTrigger>

                        <DropdownMenu>
                            <DropdownContent>
                                <DropdownItem>{"One dropdown item"}</DropdownItem>
                                <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
                                <DropdownItem>{"Another dropdown item"}</DropdownItem>

                                <DropdownDivider />

                                <DropdownItem>{"A separate item"}</DropdownItem>
                            </DropdownContent>
                        </DropdownMenu>
                    </Dropdown>
                </Block>

                <Block>{"The above are pagination with various modifiers."}</Block>
            </Section>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
