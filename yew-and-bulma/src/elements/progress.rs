use yew::{function_component, html, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

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
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ProgressBarProperties {
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
        <progress id={props.id.clone()} {class} value={props.value.map(|n| n.to_string())} max={props.max.to_string()}
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
            { props.value.unwrap_or(15.0) }{"%"}
        </progress>
    }
}
