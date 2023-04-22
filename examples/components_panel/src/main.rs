use yew::prelude::*;
use yew_and_bulma::{
    components::{
        panel::{Panel, PanelBlock, PanelHeading, PanelIcon, PanelTabs},
        tabs::Tab,
    },
    elements::{block::Block, button::Button},
    helpers::color::Color,
    layout::{container::Container, section::Section},
};

#[function_component(App)]
fn app() -> Html {
    let tabs = vec![
        Tab("All".into(), true),
        Tab("Public".into(), false),
        Tab("Private".into(), false),
    ];

    html! {
        <Container>
            <Section>
                <Panel>
                    <PanelHeading>{"Repositories"}</PanelHeading>

                    <PanelTabs tabs={tabs.clone()} />

                    <PanelBlock active=true>
                        <PanelIcon class="material-symbols-outlined">{"folder_open"}</PanelIcon>
                        {"yew-and-bulma"}
                    </PanelBlock>

                    <PanelBlock>
                        <PanelIcon class="material-symbols-outlined">{"folder_open"}</PanelIcon>
                        {"bulma"}
                    </PanelBlock>

                    <PanelBlock>
                        <PanelIcon class="material-symbols-outlined">{"folder_open"}</PanelIcon>
                        {"yew"}
                    </PanelBlock>

                    <PanelBlock>
                        <Button fullwidth=true>{"Reset filters"}</Button>
                    </PanelBlock>
                </Panel>
                <Block>{"The above is a normal panel component, without anything special."}</Block>
            </Section>

            <Section>
                <Panel color={Color::Primary}>
                    <PanelHeading>{"Repositories"}</PanelHeading>

                    <PanelTabs tabs={tabs.clone()} />

                    <PanelBlock active=true>
                        <PanelIcon class="material-symbols-outlined">{"folder_open"}</PanelIcon>
                        {"yew-and-bulma"}
                    </PanelBlock>

                    <PanelBlock>
                        <PanelIcon class="material-symbols-outlined">{"folder_open"}</PanelIcon>
                        {"bulma"}
                    </PanelBlock>

                    <PanelBlock>
                        <PanelIcon class="material-symbols-outlined">{"folder_open"}</PanelIcon>
                        {"yew"}
                    </PanelBlock>

                    <PanelBlock>
                        <Button fullwidth=true>{"Reset filters"}</Button>
                    </PanelBlock>
                </Panel>
                <Panel color={Color::Success}>
                    <PanelHeading>{"Repositories"}</PanelHeading>

                    <PanelTabs tabs={tabs.clone()} />

                    <PanelBlock active=true>
                        <PanelIcon class="material-symbols-outlined">{"folder_open"}</PanelIcon>
                        {"yew-and-bulma"}
                    </PanelBlock>

                    <PanelBlock>
                        <PanelIcon class="material-symbols-outlined">{"folder_open"}</PanelIcon>
                        {"bulma"}
                    </PanelBlock>

                    <PanelBlock>
                        <PanelIcon class="material-symbols-outlined">{"folder_open"}</PanelIcon>
                        {"yew"}
                    </PanelBlock>

                    <PanelBlock>
                        <Button fullwidth=true>{"Reset filters"}</Button>
                    </PanelBlock>
                </Panel>
                <Panel color={Color::Link}>
                    <PanelHeading>{"Repositories"}</PanelHeading>

                    <PanelTabs tabs={tabs} />

                    <PanelBlock active=true>
                        <PanelIcon class="material-symbols-outlined">{"folder_open"}</PanelIcon>
                        {"yew-and-bulma"}
                    </PanelBlock>

                    <PanelBlock>
                        <PanelIcon class="material-symbols-outlined">{"folder_open"}</PanelIcon>
                        {"bulma"}
                    </PanelBlock>

                    <PanelBlock>
                        <PanelIcon class="material-symbols-outlined">{"folder_open"}</PanelIcon>
                        {"yew"}
                    </PanelBlock>

                    <PanelBlock>
                        <Button fullwidth=true>{"Reset filters"}</Button>
                    </PanelBlock>
                </Panel>

                <Block>{"The above are panels with various styles."}</Block>
            </Section>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
