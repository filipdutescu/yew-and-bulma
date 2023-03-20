use yew::{function_component, html, Children, Html, Properties};

use crate::utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size};

#[derive(Properties, PartialEq)]
pub struct ContentProperties {
    #[prop_or_default]
    pub size: Option<Size>,
    pub children: Children,
}

#[function_component(Content)]
pub fn content(props: &ContentProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| format!("{IS_PREFIX}-{size}"))
        .unwrap_or("".to_owned());
    let class = ClassBuilder::default()
        .with_custom_class("content")
        .with_custom_class(&size)
        .build();

    html! {
        <div {class}>
            { for props.children.iter() }
        </div>
    }
}
