use yew::{function_component, html, Children, Html, Properties};

use crate::utils::{class::ClassBuilder, constants::IS_PREFIX};

#[derive(PartialEq)]
pub enum Size {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl From<&Size> for String {
    fn from(value: &Size) -> Self {
        match value {
            Size::One => "1",
            Size::Two => "2",
            Size::Three => "3",
            Size::Four => "4",
            Size::Five => "5",
            Size::Six => "6",
        }
        .to_owned()
    }
}

#[derive(Properties, PartialEq)]
pub struct TitleProperties {
    #[prop_or(Size::Three)]
    pub size: Size,
    #[prop_or_default]
    pub spaced: bool,
    pub children: Children,
}

impl From<&TitleProperties> for String {
    fn from(value: &TitleProperties) -> Self {
        let mut modifier_classes = String::with_capacity(if value.spaced { 20 } else { 10 });

        modifier_classes.push_str("title");
        modifier_classes.push_str(&format!(" {IS_PREFIX}-{}", String::from(&value.size)));
        if value.spaced {
            modifier_classes.push_str(" is-spaced");
        }

        modifier_classes
    }
}

#[function_component(Title)]
pub fn title(props: &TitleProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class(&String::from(props))
        .build();

    html! {
        <@{format!("h{}", String::from(&props.size))} {class}>{ for props.children.iter() }</@>
    }
}

#[derive(Properties, PartialEq)]
pub struct SubtitleProperties {
    #[prop_or(Size::Five)]
    pub size: Size,
    #[prop_or_default]
    pub spaced: bool,
    pub children: Children,
}

impl From<&SubtitleProperties> for String {
    fn from(value: &SubtitleProperties) -> Self {
        let mut modifier_classes = String::with_capacity(if value.spaced { 23 } else { 13 });

        modifier_classes.push_str("subtitle");
        modifier_classes.push_str(&format!(" {IS_PREFIX}-{}", String::from(&value.size)));
        if value.spaced {
            modifier_classes.push_str(" is-spaced");
        }

        modifier_classes
    }
}

#[function_component(Subtitle)]
pub fn subtitle(props: &SubtitleProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class(&String::from(props))
        .build();

    html! {
        <@{format!("h{}", String::from(&props.size))} {class}>{ for props.children.iter() }</@>
    }
}
