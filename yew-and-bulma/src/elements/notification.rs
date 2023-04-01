use yew::{function_component, html, Children, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

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
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct NotificationProperties {
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
        <div id={props.id.clone()} {class}
            onclick={props.onclick.clone()} onwheel={props.onwheel.clone()} onscroll={props.onscroll.clone()}
            onmousedown={props.onmousedown.clone()} onmousemove={props.onmousemove.clone()} onmouseout={props.onmouseout.clone()} onmouseover={props.onmouseover.clone()} onmouseup={props.onmouseup.clone()}
            ondrag={props.ondrag.clone()} ondragend={props.ondragend.clone()} ondragenter={props.ondragenter.clone()} ondragleave={props.ondragleave.clone()} ondragover={props.ondragover.clone()} ondragstart={props.ondragstart.clone()} ondrop={props.ondrop.clone()}
            oncopy={props.oncopy.clone()} oncut={props.oncut.clone()} onpaste={props.onpaste.clone()}
            onkeydown={props.onkeydown.clone()} onkeypress={props.onkeypress.clone()} onkeyup={props.onkeyup.clone()}
            onblur={props.onblur.clone()} onchange={props.onchange.clone()} oncontextmenu={props.oncontextmenu.clone()} onfocus={props.onfocus.clone()} oninput={props.oninput.clone()} oninvalid={props.oninvalid.clone()} onreset={props.onreset.clone()} onselect={props.onselect.clone()} onsubmit={props.onsubmit.clone()}
            onabort={props.onabort.clone()} oncanplay={props.oncanplay.clone()} oncanplaythrough={props.oncanplaythrough.clone()} oncuechange={props.oncuechange.clone()}
            ondurationchange={props.ondurationchange.clone()} onemptied={props.onemptied.clone()} onended={props.onended.clone()} onerror={props.onerror.clone()}
            onloadeddata={props.onloadeddata.clone()} onloadedmetadata={props.onloadedmetadata.clone()} onloadstart={props.onloadstart.clone()} onpause={props.onpause.clone()}
            onplay={props.onplay.clone()} onplaying={props.onplaying.clone()} onprogress={props.onprogress.clone()} onratechange={props.onratechange.clone()}
            onseeked={props.onseeked.clone()} onseeking={props.onseeking.clone()} onstalled={props.onstalled.clone()} onsuspend={props.onsuspend.clone()}
            ontimeupdate={props.ontimeupdate.clone()} onvolumechange={props.onvolumechange.clone()} onwaiting={props.onwaiting.clone()}>
            if props.delete_button {
                <Delete />
            }
            { for props.children.iter() }
        </div>
    }
}
