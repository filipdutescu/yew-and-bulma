use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, Html, Properties,
};
use yew_and_bulma_macros::base_component_properties;

use crate::utils::class::ClassBuilder;

/// Defines the properties of the [Bulma media object element][bd].
///
/// Defines the properties of the media object element, based on the
/// specification found in the [Bulma media object element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaContent, MediaLeft, MediaRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaLeft>{"Avatar goes here."}</MediaLeft>
///
///             <MediaContent>{"Content goes here."}</MediaContent>
///
///             <MediaRight>{"Dismiss goes here for example."}</MediaRight>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MediaProperties {
    /// The list of elements found inside the [media element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma media element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/media-object/
    pub children: ChildrenRenderer<MediaItem>,
}

/// Yew implementation of the [Bulma media object element][bd].
///
/// Yew implementation of the media object element, based on the specification
/// found in the [Bulma media object element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaContent, MediaLeft, MediaRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaLeft>{"Avatar goes here."}</MediaLeft>
///
///             <MediaContent>{"Content goes here."}</MediaContent>
///
///             <MediaRight>{"Dismiss goes here for example."}</MediaRight>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[function_component(Media)]
pub fn media(props: &MediaProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("media")
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <div id={&props.id} {class}
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

/// Defines the possible types of children from a [Bulma media object element][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma media object element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaContent, MediaLeft, MediaRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaLeft>{"Avatar goes here."}</MediaLeft>
///
///             <MediaContent>{"Content goes here."}</MediaContent>
///
///             <MediaRight>{"Dismiss goes here for example."}</MediaRight>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[derive(Clone, PartialEq)]
pub enum MediaItem {
    MediaLeft(VChild<MediaLeft>),
    MediaContent(VChild<MediaContent>),
    MediaRight(VChild<MediaRight>),
}

impl From<VChild<MediaLeft>> for MediaItem {
    fn from(value: VChild<MediaLeft>) -> Self {
        MediaItem::MediaLeft(value)
    }
}

impl From<VChild<MediaContent>> for MediaItem {
    fn from(value: VChild<MediaContent>) -> Self {
        MediaItem::MediaContent(value)
    }
}

impl From<VChild<MediaRight>> for MediaItem {
    fn from(value: VChild<MediaRight>) -> Self {
        MediaItem::MediaRight(value)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Html> for MediaItem {
    fn into(self) -> Html {
        match self {
            MediaItem::MediaLeft(ml) => ml.into(),
            MediaItem::MediaContent(mc) => mc.into(),
            MediaItem::MediaRight(mr) => mr.into(),
        }
    }
}

/// Defines the properties of the [Bulma media left element][bd].
///
/// Defines the properties of the media left element, based on the
/// specification found in the [Bulma media left element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaLeft};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaLeft>{"Left goes here."}</MediaLeft>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MediaLeftProperties {
    /// The list of elements found inside the [media left element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma media left element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/media-object/
    pub children: Children,
}

/// Yew implementation of the [Bulma media left element][bd].
///
/// Yew implementation of the media left element, based on the specification
/// found in the [Bulma media left element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaLeft};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaLeft>{"Left goes here."}</MediaLeft>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[function_component(MediaLeft)]
pub fn media_left(props: &MediaLeftProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("media-left")
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <div id={&props.id} {class}
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

/// Defines the properties of the [Bulma media content element][bd].
///
/// Defines the properties of the media content element, based on the
/// specification found in the [Bulma media content element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaContent>{"Content goes here."}</MediaContent>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MediaContentProperties {
    /// The list of elements found inside the [media content element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma media content element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/media-object/
    pub children: Children,
}

/// Yew implementation of the [Bulma media content element][bd].
///
/// Yew implementation of the media content element, based on the specification
/// found in the [Bulma media content element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaContent>{"Content goes here."}</MediaContent>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[function_component(MediaContent)]
pub fn media_content(props: &MediaContentProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("media-content")
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <div id={&props.id} {class}
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

/// Defines the properties of the [Bulma media right element][bd].
///
/// Defines the properties of the media right element, based on the
/// specification found in the [Bulma media right element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaRight>{"Right goes here."}</MediaRight>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MediaRightProperties {
    /// The list of elements found inside the [media right element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma media right element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/media-object/
    pub children: Children,
}

/// Yew implementation of the [Bulma media right element][bd].
///
/// Yew implementation of the media right element, based on the specification
/// found in the [Bulma media right element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaRight>{"Right goes here."}</MediaRight>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[function_component(MediaRight)]
pub fn media_right(props: &MediaRightProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("media-right")
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <div id={&props.id} {class}
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
