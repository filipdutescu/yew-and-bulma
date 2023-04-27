use yew::{function_component, html, virtual_dom::VNode, AttrValue, Children, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

/// Provides utilities for CSS class manipulation.
///
/// The most important element contained in this module is the
/// [`crate::utils::class::ClassBuilder`] struct, which is used to make it
/// easier to generate the [HTML class attribute][class] value of an element.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::color::TextColor,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the text color set to primary.
/// #[function_component(ColoredTextDiv)]
/// fn colored_text_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_text_color(Some(TextColor::Primary))
///         .build();
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [class]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes#class
pub mod class;
/// Provides various constants in a centralized place.
///
/// Defines constants such as Bulma class name prefixes (ie for `has-text-*`,
/// `is-size-*`, `has-background-*` etc.).
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::color::TextColor,
///     utils::constants::HAS_TEXT_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the text color set to primary.
/// #[function_component(ColoredTextDiv)]
/// fn colored_text_div() -> Html {
///     let text_color = TextColor::Primary;
///     let class = classes![format!("{HAS_TEXT_PREFIX}-{text_color}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
pub mod constants;
/// Provides utilities for Bulma size-related styling.
///
/// Defines various utilities, such as Bulma common size modifiers (ie for
/// `are-small`, `is-large` etc.).
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     utils::{constants::IS_PREFIX, size::Size},
/// };
///
/// // Create a `<div>` HTML element that has the size set to large.
/// #[function_component(LargeDiv)]
/// fn large_div() -> Html {
///     let size = Size::Large;
///     let class = classes![format!("{IS_PREFIX}-{size}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
pub mod size;

#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct BaseComponentProperties {
    /// The [HTML tag][tag] name to be used for the component
    ///
    /// Specifies what [HTML tag][tag] name should be used for the base
    /// component.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::BaseComponent;
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <BaseComponent tag="span">{"This is text inside a span"}</BaseComponent>
    ///     }
    /// }
    /// ```
    ///
    /// [tag]: https://developer.mozilla.org/en-US/docs/Glossary/Tag
    pub tag: AttrValue,
    /// The list of elements found inside the base component.
    ///
    /// Defines the elements that will be found inside the base component which
    /// will receive these properties.
    #[prop_or_default]
    pub children: Children,
}

trait SizedIntoAttribute: Into<AttrValue> + Sized {}

#[function_component]
pub fn BaseComponent(props: &BaseComponentProperties) -> Html {
    let mut html = if !props.children.is_empty() {
        html! {
            <@{props.tag.to_string()} id={props.id.clone()} class={props.class.clone()}
                title={props.title.clone()} role={props.role.clone()} aria-label={props.aria_label.clone()} aria-current={props.aria_current.clone()}
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
            </@>
        }
    } else {
        html! {
            <@{props.tag.to_string()} id={props.id.clone()} class={props.class.clone()}
                title={props.title.clone()} role={props.role.clone()} aria-label={props.aria_label.clone()} aria-current={props.aria_current.clone()}
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
                ontimeupdate={props.ontimeupdate.clone()} onvolumechange={props.onvolumechange.clone()} onwaiting={props.onwaiting.clone()} />
        }
    };

    if let VNode::VTag(tag) = &mut html {
        for (key, val) in props.attrs.iter() {
            tag.add_attribute(key, val.clone());
        }
    }

    html
}
