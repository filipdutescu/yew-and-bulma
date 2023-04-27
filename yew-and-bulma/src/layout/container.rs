use std::fmt::Display;

use yew::html;
use yew::{function_component, Children, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

use crate::utils::BaseComponent;
use crate::utils::{class::ClassBuilder, constants::IS_PREFIX};

/// Defines the points from which a [container element][bd] is not full width.
///
/// Defines the points from which a [Bulma container element][bd] no longer has
/// full width, but rather a fixed one. Those points are viewports from which
/// the component is "active", concretely, space around the children is
/// visible.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::container::{Container, Width};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Container width={Width::FullHD}>
///             {"This is some text in a container."}
///         </Container>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/container/#overview
#[derive(PartialEq)]
pub enum Width {
    Widescreen,
    FullHD,
    MaxDesktop,
    MaxWidescreen,
}

impl Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = match self {
            Width::Widescreen => "widescreen",
            Width::FullHD => "fullhd",
            Width::MaxDesktop => "max-desktop",
            Width::MaxWidescreen => "max-widescreen",
        };

        write!(f, "{width}")
    }
}

/// Defines the properties of the [Bulma container element][bd].
///
/// Defines the properties of the container element, based on the specification
/// found in the [Bulma container element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::container::Container;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Container>{"This is some text in a container."}</Container>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/container/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ContainerProperties {
    /// Sets the point from which the [container element][bd] is no longer full
    /// width.
    ///
    /// Sets the viewport point from which the [Bulma container element][bd],
    /// which will receive these properties, is no longer full width.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::container::{Container, Width};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Container width={Width::FullHD}>
    ///             {"This is some text in a container."}
    ///         </Container>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/layout/container/#overview
    #[prop_or_default]
    pub width: Option<Width>,
    /// Whether or not the [container element][bd] should be fluid.
    ///
    /// Whether or not the [Bulma container element][bd], which will receive
    /// these properties, should be a fluid one. This means is won't have a
    /// maximum width, but rather a fixed horizontal margin.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::container::Container;
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Container fluid={true}>
    ///             {"This is some text in a container."}
    ///         </Container>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/layout/container/#fluid-container
    #[prop_or_default]
    pub fluid: bool,
    /// The list of elements found inside the [container element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma container element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/container/
    pub children: Children,
}

/// Yew implementation of the [Bulma container element][bd].
///
/// Yew implementation of the container element, based on the specification
/// found in the [Bulma container element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::container::Container;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Container>{"This is some text in a container."}</Container>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/container/
#[function_component(Container)]
pub fn container(props: &ContainerProperties) -> Html {
    let width = props
        .width
        .as_ref()
        .map(|width| format!("{IS_PREFIX}-{width}"))
        .unwrap_or("".to_string());
    let fluid = if props.fluid { "is-fluid" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("container")
        .with_custom_class(&width)
        .with_custom_class(fluid)
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}
