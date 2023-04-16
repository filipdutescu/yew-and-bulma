use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, Html, Properties,
};
use yew_and_bulma_macros::{base_component_properties, TypedChildren};

use crate::helpers::visibility::Viewport;
use crate::utils::constants::IS_NARROW;
use crate::utils::{
    class::ClassBuilder,
    constants::{IS_OFFSET_PREFIX, IS_PREFIX},
};

/// Defines the properties of the [Bulma columns element][bd].
///
/// Defines the properties of the columns element, based on the specification
/// found in the [Bulma columns element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::columns::{Column, Columns};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Columns>
///             <Column>
///                 {"This is some text in a column."}
///             </Column>
///         </Columns>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/columns/basics
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ColumnsProperties {
    /// Sets the viewport from which the columns inside the [Bulma columns element][bd]
    /// should be active.
    ///
    /// Sets the viewport from which the columns inside the
    /// [Bulma columns element][bd] should be active, which will receive these
    /// properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     columns::{Column, Columns},
    ///     helpers::visibility::Viewport,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Columns viewport={Viewport::Mobile}>
    ///             <Column>
    ///                 {"First column"}
    ///             </Column>
    ///
    ///             <Column>
    ///                 {"Second column"}
    ///             </Column>
    ///         </Columns>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/columns/responsiveness/
    #[prop_or_default]
    pub viewport: Option<Viewport>,
    /// Sets the gap size between the columns inside the [Bulma columns element][bd].
    ///
    /// Sets the gap size between the columns inside the
    /// [Bulma columns element][bd] should be active, which will receive these
    /// properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::columns::{Column, Columns, GapSize};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Columns gap_size={GapSize::Five}>
    ///             <Column>
    ///                 {"First column"}
    ///             </Column>
    ///
    ///             <Column>
    ///                 {"Second column"}
    ///             </Column>
    ///         </Columns>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/columns/gap/#variable-gap
    #[prop_or_default]
    pub gap_size: Option<GapSize>,
    /// Sets the size of the [Bulma column element][bd] for a viewport.
    ///
    /// Sets the size for a viewport of the [Bulma column element][bd] which
    /// will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     columns::{Column, Columns, GapSize},
    ///     helpers::visibility::Viewport,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     let mut viewport_gap_sizes = HashMap::new();
    ///     viewport_gap_sizes.insert(Viewport::Mobile, GapSize::Five);
    ///
    ///     html! {
    ///         <Columns {viewport_gap_sizes}>
    ///             <Column>
    ///                 {"First column"}
    ///             </Column>
    ///
    ///             <Column>
    ///                 {"Second column"}
    ///             </Column>
    ///         </Columns>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/columns/gap/#variable-gap
    #[prop_or_default]
    pub viewport_gap_sizes: HashMap<Viewport, GapSize>,
    /// Whether to remove the gap between columns inside the [columns element][bd].
    ///
    /// Whether or not to remove the gap between columns found inside the
    /// [Bulma columns element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::columns::{Column, Columns};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Columns gapless=true>
    ///             <Column>
    ///                 {"First column"}
    ///             </Column>
    ///
    ///             <Column>
    ///                 {"Second column"}
    ///             </Column>
    ///         </Columns>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/columns/gap/#gapless
    #[prop_or_default]
    pub gapless: bool,
    /// Whether to center vertically the columns inside the [columns element][bd].
    ///
    /// Whether or not to center vertically the columns found inside the
    /// [Bulma columns element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::columns::{Column, Columns};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Columns center_vertically=true>
    ///             <Column>
    ///                 {"First column"}
    ///             </Column>
    ///
    ///             <Column>
    ///                 {"Second column"}
    ///             </Column>
    ///         </Columns>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/columns/other/#vertical-alignment
    #[prop_or_default]
    pub center_vertically: bool,
    /// Whether to center the columns inside the [columns element][bd].
    ///
    /// Whether or not to center the columns found inside the
    /// [Bulma columns element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::columns::{Column, Columns};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Columns centered=true>
    ///             <Column>
    ///                 {"First column"}
    ///             </Column>
    ///
    ///             <Column>
    ///                 {"Second column"}
    ///             </Column>
    ///         </Columns>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/columns/other/#centering-columns
    #[prop_or_default]
    pub centered: bool,
    /// Whether the columns inside the [columns element][bd] should be multilined.
    ///
    /// Whether or not the columns found inside the [Bulma columns element][bd],
    /// which will receive these properties, should be spread on multiple lines.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::columns::{Column, Columns};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Columns multiline=true>
    ///             <Column>
    ///                 {"First column"}
    ///             </Column>
    ///
    ///             <Column>
    ///                 {"Second column"}
    ///             </Column>
    ///         </Columns>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/columns/gap/#gapless
    #[prop_or_default]
    pub multiline: bool,
    /// The list of elements found inside the [columns element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma columns element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/columns/basics
    pub children: ChildrenRenderer<ColumnsItem>,
}

/// Enum defining the possible column gap sizes, as described in the
/// [Bulma documentation][bd].
///
/// Defines all gap size values that columns can have, as described in the
/// [Bulma columns documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::columns::{Column, Columns, GapSize};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Columns gap_size={GapSize::Five}>
///             <Column>
///                 {"First column"}
///             </Column>
///
///             <Column>
///                 {"Second column"}
///             </Column>
///         </Columns>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/columns/gap/#variable-gap
#[derive(Eq, Hash, PartialEq)]
pub enum GapSize {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Display for GapSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gap_size = match self {
            GapSize::Zero => "0",
            GapSize::One => "1",
            GapSize::Two => "2",
            GapSize::Three => "3",
            GapSize::Four => "4",
            GapSize::Five => "5",
            GapSize::Six => "6",
            GapSize::Seven => "7",
            GapSize::Eight => "8",
        };

        write!(f, "{gap_size}")
    }
}

/// Defines the possible types of children from a [Bulma columns element][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma columns element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::columns::{Column, Columns};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Columns>
///             <Column>
///                 {"This is some text in a column."}
///             </Column>
///         </Columns>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/columns/basics
#[derive(Clone, PartialEq, TypedChildren)]
pub enum ColumnsItem {
    Columns(VChild<Columns>),
    Column(VChild<Column>),
}

/// Yew implementation of the [Bulma columns element][bd].
///
/// Yew implementation of the columns element, based on the specification found
/// in the [Bulma columns element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::columns::{Column, Columns};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Columns>
///             <Column>
///                 {"This is some text in a column."}
///             </Column>
///         </Columns>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/columns/basics
#[function_component(Columns)]
pub fn columns(props: &ColumnsProperties) -> Html {
    let viewport = props
        .viewport
        .as_ref()
        .map(|viewport| format!("{IS_PREFIX}-{viewport}"))
        .unwrap_or("".to_owned());
    let multiline = if props.multiline { "is-multiline" } else { "" };
    let gapless = if props.gapless { "is-gapless" } else { "" };
    let gap_size = props
        .gap_size
        .as_ref()
        .map(|gap_size| format!("{IS_PREFIX}-{gap_size}"))
        .unwrap_or("".to_owned());
    let mut viewport_gap_sizes = String::new();
    props
        .viewport_gap_sizes
        .iter()
        .for_each(|(viewport, gap_size)| {
            viewport_gap_sizes.push_str(&format!("{IS_PREFIX}-{gap_size}-{viewport}"))
        });
    let is_variable = if gap_size.is_empty() && viewport_gap_sizes.is_empty() {
        ""
    } else {
        "is-variable"
    };
    let center_vertically = if props.center_vertically {
        "is-vcentered"
    } else {
        ""
    };
    let centered = if props.centered { "is-centered" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("columns")
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .with_custom_class(&viewport)
        .with_custom_class(multiline)
        .with_custom_class(gapless)
        .with_custom_class(is_variable)
        .with_custom_class(&gap_size)
        .with_custom_class(&viewport_gap_sizes)
        .with_custom_class(center_vertically)
        .with_custom_class(centered)
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

/// Enum defining the possible column sizes, as described in the
/// [Bulma documentation][bd].
///
/// Defines all size values that columns can have, as described in the
/// [Bulma columns documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::columns::{Column, Columns, Size};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Columns>
///             <Column size={Size::ThreeQuarters}>
///                 {"First column"}
///             </Column>
///
///             <Column>
///                 {"Second column"}
///             </Column>
///         </Columns>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/columns/sizes/
#[derive(Eq, Hash, PartialEq)]
pub enum Size {
    FourFifths,
    ThreeQuarters,
    TwoThirds,
    ThreeFifths,
    Half,
    TwoFifths,
    OneThird,
    OneQuarter,
    OneFifth,
    Full,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Size::FourFifths => "four-fifths",
            Size::ThreeQuarters => "three-quarters",
            Size::TwoThirds => "two-thirds",
            Size::ThreeFifths => "three-fifths",
            Size::Half => "half",
            Size::TwoFifths => "two-fifths",
            Size::OneThird => "one-third",
            Size::OneQuarter => "one-quarter",
            Size::OneFifth => "one-fifth",
            Size::Full => "full",
            Size::One => "1",
            Size::Two => "2",
            Size::Three => "3",
            Size::Four => "4",
            Size::Five => "5",
            Size::Six => "6",
            Size::Seven => "7",
            Size::Eight => "8",
            Size::Nine => "9",
            Size::Ten => "10",
            Size::Eleven => "11",
            Size::Twelve => "12",
        };

        write!(f, "{size}")
    }
}

/// Defines the properties of the [Bulma column element][bd].
///
/// Defines the properties of the column element, based on the specification
/// found in the [Bulma column element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::columns::{Column, Columns};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Columns>
///             <Column>
///                 {"This is some text in a column."}
///             </Column>
///         </Columns>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/columns/basics
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ColumnProperties {
    /// Sets the size of the [Bulma column element][bd].
    ///
    /// Sets the size of the [Bulma column element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::columns::{Column, Columns, Size};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Columns>
    ///             <Column size={Size::TwoThirds}>
    ///                 {"First column"}
    ///             </Column>
    ///
    ///             <Column>
    ///                 {"Second column"}
    ///             </Column>
    ///         </Columns>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/columns/sizes/
    #[prop_or_default]
    pub size: Option<Size>,
    /// Sets the size of the [Bulma column element][bd] for a viewport.
    ///
    /// Sets the size for a viewport of the [Bulma column element][bd] which
    /// will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     columns::{Column, Columns, Size},
    ///     helpers::visibility::Viewport,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     let mut viewport_sizes = HashMap::new();
    ///     viewport_sizes.insert(Viewport::Mobile, Size::Half);
    ///
    ///     html! {
    ///         <Columns>
    ///             <Column size={Size::TwoThirds} {viewport_sizes}>
    ///                 {"First column"}
    ///             </Column>
    ///
    ///             <Column>
    ///                 {"Second column"}
    ///             </Column>
    ///         </Columns>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/columns/responsiveness/#different-column-sizes-per-breakpoint
    #[prop_or_default]
    pub viewport_sizes: HashMap<Viewport, Size>,
    /// Sets the offset of the [Bulma column element][bd].
    ///
    /// Sets the offset of the [Bulma column element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::columns::{Column, Columns, Size};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Columns>
    ///             <Column size={Size::Half} offset={Size::OneQuarter}>
    ///                 {"This is some text in a column"}
    ///             </Column>
    ///         </Columns>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/column/sizes/#offset
    #[prop_or_default]
    pub offset: Option<Size>,
    /// Whether or not the [Bulma column element][bd] should be narrow.
    ///
    /// Whether or not the [Bulma column element][bd], which will receive these
    /// properties, will be narrow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::columns::{Column, Columns};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Columns>
    ///             <Column narrow=true>
    ///                 {"First column"}
    ///             </Column>
    ///
    ///             <Column>
    ///                 {"Second column"}
    ///             </Column>
    ///         </Columns>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/columns/sizes/#narrow-column
    #[prop_or_default]
    pub narrow: bool,
    /// Whether or not the [Bulma column element][bd] should be narrow for a
    /// viewport.
    ///
    /// Whether or not the [Bulma column element][bd], which will receive these
    /// properties, will be narrow for a viewport.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashSet;
    ///
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     columns::{Column, Columns},
    ///     helpers::visibility::Viewport,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     let mut narrow_viewports = HashSet::new();
    ///     narrow_viewports.insert(Viewport::Mobile);
    ///     html! {
    ///         <Columns>
    ///             <Column {narrow_viewports}>
    ///                 {"First column"}
    ///             </Column>
    ///
    ///             <Column>
    ///                 {"Second column"}
    ///             </Column>
    ///         </Columns>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/columns/sizes/#narrow-column
    #[prop_or_default]
    pub narrow_viewports: HashSet<Viewport>,
    /// The list of elements found inside the [column element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma column element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/columns/basics
    pub children: Children,
}

/// Yew implementation of the [Bulma column element][bd].
///
/// Yew implementation of the column element, based on the specification found
/// in the [Bulma column element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::columns::{Column, Columns};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Columns>
///             <Column>
///                 {"This is some text in a column."}
///             </Column>
///         </Columns>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/columns/basics
#[function_component(Column)]
pub fn column(props: &ColumnProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| format!("{IS_PREFIX}-{size}"))
        .unwrap_or("".to_owned());
    let mut viewport_sizes = String::new();
    props.viewport_sizes.iter().for_each(|(size, viewport)| {
        viewport_sizes.push_str(&format!("{IS_PREFIX}-{size}-{viewport}"))
    });
    let offset = props
        .offset
        .as_ref()
        .map(|offset| format!("{IS_OFFSET_PREFIX}-{offset}"))
        .unwrap_or("".to_owned());
    let narrow = if props.narrow { IS_NARROW } else { "" };
    let mut narrow_viewports = String::new();
    props
        .narrow_viewports
        .iter()
        .for_each(|viewport| narrow_viewports.push_str(&format!("{IS_NARROW}-{viewport}")));
    let class = ClassBuilder::default()
        .with_custom_class("column")
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .with_custom_class(&size)
        .with_custom_class(&offset)
        .with_custom_class(narrow)
        .with_custom_class(&viewport_sizes)
        .with_custom_class(&narrow_viewports)
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
