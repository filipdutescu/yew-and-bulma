use yew::html;
use yew::{function_component, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

use crate::utils::BaseComponent;
use crate::utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size};

/// Defines the properties of the [Bulma delete element][bd].
///
/// Defines the properties of the delete element, based on the specification
/// found in the [Bulma delete element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::delete::Delete;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Delete />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/delete/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct DeleteProperties {
    /// Sets the size of the [Bulma delete element][bd].
    ///
    /// Sets the size of the [Bulma delete element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::delete::Delete,
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Delete size={Size::Large} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/delete/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
}

/// Yew implementation of the [Bulma delete element][bd].
///
/// Yew implementation of the delete element, based on the specification found
/// in the [Bulma delete element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::delete::Delete;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Delete />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/delete/
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
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="button" {class} ..props.into() />
    }
}
