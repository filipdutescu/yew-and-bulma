use yew::html;
use yew::{function_component, Children, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

use crate::utils::class::ClassBuilder;
use crate::utils::BaseComponent;

/// Defines the properties of the [Bulma box element][bd].
///
/// Defines the properties of the box element, based on the specification found
/// in the [Bulma box element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::r#box::Box;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Box>{"This is some text in a box."}</Box>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/box/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct BoxProperties {
    /// The list of elements found inside the [box element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma box element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/box/
    pub children: Children,
}

/// Yew implementation of the [Bulma box element][bd].
///
/// Yew implementation of the box element, based on the specification found in
/// the [Bulma box element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::r#box::Box;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Box>{"This is some text in a box."}</Box>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/box/
#[function_component(Box)]
pub fn block(props: &BoxProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("box")
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}
