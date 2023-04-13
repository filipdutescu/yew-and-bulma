use yew::prelude::*;
use yew_and_bulma::{
    components::breadcrumb::{Align, Breadcrumb, Crumb, Separator},
    elements::block::Block,
    layout::{container::Container, section::Section},
    utils::size::Size,
};

#[function_component(App)]
fn app() -> Html {
    let crumbs: Vec<_> = vec![
        Crumb(AttrValue::from("#"), html! { {"Yew and Bulma"} }),
        Crumb(AttrValue::from("#"), html! { {"Examples"} }),
        Crumb(AttrValue::from("#"), html! { {"Breadcrumb"} }),
        Crumb(AttrValue::from("#"), html! { {"Basic"} }),
    ];
    let crumbs_separator: Vec<_> = vec![
        Crumb(AttrValue::from("#"), html! { {"Yew and Bulma"} }),
        Crumb(AttrValue::from("#"), html! { {"Examples"} }),
        Crumb(AttrValue::from("#"), html! { {"Breadcrumb"} }),
        Crumb(AttrValue::from("#"), html! { {"Separator"} }),
    ];
    let crumbs_align: Vec<_> = vec![
        Crumb(AttrValue::from("#"), html! { {"Yew and Bulma"} }),
        Crumb(AttrValue::from("#"), html! { {"Examples"} }),
        Crumb(AttrValue::from("#"), html! { {"Breadcrumb"} }),
        Crumb(AttrValue::from("#"), html! { {"Alignment"} }),
    ];
    let crumbs_sizes: Vec<_> = vec![
        Crumb(AttrValue::from("#"), html! { {"Yew and Bulma"} }),
        Crumb(AttrValue::from("#"), html! { {"Examples"} }),
        Crumb(AttrValue::from("#"), html! { {"Breadcrumb"} }),
        Crumb(AttrValue::from("#"), html! { {"Sizes"} }),
    ];

    html! {
        <Container>
            <Section>
                <Breadcrumb {crumbs} />
                <Block>{"The above is a normal breadcrumb, without anything special."}</Block>
            </Section>

            <Section>
                <Breadcrumb separator={Separator::Arrow} crumbs={crumbs_separator.clone()} />
                <Breadcrumb separator={Separator::Bullet} crumbs={crumbs_separator.clone()} />
                <Breadcrumb separator={Separator::Dot} crumbs={crumbs_separator.clone()} />
                <Breadcrumb separator={Separator::Succeeds} crumbs={crumbs_separator} />

                <Block>{"The above are breadcrumbs with various separators."}</Block>
            </Section>

            <Section>
                <Breadcrumb align={Align::Center} crumbs={crumbs_align.clone()} />
                <Breadcrumb align={Align::Right} crumbs={crumbs_align} />

                <Block>{"The above are breadcrumbs with different alignments."}</Block>
            </Section>

            <Section>
                <Breadcrumb size={Size::Small} crumbs={crumbs_sizes.clone()} />
                <Breadcrumb size={Size::Medium} crumbs={crumbs_sizes.clone()} />
                <Breadcrumb size={Size::Large} crumbs={crumbs_sizes} />

                <Block>{"The above are breadcrumbs with different sizes."}</Block>
            </Section>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
