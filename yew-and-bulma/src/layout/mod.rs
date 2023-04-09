/// Provides utilities for creating [container elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma container elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::container::Container;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Container>{"This is some text in a container."}</Container>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/container/
pub mod container;
/// Provides utilities for creating [section elements][bd] in Yew.
///
/// Defines the necessary components to build, style and modify
/// [Bulma section elements][bd] in Yew.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::section::Section;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Section>{"This is some text in a section."}</Section>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/section/
pub mod section;
