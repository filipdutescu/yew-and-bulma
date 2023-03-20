use yew::{function_component, html, Children, Html, Properties};

use crate::{
    helpers::color::Color,
    utils::{
        class::ClassBuilder,
        constants::{ARE_PREFIX, IS_PREFIX},
        size::Size,
    },
};

#[derive(Properties, PartialEq)]
pub struct TagsProperties {
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub addons: bool,
    pub children: Children,
}

#[function_component(Tags)]
pub fn tags(props: &TagsProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| {
            if Size::Small == *size {
                "".to_owned()
            } else {
                format!("{ARE_PREFIX}-{size}")
            }
        })
        .unwrap_or("".to_owned());
    let addons = if props.addons { "has-addons" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("tags")
        .with_custom_class(&size)
        .with_custom_class(addons)
        .build();

    html! {
        <div {class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TagProperties {
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub light: Option<bool>,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub delete: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Tag)]
pub fn tag(props: &TagProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| {
            if Size::Small == *size {
                "".to_owned()
            } else {
                format!("{IS_PREFIX}-{size}")
            }
        })
        .unwrap_or("".to_owned());
    let rounded = if props.rounded { "is-rounded" } else { "" };
    let delete = if props.delete { "is-delete" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("tag")
        .with_color(props.color)
        .is_light(props.light)
        .with_custom_class(&size)
        .with_custom_class(rounded)
        .with_custom_class(delete)
        .build();
    let tag = (if props.delete { "a" } else { "span" }).to_string();

    html! {
        <@{tag} {class}>
            { for props.children.iter() }
        </@>
    }
}
