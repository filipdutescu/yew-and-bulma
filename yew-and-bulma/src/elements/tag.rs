use yew::{function_component, html, Children, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

use crate::{
    helpers::color::Color,
    utils::{
        class::ClassBuilder,
        constants::{ARE_PREFIX, IS_PREFIX},
        size::Size,
    },
};

/// Defines the properties of the [Bulma tags element][bd].
///
/// Defines the properties of the tags element, based on the specification
/// found in the [Bulma tags element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::tag::{Tag, Tags};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Tags>
///             <Tag>{"Tag label"}</Tag>
///             <Tag>{"Tag label"}</Tag>
///             <Tag>{"Tag label"}</Tag>
///         </Tags>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/tag/#list-of-tags
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct TagsProperties {
    /// Sets the size of the elements found inside the [tag element][bd].
    ///
    /// Sets the size of the elements that will be found inside the
    /// [Bulma tag element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::tag::{Tag, Tags},
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Tags size={Size::Large}>
    ///             <Tag>{"Tag label"}</Tag>
    ///             <Tag>{"Tag label"}</Tag>
    ///             <Tag>{"Tag label"}</Tag>
    ///         </Tags>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/tag/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
    /// Whether to attach the tags found inside the [tags element][bd].
    ///
    /// Whether or not to attach the tags that will be found inside the
    /// [Bulma tags element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::tag::{Tag, Tags},
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Tags addons=true>
    ///             <Tag>{"build"}</Tag>
    ///             <Tag>{"passing"}</Tag>
    ///         </Tags>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/tag/#tag-addons
    #[prop_or_default]
    pub addons: bool,
    /// The list of elements found inside the [tags element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma tags element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/tag/
    pub children: Children,
}

/// Yew implementation of the [Bulma tags element][bd].
///
/// Yew implementation of the tags element, based on the specification found in
/// the [Bulma tags element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::tag::{Tag, Tags};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Tags>
///             <Tag>{"Tag label"}</Tag>
///         </Tags>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/tag/#list-of-tags
#[function_component(Tags)]
pub fn tags(props: &TagsProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| {
            if Size::Small == *size {
                "".to_owned()
            } else {
                format!("{ARE_PREFIX}-{size}")
            }
        })
        .unwrap_or("".to_owned());
    let addons = if props.addons { "has-addons" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("tags")
        .with_custom_class(&size)
        .with_custom_class(addons)
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

/// Defines the properties of the [Bulma tag element][bd].
///
/// Defines the properties of the tag element, based on the specification found
/// in the [Bulma tag element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::tag::Tag;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Tag>{"Tag label"}</Tag>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/tag/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct TagProperties {
    /// Sets the size of the [Bulma tag element][bd].
    ///
    /// Sets the size of the [Bulma tag element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::tag::Tag,
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Tag size={Size::Large}>{"Tag label"}</Tag>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/tag/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
    /// Sets the color of the [Bulma tag element][bd].
    ///
    /// Sets the color of the [Bulma tag element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::tag::Tag,
    ///     helpers::color::Color,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Tag color={Color::Primary}>{"Tag label"}</Tag>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/tag/#colors
    #[prop_or_default]
    pub color: Option<Color>,
    /// Whether or not the color of the [tag element][bd] should be light.
    ///
    /// Whether or not the color of the [Bulma tag element][bd], which will
    /// receive these properties, should be of the light variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::tag::Tag,
    ///     helpers::color::Color,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Tag light=true color={Color::Primary}>{"Tag label"}</Tag>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/tag/#colors
    #[prop_or_default]
    pub light: Option<bool>,
    /// Whether or not the [Bulma tag element][bd] should be rounded.
    ///
    /// Whether or not the [Bulma tag element][bd], which will receive these
    /// properties, will be rounded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::tag::Tag;
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Tag rounded=true>{"Tag label"}</Tag>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/tag/#modifiers
    #[prop_or_default]
    pub rounded: bool,
    /// Whether or not the [Bulma element][bd] should become a delete button.
    ///
    /// Whether or not the [Bulma tag element][bd], which will receive these
    /// properties, will become a delete button.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::tag::Tag;
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Tag delete=true />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/tag/#modifiers
    #[prop_or_default]
    pub delete: bool,
    /// The list of elements found inside the [tag element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma tag element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/tag/
    #[prop_or_default]
    pub children: Children,
}

/// Yew implementation of the [Bulma tag element][bd].
///
/// Yew implementation of the tag element, based on the specification found in
/// the [Bulma tag element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::tag::Tag;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Tag>{"Tag label"}</Tag>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/tag/
#[function_component(Tag)]
pub fn tag(props: &TagProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| {
            if Size::Small == *size {
                "".to_owned()
            } else {
                format!("{IS_PREFIX}-{size}")
            }
        })
        .unwrap_or("".to_owned());
    let rounded = if props.rounded { "is-rounded" } else { "" };
    let delete = if props.delete { "is-delete" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("tag")
        .with_color(props.color)
        .is_light(props.light)
        .with_custom_class(&size)
        .with_custom_class(rounded)
        .with_custom_class(delete)
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();
    let tag = (if props.delete { "a" } else { "span" }).to_string();

    html! {
        <@{tag} id={props.id.clone()} {class}
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
}
