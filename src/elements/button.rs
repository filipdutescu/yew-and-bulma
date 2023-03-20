use yew::{function_component, html, Children, Classes, Html, Properties};

use crate::{
    helpers::color::Color,
    utils::size::Size,
    utils::{
        class::ClassBuilder,
        constants::{ARE_PREFIX, IS_PREFIX},
    },
};

#[derive(Default, PartialEq)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Properties, PartialEq)]
pub struct ButtonsProperties {
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub addons: bool,
    #[prop_or_default]
    pub align: Align,
    pub children: Children,
}

impl From<&Align> for String {
    fn from(value: &Align) -> Self {
        match value {
            Align::Left => "".to_owned(),
            Align::Center => format!("{IS_PREFIX}-centered"),
            Align::Right => format!("{IS_PREFIX}-right"),
        }
    }
}

#[function_component(Buttons)]
pub fn buttons(props: &ButtonsProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| {
            if Size::Normal == *size {
                "".to_owned()
            } else {
                format!("{ARE_PREFIX}-{size}")
            }
        })
        .unwrap_or("".to_owned());
    let addons = if props.addons { "has-addons" } else { "" }.to_owned();
    let class = ClassBuilder::default()
        .with_custom_class("buttons")
        .with_custom_class(&size)
        .with_custom_class(&addons)
        .with_custom_class(&String::from(&props.align))
        .build();

    html! {
        <div {class}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(PartialEq)]
pub enum State {
    Normal,
    Hover,
    Focus,
    Active,
    Loading,
    Static,
}

impl From<&State> for String {
    fn from(value: &State) -> Self {
        let state = match value {
            State::Normal => "normal",
            State::Hover => "hover",
            State::Focus => "focus",
            State::Active => "active",
            State::Loading => "loading",
            State::Static => "static",
        };

        format!("{IS_PREFIX}-{state}")
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
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub responsive: bool,
    #[prop_or_default]
    pub fullwidth: bool,
    #[prop_or_default]
    pub style: Option<Style>,
    #[prop_or_default]
    pub state: Option<State>,
    #[prop_or_default]
    pub disabled: bool,
    pub children: Children,
}

impl From<&ButtonProperties> for Classes {
    fn from(value: &ButtonProperties) -> Self {
        let size = value
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
        let style = value
            .style
            .as_ref()
            .map(String::from)
            .unwrap_or("".to_string());
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
        let state = value
            .state
            .as_ref()
            .map(String::from)
            .unwrap_or("".to_owned());

        ClassBuilder::default()
            .with_custom_class("button")
            .with_color(value.color)
            .is_light(value.light)
            .with_custom_class(&size)
            .with_custom_class(&responsive)
            .with_custom_class(&fullwidth)
            .with_custom_class(&style)
            .with_custom_class(&state)
            .build()
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let class: Classes = props.into();

    html! {
        <button {class} disabled={props.disabled}>{ for props.children.iter() }</button>
    }
}
