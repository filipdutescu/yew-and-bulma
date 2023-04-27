/// Used to create classes using the `has-text-*` prefix.
///
/// Used to create classes using the `has-text-*` prefix, such as those from
/// the [typography Bulma helpers][bd].
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
///
/// [bd]: https://bulma.io/documentation/helpers/typography-helpers/
pub const HAS_TEXT_PREFIX: &str = "has-text";
/// Used to create classes using the `has-text-weight-*` prefix.
///
/// Used to create classes using the `has-text-weight-*` prefix, such as those
/// from the [typography Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::typography::TextWeight,
///     utils::constants::HAS_TEXT_WEIGHT_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the light font weight.
/// #[function_component(LightTextDiv)]
/// fn light_text_div() -> Html {
///     let text_weight = TextWeight::Light;
///     let class = classes![format!("{HAS_TEXT_WEIGHT_PREFIX}-{text_weight}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#text-weight
pub const HAS_TEXT_WEIGHT_PREFIX: &str = "has-text-weight";
/// Used to create classes using the `has-background-*` prefix.
///
/// Used to create classes using the `has-background-*` prefix, such as those
/// from the [color Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::color::BackgroundColor,
///     utils::constants::HAS_BACKGROUND_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the background color set to primary.
/// #[function_component(ColoredBackgroundDiv)]
/// fn colored_background_div() -> Html {
///     let background_color = BackgroundColor::Primary;
///     let class = classes![format!("{HAS_BACKGROUND_PREFIX}-{background_color}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/color-helpers/#background-color
pub const HAS_BACKGROUND_PREFIX: &str = "has-background";
/// Used to create classes using the `m-*` or `m*-*` prefix.
///
/// Used to create classes using the `m-*` or `m*-*` prefix, such as those from
/// the [spacing Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::spacing::Direction,
///     helpers::spacing::Spacing,
///     utils::constants::MARGIN_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the marign set to 2.
/// #[function_component(SpacedDiv)]
/// fn spaced_div() -> Html {
///     let class = classes![
///         format!("{MARGIN_PREFIX}{}-{}", Direction::All, Spacing::Two),
///     ];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/spacing-helpers/
pub const MARGIN_PREFIX: &str = "m";
/// Used to create classes using the `p-*` or `p*-*` prefix.
///
/// Used to create classes using the `p-*` or `p*-*` prefix, such as those from
/// the [spacing Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::spacing::Direction,
///     helpers::spacing::Spacing,
///     utils::constants::PADDING_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the padding set to 2.
/// #[function_component(SpacedDiv)]
/// fn spaced_div() -> Html {
///     let class = classes![
///         format!("{PADDING_PREFIX}{}-{}", Direction::All, Spacing::Two),
///     ];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/spacing-helpers/
pub const PADDING_PREFIX: &str = "p";
/// Used to create classes using the `is-size-*` prefix.
///
/// Used to create classes using the `is-size-*` prefix, such as those from the
/// [typography Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::typography::TextSize,
///     utils::constants::IS_SIZE_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the font size set to 3.
/// #[function_component(TextSize3Div)]
/// fn text_size_3_div() -> Html {
///     let text_size = TextSize::Three;
///     let class = classes![format!("{IS_SIZE_PREFIX}-{text_size}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#size
pub const IS_SIZE_PREFIX: &str = "is-size";
/// Used to create classes using the `is-*` prefix.
///
/// Used to create classes using the `is-*` prefix, such as those from the
/// [visibility Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::visibility::Display,
///     utils::constants::IS_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the display set to flex.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     let display = Display::Flex;
///     let class = classes![format!("{IS_PREFIX}-{display}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/visibility-helpers/#show
pub const IS_PREFIX: &str = "is";
/// Used to create classes using the `is-family-*` prefix.
///
/// Used to create classes using the `is-family-*` prefix, such as those from
/// the [typography Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::typography::FontFamily,
///     utils::constants::IS_FONT_FAMILY_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the code font family.
/// #[function_component(CodeFontDiv)]
/// fn code_font_div() -> Html {
///     let font_family = FontFamily::Code;
///     let class = classes![format!("{IS_FONT_FAMILY_PREFIX}-{font_family}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#font-family
pub const IS_FONT_FAMILY_PREFIX: &str = "is-family";
/// Used to create classes using the `is-flex-direction-*` prefix.
///
/// Used to create classes using the `is-flex-direction-*` prefix, such as
/// those from the [Flexbox Bulma helpers][bd].
///
/// # Examples
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
pub const IS_FLEX_DIRECTION_PREFIX: &str = "is-flex-direction";
/// Used to create classes using the `is-flex-wrap-*` prefix.
///
/// Used to create classes using the `is-flex-wrap-*` prefix, such as those
/// from the [Flexbox Bulma helpers][bd].
///
/// # Examples
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
pub const IS_FLEX_WRAP_PREFIX: &str = "is-flex-wrap";
/// Used to create classes using the `is-justify-content-*` prefix.
///
/// Used to create classes using the `is-justify-content-*` prefix, such as
/// those from the [Flexbox Bulma helpers][bd].
///
/// # Examples
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
pub const IS_JUSTIFY_CONTENT_PREFIX: &str = "is-justify-content";
/// Used to create classes using the `is-align-content-*` prefix.
///
/// Used to create classes using the `is-align-content-*` prefix, such as those
/// from the [Flexbox Bulma helpers][bd].
///
/// # Examples
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
pub const IS_ALIGN_CONTENT_PREFIX: &str = "is-align-content";
/// Used to create classes using the `is-align-items-*` prefix.
///
/// Used to create classes using the `is-align-items-*` prefix, such as those
/// from the [Flexbox Bulma helpers][bd].
///
/// # Examples
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
pub const IS_ALIGN_ITEMS_PREFIX: &str = "is-align-items";
/// Used to create classes using the `is-align-self-*` prefix.
///
/// Used to create classes using the `is-align-self-*` prefix, such as those
/// from the [Flexbox Bulma helpers][bd].
///
/// # Examples
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
pub const IS_ALIGN_SELF_PREFIX: &str = "is-align-self";
/// Used to create classes using the `is-flow-grow-*` prefix.
///
/// Used to create classes using the `is-flow-grow-*` prefix, such as those
/// from the [Flexbox Bulma helpers][bd].
///
/// # Examples
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
/// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#flex-grow-and-flex-shrink
pub const IS_FLEX_GROW_PREFIX: &str = "is-flex-grow";
/// Used to create classes using the `is-flow-shrink-*` prefix.
///
/// Used to create classes using the `is-flow-shrink-*` prefix, such as those
/// from the [Flexbox Bulma helpers][bd].
///
/// # Examples
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
pub const IS_FLEX_SHRINK_PREFIX: &str = "is-flex-shrink";
/// Defines the `is-clearfix` [Bulma helper class][bd].
///
/// Defines the `is-clearfix` class described in the [other Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::IS_CLEARFIX;
///
/// // Create a `<div>` HTML element that fixes its floating children.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     html!{
///         <div class={IS_CLEARFIX}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/other-helpers/
pub const IS_CLEARFIX: &str = "is-clearfix";
/// Defines the `is-pulled-left` [Bulma helper class][bd].
///
/// Defines the `is-pulled-left` class described in the
/// [other Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::IS_PULLED_LEFT;
///
/// // Create a `<div>` HTML element that is moved to the left.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     html!{
///         <div class={IS_PULLED_LEFT}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/other-helpers/
pub const IS_PULLED_LEFT: &str = "is-pulled-left";
/// Defines the `is-pulled-right` [Bulma helper class][bd].
///
/// Defines the `is-pulled-right` class described in the
/// [other Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::IS_PULLED_RIGHT;
///
/// // Create a `<div>` HTML element that is moved to the right.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     html!{
///         <div class={IS_PULLED_RIGHT}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/other-helpers/
pub const IS_PULLED_RIGHT: &str = "is-pulled-right";
/// Defines the `is-overlay` [Bulma helper class][bd].
///
/// Defines the `is-overlay` class described in the [other Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::IS_OVERLAY;
///
/// // Create a `<div>` HTML element that covers its first positioned parent.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     html!{
///         <div class={IS_OVERLAY}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/other-helpers/
pub const IS_OVERLAY: &str = "is-overlay";
/// Defines the `is-clipped` [Bulma helper class][bd].
///
/// Defines the `is-clipped` class described in the [other Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::IS_CLIPPED;
///
/// // Create a `<div>` HTML element that has its overflow hidden.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     html!{
///         <div class={IS_CLIPPED}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/other-helpers/
pub const IS_CLIPPED: &str = "is-clipped";
/// Defines the `is-shadowless` [Bulma helper class][bd].
///
/// Defines the `is-shadowless` class described in the
/// [other Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::IS_RADIUSLESS;
///
/// // Create a `<div>` HTML element that is radiusless.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     html!{
///         <div class={IS_RADIUSLESS}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/other-helpers/
pub const IS_RADIUSLESS: &str = "is-radiusless";
/// Defines the `is-radiusless` [Bulma helper class][bd].
///
/// Defines the `is-radiusless` class described in the
/// [other Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::IS_SHADOWLESS;
///
/// // Create a `<div>` HTML element that is shadowless.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     html!{
///         <div class={IS_SHADOWLESS}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/other-helpers/
pub const IS_SHADOWLESS: &str = "is-shadowless";
/// Defines the `is-unselectable` [Bulma helper class][bd].
///
/// Defines the `is-unselectable` class described in the
/// [other Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::IS_UNSELECTABLE;
///
/// // Create a `<div>` HTML element that is unselectable.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     html!{
///         <div class={IS_UNSELECTABLE}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/other-helpers/
pub const IS_UNSELECTABLE: &str = "is-unselectable";
/// Defines the `is-clickable` [Bulma helper class][bd].
///
/// Defines the `is-clickable` class described in the
/// [other Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::IS_CLICKABLE;
///
/// // Create a `<div>` HTML element that displays as clickable.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     html!{
///         <div class={IS_CLICKABLE}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/other-helpers/
pub const IS_CLICKABLE: &str = "is-clickable";
/// Defines the `is-relative` [Bulma class][bd].
///
/// Defines the `is-relative` class described in the [other Bulma helpers][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::IS_RELATIVE;
///
/// // Create a `<div>` HTML element that has the position set to relative.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     html!{
///         <div class={IS_RELATIVE}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/other-helpers/
pub const IS_RELATIVE: &str = "is-relative";
/// Defines the `is-light` [Bulma class][bd].
///
/// Defines the `is-light` class, used for shading the color of various
/// elements and components, such as the [`crate::elements::button::Button`] or
/// [`crate::elements::tag::Tag`].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::IS_LIGHT;
///
/// // Create a `<div>` HTML element that has the color set to light.
/// #[function_component(LightDiv)]
/// fn light_div() -> Html {
///     html!{
///         <div class={IS_LIGHT}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/customize/variables/
pub const IS_LIGHT: &str = "is-light";
/// Used to create classes using the `are-*` prefix.
///
/// Used to create classes using the `are-*` prefix, such as size modifiers for
/// the [`crate::elements::button::Buttons`].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::ARE_PREFIX;
///
/// // Create a `<div>` HTML element that has the buttons small.
/// #[function_component(SmallButtonsDiv)]
/// fn small_buttons_div() -> Html {
///     let class = format!("buttons {ARE_PREFIX}-small");
///
///     html!{
///         <div {class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/customize/variables/
pub const ARE_PREFIX: &str = "are";
/// Used to create classes using the `is-offset-*` offset.
///
/// Used to create classes using the `is-offset-*` offset, such as those from
/// the [Bulma column sizes][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::visibility::Display,
///     utils::constants::IS_OFFSET_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the offset set to half.
/// #[function_component(OffsetDiv)]
/// fn offset_div() -> Html {
///     let class = classes![format!("{IS_OFFSET_PREFIX}-half")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/columns/sizes/#offset
pub const IS_OFFSET_PREFIX: &str = "is-offset";
/// Defines the `is-narrow` [Bulma class][bd].
///
/// Defines the `is-narrow` class, used for elements such as the
/// [`crate::elements::table::Table`] or [`crate::columns::Column`].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::utils::constants::IS_NARROW;
///
/// // Create a `<div>` HTML element that has the narrow class set.
/// #[function_component(NarrowDiv)]
/// fn narrow_div() -> Html {
///     html!{
///         <div class={IS_NARROW}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/customize/variables/
pub const IS_NARROW: &str = "is-narrow";
/// Used to create classes using the `has-*` prefix.
///
/// Used to create classes using the `has-*` prefix, such as those from the
/// [breadcrumb Bulma component separators][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     components::breadcrumb::Separator,
///     utils::constants::HAS_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the dot separator breadcrumb class set.
/// #[function_component(DotSeparatorDiv)]
/// fn dot_separator_div() -> Html {
///     let dot_separator = Separator::Dot;
///     let class = classes![String::from(dot_separator)];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/breadcrumb/#alternative-separators
pub const HAS_PREFIX: &str = "has";
