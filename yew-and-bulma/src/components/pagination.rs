use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, Html, Properties,
};
use yew_and_bulma_macros::{base_component_properties, TypedChildren};

use crate::utils::BaseComponent;
use crate::utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size};

/// Defines the possible alignment of a [Bulma pagination component][bd].
///
/// Defines the possible alignment of a [Bulma pagination element][bd], inside
/// its parent.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Align,
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination align={Align::Center}>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/#alignment
#[derive(PartialEq)]
pub enum Align {
    // TODO: use #[default] when updating the MSRV
    Left,
    Center,
    Right,
}

impl From<&Align> for String {
    fn from(value: &Align) -> Self {
        match value {
            Align::Left => "".to_owned(),
            Align::Center => format!("{IS_PREFIX}-centered"),
            Align::Right => format!("{IS_PREFIX}-right"),
        }
    }
}

/// Defines the properties of the [Bulma pagination component][bd].
///
/// Defines the properties of the pagination component, based on the
/// specification found in the [Bulma pagination component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct PaginationProperties {
    /// Sets the size of the [pagination component][bd].
    ///
    /// Sets the size of the [Bulma pagination component][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     layout::pagination::{
    ///         Pagination,
    ///         PaginationEllipsis,
    ///         PaginationLink,
    ///         PaginationList,
    ///         PaginationNext,
    ///         PaginationPrevious,
    ///     },
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Pagination size={Size::Large}>
    ///             <PaginationList>
    ///                 <PaginationLink page={1} />
    ///                 <PaginationEllipsis />
    ///                 <PaginationLink page={24} />
    ///                 <PaginationLink page={25} />
    ///                 <PaginationLink page={26} />
    ///                 <PaginationEllipsis />
    ///                 <PaginationLink page={50} />
    ///             </PaginationList>
    ///
    ///             <PaginationNext>{"Next"}</PaginationNext>
    ///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
    ///         </Pagination>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/pagination/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
    /// Sets the alignment of a [Bulma pagination component][bd].
    ///
    /// Sets the alignment of a [Bulma pagination component][bd], which will
    /// receive these properties, inside its parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::pagination::{
    ///     Align,
    ///     Pagination,
    ///     PaginationEllipsis,
    ///     PaginationLink,
    ///     PaginationList,
    ///     PaginationNext,
    ///     PaginationPrevious,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Pagination align={Align::Center}>
    ///             <PaginationList>
    ///                 <PaginationLink page={1} />
    ///                 <PaginationEllipsis />
    ///                 <PaginationLink page={24} />
    ///                 <PaginationLink page={25} />
    ///                 <PaginationLink page={26} />
    ///                 <PaginationEllipsis />
    ///                 <PaginationLink page={50} />
    ///             </PaginationList>
    ///
    ///             <PaginationNext>{"Next"}</PaginationNext>
    ///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
    ///         </Pagination>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/pagination/
    #[prop_or(Align::Left)]
    pub align: Align,
    /// Whether or not the [Bulma pagination element][bd] should be rounded.
    ///
    /// Whether or not the [Bulma pagination element][bd], which will receive these
    /// properties, will be rounded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::pagination::{
    ///     Pagination,
    ///     PaginationEllipsis,
    ///     PaginationLink,
    ///     PaginationList,
    ///     PaginationNext,
    ///     PaginationPrevious,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Pagination rounded=true>
    ///             <PaginationList>
    ///                 <PaginationLink page={1} />
    ///                 <PaginationEllipsis />
    ///                 <PaginationLink page={24} />
    ///                 <PaginationLink page={25} />
    ///                 <PaginationLink page={26} />
    ///                 <PaginationEllipsis />
    ///                 <PaginationLink page={50} />
    ///             </PaginationList>
    ///
    ///             <PaginationNext>{"Next"}</PaginationNext>
    ///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
    ///         </Pagination>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/pagination/#styles
    #[prop_or_default]
    pub rounded: bool,
    /// The list of elements found inside the [pagination component][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma pagination component][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/pagination/
    pub children: ChildrenRenderer<PaginationItem>,
}

/// Yew implementation of the [Bulma pagination component][bd].
///
/// Yew implementation of the pagination component, based on the specification
/// found in the [Bulma pagination component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[function_component(Pagination)]
pub fn pagination(props: &PaginationProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| {
            if *size == Size::Normal {
                "".to_owned()
            } else {
                format!("{IS_PREFIX}-{size}")
            }
        })
        .unwrap_or("".to_owned());
    let rounded = if props.rounded { "is-rounded" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("pagination")
        .with_custom_class(&size)
        .with_custom_class(&String::from(&props.align))
        .with_custom_class(rounded)
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="nav" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the possible types of children from a [Bulma pagination component][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma pagination component][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[derive(Clone, PartialEq, TypedChildren)]
pub enum PaginationItem {
    PaginationList(VChild<PaginationList>),
    PaginationNext(VChild<PaginationNext>),
    PaginationPrevious(VChild<PaginationPrevious>),
}

/// Defines the properties of the [Bulma pagination next element][bd].
///
/// Defines the properties of the pagination next element, based on the
/// specification found in the [Bulma pagination component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct PaginationNextProperties {
    /// Whether or not the [Bulma pagination next element][bd] should be disabled.
    ///
    /// Whether or not the [Bulma pagination next element][bd], which will
    /// receive these properties, will be disabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::pagination::{
    ///     Pagination,
    ///     PaginationEllipsis,
    ///     PaginationLink,
    ///     PaginationList,
    ///     PaginationNext,
    ///     PaginationPrevious,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Pagination>
    ///             <PaginationList>
    ///                 <PaginationLink page={1} />
    ///                 <PaginationEllipsis />
    ///                 <PaginationLink page={24} />
    ///                 <PaginationLink page={25} />
    ///                 <PaginationLink page={26} />
    ///                 <PaginationEllipsis />
    ///                 <PaginationLink page={50} />
    ///             </PaginationList>
    ///
    ///             <PaginationNext disabled=true>{"Next"}</PaginationNext>
    ///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
    ///         </Pagination>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/pagination/
    #[prop_or_default]
    pub disabled: bool,
    /// The list of elements found inside the [pagination next element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma pagination next element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/pagination/
    pub children: Children,
}

/// Yew implementation of the [Bulma pagination next element][bd].
///
/// Yew implementation of the pagination next element, based on the
/// specification found in the [Bulma pagination component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[function_component(PaginationNext)]
pub fn pagination_next(props: &PaginationNextProperties) -> Html {
    let disabled = if props.disabled { "is-disabled" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("pagination-next")
        .with_custom_class(disabled)
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="a" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma pagination previous element][bd].
///
/// Defines the properties of the pagination previous element, based on the
/// specification found in the [Bulma pagination component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct PaginationPreviousProperties {
    /// Whether or not the [Bulma pagination previous element][bd] should be disabled.
    ///
    /// Whether or not the [Bulma pagination previous element][bd], which will
    /// receive these properties, will be disabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::pagination::{
    ///     Pagination,
    ///     PaginationEllipsis,
    ///     PaginationLink,
    ///     PaginationList,
    ///     PaginationNext,
    ///     PaginationPrevious,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Pagination>
    ///             <PaginationList>
    ///                 <PaginationLink page={1} />
    ///                 <PaginationEllipsis />
    ///                 <PaginationLink page={24} />
    ///                 <PaginationLink page={25} />
    ///                 <PaginationLink page={26} />
    ///                 <PaginationEllipsis />
    ///                 <PaginationLink page={50} />
    ///             </PaginationList>
    ///
    ///             <Paginationprevious>{"Next"}</PaginationNext>
    ///             <PaginationPrevious disabled=true>{"Previous"}</PaginationPrevious>
    ///         </Pagination>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/pagination/
    #[prop_or_default]
    pub disabled: bool,
    /// The list of elements found inside the [pagination previous element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma pagination previous element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/pagination/
    pub children: Children,
}

/// Yew implementation of the [Bulma pagination previous element][bd].
///
/// Yew implementation of the pagination previous element, based on the
/// specification found in the [Bulma pagination component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[function_component(PaginationPrevious)]
pub fn pagination_previous(props: &PaginationPreviousProperties) -> Html {
    let disabled = if props.disabled { "is-disabled" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("pagination-previous")
        .with_custom_class(disabled)
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="a" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma pagination list element][bd].
///
/// Defines the properties of the pagination list element, based on the
/// specification found in the [Bulma pagination component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct PaginationListProperties {
    /// The list of elements found inside the [pagination list element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma pagination list element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/pagination/
    pub children: ChildrenRenderer<PaginationListItem>,
}

/// Yew implementation of the [Bulma pagination list element][bd].
///
/// Yew implementation of the pagination list element, based on the
/// specification found in the [Bulma pagination component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[function_component(PaginationList)]
pub fn pagination_list(props: &PaginationListProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("pagination-list")
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="ul" {class} ..props.into()>
            { for props.children.iter().map(|c| yew::html! { <li>{c}</li> }) }
        </BaseComponent>
    }
}

/// Defines the possible types of children from a
/// [Bulma pagination list element][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma pagination list element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[derive(Clone, PartialEq, TypedChildren)]
pub enum PaginationListItem {
    PaginationEllipsis(VChild<PaginationEllipsis>),
    PaginationLink(VChild<PaginationLink>),
}

/// Defines the properties of the [Bulma pagination ellipsis element][bd].
///
/// Defines the properties of the pagination ellipsis element, based on the
/// specification found in the [Bulma pagination component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct PaginationEllipsisProperties {
    /// The ellipsis of elements found inside the [pagination ellipsis element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma pagination ellipsis element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/pagination/
    #[prop_or_default]
    pub children: Option<Children>,
}

/// Yew implementation of the [Bulma pagination ellipsis element][bd].
///
/// Yew implementation of the pagination ellipsis element, based on the
/// specification found in the [Bulma pagination component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[function_component(PaginationEllipsis)]
pub fn pagination_ellipsis(props: &PaginationEllipsisProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("pagination-ellipsis")
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="span" {class} ..props.into()>
            if let Some(children) = &props.children {
                { for children.iter() }
            } else {
                {"…"}
            }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma pagination link element][bd].
///
/// Defines the properties of the pagination link element, based on the
/// specification found in the [Bulma pagination component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct PaginationLinkProperties {
    #[prop_or_default]
    pub current: bool,
    pub page: usize,
}

/// Yew implementation of the [Bulma pagination link element][bd].
///
/// Yew implementation of the pagination link element, based on the
/// specification found in the [Bulma pagination component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::pagination::{
///     Pagination,
///     PaginationEllipsis,
///     PaginationLink,
///     PaginationList,
///     PaginationNext,
///     PaginationPrevious,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Pagination>
///             <PaginationList>
///                 <PaginationLink page={1} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={24} />
///                 <PaginationLink page={25} />
///                 <PaginationLink page={26} />
///                 <PaginationEllipsis />
///                 <PaginationLink page={50} />
///             </PaginationList>
///
///             <PaginationNext>{"Next"}</PaginationNext>
///             <PaginationPrevious>{"Previous"}</PaginationPrevious>
///         </Pagination>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/pagination/
#[function_component(PaginationLink)]
pub fn pagination_link(props: &PaginationLinkProperties) -> Html {
    let current = if props.current { "is-current" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("pagination-link")
        .with_custom_class(current)
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="a" {class} ..props.into()>
            { props.page }
        </BaseComponent>
    }
}
