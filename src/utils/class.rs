use std::collections::HashSet;

use yew::{classes, Classes};

use crate::{
    helpers::{
        color::{BackgroundColor, Color, TextColor},
        flexbox::{
            AlignContent, AlignItems, AlignSelf, FlexDirection, FlexShrinkGrowFactor, FlexWrap,
            JustifyContent,
        },
        spacing::{Direction, Spacing},
        typography::{FontFamily, TextAlignment, TextDecoration, TextSize, TextWeight},
        visibility::{Display, Viewport},
    },
    utils::constants::{
        HAS_BACKGROUND_PREFIX, HAS_TEXT_PREFIX, HAS_TEXT_WEIGHT_PREFIX, IS_ALIGN_CONTENT_PREFIX,
        IS_ALIGN_ITEMS_PREFIX, IS_ALIGN_SELF_PREFIX, IS_CLEARFIX, IS_CLICKABLE, IS_CLIPPED,
        IS_FLEX_DIRECTION_PREFIX, IS_FLEX_GROW_PREFIX, IS_FLEX_SHRINK_PREFIX, IS_FLEX_WRAP_PREFIX,
        IS_FONT_FAMILY_PREFIX, IS_JUSTIFY_CONTENT_PREFIX, IS_LIGHT, IS_OVERLAY, IS_PREFIX,
        IS_PULLED_LEFT, IS_PULLED_RIGHT, IS_RADIUSLESS, IS_RELATIVE, IS_SHADOWLESS, IS_SIZE_PREFIX,
        IS_UNSELECTABLE, MARGIN_PREFIX, PADDING_PREFIX,
    },
};

/// Groups together the possible text modifiers
///
/// Logical struct which groups together all the possible text modifiers that
/// have a Bulma class equivalent. This is meant to be an internal resource of
/// the [`crate::utils::class::ClassBuilder`] struct, meant to make it easier
/// to maintain and expand the builder.
#[derive(Clone, Debug, Default, PartialEq)]
struct TextModifiers {
    color: Option<TextColor>,
    size: Option<TextSize>,
    viewport_sizes: HashSet<(TextSize, Viewport)>,
    alignment: Option<TextAlignment>,
    viewport_alignments: HashSet<(TextAlignment, Viewport)>,
    decorations: HashSet<TextDecoration>,
    weight: Option<TextWeight>,
    font_family: Option<FontFamily>,
}

impl From<TextModifiers> for Classes {
    fn from(value: TextModifiers) -> Self {
        let color = value.color.map(|tc| format!("{HAS_TEXT_PREFIX}-{tc}"));
        let size = value.size.map(|ts| format!("{IS_SIZE_PREFIX}-{ts}"));
        let viewport_sizes: Vec<_> = value
            .viewport_sizes
            .iter()
            .map(|(size, viewport)| format!("{IS_SIZE_PREFIX}-{size}-{viewport}"))
            .collect();
        let alignment = value
            .alignment
            .map(|alignment| format!("{HAS_TEXT_PREFIX}-{alignment}"));
        let viewport_alignments: Vec<_> = value
            .viewport_alignments
            .iter()
            .map(|(alignment, viewport)| format!("{HAS_TEXT_PREFIX}-{alignment}-{viewport}"))
            .collect();
        let decorations: Vec<_> = value
            .decorations
            .iter()
            .map(|decoration| format!("{IS_PREFIX}-{decoration}"))
            .collect();
        let weight = value
            .weight
            .map(|weight| format!("{HAS_TEXT_WEIGHT_PREFIX}-{weight}"));
        let font_family = value
            .font_family
            .map(|font_family| format!("{IS_FONT_FAMILY_PREFIX}-{font_family}"));

        classes![
            color,
            size,
            viewport_sizes,
            alignment,
            viewport_alignments,
            decorations,
            weight,
            font_family,
        ]
    }
}

/// Groups together the possible alignment modifiers
///
/// Logical struct which groups together all the possible alignment modifiers
/// that have a Bulma class equivalent. This is meant to be an internal
/// resource the [`crate::utils::class::ClassBuilder`] struct, meant to make it
/// easier to maintain and expand the builder.
#[derive(Clone, Debug, Default, PartialEq)]
struct AlignmentModifiers {
    flex_direction: Option<FlexDirection>,
    flex_wrap: Option<FlexWrap>,
    justify_content: Option<JustifyContent>,
    align_content: Option<AlignContent>,
    align_items: Option<AlignItems>,
    align_self: Option<AlignSelf>,
    flex_grow: Option<FlexShrinkGrowFactor>,
    flex_shrink: Option<FlexShrinkGrowFactor>,
}

impl From<AlignmentModifiers> for Classes {
    fn from(value: AlignmentModifiers) -> Self {
        let flex_direction = value
            .flex_direction
            .map(|flex_direction| format!("{IS_FLEX_DIRECTION_PREFIX}-{flex_direction}"));
        let flex_wrap = value
            .flex_wrap
            .map(|flex_wrap| format!("{IS_FLEX_WRAP_PREFIX}-{flex_wrap}"));
        let justify_content = value
            .justify_content
            .map(|justify_content| format!("{IS_JUSTIFY_CONTENT_PREFIX}-{justify_content}"));
        let align_content = value
            .align_content
            .map(|align_content| format!("{IS_ALIGN_CONTENT_PREFIX}-{align_content}"));
        let align_items = value
            .align_items
            .map(|align_items| format!("{IS_ALIGN_ITEMS_PREFIX}-{align_items}"));
        let align_self = value
            .align_self
            .map(|align_self| format!("{IS_ALIGN_SELF_PREFIX}-{align_self}"));
        let flex_grow = value
            .flex_grow
            .map(|flex_grow| format!("{IS_FLEX_GROW_PREFIX}-{flex_grow}"));
        let flex_shrink = value
            .flex_shrink
            .map(|flex_shrink| format!("{IS_FLEX_SHRINK_PREFIX}-{flex_shrink}"));

        classes![
            flex_direction,
            flex_wrap,
            justify_content,
            align_content,
            align_items,
            align_self,
            flex_grow,
            flex_shrink,
        ]
    }
}

/// Groups together the possible other modifiers
///
/// Logical struct which groups together all the possible other modifiers that
/// have a Bulma class equivalent. This is meant to be an internal resource of
/// the [`crate::utils::class::ClassBuilder`] struct, meant to make it easier
/// to maintain and expand the builder.
///
/// Other refers to the [Bulma Other helpers][bd].
///
/// [bd]: bulma.io/documentation/helpers/other-helpers/
#[derive(Clone, Debug, Default, PartialEq)]
struct OtherModifiers {
    is_clearfix: Option<bool>,
    is_pulled_left: Option<bool>,
    is_pulled_right: Option<bool>,
    is_overlay: Option<bool>,
    is_clipped: Option<bool>,
    is_radiusless: Option<bool>,
    is_shadowless: Option<bool>,
    is_unselectable: Option<bool>,
    is_clickable: Option<bool>,
    is_relative: Option<bool>,
}

impl From<OtherModifiers> for Classes {
    fn from(value: OtherModifiers) -> Self {
        let is_clearfix = value
            .is_clearfix
            .map(|is_clearfix| if is_clearfix { IS_CLEARFIX } else { "" });
        let is_pulled_left =
            value
                .is_pulled_left
                .map(|is_pulled_left| if is_pulled_left { IS_PULLED_LEFT } else { "" });
        let is_pulled_right =
            value
                .is_pulled_right
                .map(|is_pulled_right| if is_pulled_right { IS_PULLED_RIGHT } else { "" });
        let is_overlay = value
            .is_overlay
            .map(|is_overlay| if is_overlay { IS_OVERLAY } else { "" });
        let is_clipped = value
            .is_clipped
            .map(|is_clipped| if is_clipped { IS_CLIPPED } else { "" });
        let is_radiusless = value
            .is_radiusless
            .map(|is_radiusless| if is_radiusless { IS_RADIUSLESS } else { "" });
        let is_shadowless = value
            .is_shadowless
            .map(|is_shadowless| if is_shadowless { IS_SHADOWLESS } else { "" });
        let is_unselectable =
            value
                .is_unselectable
                .map(|is_unselectable| if is_unselectable { IS_UNSELECTABLE } else { "" });
        let is_clickable = value
            .is_clickable
            .map(|is_clickable| if is_clickable { IS_CLICKABLE } else { "" });
        let is_relative = value
            .is_relative
            .map(|is_relative| if is_relative { IS_RELATIVE } else { "" });

        classes!(
            is_clearfix,
            is_pulled_left,
            is_pulled_right,
            is_overlay,
            is_clipped,
            is_radiusless,
            is_shadowless,
            is_unselectable,
            is_clickable,
            is_relative,
        )
    }
}

/// CSS class builder for Bulma and custom classes.
///
/// Used to build various combination of CSS classes, implementing most options
/// found in the [Bulma helpers][bd]. It provides a Rust API for generating
/// styles for any HTML component. It also allows for custom classes to be
/// used.
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
/// [bd]: https://bulma.io/documentation/helpers/
#[derive(Clone, Debug, Default)]
pub struct ClassBuilder {
    custom_classes: HashSet<String>,
    text_modifiers: TextModifiers,
    background_color: Option<BackgroundColor>,
    color: Option<Color>,
    is_light: Option<bool>,
    display: Option<Display>,
    viewport_displays: HashSet<(Display, Viewport)>,
    alignment_modifiers: AlignmentModifiers,
    margins: HashSet<(Direction, Spacing)>,
    paddings: HashSet<(Direction, Spacing)>,
    other_modifiers: OtherModifiers,
}

impl ClassBuilder {
    /// Add a custom CSS class to the current list of classes.
    ///
    /// Add a new custom CSS class to the current list of classes that the
    /// builder will create. The input string is no validated to check if it
    /// is in fact a valid CSS class name. Rather, it is assumed the caller has
    /// checked it prior to the call.
    ///
    /// > _If you add the same class multiple times, it will only appear once
    /// in the final list._
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilder;
    ///
    /// // Create a `<div>` HTML element that has a custom class.
    /// #[function_component(CustomClassDiv)]
    /// fn custom_class_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .with_custom_class("my-awesome-div")
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    pub fn with_custom_class(mut self, custom_class: &str) -> Self {
        if !custom_class.trim().is_empty() {
            self.custom_classes.insert(custom_class.to_owned());
        }
        self
    }

    /// Removes a custom CSS class to the current list of classes, if it exists.
    ///
    /// Removes an existing custom CSS class to the current list of classes that
    /// the builder will create. The input string is no validated to check if it
    /// is in fact a valid CSS class name. Rather, it is assumed the caller has
    /// checked it prior to the call.
    ///
    /// Removing the same class multiple times has the same result as trying to
    /// remove an inexisting one, concretely, nothing will happen.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilder;
    ///
    /// // Create a `<div>` HTML element that does not have the
    /// // `my-awesome-div` custom class.
    /// #[function_component(MyNormalDiv)]
    /// fn my_normal_div() -> Html {
    ///     // Assume that instead of the default builder, one where the class
    ///     // to be removed is actually used.
    ///     let class = ClassBuilder::default()
    ///         .without_custom_class("my-awesome-div")
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    pub fn without_custom_class(mut self, custom_class: &str) -> Self {
        self.custom_classes.remove(custom_class);
        self
    }

    /// Set the text color using a [Bulma text color helper][bd].
    ///
    /// Set a [Bulma text color helper class][bd] to be added to the current
    /// list of classes. To remove a [text color helper][bd], simply pass
    /// `None` to the call. Every call to this method overrides the previous
    /// value to the one received.
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
    /// [bd]: https://bulma.io/documentation/helpers/color-helpers/#text-color
    pub fn with_text_color(mut self, color: Option<TextColor>) -> Self {
        self.text_modifiers.color = color;
        self
    }

    /// Set the background color using a [Bulma background color helper][bd].
    ///
    /// Set a [Bulma background color helper class][bd] to be added to the
    /// current list of classes. To remove a [background color helper][bd],
    /// simply pass `None` to the call. Every call to this method overrides the
    /// previous value to the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::color::BackgroundColor,
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that has the background color set to
    /// // primary.
    /// #[function_component(ColoredBackgroundDiv)]
    /// fn colored_background_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .with_background_color(Some(BackgroundColor::Primary))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/color-helpers/#background-color
    pub fn with_background_color(mut self, color: Option<BackgroundColor>) -> Self {
        self.background_color = color;
        self
    }

    /// Set the color using a [Bulma color variable class][bd].
    ///
    /// Set the color with a [Bulma color variable class][bd] to be added to
    /// the current list of classes. To remove a [color variable class][bd],
    /// simply pass `None` to the call. Every call to this method overrides the
    /// previous value to the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::color::Color,
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that has the color set to primary
    /// #[function_component(ColoredDiv)]
    /// fn colored_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .with_color(Some(Color::Primary))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/customize/variables/
    pub fn with_color(mut self, color: Option<Color>) -> Self {
        self.color = color;
        self
    }

    /// Set the light modifiers for the existing color.
    ///
    /// Set the light modifier for the existing used color, by appending the
    /// `is-light` class to the current list of classes. To remove the
    /// modifier, simply pass `None` to the call. Every call to this method
    /// overrides the previous value to the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilde;
    ///
    /// // Create a `<div>` HTML element that has the light modifier set
    /// #[function_component(LightDiv)]
    /// fn light_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .is_light(Some(true))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/customize/variables/
    pub fn is_light(mut self, is_light: Option<bool>) -> Self {
        self.is_light = is_light;
        self
    }

    /// Set the text size using a [Bulma text size helper][bd].
    ///
    /// Set a [Bulma text size helper class][bd] to be added to the current
    /// list of classes. To remove a [text size helper][bd], simply pass
    /// `None` to the call. Every call to this method overrides the previous
    /// value to the one received.
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
    /// // Create a `<div>` HTML element that has the font size set to 3.
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
    /// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#size
    pub fn with_text_size(mut self, text_size: Option<TextSize>) -> Self {
        self.text_modifiers.size = text_size;
        self
    }

    /// Add a text size for a specific viewport width using a
    /// [Bulma responsive text size helper][bd].
    ///
    /// Set a [Bulma responsive text size helper class][bd] to be added to the
    /// current list of classes.
    ///
    /// > _If you add the same viewport size multiple times, it will only
    /// appear once in the final list._
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::typography::TextSize,
    ///     helpers::visibility::Viewport,
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that has the text size set to 2 for the
    /// // tablet viewport.
    /// #[function_component(TabletTextSize2Div)]
    /// fn tablet_text_size_2_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .with_text_viewport_size(TextSize::Two, Viewport::Tablet)
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#responsive-size
    pub fn with_text_viewport_size(mut self, text_size: TextSize, viewport: Viewport) -> Self {
        self.text_modifiers
            .viewport_sizes
            .insert((text_size, viewport));
        self
    }

    /// Remove a text size for a specific viewport width, if it exists.
    ///
    /// Remove a [Bulma responsive text size helper class][bd], from the current
    /// list of classes, if it exists.
    ///
    /// Removing the same specifier multiple times has the same result as trying
    /// to remove an inexisting one, concretely, nothing will happen.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::typography::TextSize,
    ///     helpers::visibility::Viewport,
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that does not have the text size set to
    /// // 2 for the tablet viewport.
    /// #[function_component(NormalDiv)]
    /// fn normal_div() -> Html {
    ///     // Assume that instead of the default builder, one where the class
    ///     // to be removed is actually used.
    ///     let class = ClassBuilder::default()
    ///         .without_text_viewport_size(TextSize::Two, Viewport::Tablet)
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#responsive-size
    pub fn without_text_viewport_size(mut self, text_size: TextSize, viewport: Viewport) -> Self {
        self.text_modifiers
            .viewport_sizes
            .remove(&(text_size, viewport));
        self
    }

    /// Set the text alignment using a [Bulma alignment helper][bd].
    ///
    /// Set a [Bulma text alignment helper class][bd] to be added to the current
    /// list of classes. To remove a [text alignment helper][bd], simply pass
    /// `None` to the call. Every call to this method overrides the previous
    /// value to the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::typography::TextAlignment,
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that has the text center aligned.
    /// #[function_component(TextCenteredDiv)]
    /// fn text_centered_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .with_text_alignment(Some(TextAlignment::Centered))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#alignment
    pub fn with_text_alignment(mut self, text_alignment: Option<TextAlignment>) -> Self {
        self.text_modifiers.alignment = text_alignment;
        self
    }

    /// Add a text alignment for a specific viewport width using a
    /// [Bulma responsive text alignment helper][bd].
    ///
    /// Set a [Bulma responsive text alignment helper class][bd] to be added to
    /// the current list of classes.
    ///
    /// > _If you add the same viewport alignment multiple times, it will only
    /// appear once in the final list._
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::typography::TextAlignment,
    ///     helpers::visibility::Viewport,
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that has the text center aligned on
    /// // tablets.
    /// #[function_component(TextCenteredTabletDiv)]
    /// fn text_centered_tablet_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .with_text_viewport_alignment(TextAlignment::Centered, Viewport::Tablet)
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#responsive-alignment
    pub fn with_text_viewport_alignment(
        mut self,
        text_alignment: TextAlignment,
        viewport: Viewport,
    ) -> Self {
        self.text_modifiers
            .viewport_alignments
            .insert((text_alignment, viewport));
        self
    }

    /// Remove a text alignment for a specific viewport width, if it exists.
    ///
    /// Remove a [Bulma responsive text alignment helper class][bd], from the
    /// current list of classes, if it exists.
    ///
    /// Removing the same specifier multiple times has the same result as trying
    /// to remove an inexisting one, concretely, nothing will happen.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::typography::TextAlignment,
    ///     helpers::visibility::Viewport,
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that does not have the text center
    /// // aligned on tablets.
    /// #[function_component(NormalDiv)]
    /// fn normal_div() -> Html {
    ///     // Assume that instead of the default builder, one where the class
    ///     // to be removed is actually used.
    ///     let class = ClassBuilder::default()
    ///         .without_text_viewport_alignment(TextAlignment::Centered, Viewport::Tablet)
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#responsive-alignment
    pub fn without_text_viewport_alignment(
        mut self,
        text_alignment: TextAlignment,
        viewport: Viewport,
    ) -> Self {
        self.text_modifiers
            .viewport_alignments
            .remove(&(text_alignment, viewport));
        self
    }

    /// Set the text decoration using a [Bulma text transformation helper][bd].
    ///
    /// Set a [Bulma text transformation helper class][bd] to be added to the
    /// current list of classes.
    ///
    /// > _If you add the same viewport alignment multiple times, it will only
    /// appear once in the final list._
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::typography::TextDecoration,
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that has the text italic.
    /// #[function_component(ItalicTextDiv)]
    /// fn italic_text_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .with_text_decoration(TextDecoration::Italic)
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#text-transformation
    pub fn with_text_decoration(mut self, text_decoration: TextDecoration) -> Self {
        self.text_modifiers.decorations.insert(text_decoration);
        self
    }

    /// Remove a text decoration, which is using a
    /// [Bulma text transformation helper][bd], if it exists.
    ///
    /// Remove a [Bulma text transformation helper class][bd], from the current
    /// list of classes, if it exists.
    ///
    /// Removing the same specifier multiple times has the same result as trying
    /// to remove an inexisting one, concretely, nothing will happen.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::typography::TextDecoration,
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that does not have italic text.
    /// #[function_component(NormalTextDiv)]
    /// fn normal_text_div() -> Html {
    ///     // Assume that instead of the default builder, one where the class
    ///     // to be removed is actually used.
    ///     let class = ClassBuilder::default()
    ///         .without_text_decoration(TextDecoration::Italic)
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#text-transformation
    pub fn without_text_decoration(mut self, text_decoration: TextDecoration) -> Self {
        self.text_modifiers.decorations.remove(&text_decoration);
        self
    }

    /// Set the text weight using a [Bulma weight helper][bd].
    ///
    /// Set a [Bulma text weight helper class][bd] to be added to the current
    /// list of classes. To remove a [text weight helper][bd], simply pass
    /// `None` to the call. Every call to this method overrides the previous
    /// value to the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::typography::TextWeight,
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that has the text weight semi bold.
    /// #[function_component(SemiBoldTextDiv)]
    /// fn semi_bold_text_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .with_text_weight(Some(TextWeight::SemiBold))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#text-weight
    pub fn with_text_weight(mut self, text_weight: Option<TextWeight>) -> Self {
        self.text_modifiers.weight = text_weight;
        self
    }

    /// Set the font family using a [Bulma font family helper][bd].
    ///
    /// Set a [Bulma font family helper class][bd] to be added to the current
    /// list of classes. To remove a [font family helper][bd], simply pass
    /// `None` to the call. Every call to this method overrides the previous
    /// value to the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::typography::FontFamily,
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that has the code font family.
    /// #[function_component(CodeFontDiv)]
    /// fn code_font_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .with_font_family(Some(FontFamily::Code))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#font-family
    pub fn with_font_family(mut self, font_family: Option<FontFamily>) -> Self {
        self.text_modifiers.font_family = font_family;
        self
    }

    /// Set the display CSS property using a [Bulma display helper][bd].
    ///
    /// Set a [Bulma display helper class][bd] to be added to the current list
    /// of classes. To remove a [display helper][bd], simply pass `None` to the
    /// call. Every call to this method overrides the previous value to the one
    /// received.
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
    /// [bd]: https://bulma.io/documentation/helpers/visibility-helpers/#show
    pub fn with_display(mut self, display: Option<Display>) -> Self {
        self.display = display;
        self
    }

    /// Add a display for a specific viewport width using a
    /// [Bulma responsive display helper][bd].
    ///
    /// Set a [Bulma responsive display helper class][bd] to be added to
    /// the current list of classes.
    ///
    /// > _If you add the same viewport display multiple times, it will only
    /// appear once in the final list._
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::visibility::{Display, Viewport},
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that has the display set to flex for the
    /// // tablet viewport.
    /// #[function_component(FlexDiv)]
    /// fn flex_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .with_viewport_display(Display::Flex, Viewport::Tablet)
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/visibility-helpers/#show
    pub fn with_viewport_display(mut self, display: Display, viewport: Viewport) -> Self {
        self.viewport_displays.insert((display, viewport));
        self
    }

    /// Remove a display for a specific viewport width, if it exists.
    ///
    /// Remove a [Bulma responsive display helper class][bd], from the
    /// current list of classes, if it exists.
    ///
    /// Removing the same specifier multiple times has the same result as trying
    /// to remove an inexisting one, concretely, nothing will happen.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::visibility::{Display, Viewport},
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that does not have the display set to
    /// //flex for the tablet viewport.
    /// #[function_component(NormalDiv)]
    /// fn normal_div() -> Html {
    ///     // Assume that instead of the default builder, one where the class
    ///     // to be removed is actually used.
    ///     let class = ClassBuilder::default()
    ///         .without_viewport_display(Display::Flex, Viewport::Tablet)
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/visibility-helpers/#show
    pub fn without_viewport_display(mut self, display: Display, viewport: Viewport) -> Self {
        self.viewport_displays.remove(&(display, viewport));
        self
    }

    /// Set the flex direction using a [Bulma flex direction helper][bd].
    ///
    /// Set a [Bulma flex direction helper class][bd] to be added to the current
    /// list of classes. To remove a [flex direction helper][bd], simply pass
    /// `None` to the call. Every call to this method overrides the previous
    /// value to the one received.
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
    /// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#flex-direction
    pub fn with_flex_direction(mut self, flex_direction: Option<FlexDirection>) -> Self {
        self.alignment_modifiers.flex_direction = flex_direction;
        self
    }

    /// Set the flex wrap using a [Bulma flex wrap helper][bd].
    ///
    /// Set a [Bulma flex wrap helper class][bd] to be added to the current
    /// list of classes. To remove a [flex wrap helper][bd], simply pass
    /// `None` to the call. Every call to this method overrides the previous
    /// value to the one received.
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
    /// // Create a `<div>` HTML element that has flex wrap.
    /// // The `<p>` children are there to highlight the wrap.
    /// #[function_component(FlexDirColDiv)]
    /// fn flex_dir_col_div() -> Html {
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
    /// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#flex-wrap
    pub fn with_flex_wrap(mut self, flex_wrap: Option<FlexWrap>) -> Self {
        self.alignment_modifiers.flex_wrap = flex_wrap;
        self
    }

    /// Set the justify content using a [Bulma justify content helper][bd].
    ///
    /// Set a [Bulma justify content helper class][bd] to be added to the
    /// current list of classes. To remove a [justify content helper][bd],
    /// simply pass `None` to the call. Every call to this method overrides the
    /// previous value to the one received.
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
    /// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#justify-content
    pub fn with_justify_content(mut self, justify_content: Option<JustifyContent>) -> Self {
        self.alignment_modifiers.justify_content = justify_content;
        self
    }

    /// Set the align content using a [Bulma align content helper][bd].
    ///
    /// Set a [Bulma align content helper class][bd] to be added to the current
    /// list of classes. To remove a [align content helper][bd], simply pass
    /// `None` to the call. Every call to this method overrides the previous
    /// value to the one received.
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
    /// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#align-content
    pub fn with_align_content(mut self, align_content: Option<AlignContent>) -> Self {
        self.alignment_modifiers.align_content = align_content;
        self
    }

    /// Set the align items using a [Bulma align items helper][bd].
    ///
    /// Set a [Bulma align items helper class][bd] to be added to the current
    /// list of classes. To remove a [align items helper][bd], simply pass
    /// `None` to the call. Every call to this method overrides the previous
    /// value to the one received.
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
    /// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#align-items
    pub fn with_align_items(mut self, align_items: Option<AlignItems>) -> Self {
        self.alignment_modifiers.align_items = align_items;
        self
    }

    /// Set the align self using a [Bulma align self helper][bd].
    ///
    /// Set a [Bulma align self helper class][bd] to be added to the current
    /// list of classes. To remove a [align self helper][bd], simply pass
    /// `None` to the call. Every call to this method overrides the previous
    /// value to the one received.
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
    /// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#align-self
    pub fn with_align_self(mut self, align_self: Option<AlignSelf>) -> Self {
        self.alignment_modifiers.align_self = align_self;
        self
    }

    /// Set the flex grow using a [Bulma flex grow helper][bd].
    ///
    /// Set a [Bulma flex grow helper class][bd] to be added to the current
    /// list of classes. To remove a [flex grow helper][bd], simply pass `None`
    /// to the call. Every call to this method overrides the previous value to
    /// the one received.
    ///
    /// # Examples
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
    /// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#flex-grow-and-flex-shrink
    pub fn with_flex_grow(mut self, flex_grow: Option<FlexShrinkGrowFactor>) -> Self {
        self.alignment_modifiers.flex_grow = flex_grow;
        self
    }

    /// Set the flex shrink using a [Bulma flex shrink helper][bd].
    ///
    /// Set a [Bulma flex shrink helper class][bd] to be added to the current
    /// list of classes. To remove a [flex shrink helper][bd], simply pass
    /// `None` to the call. Every call to this method overrides the previous
    /// value to the one received.
    ///
    /// # Examples
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
    /// [bd]: https://bulma.io/documentation/helpers/flexbox-helpers/#flex-grow-and-flex-shrink
    pub fn with_flex_shrink(mut self, flex_shrink: Option<FlexShrinkGrowFactor>) -> Self {
        self.alignment_modifiers.flex_shrink = flex_shrink;
        self
    }

    /// Set a the margin using a [Bulma margin helper][bd].
    ///
    /// Set a [Bulma margin helper class][bd] to be added to the current
    /// list of classes.
    ///
    /// > _If you add the same viewport alignment multiple times, it will only
    /// appear once in the final list._
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::spacing::{Direction, Spacing},
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that has the margin set to 2.
    /// #[function_component(SpacedDiv)]
    /// fn spaced_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .with_margin(Direction::All, Spacing::Two)
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/spacing-helpers/
    pub fn with_margin(mut self, direction: Direction, spacing: Spacing) -> Self {
        self.margins.insert((direction, spacing));
        self
    }

    /// Remove a margin specifier, if it exists.
    ///
    /// Remove a [Bulma margin helper class][bd], from the current list of
    /// classes, if it exists.
    ///
    /// Removing the same specifier multiple times has the same result as trying
    /// to remove an inexisting one, concretely, nothing will happen.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::spacing::{Direction, Spacing},
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that does not have the margin set to 2.
    /// #[function_component(NormalDiv)]
    /// fn normal_div() -> Html {
    ///     // Assume that instead of the default builder, one where the class
    ///     // to be removed is actually used.
    ///     let class = ClassBuilder::default()
    ///         .without_margin(Direction::All, Spacing::Two)
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/spacing-helpers/
    pub fn without_margin(mut self, direction: Direction, spacing: Spacing) -> Self {
        self.margins.remove(&(direction, spacing));
        self
    }

    /// Set a the padding using a [Bulma padding helper][bd].
    ///
    /// Set a [Bulma padding helper class][bd] to be added to the current
    /// list of classes.
    ///
    /// > _If you add the same viewport alignment multiple times, it will only
    /// appear once in the final list._
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::spacing::{Direction, Spacing},
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that has the padding set to 2.
    /// #[function_component(SpacedDiv)]
    /// fn spaced_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .with_padding(Direction::All, Spacing::Two)
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/spacing-helpers/
    pub fn with_padding(mut self, direction: Direction, spacing: Spacing) -> Self {
        self.paddings.insert((direction, spacing));
        self
    }

    /// Remove a padding specifier, if it exists.
    ///
    /// Remove a [Bulma padding helper class][bd], from the current list of
    /// classes, if it exists.
    ///
    /// Removing the same specifier multiple times has the same result as trying
    /// to remove an inexisting one, concretely, nothing will happen.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     helpers::spacing::{Direction, Spacing},
    ///     utils::class::ClassBuilder,
    /// };
    ///
    /// // Create a `<div>` HTML element that does not have the padding set to 2.
    /// #[function_component(NormalDiv)]
    /// fn normal_div() -> Html {
    ///     // Assume that instead of the default builder, one where the class
    ///     // to be removed is actually used.
    ///     let class = ClassBuilder::default()
    ///         .without_padding(Direction::All, Spacing::Two)
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/spacing-helpers/
    pub fn without_padding(mut self, direction: Direction, spacing: Spacing) -> Self {
        self.paddings.remove(&(direction, spacing));
        self
    }

    /// Set the [Bulma clearfix helper][bd].
    ///
    /// Set the [Bulma clearfix helper class][bd] to be added to the current
    /// list of classes. To remove a [clearfix helper][bd], simply pass `None`
    /// to the call. Every call to this method overrides the previous value to
    /// the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilder;
    ///
    /// // Create a `<div>` HTML element that has the clearfix Bulma class.
    /// #[function_component(ClearfixDiv)]
    /// fn clearfix_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .is_clearfix(Some(true))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/other-helpers/
    pub fn is_clearfix(mut self, is_clearfix: Option<bool>) -> Self {
        self.other_modifiers.is_clearfix = is_clearfix;
        self
    }

    /// Set the [Bulma pulled left helper][bd].
    ///
    /// Set the [Bulma pulled left helper class][bd] to be added to the current
    /// list of classes. To remove a [pulled left helper][bd], simply pass `None`
    /// to the call. Every call to this method overrides the previous value to
    /// the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilder;
    ///
    /// // Create a `<div>` HTML element that has the pulled left Bulma class.
    /// #[function_component(PulledLeftDiv)]
    /// fn pulled_left_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .is_pulled_left(Some(true))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/other-helpers/
    pub fn is_pulled_left(mut self, is_pulled_left: Option<bool>) -> Self {
        self.other_modifiers.is_pulled_left = is_pulled_left;
        self
    }

    /// Set the [Bulma pulled right helper][bd].
    ///
    /// Set the [Bulma pulled right helper class][bd] to be added to the current
    /// list of classes. To remove a [pulled right helper][bd], simply pass `None`
    /// to the call. Every call to this method overrides the previous value to
    /// the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilder;
    ///
    /// // Create a `<div>` HTML element that has the pulled right Bulma class.
    /// #[function_component(PulledRightDiv)]
    /// fn pulled_right_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .is_pulled_right(Some(true))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/other-helpers/
    pub fn is_pulled_right(mut self, is_pulled_right: Option<bool>) -> Self {
        self.other_modifiers.is_pulled_right = is_pulled_right;
        self
    }

    /// Set the [Bulma overlay helper][bd].
    ///
    /// Set the [Bulma overlay helper class][bd] to be added to the current
    /// list of classes. To remove a [overlay helper][bd], simply pass `None`
    /// to the call. Every call to this method overrides the previous value to
    /// the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilder;
    ///
    /// // Create a `<div>` HTML element that has the overlay Bulma class.
    /// #[function_component(OverlayDiv)]
    /// fn overlay_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .is_overlay(Some(true))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/other-helpers/
    pub fn is_overlay(mut self, is_overlay: Option<bool>) -> Self {
        self.other_modifiers.is_overlay = is_overlay;
        self
    }

    /// Set the [Bulma clipped helper][bd].
    ///
    /// Set the [Bulma clipped helper class][bd] to be added to the current
    /// list of classes. To remove a [clipped helper][bd], simply pass `None`
    /// to the call. Every call to this method overrides the previous value to
    /// the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilder;
    ///
    /// // Create a `<div>` HTML element that has the clipped Bulma class.
    /// #[function_component(ClippedDiv)]
    /// fn clipped_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .is_clipped(Some(true))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/other-helpers/
    pub fn is_clipped(mut self, is_clipped: Option<bool>) -> Self {
        self.other_modifiers.is_clipped = is_clipped;
        self
    }

    /// Set the [Bulma radiusless helper][bd].
    ///
    /// Set the [Bulma radiusless helper class][bd] to be added to the current
    /// list of classes. To remove a [radiusless helper][bd], simply pass `None`
    /// to the call. Every call to this method overrides the previous value to
    /// the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilder;
    ///
    /// // Create a `<div>` HTML element that has the radiusless Bulma class.
    /// #[function_component(RadiuslessDiv)]
    /// fn radiusless_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .is_radiusless(Some(true))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/other-helpers/
    pub fn is_radiusless(mut self, is_radiusless: Option<bool>) -> Self {
        self.other_modifiers.is_radiusless = is_radiusless;
        self
    }

    /// Set the [Bulma shadowless helper][bd].
    ///
    /// Set the [Bulma shadowless helper class][bd] to be added to the current
    /// list of classes. To remove a [shadowless helper][bd], simply pass `None`
    /// to the call. Every call to this method overrides the previous value to
    /// the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilder;
    ///
    /// // Create a `<div>` HTML element that has the shadowless Bulma class.
    /// #[function_component(ShadowlessDiv)]
    /// fn shadowless_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .is_shadowless(Some(true))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/other-helpers/
    pub fn is_shadowless(mut self, is_shadowless: Option<bool>) -> Self {
        self.other_modifiers.is_shadowless = is_shadowless;
        self
    }

    /// Set the [Bulma unselectable helper][bd].
    ///
    /// Set the [Bulma unselectable helper class][bd] to be added to the
    /// current list of classes. To remove a [unselectable helper][bd], simply
    /// pass `None` to the call. Every call to this method overrides the previous
    /// value to the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilder;
    ///
    /// // Create a `<div>` HTML element that has the unselectable Bulma class.
    /// #[function_component(UnselectableDiv)]
    /// fn unselectable_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .is_unselectable(Some(true))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/other-helpers/
    pub fn is_unselectable(mut self, is_unselectable: Option<bool>) -> Self {
        self.other_modifiers.is_unselectable = is_unselectable;
        self
    }

    /// Set the [Bulma clickable helper][bd].
    ///
    /// Set the [Bulma clickable helper class][bd] to be added to the current
    /// list of classes. To remove a [clickable helper][bd], simply pass `None`
    /// to the call. Every call to this method overrides the previous value to
    /// the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilder;
    ///
    /// // Create a `<div>` HTML element that has the clickable Bulma class.
    /// #[function_component(ClickableDiv)]
    /// fn clickable_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .is_clickable(Some(true))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/other-helpers/
    pub fn is_clickable(mut self, is_clickable: Option<bool>) -> Self {
        self.other_modifiers.is_clickable = is_clickable;
        self
    }

    /// Set the [Bulma relative helper][bd].
    ///
    /// Set the [Bulma relative helper class][bd] to be added to the current
    /// list of classes. To remove a [relative helper][bd], simply pass `None`
    /// to the call. Every call to this method overrides the previous value to
    /// the one received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::utils::class::ClassBuilder;
    ///
    /// // Create a `<div>` HTML element that has the relative Bulma class.
    /// #[function_component(RelativeDiv)]
    /// fn relative_div() -> Html {
    ///     let class = ClassBuilder::default()
    ///         .is_relative(Some(true))
    ///         .build();
    ///     html!{
    ///         <div class={class}>{ "Lorem ispum..." }</div>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/helpers/other-helpers/
    pub fn is_relative(mut self, is_relative: Option<bool>) -> Self {
        self.other_modifiers.is_relative = is_relative;
        self
    }

    /// Create the [`yew::html::Classes`] object from the current
    /// configuration.
    ///
    /// Using the set values of the builder, create an instance of the
    /// [`yew::html::Classes`] from them. This consumes the builder. If no
    /// values were set in the builder, the resulting value is equivalent to
    /// calling `yew::classes!()`.
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
    pub fn build(self) -> Classes {
        let custom_classes: Vec<_> = self.custom_classes.iter().collect();
        let text_classes: Classes = self.text_modifiers.into();
        let background_color = self
            .background_color
            .map(|bc| format!("{HAS_BACKGROUND_PREFIX}-{bc}"));
        let color_class = self.color.map(|color| format!("{IS_PREFIX}-{color}"));
        let is_light_class = self
            .is_light
            .map(|is_light| (if is_light { IS_LIGHT } else { "" }).to_string());
        let display = self.display.map(|display| format!("{IS_PREFIX}-{display}"));
        let viewport_displays: Vec<_> = self
            .viewport_displays
            .iter()
            .map(|(display, viewport)| format!("{IS_PREFIX}-{display}-{viewport}"))
            .collect();
        let alignment_classes: Classes = self.alignment_modifiers.into();
        let margin_classes: Vec<_> = self
            .margins
            .iter()
            .map(|(direction, spacing)| format!("{MARGIN_PREFIX}{direction}-{spacing}"))
            .collect();
        let padding_classes: Vec<_> = self
            .paddings
            .iter()
            .map(|(direction, spacing)| format!("{PADDING_PREFIX}{direction}-{spacing}"))
            .collect();
        let other_classes: Classes = self.other_modifiers.into();

        classes!(
            custom_classes,
            text_classes,
            background_color,
            color_class,
            is_light_class,
            display,
            viewport_displays,
            alignment_classes,
            margin_classes,
            padding_classes,
            other_classes,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn text_modifiers_default_success() {
        let text_modifiers = TextModifiers::default();

        assert!(text_modifiers.color.is_none());
        assert!(text_modifiers.size.is_none());
        assert!(text_modifiers.viewport_sizes.is_empty());
        assert!(text_modifiers.alignment.is_none());
        assert!(text_modifiers.viewport_alignments.is_empty());
        assert!(text_modifiers.decorations.is_empty());
        assert!(text_modifiers.weight.is_none());
        assert!(text_modifiers.font_family.is_none());
    }

    #[test]
    fn alignment_modifiers_default_success() {
        let alignment_modifiers = AlignmentModifiers::default();

        assert!(alignment_modifiers.flex_direction.is_none());
        assert!(alignment_modifiers.flex_wrap.is_none());
        assert!(alignment_modifiers.justify_content.is_none());
        assert!(alignment_modifiers.align_content.is_none());
        assert!(alignment_modifiers.align_items.is_none());
        assert!(alignment_modifiers.align_self.is_none());
        assert!(alignment_modifiers.flex_grow.is_none());
        assert!(alignment_modifiers.flex_shrink.is_none());
    }

    #[test]
    fn other_modifiers_default_success() {
        let other_modifiers = OtherModifiers::default();

        assert!(other_modifiers.is_clearfix.is_none());
        assert!(other_modifiers.is_pulled_left.is_none());
        assert!(other_modifiers.is_pulled_right.is_none());
        assert!(other_modifiers.is_overlay.is_none());
        assert!(other_modifiers.is_clipped.is_none());
        assert!(other_modifiers.is_radiusless.is_none());
        assert!(other_modifiers.is_shadowless.is_none());
        assert!(other_modifiers.is_unselectable.is_none());
        assert!(other_modifiers.is_clickable.is_none());
        assert!(other_modifiers.is_relative.is_none());
    }

    #[test]
    fn class_builder_default_success() {
        let class_builder = ClassBuilder::default();

        assert_eq!(class_builder.text_modifiers, TextModifiers::default());
        assert!(class_builder.background_color.is_none());
        assert!(class_builder.display.is_none());
        assert!(class_builder.viewport_displays.is_empty());
        assert_eq!(
            class_builder.alignment_modifiers,
            AlignmentModifiers::default()
        );
        assert!(class_builder.margins.is_empty());
        assert!(class_builder.paddings.is_empty());
        assert_eq!(class_builder.other_modifiers, OtherModifiers::default());
    }

    #[test]
    fn class_builder_with_custom_class() {
        let expected_classes = vec!["abc", "def"];
        let classes = ClassBuilder::default()
            .with_custom_class("abc")
            .with_custom_class("def")
            .build();

        let classes = classes.to_string();
        for class in expected_classes {
            assert!(classes.contains(class));
        }
    }

    #[test]
    fn class_builder_without_custom_class() {
        let expected_classes = "def";
        let classes = ClassBuilder::default()
            .with_custom_class("abc")
            .with_custom_class("def")
            .without_custom_class("abc")
            .build();

        assert_eq!(classes.to_string(), expected_classes);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(TextColor::Primary), "has-text-primary" ; "primary converts to has-text-primary")]
    fn class_builer_with_text_color(color: Option<TextColor>, expected_color: &str) {
        let classes = ClassBuilder::default().with_text_color(color).build();

        assert_eq!(classes.to_string(), expected_color);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(BackgroundColor::Primary), "has-background-primary" ; "primary converts to has-background-primary")]
    fn class_builer_with_background_color(color: Option<BackgroundColor>, expected_color: &str) {
        let classes = ClassBuilder::default().with_background_color(color).build();

        assert_eq!(classes.to_string(), expected_color);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(Color::Primary), "is-primary" ; "primary converts to is-primary")]
    fn class_builer_with_color(color: Option<Color>, expected_color: &str) {
        let classes = ClassBuilder::default().with_color(color).build();

        assert_eq!(classes.to_string(), expected_color);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(true), "is-light" ; "true converts to is-light")]
    fn class_builer_is_light(is_light: Option<bool>, expected_light: &str) {
        let classes = ClassBuilder::default().is_light(is_light).build();

        assert_eq!(classes.to_string(), expected_light);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(TextSize::Three), "is-size-3" ; "three converts to is-size-3")]
    fn class_builer_with_text_size(text_size: Option<TextSize>, expected_size: &str) {
        let classes = ClassBuilder::default().with_text_size(text_size).build();

        assert_eq!(classes.to_string(), expected_size);
    }

    #[test]
    fn class_builer_with_text_viewport_size() {
        let expected_viewport_size = "is-size-3-desktop";
        let classes = ClassBuilder::default()
            .with_text_viewport_size(TextSize::Three, Viewport::Desktop)
            .build();

        assert_eq!(classes.to_string(), expected_viewport_size);
    }

    #[test]
    fn class_builer_without_text_viewport_size() {
        let expected_viewport_size = "";
        let classes = ClassBuilder::default()
            .with_text_viewport_size(TextSize::Three, Viewport::Desktop)
            .without_text_viewport_size(TextSize::Three, Viewport::Desktop)
            .build();

        assert_eq!(classes.to_string(), expected_viewport_size);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(TextAlignment::Centered), "has-text-centered" ; "centered converts to has-text-centered")]
    fn class_builer_with_text_alignment(
        text_alignment: Option<TextAlignment>,
        expected_text_alignment: &str,
    ) {
        let classes = ClassBuilder::default()
            .with_text_alignment(text_alignment)
            .build();

        assert_eq!(classes.to_string(), expected_text_alignment);
    }

    #[test]
    fn class_builer_with_text_viewport_alignment() {
        let expected_text_alignment = "has-text-centered-desktop";
        let classes = ClassBuilder::default()
            .with_text_viewport_alignment(TextAlignment::Centered, Viewport::Desktop)
            .build();

        assert_eq!(classes.to_string(), expected_text_alignment);
    }

    #[test]
    fn class_builer_without_text_viewport_alignment() {
        let expected_text_alignment = "";
        let classes = ClassBuilder::default()
            .with_text_viewport_alignment(TextAlignment::Centered, Viewport::Desktop)
            .without_text_viewport_alignment(TextAlignment::Centered, Viewport::Desktop)
            .build();

        assert_eq!(classes.to_string(), expected_text_alignment);
    }

    #[test]
    fn class_builer_with_text_decoration() {
        let expected_text_decoration = "is-italic";
        let classes = ClassBuilder::default()
            .with_text_decoration(TextDecoration::Italic)
            .build();

        assert_eq!(classes.to_string(), expected_text_decoration);
    }

    #[test]
    fn class_builer_without_text_decoration() {
        let expected_text_decoration = "";
        let classes = ClassBuilder::default()
            .with_text_decoration(TextDecoration::Italic)
            .without_text_decoration(TextDecoration::Italic)
            .build();

        assert_eq!(classes.to_string(), expected_text_decoration);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(TextWeight::SemiBold), "has-text-weight-semibold" ; "semi bold converts to has-text-weight-semibold")]
    fn class_builer_with_text_weight(text_weight: Option<TextWeight>, expected_weight: &str) {
        let classes = ClassBuilder::default()
            .with_text_weight(text_weight)
            .build();

        assert_eq!(classes.to_string(), expected_weight);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(FontFamily::Code), "is-family-code" ; "code converts to is-family-code")]
    fn class_builer_with_font_family(font_family: Option<FontFamily>, expected_family: &str) {
        let classes = ClassBuilder::default()
            .with_font_family(font_family)
            .build();

        assert_eq!(classes.to_string(), expected_family);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(Display::Flex), "is-flex" ; "flex converts to is-flex")]
    fn class_builer_with_display(display: Option<Display>, expected_display: &str) {
        let classes = ClassBuilder::default().with_display(display).build();

        assert_eq!(classes.to_string(), expected_display);
    }

    #[test]
    fn class_builer_with_viewport_display() {
        let expected_display = "is-flex-desktop";
        let classes = ClassBuilder::default()
            .with_viewport_display(Display::Flex, Viewport::Desktop)
            .build();

        assert_eq!(classes.to_string(), expected_display);
    }

    #[test]
    fn class_builer_without_viewport_display() {
        let expected_display = "";
        let classes = ClassBuilder::default()
            .with_viewport_display(Display::Flex, Viewport::Desktop)
            .without_viewport_display(Display::Flex, Viewport::Desktop)
            .build();

        assert_eq!(classes.to_string(), expected_display);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(FlexDirection::Column), "is-flex-direction-column" ; "column converts to is-flex-direction-column")]
    fn class_builer_with_flex_direction(
        flex_direction: Option<FlexDirection>,
        expected_direction: &str,
    ) {
        let classes = ClassBuilder::default()
            .with_flex_direction(flex_direction)
            .build();

        assert_eq!(classes.to_string(), expected_direction);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(FlexWrap::Wrap), "is-flex-wrap-wrap" ; "wrap converts to is-flex-wrap-wrap")]
    fn class_builer_with_flex_wrap(flex_wrap: Option<FlexWrap>, expected_wrap: &str) {
        let classes = ClassBuilder::default().with_flex_wrap(flex_wrap).build();

        assert_eq!(classes.to_string(), expected_wrap);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(JustifyContent::Center), "is-justify-content-center" ; "center converts to is-justify-content-center")]
    fn class_builer_with_justify_content(
        justify_content: Option<JustifyContent>,
        expected_justify_content: &str,
    ) {
        let classes = ClassBuilder::default()
            .with_justify_content(justify_content)
            .build();

        assert_eq!(classes.to_string(), expected_justify_content);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(AlignContent::Center), "is-align-content-center" ; "center converts to is-align-content-center")]
    fn class_builer_with_align_content(
        align_content: Option<AlignContent>,
        expected_align_content: &str,
    ) {
        let classes = ClassBuilder::default()
            .with_align_content(align_content)
            .build();

        assert_eq!(classes.to_string(), expected_align_content);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(AlignItems::Center), "is-align-items-center" ; "center converts to is-align-items-center")]
    fn class_builer_with_align_items(align_items: Option<AlignItems>, expected_align_items: &str) {
        let classes = ClassBuilder::default()
            .with_align_items(align_items)
            .build();

        assert_eq!(classes.to_string(), expected_align_items);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(AlignSelf::Center), "is-align-self-center" ; "center converts to is-align-self-center")]
    fn class_builer_with_align_self(align_self: Option<AlignSelf>, expected_align_self: &str) {
        let classes = ClassBuilder::default().with_align_self(align_self).build();

        assert_eq!(classes.to_string(), expected_align_self);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(FlexShrinkGrowFactor::Two), "is-flex-grow-2" ; "two converts to is-flex-grow-2")]
    fn class_builer_with_flex_grow(
        flex_grow: Option<FlexShrinkGrowFactor>,
        expected_grow_factor: &str,
    ) {
        let classes = ClassBuilder::default().with_flex_grow(flex_grow).build();

        assert_eq!(classes.to_string(), expected_grow_factor);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(FlexShrinkGrowFactor::Two), "is-flex-shrink-2" ; "two converts to is-flex-shrink-2")]
    fn class_builer_with_flex_shrink(
        flex_shrink: Option<FlexShrinkGrowFactor>,
        expected_shrink_factor: &str,
    ) {
        let classes = ClassBuilder::default()
            .with_flex_shrink(flex_shrink)
            .build();

        assert_eq!(classes.to_string(), expected_shrink_factor);
    }

    #[test]
    fn class_builer_with_margin() {
        let expected_margin = "mx-2";
        let classes = ClassBuilder::default()
            .with_margin(Direction::Horizontal, Spacing::Two)
            .build();

        assert_eq!(classes.to_string(), expected_margin);
    }

    #[test]
    fn class_builer_without_margin() {
        let expected_margin = "";
        let classes = ClassBuilder::default()
            .with_margin(Direction::Horizontal, Spacing::Two)
            .without_margin(Direction::Horizontal, Spacing::Two)
            .build();

        assert_eq!(classes.to_string(), expected_margin);
    }

    #[test]
    fn class_builer_with_padding() {
        let expected_padding = "px-2";
        let classes = ClassBuilder::default()
            .with_padding(Direction::Horizontal, Spacing::Two)
            .build();

        assert_eq!(classes.to_string(), expected_padding);
    }

    #[test]
    fn class_builer_without_padding() {
        let expected_padding = "";
        let classes = ClassBuilder::default()
            .with_padding(Direction::Horizontal, Spacing::Two)
            .without_padding(Direction::Horizontal, Spacing::Two)
            .build();

        assert_eq!(classes.to_string(), expected_padding);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(false), "" ; "false converts to empty string")]
    #[test_case(Some(true), "is-clearfix" ; "true converts to is-clearfix")]
    fn class_builder_is_clearfix(is_clearfix: Option<bool>, expected_is_clearfix: &str) {
        let classes = ClassBuilder::default().is_clearfix(is_clearfix).build();

        assert_eq!(classes.to_string(), expected_is_clearfix);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(false), "" ; "false converts to empty string")]
    #[test_case(Some(true), "is-pulled-left" ; "true converts to is-pulled-left")]
    fn class_builder_is_pulled_left(is_pulled_left: Option<bool>, expected_is_pulled_left: &str) {
        let classes = ClassBuilder::default()
            .is_pulled_left(is_pulled_left)
            .build();

        assert_eq!(classes.to_string(), expected_is_pulled_left);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(false), "" ; "false converts to empty string")]
    #[test_case(Some(true), "is-pulled-right" ; "true converts to is-pulled-right")]
    fn class_builder_is_pulled_right(
        is_pulled_right: Option<bool>,
        expected_is_pulled_right: &str,
    ) {
        let classes = ClassBuilder::default()
            .is_pulled_right(is_pulled_right)
            .build();

        assert_eq!(classes.to_string(), expected_is_pulled_right);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(false), "" ; "false converts to empty string")]
    #[test_case(Some(true), "is-overlay" ; "true converts to is-overlay")]
    fn class_builder_is_overlay(is_overlay: Option<bool>, expected_is_overlay: &str) {
        let classes = ClassBuilder::default().is_overlay(is_overlay).build();

        assert_eq!(classes.to_string(), expected_is_overlay);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(false), "" ; "false converts to empty string")]
    #[test_case(Some(true), "is-clipped" ; "true converts to is-clipped")]
    fn class_builder_is_clipped(is_clipped: Option<bool>, expected_is_clipped: &str) {
        let classes = ClassBuilder::default().is_clipped(is_clipped).build();

        assert_eq!(classes.to_string(), expected_is_clipped);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(false), "" ; "false converts to empty string")]
    #[test_case(Some(true), "is-radiusless" ; "true converts to is-radiusless")]
    fn class_builder_is_radiusless(is_radiusless: Option<bool>, expected_is_radiusless: &str) {
        let classes = ClassBuilder::default().is_radiusless(is_radiusless).build();

        assert_eq!(classes.to_string(), expected_is_radiusless);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(false), "" ; "false converts to empty string")]
    #[test_case(Some(true), "is-shadowless" ; "true converts to is-shadowless")]
    fn class_builder_is_shadowless(is_shadowless: Option<bool>, expected_is_shadowless: &str) {
        let classes = ClassBuilder::default().is_shadowless(is_shadowless).build();

        assert_eq!(classes.to_string(), expected_is_shadowless);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(false), "" ; "false converts to empty string")]
    #[test_case(Some(true), "is-unselectable" ; "true converts to is-unselectable")]
    fn class_builder_is_unselectable(
        is_unselectable: Option<bool>,
        expected_is_unselectable: &str,
    ) {
        let classes = ClassBuilder::default()
            .is_unselectable(is_unselectable)
            .build();

        assert_eq!(classes.to_string(), expected_is_unselectable);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(false), "" ; "false converts to empty string")]
    #[test_case(Some(true), "is-clickable" ; "true converts to is-clickable")]
    fn class_builder_is_clickable(is_clickable: Option<bool>, expected_is_clickable: &str) {
        let classes = ClassBuilder::default().is_clickable(is_clickable).build();

        assert_eq!(classes.to_string(), expected_is_clickable);
    }

    #[test_case(None, "" ; "none converts to empty string")]
    #[test_case(Some(false), "" ; "false converts to empty string")]
    #[test_case(Some(true), "is-relative" ; "true converts to is-relative")]
    fn class_builder_is_relative(is_relative: Option<bool>, expected_is_relative: &str) {
        let classes = ClassBuilder::default().is_relative(is_relative).build();

        assert_eq!(classes.to_string(), expected_is_relative);
    }

    #[test]
    fn class_builder_build_multiple_classes_success() {
        let expected_classes = vec![
            "is-flex",
            "is-flex-direction-column",
            "mx-3",
            "py-2",
            "has-text-success",
            "has-background-dark",
            "is-block-touch",
            "is-clickable",
        ];
        let classes = ClassBuilder::default()
            .with_display(Some(Display::Flex))
            .with_flex_direction(Some(FlexDirection::Column))
            .with_margin(Direction::Horizontal, Spacing::Three)
            .with_padding(Direction::Vertical, Spacing::Two)
            .with_text_color(Some(TextColor::Success))
            .with_background_color(Some(BackgroundColor::Dark))
            .with_viewport_display(Display::Block, Viewport::Touch)
            .is_clickable(Some(true))
            .build();

        let classes = classes.to_string();
        for expected_class in expected_classes {
            assert!(classes.contains(expected_class));
        }
    }
}
