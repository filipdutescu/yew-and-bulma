use std::fmt::Display;

use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, Html, Properties,
};
use yew_and_bulma_macros::{base_component_properties, TypedChildren};

use crate::{
    helpers::color::Color,
    utils::{class::ClassBuilder, constants::IS_PREFIX},
};

/// Defines the possible sizes of a [Bulma hero element][bd].
///
/// Defines the possible sizes that a [Bulma hero element][bd] can take.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::hero::{Hero, HeroBody, Size};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Hero size={Size::Large}>
///             <HeroBody>{"This is the hero body."}</HeroBody>
///         </Hero>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/hero/#sizes
#[derive(PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
    HalfHeight,
    FullHeight,
    FullHeightWithNavbar,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Size::Small => "small",
            Size::Medium => "medium",
            Size::Large => "large",
            Size::HalfHeight => "halfheight",
            Size::FullHeight => "fullheight",
            Size::FullHeightWithNavbar => "fullheight-with-navbar",
        };

        write!(f, "{size}")
    }
}

/// Defines the properties of the [Bulma hero element][bd].
///
/// Defines the properties of the hero element, based on the specification
/// found in the [Bulma hero element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::hero::{Hero, HeroBody};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Hero>
///             <HeroBody>{"This is the hero body."}</HeroBody>
///         </Hero>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/hero/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct HeroProperties {
    /// Sets the color of the [Bulma hero element][bd].
    ///
    /// Sets the color of the [Bulma hero element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::color::Color,
    ///     layout::hero::{Hero, HeroBody},
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Hero color={Color::Primary}>
    ///             <HeroBody>{"This is the hero body."}</HeroBody>
    ///         </Hero>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/layout/hero/#colors
    #[prop_or_default]
    pub color: Option<Color>,
    /// Sets the size of the [Bulma hero element][bd].
    ///
    /// Sets the size of the [Bulma hero element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::hero::{Hero, HeroBody, Size};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Hero size={Size::Large}>
    ///             <HeroBody>{"This is the hero body."}</HeroBody>
    ///         </Hero>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/layout/hero/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
    /// The list of elements found inside the [hero element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma hero element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/hero/
    pub children: ChildrenRenderer<HeroItem>,
}

/// Yew implementation of the [Bulma hero element][bd].
///
/// Yew implementation of the hero element, based on the specification
/// found in the [Bulma hero element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::hero::{Hero, HeroBody};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Hero>
///             <HeroBody>{"This is the hero body."}</HeroBody>
///         </Hero>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/hero/
#[function_component(Hero)]
pub fn hero(props: &HeroProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| format!("{IS_PREFIX}-{size}"))
        .unwrap_or("".to_owned());
    let class = ClassBuilder::default()
        .with_custom_class("hero")
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

/// Defines the possible types of children from a [Bulma hero object element][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma hero object element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::hero::{Hero, HeroBody, HeroHead, HeroFoot};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Hero>
///             <HeroHead>{"Hero title"}</HeroHead>
///             <HeroBody>{"Hero body"}</HeroBody>
///             <HeroFoot>{"Hero footer"}</HeroFoot>
///         </Hero>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/hero/
#[derive(Clone, PartialEq, TypedChildren)]
pub enum HeroItem {
    HeroHead(VChild<HeroHead>),
    HeroBody(VChild<HeroBody>),
    HeroFoot(VChild<HeroFoot>),
}

/// Defines the properties of the [Bulma hero head element][bd].
///
/// Defines the properties of the hero head element, based on the specification
/// found in the [Bulma hero head element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::hero::{Hero, HeroBody, HeroHead};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Hero>
///             <HeroHead>{"This is the hero head."}</HeroHead>
///             <HeroBody>{"This is the hero body."}</HeroBody>
///         </Hero>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/hero/#fullheight-hero-in-3-parts
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct HeroHeadProperties {
    /// The list of elements found inside the [hero head element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma hero head element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/hero/
    pub children: Children,
}

/// Yew implementation of the [Bulma hero head element][bd].
///
/// Yew implementation of the hero head element, based on the specification
/// found in the [Bulma hero head element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::hero::{Hero, HeroBody, HeroHead};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Hero>
///             <HeroHead>{"This is the hero head."}</HeroHead>
///             <HeroBody>{"This is the hero body."}</HeroBody>
///         </Hero>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/hero/#fullheight-hero-in-3-parts
#[function_component(HeroHead)]
pub fn hero_head(props: &HeroHeadProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("hero-head")
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

/// Defines the properties of the [Bulma hero body element][bd].
///
/// Defines the properties of the hero body element, based on the specification
/// found in the [Bulma hero body element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::hero::{Hero, HeroBody};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Hero>
///             <HeroBody>{"This is the hero body."}</HeroBody>
///         </Hero>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/hero/#fullheight-hero-in-3-parts
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct HeroBodyProperties {
    /// The list of elements found inside the [hero body element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma hero body element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/hero/
    pub children: Children,
}

/// Yew implementation of the [Bulma hero body element][bd].
///
/// Yew implementation of the hero body element, based on the specification
/// found in the [Bulma hero body element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::hero::{Hero, HeroBody};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Hero>
///             <HeroBody>{"This is the hero body."}</HeroBody>
///         </Hero>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/hero/#fullheight-hero-in-3-parts
#[function_component(HeroBody)]
pub fn hero_body(props: &HeroBodyProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("hero-body")
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

/// Defines the properties of the [Bulma hero foot element][bd].
///
/// Defines the properties of the hero foot element, based on the specification
/// found in the [Bulma hero foot element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::hero::{Hero, HeroBody, HeroFoot};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Hero>
///             <HeroBody>{"This is the hero body."}</HeroBody>
///             <HeroFoot>{"This is the hero footer."}</HeroFoot>
///         </Hero>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/hero/#fullheight-hero-in-3-parts
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct HeroFootProperties {
    /// The list of elements found inside the [hero foot element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma hero foot element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/hero/
    pub children: Children,
}

/// Yew implementation of the [Bulma hero foot element][bd].
///
/// Yew implementation of the hero foot element, based on the specification
/// found in the [Bulma hero foot element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::hero::{Hero, HeroBody, HeroFoot};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Hero>
///             <HeroBody>{"This is the hero body."}</HeroBody>
///             <HeroFoot>{"This is the hero footer."}</HeroFoot>
///         </Hero>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/hero/#fullheight-hero-in-3-parts
#[function_component(HeroFoot)]
pub fn hero_foot(props: &HeroFootProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("hero-foot")
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
