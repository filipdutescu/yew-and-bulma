use yew::{function_component, html, Children, Html, Properties};

use crate::utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size};

/// Defines the properties of the [Bulma content element][bd].
///
/// Defines the properties of the content element, based on the specification
/// found in the [Bulma content element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::content::Content;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Content>
///             <h1>{"Article title"}</h1>
///
///             <p>{"Lorem ipsum..."}</p>
///         </Content>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/content/
#[derive(Properties, PartialEq)]
pub struct ContentProperties {
    /// Sets the size of the elements found inside the [content element][bd].
    ///
    /// Sets the size of the elements that will be found inside the
    /// [Bulma content element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::content::Content,
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Content size={Size::Small}>
    ///             <h1>{"Article title"}</h1>
    ///
    ///             <p>{"Lorem ipsum..."}</p>
    ///         </Content>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/content/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
    /// The list of elements found inside the [content element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma content element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/content/
    pub children: Children,
}

/// Yew implementation of the [Bulma content element][bd].
///
/// Yew implementation of the content element, based on the specification found
/// in the [Bulma content element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::content::Content;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Content>
///             <h1>{"Article title"}</h1>
///
///             <p>{"Lorem ipsum..."}</p>
///         </Content>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/content/
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
