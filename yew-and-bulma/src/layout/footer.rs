use yew::html;
use yew::{function_component, Children, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

use crate::utils::class::ClassBuilder;
use crate::utils::BaseComponent;

/// Defines the points from which a [footer element][bd] is not full width.
///
/// Defines the points from which a [Bulma footer element][bd] no longer has
/// full width, but rather a fixed one. Those points are viewports from which
/// the component is "active", concretely, space around the children is
/// visible.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::footer::Footer;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Footer>
///             {"This is some text in a footer."}
///         </Footer>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/footer/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct FooterProperties {
    /// The list of elements found inside the [footer element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma footer element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/footer/
    pub children: Children,
}

/// Yew implementation of the [Bulma footer element][bd].
///
/// Yew implementation of the footer element, based on the specification
/// found in the [Bulma footer element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::footer::Footer;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Footer>
///             {"This is some text in a footer."}
///         </Footer>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/footer/
#[function_component(Footer)]
pub fn footer(props: &FooterProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("footer")
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
