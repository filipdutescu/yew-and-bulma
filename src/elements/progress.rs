use yew::{function_component, html, Html, Properties};

use crate::{
    helpers::color::Color,
    utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size},
};

#[derive(Properties, PartialEq)]
pub struct ProgressBarProperties {
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub value: Option<f64>,
    #[prop_or(100.0)]
    pub max: f64,
}

#[function_component(ProgressBar)]
pub fn progress_bar(props: &ProgressBarProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| format!("{IS_PREFIX}-{size}"))
        .unwrap_or("".to_owned());
    let class = ClassBuilder::default()
        .with_custom_class("progress")
        .with_color(props.color)
        .with_custom_class(&size)
        .build();

    html! {
        <progress {class} value={props.value.map(|n| n.to_string())} max={props.max.to_string()}>{ props.value.unwrap_or(15.0) }{"%"}</progress>
    }
}
