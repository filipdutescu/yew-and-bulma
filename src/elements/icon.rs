use yew::{function_component, html, AttrValue, ChildrenWithProps, Classes, Html, Properties};

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
    pub class: Classes,
}

#[function_component(Icon)]
pub fn icon(props: &IconProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("icon")
        .with_text_color(props.color)
        .with_custom_class(&if let Some(size) = &props.size {
            format!("{IS_PREFIX}-{size}")
        } else {
            "".to_owned()
        })
        .build();

    html! {
        <>
        <span {class}>
            <i class={props.class.clone()}></i>
        </span>
        if !props.text.is_empty() {
            <span>{ &props.text }</span>
        }
        </>
    }
}
