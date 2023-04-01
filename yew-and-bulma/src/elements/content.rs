use yew::{function_component, html, Children, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

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
#[base_component_properties]
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
