use yew::{function_component, html, Children, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

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
#[base_component_properties]
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
            { for props.children.iter() }
        </div>
    }
}
