use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, AttrValue, Children, Classes,
    Html, Properties,
};

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
/// use yew_and_bulma::column::{Column, Columns};
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
/// [bd]: https://bulma.io/documentation/elements/columns/basics
#[derive(Properties, PartialEq)]
pub struct ColumnsProperties {
    /// Sets the [HTML id attribute][id] of the element.
    ///
    /// Sets the [HTML id attrbiute][id] of the element which will receive
    /// these properties.
    ///
    /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Sets the classes to be appended to the [HTML class attribute][class].
    ///
    /// Sets the classes to be appended to [HTML class attrbiute][class] of the
    /// element which will receive these properties.
    ///
    /// [class]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class
    #[prop_or_default]
    pub class: Option<Classes>,
    /// Sets the viewport from which the columns inside the [Bulma columns element][bd]
    /// should be active.
    ///
    /// Sets the viewport from which the columns inside the
    /// [Bulma column element][bd] should be active, which will receive these
    /// properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     column::{Column, Columns},
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
    /// [bd]: https://bulma.io/documentation/elements/columns/basics
    pub children: ChildrenRenderer<ColumnsItem>,
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
/// use yew_and_bulma::column::{Column, Columns};
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
/// [bd]: https://bulma.io/documentation/elements/columns/basics
#[derive(Clone, PartialEq)]
pub enum ColumnsItem {
    Columns(VChild<Columns>),
    Column(VChild<Column>),
}

impl From<VChild<Columns>> for ColumnsItem {
    fn from(value: VChild<Columns>) -> Self {
        ColumnsItem::Columns(value)
    }
}

impl From<VChild<Column>> for ColumnsItem {
    fn from(value: VChild<Column>) -> Self {
        ColumnsItem::Column(value)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Html> for ColumnsItem {
    fn into(self) -> Html {
        match self {
            ColumnsItem::Columns(columns) => columns.into(),
            ColumnsItem::Column(column) => column.into(),
        }
    }
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
/// use yew_and_bulma::column::{Column, Columns};
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
/// [bd]: https://bulma.io/documentation/elements/columns/basics
#[function_component(Columns)]
pub fn columns(props: &ColumnsProperties) -> Html {
    let viewport = props
        .viewport
        .as_ref()
        .map(|viewport| format!("{IS_PREFIX}-{viewport}"))
        .unwrap_or("".to_owned());
    let multiline = if props.multiline { "is-multiline" } else { "" };
    let gapless = if props.gapless { "is-gapless" } else { "" };
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
        .build();

    html! {
        <div id={props.id.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}

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
/// use yew_and_bulma::column::{Column, Columns};
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
/// [bd]: https://bulma.io/documentation/elements/columns/basics
#[derive(Properties, PartialEq)]
pub struct ColumnProperties {
    /// Sets the [HTML id attribute][id] of the element.
    ///
    /// Sets the [HTML id attrbiute][id] of the element which will receive
    /// these properties.
    ///
    /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Sets the classes to be appended to the [HTML class attribute][class].
    ///
    /// Sets the classes to be appended to [HTML class attrbiute][class] of the
    /// element which will receive these properties.
    ///
    /// [class]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class
    #[prop_or_default]
    pub class: Option<Classes>,
    /// Sets the size of the [Bulma column element][bd].
    ///
    /// Sets the size of the [Bulma column element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::column::{Column, Columns, Size};
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
    /// [bd]: https://bulma.io/documentation/elements/columns/sizes/
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
    ///     column::{Column, Columns, Size},
    ///     helpers::visbility::Viewport,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     let mut viewport_sizes = HashMap::new::<Size, Viewport>();
    ///     viewport_sizes.insert(Size::Half, Viewport::Mobile);
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
    /// [bd]: https://bulma.io/documentation/elements/columns/responsiveness/#different-column-sizes-per-breakpoint
    #[prop_or_default]
    pub viewport_sizes: HashMap<Size, Viewport>,
    /// Sets the offset of the [Bulma column element][bd].
    ///
    /// Sets the offset of the [Bulma column element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::column::{Column, Columns, Size};
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
    /// [bd]: https://bulma.io/documentation/elements/column/sizes/#offset
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
    /// use yew_and_bulma::column::{Column, Columns};
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
    ///     column::{Column, Columns},
    ///     helpers::visibility::Viewport,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     let mut narrow_viewports = HashSet::new::<Viewport>();
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
    /// [bd]: https://bulma.io/documentation/elements/columns/basics
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
/// use yew_and_bulma::column::{Column, Columns};
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
/// [bd]: https://bulma.io/documentation/elements/columns/basics
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
        <div id={props.id.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
