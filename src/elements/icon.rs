use yew::{function_component, html, AttrValue, ChildrenWithProps, Html, Properties};

use crate::{
    helpers::color::TextColor,
    utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size},
};

#[derive(Properties, PartialEq)]
pub struct IconTextProperties {
    #[prop_or_default]
    pub flex: bool,
    #[prop_or_default]
    pub color: Option<TextColor>,
    pub children: ChildrenWithProps<Icon>,
}

#[function_component(IconText)]
pub fn icon(props: &IconTextProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("icon-text")
        .with_text_color(props.color)
        .build();

    html! {
        <@{(if props.flex { "div" } else { "span" }).to_string()} {class}>
            { for props.children.iter() }
        </@>
    }
}

#[derive(Properties, PartialEq)]
pub struct IconProperties {
    #[prop_or_default]
    pub text: AttrValue,
    #[prop_or_default]
    pub color: Option<TextColor>,
    #[prop_or_default]
    pub size: Option<Size>,
    pub icon: Html,
}

#[function_component(Icon)]
pub fn icon(props: &IconProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| {
            if Size::Normal == *size {
                "".to_owned()
            } else {
                format!("{IS_PREFIX}-{size}")
            }
        })
        .unwrap_or("".to_owned());
    let class = ClassBuilder::default()
        .with_custom_class("icon")
        .with_text_color(props.color)
        .with_custom_class(&size)
        .build();

    html! {
        <>
        <span {class}>
            { props.icon.clone() }
        </span>
        if !props.text.is_empty() {
            <span>{ &props.text }</span>
        }
        </>
    }
}
