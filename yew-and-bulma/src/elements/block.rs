use yew::html;
use yew::{function_component, Children, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

use crate::utils::class::ClassBuilder;
use crate::utils::BaseComponent;

/// Defines the properties of the [Bulma block element][bd].
///
/// Defines the properties of the block element, based on the specification
/// found in the [Bulma block element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::block::Block;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Block>{"This is some text in a block."}</Block>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/block/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct BlockProperties {
    /// The list of elements found inside the [block element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma block element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/block/
    pub children: Children,
}

/// Yew implementation of the [Bulma block element][bd].
///
/// Yew implementation of the block element, based on the specification found
/// in the [Bulma block element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::block::Block;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Block>{"This is some text in a block."}</Block>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/block/
#[function_component(Block)]
pub fn block(props: &BlockProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("block")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}
