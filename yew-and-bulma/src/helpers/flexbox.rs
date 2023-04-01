use std::fmt::Display;

/// Enum defining the possible flex direction values, as described in the
/// [Bulma documentation][bd].
///
/// Defines the possible [`flex-direction`] property values that Bulma provides
/// [helpers][bd] for. Since all of the Bulma classes use the
/// `is-flex-direction-*` prefix, this is needed to be included when formatting
/// the flex direction value. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::FlexDirection,
///     helpers::visibility::Display,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the column flex direction.
/// // The `<p>` children are there to highlight the direction.
/// #[function_component(FlexDirColDiv)]
/// fn flex_dir_col_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_display(Some(Display::Flex))
///         .with_flex_direction(Some(FlexDirection::Column))
///         .build();
///     html!{
///         <div class={class}>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// It is also possible to use them wihtout the
/// [`crate::utils::class::ClassBuilder`], instead formatting the class names
/// manually, using the constants defined in [`crate::utils::constants`]:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::FlexDirection,
///     helpers::visibility::Display,
///     utils::constants::IS_PREFIX,
///     utils::constants::IS_FLEX_DIRECTION_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the column flex direction.
/// // The `<p>` children are there to highlight the direction.
/// #[function_component(FlexDirColDiv)]
/// fn flex_dir_col_div() -> Html {
///     let display = Display::Flex;
///     let flex_direction = FlexDirection::Column;
///     let class = classes![
///         format!("{IS_PREFIX}-{display}"),
///         format!("{IS_FLEX_DIRECTION_PREFIX}-{flex_direction}"),
///     ];
///     html!{
///         <div class={class}>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#flex-direction
/// [`flex-direction`]: https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction
#[derive(Clone, Debug, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl Display for FlexDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flex_direction = match self {
            FlexDirection::Row => "row",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::Column => "column",
            FlexDirection::ColumnReverse => "column-reverse",
        };

        write!(f, "{flex_direction}")
    }
}

/// Enum defining the possible flex wrap values, as described in the
/// [Bulma documentation][bd].
///
/// Defines the possible [`flex-wrap`] property values that Bulma provides
/// [helpers][bd] for. Since all of the Bulma classes use the
/// `is-flex-wrap-*` prefix, this is needed to be included when formatting
/// the flex wrap value. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::FlexWrap,
///     helpers::visibility::Display,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the wrap flex wrap.
/// // The `<p>` children are there to highlight the wrap (might need resize
/// // of the screen size to become evident).
/// #[function_component(FlexWrapWrapDiv)]
/// fn flex_wrap_wrap_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_display(Some(Display::Flex))
///         .with_flex_wrap(Some(FlexWrap::Wrap))
///         .build();
///     html!{
///         <div class={class}>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// It is also possible to use them wihtout the
/// [`crate::utils::class::ClassBuilder`], instead formatting the class names
/// manually, using the constants defined in [`crate::utils::constants`]:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::FlexWrap,
///     helpers::visibility::Display,
///     utils::constants::IS_PREFIX,
///     utils::constants::IS_FLEX_WRAP_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the wrap flex wrap.
/// // The `<p>` children are there to highlight the wrap (might need resize
/// // of the screen size to become evident).
/// #[function_component(FlexWrapWrapDiv)]
/// fn flex_wrap_wrap_div() -> Html {
///     let display = Display::Flex;
///     let flex_wrap = FlexWrap::Wrap;
///     let class = classes![
///         format!("{IS_PREFIX}-{display}"),
///         format!("{IS_FLEX_WRAP_PREFIX}-{flex_wrap}"),
///     ];
///     html!{
///         <div class={class}>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#flex-wrap
/// [`flex-wrap`]: https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap
#[derive(Clone, Debug, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Display for FlexWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wrap_value = match self {
            FlexWrap::NoWrap => "no-wrap",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
        };

        write!(f, "{wrap_value}")
    }
}

/// Enum defining the possible justify content values, as described in the
/// [Bulma documentation][bd].
///
/// Defines the possible [`justify-content`] property values that Bulma provides
/// [helpers][bd] for. Since all of the Bulma classes use the
/// `is-justify-content-*` prefix, this is needed to be included when formatting
/// the justify content value. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::JustifyContent,
///     helpers::visibility::Display,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the center justify content value.
/// // The `<p>` children are there to highlight the justify (might need resize
/// // of the screen size to become evident).
/// #[function_component(JustifyContentCenterDiv)]
/// fn justify_content_center_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_display(Some(Display::Flex))
///         .with_justify_content(Some(JustifyContent::Center))
///         .build();
///     html!{
///         <div class={class}>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// It is also possible to use them wihtout the
/// [`crate::utils::class::ClassBuilder`], instead formatting the class names
/// manually, using the constants defined in [`crate::utils::constants`]:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::JustifyContent,
///     helpers::visibility::Display,
///     utils::constants::IS_PREFIX,
///     utils::constants::IS_JUSTIFY_CONTENT_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the center justify content value.
/// // The `<p>` children are there to highlight the justify (might need resize
/// // of the screen size to become evident).
/// #[function_component(JustifyContentCenterDiv)]
/// fn justify_content_center_div() -> Html {
///     let display = Display::Flex;
///     let justify_content = JustifyContent::Center;
///     let class = classes![
///         format!("{IS_PREFIX}-{display}"),
///         format!("{IS_JUSTIFY_CONTENT_PREFIX}-{justify_content}"),
///     ];
///     html!{
///         <div class={class}>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#justify-content
/// [`justify-content`]: https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content
#[derive(Clone, Debug, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Start,
    End,
    Left,
    Right,
}

impl Display for JustifyContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let justify_content = match self {
            JustifyContent::FlexStart => "flex-start",
            JustifyContent::FlexEnd => "flex-end",
            JustifyContent::Center => "center",
            JustifyContent::SpaceBetween => "space-between",
            JustifyContent::SpaceAround => "space-around",
            JustifyContent::SpaceEvenly => "space-evenly",
            JustifyContent::Start => "start",
            JustifyContent::End => "end",
            JustifyContent::Left => "left",
            JustifyContent::Right => "right",
        };

        write!(f, "{justify_content}")
    }
}

/// Enum defining the possible align content values, as described in the
/// [Bulma documentation][bd].
///
/// Defines the possible [`align-content`] property values that Bulma provides
/// [helpers][bd] for. Since all of the Bulma classes use the
/// `is-align-content-*` prefix, this is needed to be included when formatting
/// the align content value. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::AlignContent,
///     helpers::visibility::Display,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the center align content value.
/// // The `<p>` children are there to highlight the align (might need resize
/// // of the screen size to become evident).
/// #[function_component(AlignContentCenterDiv)]
/// fn align_content_center_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_display(Some(Display::Flex))
///         .with_align_content(Some(AlignContent::Center))
///         .build();
///     html!{
///         <div class={class}>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// It is also possible to use them wihtout the
/// [`crate::utils::class::ClassBuilder`], instead formatting the class names
/// manually, using the constants defined in [`crate::utils::constants`]:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::AlignContent,
///     helpers::visibility::Display,
///     utils::constants::IS_PREFIX,
///     utils::constants::IS_ALIGN_CONTENT_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the center align content value.
/// // The `<p>` children are there to highlight the align (might need resize
/// // of the screen size to become evident).
/// #[function_component(AlignContentCenterDiv)]
/// fn align_content_center_div() -> Html {
///     let display = Display::Flex;
///     let align_content = AlignContent::Center;
///     let class = classes![
///         format!("{IS_PREFIX}-{display}"),
///         format!("{IS_ALIGN_CONTENT_PREFIX}-{align_content}"),
///     ];
///     html!{
///         <div class={class}>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#align-content
/// [`align-content`]: https://developer.mozilla.org/en-US/docs/Web/CSS/align-content
#[derive(Clone, Debug, PartialEq)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
    Start,
    End,
    Baseline,
}

impl Display for AlignContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let align_content = match self {
            AlignContent::FlexStart => "flex-start",
            AlignContent::FlexEnd => "flex-end",
            AlignContent::Center => "center",
            AlignContent::SpaceBetween => "space-between",
            AlignContent::SpaceAround => "space-around",
            AlignContent::SpaceEvenly => "space-evenly",
            AlignContent::Stretch => "stretch",
            AlignContent::Start => "start",
            AlignContent::End => "end",
            AlignContent::Baseline => "baseline",
        };

        write!(f, "{align_content}")
    }
}

/// Enum defining the possible align items values, as described in the
/// [Bulma documentation][bd].
///
/// Defines the possible [`align-items`] property values that Bulma provides
/// [helpers][bd] for. Since all of the Bulma classes use the
/// `is-align-items-*` prefix, this is needed to be included when formatting
/// the align items value. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::AlignItems,
///     helpers::visibility::Display,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the center align items value.
/// // The `<p>` children are there to highlight the align (might need resize
/// // of the screen size to become evident).
/// #[function_component(AlignItemsCenterDiv)]
/// fn align_items_center_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_display(Some(Display::Flex))
///         .with_align_items(Some(AlignItems::Center))
///         .build();
///     html!{
///         <div class={class}>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// It is also possible to use them wihtout the
/// [`crate::utils::class::ClassBuilder`], instead formatting the class names
/// manually, using the constants defined in [`crate::utils::constants`]:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::AlignItems,
///     helpers::visibility::Display,
///     utils::constants::IS_PREFIX,
///     utils::constants::IS_ALIGN_ITEMS_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the center align items value.
/// // The `<p>` children are there to highlight the align (might need resize
/// // of the screen size to become evident).
/// #[function_component(AlignItemsCenterDiv)]
/// fn align_items_center_div() -> Html {
///     let display = Display::Flex;
///     let align_items = AlignItems::Center;
///     let class = classes![
///         format!("{IS_PREFIX}-{display}"),
///         format!("{IS_ALIGN_ITEMS_PREFIX}-{align_items}"),
///     ];
///     html!{
///         <div class={class}>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#align-items
/// [`align-items`]: https://developer.mozilla.org/en-US/docs/Web/CSS/align-items
#[derive(Clone, Debug, PartialEq)]
pub enum AlignItems {
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Start,
    End,
    SelfStart,
    SelfEnd,
}

impl Display for AlignItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let align_items = match self {
            AlignItems::Stretch => "stretch",
            AlignItems::FlexStart => "flex-start",
            AlignItems::FlexEnd => "flex-end",
            AlignItems::Center => "center",
            AlignItems::Baseline => "baseline",
            AlignItems::Start => "start",
            AlignItems::End => "end",
            AlignItems::SelfStart => "self-start",
            AlignItems::SelfEnd => "self-end",
        };

        write!(f, "{align_items}")
    }
}

/// Enum defining the possible align self values, as described in the
/// [Bulma documentation][bd].
///
/// Defines the possible [`align-self`] property values that Bulma provides
/// [helpers][bd] for. Since all of the Bulma classes use the
/// `is-align-self-*` prefix, this is needed to be included when formatting
/// the align self value. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::AlignSelf,
///     helpers::visibility::Display,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the center align self value.
/// // The `<p>` children are there to highlight the align (might need resize
/// // of the screen size to become evident).
/// #[function_component(AlignSelfCenterDiv)]
/// fn align_self_center_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_display(Some(Display::Flex))
///         .with_align_self(Some(AlignSelf::Center))
///         .build();
///     html!{
///         <div class={class}>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// It is also possible to use them wihtout the
/// [`crate::utils::class::ClassBuilder`], instead formatting the class names
/// manually, using the constants defined in [`crate::utils::constants`]:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::AlignSelf,
///     helpers::visibility::Display,
///     utils::constants::IS_PREFIX,
///     utils::constants::IS_ALIGN_SELF_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the center align self value.
/// // The `<p>` children are there to highlight the align (might need resize
/// // of the screen size to become evident).
/// #[function_component(AlignSelfCenterDiv)]
/// fn align_self_center_div() -> Html {
///     let display = Display::Flex;
///     let align_self = AlignSelf::Center;
///     let class = classes![
///         format!("{IS_PREFIX}-{display}"),
///         format!("{IS_ALIGN_SELF_PREFIX}-{align_self}"),
///     ];
///     html!{
///         <div class={class}>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#align-self
/// [`align-self`]: https://developer.mozilla.org/en-US/docs/Web/CSS/align-self
#[derive(Clone, Debug, PartialEq)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Display for AlignSelf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let align_self = match self {
            AlignSelf::Auto => "auto",
            AlignSelf::FlexStart => "flex-start",
            AlignSelf::FlexEnd => "flex-end",
            AlignSelf::Center => "center",
            AlignSelf::Baseline => "baseline",
            AlignSelf::Stretch => "stretch",
        };

        write!(f, "{align_self}")
    }
}

/// Enum defining the possible flex shrink and grow values, as described in the
/// [Bulma documentation][bd].
///
/// Defines the possible [`flex-grow`] and [`flex-shrink`] property values that
/// Bulma provides [helpers][bd] for. Since all of the Bulma classes use the
/// `is-flex-grow-*` or `is-flex-shrink-*` prefixes, those are needed to be
/// included when formatting the flex direction value. This can be simplified
/// by using the [`crate::utils::class::ClassBuilder`] instead of manually
/// handling creation of the class strings.
///
/// # Examples
///
/// Here is an example making use of the [`flex-grow`] property:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::FlexShrinkGrowFactor,
///     helpers::visibility::Display,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the flex display.
/// // The `<p>` children are there to highlight the flex grow (might need
/// // resize of the screen size to become evident). The first element is the
/// // one having the flex grow set.
/// #[function_component(FlexGrow2Div)]
/// fn flex_grow_2_div() -> Html {
///     let flex_display_class = ClassBuilder::default()
///         .with_display(Some(Display::Flex))
///         .build();
///     let flex_grow_class = ClassBuilder::default()
///         .with_flex_grow(Some(FlexShrinkGrowFactor::Two))
///         .build();
///     html!{
///         <div class={flex_display_class}>
///             <p class={flex_grow_class}>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// Here is an example making use of the [`flex-shrink`] property:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::FlexShrinkGrowFactor,
///     helpers::visibility::Display,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the flex display.
/// // The `<p>` children are there to highlight the flex shrink (might need
/// // resize of the screen size to become evident). The first element is the
/// // one having the flex shrink set.
/// #[function_component(FlexShrink2Div)]
/// fn flex_shrink_2_div() -> Html {
///     let flex_display_class = ClassBuilder::default()
///         .with_display(Some(Display::Flex))
///         .build();
///     let flex_shrink_class = ClassBuilder::default()
///         .with_flex_shrink(Some(FlexShrinkGrowFactor::Two))
///         .build();
///     html!{
///         <div class={flex_display_class}>
///             <p class={flex_shrink_class}>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// It is also possible to use them wihtout the
/// [`crate::utils::class::ClassBuilder`], instead formatting the class names
/// manually, using the constants defined in [`crate::utils::constants`]:
///
/// To use of the [`flex-grow`] property, with this method:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::FlexShrinkGrowFactor,
///     helpers::visibility::Display,
///     utils::constants::IS_PREFIX,
///     utils::constants::IS_FLEX_GROW_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the flex display.
/// // The `<p>` children are there to highlight the flex grow (might need
/// // resize of the screen size to become evident). The first element is the
/// // one having the flex grow set.
/// #[function_component(FlexGrow2Div)]
/// fn flex_grow_2_div() -> Html {
///     let display = Display::Flex;
///     let grow_factor = FlexShrinkGrowFactor::Two;
///     let display_class = classes![format!("{IS_PREFIX}-{display}")];
///     let grow_class = classes![format!("{IS_FLEX_GROW_PREFIX}-{grow_factor}")];
///     html!{
///         <div class={display_class}>
///             <p class={grow_class}>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// To use of the [`flex-shrink`] property, with this method:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::flexbox::FlexShrinkGrowFactor,
///     helpers::visibility::Display,
///     utils::constants::IS_PREFIX,
///     utils::constants::IS_FLEX_SHRINK_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the flex display.
/// // The `<p>` children are there to highlight the flex shrink (might need
/// // resize of the screen size to become evident). The first element is the
/// // one having the flex shrink set.
/// #[function_component(FlexShrink2Div)]
/// fn flex_shrink_2_div() -> Html {
///     let display = Display::Flex;
///     let shrink_factor = FlexShrinkGrowFactor::Two;
///     let display_class = classes![format!("{IS_PREFIX}-{display}")];
///     let shrink_class = classes![format!("{IS_FLEX_SHRINK_PREFIX}-{shrink_factor}")];
///     html!{
///         <div class={display_class}>
///             <p class={shrink_class}>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///             <p>{ "Lorem ispum..." }</p>
///         </div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#flex-grow-and-flex-shrink
/// [`flex-grow`]: https://developer.mozilla.org/en-US/docs/Web/CSS/flex-grow
/// [`flex-shrink`]: https://developer.mozilla.org/en-US/docs/Web/CSS/flex-shrink
#[derive(Clone, Debug, PartialEq)]
pub enum FlexShrinkGrowFactor {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Display for FlexShrinkGrowFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let factor_value = match self {
            FlexShrinkGrowFactor::Zero => "0",
            FlexShrinkGrowFactor::One => "1",
            FlexShrinkGrowFactor::Two => "2",
            FlexShrinkGrowFactor::Three => "3",
            FlexShrinkGrowFactor::Four => "4",
            FlexShrinkGrowFactor::Five => "5",
        };

        write!(f, "{factor_value}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(FlexDirection::Row, "row" ; "row converts to row")]
    #[test_case(FlexDirection::RowReverse, "row-reverse" ; "row reverse converts to row-reverse")]
    #[test_case(FlexDirection::Column, "column" ; "column  converts to column")]
    #[test_case(FlexDirection::ColumnReverse, "column-reverse" ; "column reverse converts to column-reverse")]
    fn flex_direction_values_to_string(given_direction: FlexDirection, expected_direction: &str) {
        let converted_direction = format!("{given_direction}");

        assert_eq!(converted_direction, expected_direction);
    }

    #[test_case(FlexWrap::NoWrap, "no-wrap" ; "no wrap converts to no-wrap")]
    #[test_case(FlexWrap::Wrap, "wrap" ; "wrap converts to wrap")]
    #[test_case(FlexWrap::WrapReverse, "wrap-reverse" ; "wrap reverse converts to wrap-reverse")]
    fn flex_wrap_values_to_string(given_wrap: FlexWrap, expected_wrap: &str) {
        let converted_wrap = format!("{given_wrap}");

        assert_eq!(converted_wrap, expected_wrap);
    }

    #[test_case(JustifyContent::FlexStart, "flex-start" ; "flex start converts to flex-start")]
    #[test_case(JustifyContent::FlexEnd, "flex-end" ; "flex end converts to flex-end")]
    #[test_case(JustifyContent::Center, "center" ; "center converts to center")]
    #[test_case(JustifyContent::SpaceBetween, "space-between" ; "space between converts to space-between")]
    #[test_case(JustifyContent::SpaceAround, "space-around" ; "space around converts to space-around")]
    #[test_case(JustifyContent::SpaceEvenly, "space-evenly" ; "space evenly converts to space-evenly")]
    #[test_case(JustifyContent::Start, "start" ; "start converts to start")]
    #[test_case(JustifyContent::End, "end" ; "end converts to end")]
    #[test_case(JustifyContent::Left, "left" ; "left converts to left")]
    #[test_case(JustifyContent::Right, "right" ; "right converts to right")]
    fn justify_content_values_to_string(given_justify: JustifyContent, expected_justify: &str) {
        let converted_justify = format!("{given_justify}");

        assert_eq!(converted_justify, expected_justify);
    }

    #[test_case(AlignContent::FlexStart, "flex-start" ; "flex start converts to flex-start")]
    #[test_case(AlignContent::FlexEnd, "flex-end" ; "flex end converts to flex-end")]
    #[test_case(AlignContent::Center, "center" ; "center converts to center")]
    #[test_case(AlignContent::SpaceBetween, "space-between" ; "space between converts to space-between")]
    #[test_case(AlignContent::SpaceAround, "space-around" ; "space around converts to space-around")]
    #[test_case(AlignContent::SpaceEvenly, "space-evenly" ; "space evenly converts to space-evenly")]
    #[test_case(AlignContent::Stretch, "stretch" ; "stretch converts to stretch")]
    #[test_case(AlignContent::Start, "start" ; "start converts to start")]
    #[test_case(AlignContent::End, "end" ; "end converts to end")]
    #[test_case(AlignContent::Baseline, "baseline" ; "baseline converts to baseline")]
    fn align_content_values_to_string(given_align: AlignContent, expected_align: &str) {
        let converted_align = format!("{given_align}");

        assert_eq!(converted_align, expected_align);
    }

    #[test_case(AlignItems::Stretch, "stretch" ; "stretch converts to stretch")]
    #[test_case(AlignItems::FlexStart, "flex-start" ; "flex start converts to flex-start")]
    #[test_case(AlignItems::FlexEnd, "flex-end" ; "flex end converts to flex-end")]
    #[test_case(AlignItems::Center, "center" ; "center converts to center")]
    #[test_case(AlignItems::Baseline, "baseline" ; "baseline converts to baseline")]
    #[test_case(AlignItems::Start, "start" ; "start converts to start")]
    #[test_case(AlignItems::End, "end" ; "end converts to end")]
    #[test_case(AlignItems::SelfStart, "self-start" ; "self start converts to self-start")]
    #[test_case(AlignItems::SelfEnd, "self-end" ; "start end converts to self-end")]
    fn align_items_values_to_string(given_align: AlignItems, expected_align: &str) {
        let converted_align = format!("{given_align}");

        assert_eq!(converted_align, expected_align);
    }

    #[test_case(AlignSelf::Auto, "auto" ; "auto converts to auto")]
    #[test_case(AlignSelf::FlexStart, "flex-start" ; "flex start converts to flex-start")]
    #[test_case(AlignSelf::FlexEnd, "flex-end" ; "flex end converts to flex-end")]
    #[test_case(AlignSelf::Center, "center" ; "center converts to center")]
    #[test_case(AlignSelf::Baseline, "baseline" ; "baseline converts to baseline")]
    #[test_case(AlignSelf::Stretch, "stretch" ; "stretch converts to stretch")]
    fn align_self_values_to_string(given_align: AlignSelf, expected_align: &str) {
        let converted_align = format!("{given_align}");

        assert_eq!(converted_align, expected_align);
    }

    #[test_case(FlexShrinkGrowFactor::Zero, "0" ; "zero converts to 0")]
    #[test_case(FlexShrinkGrowFactor::One, "1" ; "one converts to 1")]
    #[test_case(FlexShrinkGrowFactor::Two, "2" ; "two converts to 2")]
    #[test_case(FlexShrinkGrowFactor::Three, "3" ; "three converts to 3")]
    #[test_case(FlexShrinkGrowFactor::Four, "4" ; "four converts to 4")]
    #[test_case(FlexShrinkGrowFactor::Five, "5" ; "five converts to 5")]
    fn flex_shrink_grow_factor_values_to_string(
        factor: FlexShrinkGrowFactor,
        expected_factor: &str,
    ) {
        let converted_factor = format!("{factor}");

        assert_eq!(converted_factor, expected_factor);
    }
}
