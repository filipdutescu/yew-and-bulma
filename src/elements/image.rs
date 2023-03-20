use yew::{function_component, html, AttrValue, Children, Html, Properties};

use crate::utils::{class::ClassBuilder, constants::IS_PREFIX};

#[derive(Properties, PartialEq)]
pub struct ImageProperties {
    #[prop_or_default]
    pub fullwidth: bool,
    #[prop_or_default]
    pub rounded: bool,
    pub src: AttrValue,
}

#[function_component(Image)]
pub fn image(props: &ImageProperties) -> Html {
    let fullwidth = if props.fullwidth { "is-fullwidth" } else { "" };
    let rounded = if props.rounded { "is-rounded" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class(fullwidth)
        .with_custom_class(rounded)
        .build();

    html! {
        <img {class} src={props.src.clone()} />
    }
}

#[derive(PartialEq)]
pub enum Size {
    Pixels16x16,
    Pixels24x24,
    Pixels32x32,
    Pixels48x48,
    Pixels64x64,
    Pixels96x96,
    Pixels128x128,
    RatioSquare,
    Ratio1x1,
    Ratio5x4,
    Ratio4x3,
    Ratio3x2,
    Ratio5x3,
    Ratio16x9,
    Ratio2x1,
    Ratio3x1,
    Ratio4x5,
    Ratio3x4,
    Ratio2x3,
    Ratio3x5,
    Ratio9x16,
    Ratio1x2,
    Ratio1x3,
}

impl From<&Size> for String {
    fn from(value: &Size) -> Self {
        match value {
            Size::Pixels16x16 => format!("{IS_PREFIX}-16x16"),
            Size::Pixels24x24 => format!("{IS_PREFIX}-24x24"),
            Size::Pixels32x32 => format!("{IS_PREFIX}-32x32"),
            Size::Pixels48x48 => format!("{IS_PREFIX}-48x48"),
            Size::Pixels64x64 => format!("{IS_PREFIX}-64x64"),
            Size::Pixels96x96 => format!("{IS_PREFIX}-96x96"),
            Size::Pixels128x128 => format!("{IS_PREFIX}-128x128"),
            Size::RatioSquare => format!("{IS_PREFIX}-square"),
            Size::Ratio1x1 => format!("{IS_PREFIX}-1by1"),
            Size::Ratio5x4 => format!("{IS_PREFIX}-5by4"),
            Size::Ratio4x3 => format!("{IS_PREFIX}-4by3"),
            Size::Ratio3x2 => format!("{IS_PREFIX}-3by2"),
            Size::Ratio5x3 => format!("{IS_PREFIX}-5by3"),
            Size::Ratio16x9 => format!("{IS_PREFIX}-16by9"),
            Size::Ratio2x1 => format!("{IS_PREFIX}-2by1"),
            Size::Ratio3x1 => format!("{IS_PREFIX}-3by1"),
            Size::Ratio4x5 => format!("{IS_PREFIX}-4by5"),
            Size::Ratio3x4 => format!("{IS_PREFIX}-3by4"),
            Size::Ratio2x3 => format!("{IS_PREFIX}-2by3"),
            Size::Ratio3x5 => format!("{IS_PREFIX}-3by5"),
            Size::Ratio9x16 => format!("{IS_PREFIX}-9by16"),
            Size::Ratio1x2 => format!("{IS_PREFIX}-1by2"),
            Size::Ratio1x3 => format!("{IS_PREFIX}-1by3"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct FigureProperties {
    #[prop_or_default]
    pub size: Option<Size>,
    pub children: Children,
}

#[function_component(Figure)]
pub fn figure(props: &FigureProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(String::from)
        .unwrap_or("".to_owned());
    let class = ClassBuilder::default()
        .with_custom_class("image")
        .with_custom_class(&size)
        .build();

    html! {
        <figure {class}>
            { for props.children.iter() }
        </figure>
    }
}
