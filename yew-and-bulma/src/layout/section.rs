use yew::html;
use yew::{function_component, Children, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

use crate::utils::BaseComponent;
use crate::utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size};

/// Defines the points from which a [section element][bd] is not full width.
///
/// Defines the points from which a [Bulma section element][bd] no longer has
/// full width, but rather a fixed one. Those points are viewports from which
/// the component is "active", concretely, space around the children is
/// visible.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{layout::section::Section, utils::size::Size};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Section size={Size::Large}>
///             {"This is some text in a section."}
///         </Section>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/section/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct SectionProperties {
    /// Sets the size of the [section element][bd].
    ///
    /// Sets the size of the [Bulma section element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     layout::section::Section,
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Section size={Size::Large}>
    ///             {"This is some text in a large section"}
    ///         </Section>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/layout/section/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
    /// The list of elements found inside the [section element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma section element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/section/
    pub children: Children,
}

/// Yew implementation of the [Bulma section element][bd].
///
/// Yew implementation of the section element, based on the specification
/// found in the [Bulma section element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::section::Section;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Section>{"This is some text in a section."}</Section>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/section/
#[function_component(Section)]
pub fn section(props: &SectionProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| {
            if *size != Size::Medium && *size != Size::Large {
                "".to_owned()
            } else {
                format!("{IS_PREFIX}-{size}")
            }
        })
        .unwrap_or("".to_owned());
    let class = ClassBuilder::default()
        .with_custom_class("section")
        .with_custom_class(&size)
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
