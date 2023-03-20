use yew::{function_component, html, Children, Html, Properties};

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
    html! {
        <div class="box">
            { for props.children.iter() }
        </div>
    }
}
