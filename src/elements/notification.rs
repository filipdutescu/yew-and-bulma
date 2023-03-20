use yew::{function_component, html, Children, Html, Properties};

use crate::{elements::delete::Delete, helpers::color::Color, utils::class::ClassBuilder};

#[derive(Properties, PartialEq)]
pub struct NotificationProperties {
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub light: Option<bool>,
    #[prop_or(true)]
    pub delete_button: bool,
    pub children: Children,
}

#[function_component(Notification)]
pub fn notification(props: &NotificationProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("notification")
        .with_color(props.color)
        .is_light(props.light)
        .build();

    html! {
        <div {class}>
            if props.delete_button {
                <Delete />
            }
            { for props.children.iter() }
        </div>
    }
}
