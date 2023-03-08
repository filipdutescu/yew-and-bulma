use std::fmt::Display;

/// Enum defining the possible direction values, as described in the
/// [Bulma documentation][bd].
///
/// Defines the possible direction values, according to the
/// [Bulma documentation][bd]. They _**must be combined**_ with the
/// [`crate::helpers::spacing::Spacing`] values and either the
/// [`crate::utils::constants::MARGIN_PREFIX`] or
/// [`crate::utils::constants::PADDING_PREFIX`]. Since all of the Bulma classes
/// use the `m-*`, `m*-*`, `p-*` or `p*-*` prefixes, both are needed to be
/// included when formatting the display value. This can be simplified by using
/// the [`crate::utils::class::ClassBuilder`] instead of manually handling
/// creation of the class strings.
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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    All,
    Top,
    Right,
    Bottom,
    Left,
    Horizontal,
    Vertical,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction_name = match self {
            Direction::All => "",
            Direction::Top => "t",
            Direction::Right => "r",
            Direction::Bottom => "b",
            Direction::Left => "l",
            Direction::Horizontal => "x",
            Direction::Vertical => "y",
        };

        write!(f, "{direction_name}")
    }
}

/// Enum defining the possible spacing values, as described in the
/// [Bulma documentation][bd].
///
/// Defines the possible spacing values, according to the
/// [Bulma documentation][bd]. They _**must be combined**_ with the
/// [`crate::helpers::spacing::Direction`] values and either the
/// [`crate::utils::constants::MARGIN_PREFIX`] or
/// [`crate::utils::constants::PADDING_PREFIX`]. Since all of the Bulma classes
/// use the `m-*`, `m*-*`, `p-*` or `p*-*` prefixes, both are needed to be
/// included when formatting the display value. This can be simplified by using
/// the [`crate::utils::class::ClassBuilder`] instead of manually handling
/// creation of the class strings.
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
/// // Create a `<div>` HTML element that has the margin and padding set to 2.
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
/// // Create a `<div>` HTML element that has the margin and padding set to 2.
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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Spacing {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Display for Spacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spacing_value = match self {
            Spacing::Zero => "0",
            Spacing::One => "1",
            Spacing::Two => "2",
            Spacing::Three => "3",
            Spacing::Four => "4",
            Spacing::Five => "5",
            Spacing::Six => "6",
        };

        write!(f, "{spacing_value}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Direction::All, "" ; "all converts to empty string")]
    #[test_case(Direction::Top, "t" ; "top converts to t")]
    #[test_case(Direction::Right, "r" ; "right converts to r")]
    #[test_case(Direction::Bottom, "b" ; "bottom converts to b")]
    #[test_case(Direction::Left, "l" ; "left converts to l")]
    #[test_case(Direction::Horizontal, "x" ; "horizontal converts to x")]
    #[test_case(Direction::Vertical, "y" ; "vertical converts to y")]
    fn direction_values_to_string(direction: Direction, expected_direction: &str) {
        let converted_direction = format!("{direction}");

        assert_eq!(converted_direction, expected_direction);
    }

    #[test_case(Spacing::Zero, "0" ; "zero converts to 0")]
    #[test_case(Spacing::One, "1" ; "one converts to 1")]
    #[test_case(Spacing::Two, "2" ; "two converts to 2")]
    #[test_case(Spacing::Three, "3" ; "three converts to 3")]
    #[test_case(Spacing::Four, "4" ; "four converts to 4")]
    #[test_case(Spacing::Five, "5" ; "five converts to 5")]
    #[test_case(Spacing::Six, "6" ; "six converts to 6")]
    fn spacing_values_to_string(spacing: Spacing, expected_spacing: &str) {
        let converted_spacing = format!("{spacing}");

        assert_eq!(converted_spacing, expected_spacing);
    }
}
