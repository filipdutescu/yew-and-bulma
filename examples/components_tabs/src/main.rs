use yew::prelude::*;
use yew_and_bulma::{
    components::tabs::{Align, Style, Tab, Tabs},
    elements::block::Block,
    layout::{container::Container, section::Section},
    utils::size::Size,
};

#[function_component(App)]
fn app() -> Html {
    let tabs: Vec<_> = vec![
        Tab(html! { {"Pictures"} }, true),
        Tab(html! { {"Music"} }, false),
        Tab(html! { {"Videos"} }, false),
        Tab(html! { {"Documents"} }, false),
    ];

    html! {
        <Container>
            <Section>
                <Tabs tabs={tabs.clone()} />
                <Block>{"The above is a normal tabs component, without anything special."}</Block>
            </Section>

            <Section>
                <Tabs style={Style::Boxed} tabs={tabs.clone()} />
                <Tabs style={Style::Toggle} tabs={tabs.clone()} />
                <Tabs style={Style::ToggleRounded} tabs={tabs.clone()} />

                <Block>{"The above are tabs with various styles."}</Block>
            </Section>

            <Section>
                <Tabs align={Align::Center} tabs={tabs.clone()} />
                <Tabs align={Align::Right} tabs={tabs.clone()} />

                <Block>{"The above are tabs with different alignments."}</Block>
            </Section>

            <Section>
                <Tabs size={Size::Small} tabs={tabs.clone()} />
                <Tabs size={Size::Medium} tabs={tabs.clone()} />
                <Tabs size={Size::Large} tabs={tabs.clone()} />

                <Block>{"The above are tabs with different sizes."}</Block>
            </Section>

            <Section>
                <Tabs fullwidth=true {tabs} />
                <Block>{"The above is a fullwidth tabs component."}</Block>
            </Section>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
