use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::utils::class::ClassBuilder;

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
#[derive(Properties, PartialEq)]
pub struct BoxProperties {
    /// Sets the [HTML id attribute][id] of the element.
    ///
    /// Sets the [HTML id attrbiute][id] of the element which will receive
    /// these properties.
    ///
    /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Sets the classes to be appended to the [HTML classattribute][class].
    ///
    /// Sets the classes to be appended to [HTML class attrbiute][class] of the
    /// element which will receive these properties.
    ///
    /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class
    #[prop_or_default]
    pub class: Option<Classes>,
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
        <div id={props.id.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
