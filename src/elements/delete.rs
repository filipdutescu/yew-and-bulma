use yew::{function_component, html, Html, Properties};

use crate::utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size};

#[derive(Properties, PartialEq)]
pub struct DeleteProperties {
    #[prop_or_default]
    pub size: Option<Size>,
}

#[function_component(Delete)]
pub fn delete(props: &DeleteProperties) -> Html {
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
        .with_custom_class("delete")
        .with_custom_class(&size)
        .build();

    html! {
        <button {class}></button>
    }
}
