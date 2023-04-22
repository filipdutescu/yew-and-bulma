use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, Html, Properties,
};
use yew_and_bulma_macros::{base_component_properties, TypedChildren};

use crate::utils::class::ClassBuilder;
use crate::utils::BaseComponent;

/// Defines the properties of the [Bulma media object element][bd].
///
/// Defines the properties of the media object element, based on the
/// specification found in the [Bulma media object element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaContent, MediaLeft, MediaRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaLeft>{"Avatar goes here."}</MediaLeft>
///
///             <MediaContent>{"Content goes here."}</MediaContent>
///
///             <MediaRight>{"Dismiss goes here for example."}</MediaRight>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MediaProperties {
    /// The list of elements found inside the [media element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma media element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/media-object/
    pub children: ChildrenRenderer<MediaItem>,
}

/// Yew implementation of the [Bulma media object element][bd].
///
/// Yew implementation of the media object element, based on the specification
/// found in the [Bulma media object element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaContent, MediaLeft, MediaRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaLeft>{"Avatar goes here."}</MediaLeft>
///
///             <MediaContent>{"Content goes here."}</MediaContent>
///
///             <MediaRight>{"Dismiss goes here for example."}</MediaRight>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[function_component(Media)]
pub fn media(props: &MediaProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("media")
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

/// Defines the possible types of children from a [Bulma media object element][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma media object element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaContent, MediaLeft, MediaRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaLeft>{"Avatar goes here."}</MediaLeft>
///
///             <MediaContent>{"Content goes here."}</MediaContent>
///
///             <MediaRight>{"Dismiss goes here for example."}</MediaRight>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[derive(Clone, PartialEq, TypedChildren)]
pub enum MediaItem {
    MediaLeft(VChild<MediaLeft>),
    MediaContent(VChild<MediaContent>),
    MediaRight(VChild<MediaRight>),
}

/// Defines the properties of the [Bulma media left element][bd].
///
/// Defines the properties of the media left element, based on the
/// specification found in the [Bulma media left element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaLeft};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaLeft>{"Left goes here."}</MediaLeft>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MediaLeftProperties {
    /// The list of elements found inside the [media left element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma media left element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/media-object/
    pub children: Children,
}

/// Yew implementation of the [Bulma media left element][bd].
///
/// Yew implementation of the media left element, based on the specification
/// found in the [Bulma media left element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaLeft};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaLeft>{"Left goes here."}</MediaLeft>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[function_component(MediaLeft)]
pub fn media_left(props: &MediaLeftProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("media-left")
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

/// Defines the properties of the [Bulma media content element][bd].
///
/// Defines the properties of the media content element, based on the
/// specification found in the [Bulma media content element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaContent>{"Content goes here."}</MediaContent>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MediaContentProperties {
    /// The list of elements found inside the [media content element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma media content element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/media-object/
    pub children: Children,
}

/// Yew implementation of the [Bulma media content element][bd].
///
/// Yew implementation of the media content element, based on the specification
/// found in the [Bulma media content element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaContent>{"Content goes here."}</MediaContent>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[function_component(MediaContent)]
pub fn media_content(props: &MediaContentProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("media-content")
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

/// Defines the properties of the [Bulma media right element][bd].
///
/// Defines the properties of the media right element, based on the
/// specification found in the [Bulma media right element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaRight>{"Right goes here."}</MediaRight>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MediaRightProperties {
    /// The list of elements found inside the [media right element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma media right element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/media-object/
    pub children: Children,
}

/// Yew implementation of the [Bulma media right element][bd].
///
/// Yew implementation of the media right element, based on the specification
/// found in the [Bulma media right element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::media::{Media, MediaRight};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Media>
///             <MediaRight>{"Right goes here."}</MediaRight>
///         </Media>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/media-object/
#[function_component(MediaRight)]
pub fn media_right(props: &MediaRightProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("media-right")
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
