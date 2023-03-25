use yew::{function_component, html, AttrValue, Classes, Html, Properties};

use crate::{
    helpers::color::Color,
    utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size},
};

/// Defines the properties of the [Bulma progress bar element][bd].
///
/// Defines the properties of the progress bar element, based on the
/// specification found in the [Bulma progress bar element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::progress::ProgressBar;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <ProgressBar value={15.0} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/progress/
#[derive(Properties, PartialEq)]
pub struct ProgressBarProperties {
    /// Sets the [HTML id attribute][id] of the element.
    ///
    /// Sets the [HTML id attrbiute][id] of the element which will receive
    /// these properties.
    ///
    /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Sets the classes to be appended to the [HTML class attribute][class].
    ///
    /// Sets the classes to be appended to [HTML class attrbiute][class] of the
    /// element which will receive these properties.
    ///
    /// [class]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class
    #[prop_or_default]
    pub class: Option<Classes>,
    /// Sets the color of the [Bulma progress bar element][bd].
    ///
    /// Sets the color of the [Bulma progress bar element][bd] which will
    /// receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::progress::ProgressBar,
    ///     helpers::color::Color,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <ProgressBar color={Color::Danger} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/progress/#colors
    #[prop_or_default]
    pub color: Option<Color>,
    /// Sets the size of the [Bulma progress bar element][bd].
    ///
    /// Sets the size of the [Bulma progress bar element][bd] which will
    /// receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::progress::ProgressBar,
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <ProgressBar size={Size::Large} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/progress/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
    /// Sets the value of the [Bulma progress bar element][bd].
    ///
    /// Sets the value of the [Bulma progress bar element][bd] which will
    /// receive these properties. If the value is not set, the
    /// [progress bar][bd] will be an [indeterminate][none] one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::progress::ProgressBar;
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <ProgressBar value={32.0} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/progress/
    /// [none]: https://bulma.io/documentation/elements/progress/#indeterminate
    #[prop_or_default]
    pub value: Option<f64>,
    /// Sets the maximum value that the [progress bar element][bd] can take.
    ///
    /// Sets the maximum value that the [Bulma progress bar element][bd], which
    /// will receive these properties, can take. By default it is `100.0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::progress::ProgressBar;
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <ProgressBar value={32.0} max={100.0} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/progress/
    /// [none]: https://bulma.io/documentation/elements/progress/#indeterminate
    #[prop_or(100.0)]
    pub max: f64,
}

/// Yew implementation of the [Bulma progress bar element][bd].
///
/// Yew implementation of the progress bar element, based on the specification
/// found in the [Bulma progress bar element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::progress::ProgressBar;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <ProgressBar value={15.0} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/progress/
#[function_component(ProgressBar)]
pub fn progress_bar(props: &ProgressBarProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| format!("{IS_PREFIX}-{size}"))
        .unwrap_or("".to_owned());
    let class = ClassBuilder::default()
        .with_custom_class("progress")
        .with_color(props.color)
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
        <progress id={props.id.clone()} {class} value={props.value.map(|n| n.to_string())} max={props.max.to_string()}>{ props.value.unwrap_or(15.0) }{"%"}</progress>
    }
}
