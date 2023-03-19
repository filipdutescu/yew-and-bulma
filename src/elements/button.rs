use yew::{function_component, html, Children, Classes, Html, Properties};

use crate::{
    helpers::color::Color,
    utils::class::ClassBuilder,
    utils::constants::{ARE_PREFIX, IS_PREFIX},
    utils::size::Size as ButtonsSize,
};

impl From<&ButtonsSize> for String {
    fn from(value: &ButtonsSize) -> Self {
        format!("{ARE_PREFIX}-{value}")
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonsProperties {
    #[prop_or_default]
    pub size: Option<ButtonsSize>,
    pub children: Children,
}

#[function_component(Buttons)]
pub fn buttons(props: &ButtonsProperties) -> Html {
    let size = &props
        .size
        .as_ref()
        .map(String::from)
        .unwrap_or("".to_owned());
    let class = ClassBuilder::default()
        .with_custom_class("buttons")
        .with_custom_class(size)
        .build();

    html! {
        <div {class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(PartialEq)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
}

impl From<&Size> for String {
    fn from(value: &Size) -> Self {
        match value {
            Size::Small => format!("{IS_PREFIX}-small"),
            Size::Normal => format!("{IS_PREFIX}-normal"),
            Size::Medium => format!("{IS_PREFIX}-medium"),
            Size::Large => format!("{IS_PREFIX}-large"),
        }
    }
}

#[derive(PartialEq)]
pub enum Style {
    Outlined,
    Inverted,
    InvertedOutlined,
    Rounded,
}

impl From<&Style> for String {
    fn from(value: &Style) -> Self {
        match value {
            Style::Outlined => format!("{IS_PREFIX}-outlined"),
            Style::Inverted => format!("{IS_PREFIX}-inverted"),
            Style::InvertedOutlined => format!("{IS_PREFIX}-inverted {IS_PREFIX}-outlined"),
            Style::Rounded => format!("{IS_PREFIX}-rounded"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProperties {
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub light: Option<bool>,
    #[prop_or(Size::Normal)]
    pub size: Size,
    #[prop_or_default]
    pub responsive: bool,
    #[prop_or_default]
    pub fullwidth: bool,
    #[prop_or_default]
    pub style: Option<Style>,
    pub children: Children,
}

impl From<&ButtonProperties> for Classes {
    fn from(value: &ButtonProperties) -> Self {
        let style = if let Some(style) = &value.style {
            String::from(style)
        } else {
            "".to_owned()
        };
        let fullwidth = if value.fullwidth {
            format!("{IS_PREFIX}-fullwidth")
        } else {
            "".to_owned()
        };
        let responsive = if value.responsive {
            format!("{IS_PREFIX}-responsive")
        } else {
            "".to_string()
        };

        ClassBuilder::default()
            .with_custom_class("button")
            .with_color(value.color)
            .is_light(value.light)
            .with_custom_class(&String::from(&value.size))
            .with_custom_class(&responsive)
            .with_custom_class(&fullwidth)
            .with_custom_class(&style)
            .build()
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let class: Classes = props.into();

    html! {
        <button {class}>{ for props.children.iter() }</button>
    }
}
