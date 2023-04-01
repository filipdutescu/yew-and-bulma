/// The [Bulma color helpers][bd] Rust API.
///
/// Color helpers, as defined in the [Bulma documentation][bd]. Those include
/// text and background color. For the most part, they should be used with the
/// [`crate::utils::class::ClassBuilder`] struct.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::color::TextColor,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the text color set to primary.
/// #[function_component(ColoredTextDiv)]
/// fn colored_text_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_text_color(Some(TextColor::Primary))
///         .build();
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
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
/// [bd]: https://bulma.io/documentation/helpers/color-helpers/
pub mod color;
/// The [Bulma flexbox helpers][bd] Rust API.
///
/// Flexbox helpers, as defined in the [Bulma documentation][bd]. They define
/// all possible value that the CSS Flexbox properties can take. Some examples
/// would be:
///
/// * [`flex-direction`]
/// * [`flex-wrap`]
/// * [`justify-content`]
/// * etc.
///
/// > _More examples can be found in the [Bulma documentation][bd] and the
/// reference pages, such as [Mozilla's][mdn]._
///
/// For the most part, they should be used with the
/// [`crate::utils::class::ClassBuilder`] struct. They also require that the
/// component they are used on has the `is-flex` class or the CSS property
/// `display: flex` (see [`crate::helpers::visibility::Display`] for the Rust
/// API for that, provided by this crate).
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
/// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers
/// [`flex-direction`]: https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction
/// [`flex-wrap`]: https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap
/// [`justify-content`]: https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout
pub mod flexbox;
/// The [Bulma spacing helpers][bd] Rust API.
///
/// Spacing helpers, as defined in the [Bulma documentation][bd]. Those include
/// the direction and factor for [`margin`] and [`padding`]. For the most part,
/// they should be used with the [`crate::utils::class::ClassBuilder`] struct.
/// Both [`crate::helpers::spacing::Direction`] and
/// [`crate::helpers::spacing::Spacing`] should be used together to make sense.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::spacing::Direction,
///     helpers::spacing::Spacing,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the marign and padding set to 2.
/// #[function_component(SpacedDiv)]
/// fn spaced_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_margin(Direction::All, Spacing::Two)
///         .with_padding(Direction::All, Spacing::Two)
///         .build();
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
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
///     helpers::spacing::Direction,
///     helpers::spacing::Spacing,
///     utils::constants::MARGIN_PREFIX,
///     utils::constants::PADDING_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the marign and padding set to 2.
/// #[function_component(SpacedDiv)]
/// fn spaced_div() -> Html {
///     let class = classes![
///         format!("{MARGIN_PREFIX}{}-{}", Direction::All, Spacing::Two),
///         format!("{PADDING_PREFIX}{}-{}", Direction::All, Spacing::Two),
///     ];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/spacing-helpers
/// [`margin`]: https://developer.mozilla.org/en-US/docs/Web/CSS/margin
/// [`padding`]: https://developer.mozilla.org/en-US/docs/Web/CSS/padding
pub mod spacing;
/// The [Bulma typography helpers][bd] Rust API.
///
/// Typography helpers, as defined in the [Bulma documentation][bd]. Those
/// generally operate on CSS properties such as:
///
/// * [`font-size`]
/// * [`font-weight`]
/// * [`text-align`]
/// * [`font-style`]
/// * etc.
///
/// > _More CSS properties such as those presented above can be found in the
/// [Mozilla Developer Network Web Docs][mdn]_
///
/// For the most part, they should be use with the
/// [`crate::utils::class::ClassBuilder`] struct.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::typography::TextSize,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the text size set to 3.
/// #[function_component(TextSize3Div)]
/// fn text_size_3_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_text_size(Some(TextSize::Three))
///         .build();
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
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
///     helpers::typography::TextSize,
///     utils::constants::IS_SIZE_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the text size set to 3.
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
/// [bd]: https://bulma.io/documentation/helpers/typography-helpers
/// [`font-size`]: https://developer.mozilla.org/en-US/docs/Web/CSS/font-size
/// [`font-weight`]: https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight
/// [`text-align`]: https://developer.mozilla.org/en-US/docs/Web/CSS/text-align
/// [`font-style`]: https://developer.mozilla.org/en-US/docs/Web/CSS/font-style
/// [mdn]: https://developer.mozilla.org/en-US/docs/Learn/CSS/Styling_text/Fundamentals
pub mod typography;
/// The [Bulma visibility helpers][bd] Rust API.
///
/// Visibility helpers, as defined in the [Bulma documentation][bd]. Those
/// generally operate on the [`display`] CSS property. For the most part, they
/// should be use with the [`crate::utils::class::ClassBuilder`] struct. This
/// modules also constains the viewport modifier values, found in
/// [`crate::helpers::visibility::Viewport`].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::visibility::Display,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the display set to flex.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_display(Some(Display::Flex))
///         .build();
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
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
/// [bd]: https://bulma.io/documentation/helpers/visibility-helpers
/// [`display`]: https://developer.mozilla.org/en-US/docs/Web/CSS/display
pub mod visibility;
