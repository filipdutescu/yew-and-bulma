use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::{elements::delete::Delete, helpers::color::Color, utils::class::ClassBuilder};

/// Defines the properties of the [Bulma notification element][bd].
///
/// Defines the properties of the notification element, based on the
/// specification found in the [Bulma notification element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::notification::Notification;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Notification>{"Hello, world!"}</Notification>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/notification/
#[derive(Properties, PartialEq)]
pub struct NotificationProperties {
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
    /// Sets the color of the [Bulma notification element][bd].
    ///
    /// Sets the color of the [Bulma notification element][bd] which will
    /// receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{elements::notification::Notification, helpers::color::Color};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Notification color={Color::Primary}>{"Hello, world!"}</Notification>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/notification/#colors
    #[prop_or_default]
    pub color: Option<Color>,
    /// Whether the color of the [notification element][bd] should be light.
    ///
    /// Whether or not the color of the [Bulma notification element][bd], which
    /// will receive these properties, should be of the light variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{elements::notification::Notification, helpers::color::Color};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Notification light=true color={Color::Primary}>{"Hello, world!"}</Notification>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/notification/#light-colors
    #[prop_or_default]
    pub light: Option<bool>,
    /// Whether the [notification element][bd] should have a delete button.
    ///
    /// Whether or not the [Bulma notification element][bd], which will receive
    /// these properties, should have a delete button.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::notification::Notification;
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Notification delete_button=false>{"Hello, world!"}</Notification>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/notification/
    #[prop_or(true)]
    pub delete_button: bool,
    /// The list of elements found inside the [notification element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma notification element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/notification/
    pub children: Children,
}

/// Yew implementation of the [Bulma notification element][bd].
///
/// Yew implementation of the notification element, based on the specification
/// found in the [Bulma notification element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::notification::Notification;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Notification>{"Hello, world!"}</Notification>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/notification/
#[function_component(Notification)]
pub fn notification(props: &NotificationProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("notification")
        .with_color(props.color)
        .is_light(props.light)
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
            if props.delete_button {
                <Delete />
            }
            { for props.children.iter() }
        </div>
    }
}
