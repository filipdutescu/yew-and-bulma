use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, ChildrenWithProps,
    Html, Properties,
};
use yew_and_bulma_macros::base_component_properties;

use crate::utils::class::ClassBuilder;

/// Defines the properties of the [Bulma level element][bd].
///
/// Defines the properties of the level element, based on the specification
/// found in the [Bulma level element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::level::{Level, LevelItem};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Level>
///             <LevelItem>{"This is some text in a level."}</LevelItem>
///         </Level>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/level/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct LevelProperties {
    /// Whether or not the [level element][bd] should be horizontal on mobile.
    ///
    /// Whether or not the [Bulma level element][bd], which will receive
    /// these properties, should be horizontal on mobile devices.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::level::{Level, LevelItem};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Level mobile={true}>
    ///             <LevelItem>{"This is some text in a level."}</LevelItem>
    ///         </Level>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/layout/level/#mobile-level
    #[prop_or_default]
    pub mobile: bool,
    /// The list of elements found inside the [level element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma level element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/level/
    pub children: ChildrenRenderer<LevelElement>,
}

/// Yew implementation of the [Bulma level element][bd].
///
/// Yew implementation of the level element, based on the specification
/// found in the [Bulma level element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::level::{Level, LevelItem};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Level>
///             <LevelItem>{"This is some text in a level."}</LevelItem>
///         </Level>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/level/
#[function_component(Level)]
pub fn level(props: &LevelProperties) -> Html {
    let mobile = if props.mobile { "is-mobile" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("level")
        .with_custom_class(mobile)
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

/// Defines the possible types of children from a [Bulma level element][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma level element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::level::{Level, LevelItem, LevelLeft, LevelRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Level>
///             <LevelLeft>
///                 <LevelItem>{ "Left aligned text in the level" }</LevelItem>
///             </LevelLeft>
///
///             <LevelRight>
///                 <LevelItem>{ "Right aligned text in the level" }</LevelItem>
///             </levelRight>
///         </Level>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/level/
#[derive(Clone, PartialEq)]
pub enum LevelElement {
    LevelItem(VChild<LevelItem>),
    LevelLeft(VChild<LevelLeft>),
    LevelRight(VChild<LevelRight>),
}

impl From<VChild<LevelItem>> for LevelElement {
    fn from(value: VChild<LevelItem>) -> Self {
        LevelElement::LevelItem(value)
    }
}

impl From<VChild<LevelLeft>> for LevelElement {
    fn from(value: VChild<LevelLeft>) -> Self {
        LevelElement::LevelLeft(value)
    }
}

impl From<VChild<LevelRight>> for LevelElement {
    fn from(value: VChild<LevelRight>) -> Self {
        LevelElement::LevelRight(value)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Html> for LevelElement {
    fn into(self) -> Html {
        match self {
            LevelElement::LevelItem(li) => li.into(),
            LevelElement::LevelLeft(ll) => ll.into(),
            LevelElement::LevelRight(lr) => lr.into(),
        }
    }
}

/// Defines the properties of the [Bulma level item element][bd].
///
/// Defines the properties of the level item element, based on the specification
/// found in the [Bulma level item element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::level::{Level, LevelItem};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Level>
///             <LevelItem>{"This is some text in a level."}</LevelItem>
///         </Level>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/level/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct LevelItemProperties {
    /// The list of elements found inside the [level item element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma level item element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/level/
    pub children: Children,
}

/// Yew implementation of the [Bulma level item element][bd].
///
/// Yew implementation of the level item element, based on the specification
/// found in the [Bulma level item element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::level::{Level, LevelItem};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Level>
///             <LevelItem>{"This is some text in a level."}</LevelItem>
///         </Level>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/level/
#[function_component(LevelItem)]
pub fn level_item(props: &LevelItemProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("level-item")
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

/// Defines the properties of the [Bulma level left element][bd].
///
/// Defines the properties of the level left element, based on the specification
/// found in the [Bulma level left element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::level::{Level, LevelItem, LevelLeft};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Level>
///             <LevelLeft>
///                 <LevelItem>{"This is some text in a level left element."}</LevelItem>
///             </LevelLeft>
///         </Level>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/level/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct LevelLeftProperties {
    /// The list of elements found inside the [level left element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma level left element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/level/
    pub children: ChildrenWithProps<LevelItem>,
}

/// Yew implementation of the [Bulma level left element][bd].
///
/// Yew implementation of the level left element, based on the specification
/// found in the [Bulma level left element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::level::{Level, LevelItem, LevelLeft};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Level>
///             <LevelLeft>
///                 <LevelItem>{"This is some text in a level left element."}</LevelItem>
///             </LevelLeft>
///         </Level>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/level/
#[function_component(LevelLeft)]
pub fn level_right(props: &LevelLeftProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("level-left")
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

/// Defines the properties of the [Bulma level right element][bd].
///
/// Defines the properties of the level right element, based on the specification
/// found in the [Bulma level right element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::level::{Level, LevelItem, LevelRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Level>
///             <LevelRight>
///                 <LevelItem>{"This is some text in a level right element."}</LevelItem>
///             </LevelRight>
///         </Level>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/level/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct LevelRightProperties {
    /// The list of elements found inside the [level right element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma level right element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/level/
    pub children: ChildrenWithProps<LevelItem>,
}

/// Yew implementation of the [Bulma level right element][bd].
///
/// Yew implementation of the level right element, based on the specification
/// found in the [Bulma level right element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::level::{Level, LevelItem, LevelRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Level>
///             <LevelRight>
///                 <LevelItem>{"This is some text in a level right element."}</LevelItem>
///             </LevelRight>
///         </Level>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/level/
#[function_component(LevelRight)]
pub fn level_right(props: &LevelRightProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("level-right")
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
