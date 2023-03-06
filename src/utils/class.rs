use std::collections::HashSet;

use yew::{classes, Classes};

use crate::{
    helpers::{
        color::{BackgroundColor, TextColor},
        flexbox::{
            AlignContent, AlignItems, AlignSelf, FlexDirection, FlexShrinkGrowFactor, FlexWrap,
            JustifyContent,
        },
        spacing::{Direction, Spacing},
        typography::{FontFamily, TextAlignment, TextDecoration, TextSize, TextWeight},
        visibility::{Display, Viewport},
    },
    utils::constants::HAS_BACKGROUND_PREFIX,
    utils::constants::HAS_TEXT_PREFIX,
    utils::constants::HAS_TEXT_WEIGHT_PREFIX,
    utils::constants::IS_ALIGN_CONTENT_PREFIX,
    utils::constants::IS_ALIGN_ITEMS_PREFIX,
    utils::constants::IS_ALIGN_SELF_PREFIX,
    utils::constants::IS_CLEARFIX,
    utils::constants::IS_CLICKABLE,
    utils::constants::IS_CLIPPED,
    utils::constants::IS_FLEX_DIRECTION_PREFIX,
    utils::constants::IS_FLEX_GROW_PREFIX,
    utils::constants::IS_FLEX_SHRINK_PREFIX,
    utils::constants::IS_FLEX_WRAP_PREFIX,
    utils::constants::IS_FONT_FAMILY_PREFIX,
    utils::constants::IS_JUSTIFY_CONTENT_PREFIX,
    utils::constants::IS_OVERLAY,
    utils::constants::IS_PREFIX,
    utils::constants::IS_PULLED_LEFT,
    utils::constants::IS_PULLED_RIGHT,
    utils::constants::IS_RADIUSLESS,
    utils::constants::IS_RELATIVE,
    utils::constants::IS_SHADOWLESS,
    utils::constants::IS_SIZE_PREFIX,
    utils::constants::IS_UNSELECTABLE,
    utils::constants::MARGIN_PREFIX,
    utils::constants::PADDING_PREFIX,
};

#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, Default)]
pub struct ClassBuilder {
    text_modifiers: TextModifiers,
    background_color: Option<BackgroundColor>,
    display: Option<Display>,
    viewport_displays: HashSet<(Display, Viewport)>,
    alignment_modifiers: AlignmentModifiers,
    margins: HashSet<(Direction, Spacing)>,
    paddings: HashSet<(Direction, Spacing)>,
    other_modifiers: OtherModifiers,
}

impl ClassBuilder {
    pub fn with_text_color(mut self, color: Option<TextColor>) -> Self {
        self.text_modifiers.color = color;
        self
    }

    pub fn with_background_color(mut self, color: Option<BackgroundColor>) -> Self {
        self.background_color = color;
        self
    }

    pub fn with_text_size(mut self, text_size: Option<TextSize>) -> Self {
        self.text_modifiers.size = text_size;
        self
    }

    pub fn with_text_viewport_size(mut self, text_size: TextSize, viewport: Viewport) -> Self {
        self.text_modifiers
            .viewport_sizes
            .insert((text_size, viewport));
        self
    }

    pub fn without_text_viewport_size(mut self, text_size: TextSize, viewport: Viewport) -> Self {
        self.text_modifiers
            .viewport_sizes
            .remove(&(text_size, viewport));
        self
    }

    pub fn with_text_alignment(mut self, text_alignment: Option<TextAlignment>) -> Self {
        self.text_modifiers.alignment = text_alignment;
        self
    }

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

    pub fn with_text_decoration(mut self, text_decoration: TextDecoration) -> Self {
        self.text_modifiers.decorations.insert(text_decoration);
        self
    }

    pub fn without_text_decoration(mut self, text_decoration: TextDecoration) -> Self {
        self.text_modifiers.decorations.remove(&text_decoration);
        self
    }

    pub fn with_text_weight(mut self, text_weight: Option<TextWeight>) -> Self {
        self.text_modifiers.weight = text_weight;
        self
    }

    pub fn with_font_family(mut self, font_family: Option<FontFamily>) -> Self {
        self.text_modifiers.font_family = font_family;
        self
    }

    pub fn with_display(mut self, display: Option<Display>) -> Self {
        self.display = display;
        self
    }

    pub fn with_viewport_display(mut self, display: Display, viewport: Viewport) -> Self {
        self.viewport_displays.insert((display, viewport));
        self
    }

    pub fn without_viewport_display(mut self, display: Display, viewport: Viewport) -> Self {
        self.viewport_displays.remove(&(display, viewport));
        self
    }

    pub fn with_flex_direction(mut self, flex_direction: Option<FlexDirection>) -> Self {
        self.alignment_modifiers.flex_direction = flex_direction;
        self
    }

    pub fn with_flex_wrap(mut self, flex_wrap: Option<FlexWrap>) -> Self {
        self.alignment_modifiers.flex_wrap = flex_wrap;
        self
    }

    pub fn with_justify_content(mut self, justify_content: Option<JustifyContent>) -> Self {
        self.alignment_modifiers.justify_content = justify_content;
        self
    }

    pub fn with_align_content(mut self, align_content: Option<AlignContent>) -> Self {
        self.alignment_modifiers.align_content = align_content;
        self
    }

    pub fn with_align_items(mut self, align_items: Option<AlignItems>) -> Self {
        self.alignment_modifiers.align_items = align_items;
        self
    }

    pub fn with_align_self(mut self, align_self: Option<AlignSelf>) -> Self {
        self.alignment_modifiers.align_self = align_self;
        self
    }

    pub fn with_flex_grow(mut self, flex_grow: Option<FlexShrinkGrowFactor>) -> Self {
        self.alignment_modifiers.flex_grow = flex_grow;
        self
    }

    pub fn with_flex_shrink(mut self, flex_shrink: Option<FlexShrinkGrowFactor>) -> Self {
        self.alignment_modifiers.flex_shrink = flex_shrink;
        self
    }

    pub fn with_margin(mut self, direction: Direction, spacing: Spacing) -> Self {
        self.margins.insert((direction, spacing));
        self
    }

    pub fn without_margin(mut self, direction: Direction, spacing: Spacing) -> Self {
        self.margins.remove(&(direction, spacing));
        self
    }

    pub fn with_padding(mut self, direction: Direction, spacing: Spacing) -> Self {
        self.paddings.insert((direction, spacing));
        self
    }

    pub fn without_padding(mut self, direction: Direction, spacing: Spacing) -> Self {
        self.paddings.remove(&(direction, spacing));
        self
    }

    pub fn is_clearfix(mut self, is_clearfix: Option<bool>) -> Self {
        self.other_modifiers.is_clearfix = is_clearfix;
        self
    }

    pub fn is_pulled_left(mut self, is_pulled_left: Option<bool>) -> Self {
        self.other_modifiers.is_pulled_left = is_pulled_left;
        self
    }

    pub fn is_pulled_right(mut self, is_pulled_right: Option<bool>) -> Self {
        self.other_modifiers.is_pulled_right = is_pulled_right;
        self
    }

    pub fn is_overlay(mut self, is_overlay: Option<bool>) -> Self {
        self.other_modifiers.is_overlay = is_overlay;
        self
    }

    pub fn is_clipped(mut self, is_clipped: Option<bool>) -> Self {
        self.other_modifiers.is_clipped = is_clipped;
        self
    }

    pub fn is_radiusless(mut self, is_radiusless: Option<bool>) -> Self {
        self.other_modifiers.is_radiusless = is_radiusless;
        self
    }

    pub fn is_shadowless(mut self, is_shadowless: Option<bool>) -> Self {
        self.other_modifiers.is_shadowless = is_shadowless;
        self
    }

    pub fn is_unselectable(mut self, is_unselectable: Option<bool>) -> Self {
        self.other_modifiers.is_unselectable = is_unselectable;
        self
    }

    pub fn is_clickable(mut self, is_clickable: Option<bool>) -> Self {
        self.other_modifiers.is_clickable = is_clickable;
        self
    }

    pub fn is_relative(mut self, is_relative: Option<bool>) -> Self {
        self.other_modifiers.is_relative = is_relative;
        self
    }

    pub fn build(self) -> Classes {
        let text_classes: Classes = self.text_modifiers.into();
        let background_color = self
            .background_color
            .map(|bc| format!("{HAS_BACKGROUND_PREFIX}-{bc}"));
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
            text_classes,
            background_color,
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
}
