use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, ChildrenWithProps,
    Html, Properties,
};
use yew_and_bulma_macros::{base_component_properties, TypedChildren};

use crate::utils::class::ClassBuilder;
use crate::utils::BaseComponent;

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
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
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
///             </LevelRight>
///         </Level>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/level/
#[derive(Clone, PartialEq, TypedChildren)]
pub enum LevelElement {
    LevelItem(VChild<LevelItem>),
    LevelLeft(VChild<LevelLeft>),
    LevelRight(VChild<LevelRight>),
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
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
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
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
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
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}
