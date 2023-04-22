use yew::html;
use yew::{function_component, AttrValue, ChildrenWithProps, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

use crate::utils::BaseComponent;
use crate::{
    helpers::color::TextColor,
    utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size},
};

/// Defines the properties of the [Bulma icon text element][bd].
///
/// Defines the properties of the icon text element, based on the specification
/// found in the [Bulma icon text element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::icon::{Icon, IconText};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <IconText>
///             <Icon
///                 icon={html! {
///                     <i class="fas fa-home"></i>
///                 }}
///                 text="Home" />
///         </IconText>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/icon/#icon-text
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct IconTextProperties {
    /// Whether or not the [icon element][bd] should be displayed as a block.
    ///
    /// Whether or not the [Bulma icon element][bd], which will receive these
    /// properties, should be displayed as a block, instead of inline
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::icon::{Icon, IconText};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <IconText flex=true>
    ///             <Icon
    ///                 icon={html! {
    ///                     <i class="fas fa-home"></i>
    ///                 }}
    ///                 text="Home" />
    ///         </IconText>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/icon/#icon-text
    #[prop_or_default]
    pub flex: bool,
    /// Sets the color of the text found inside the [icon text element][bd].
    ///
    /// Sets the color of the text (and icon, if not overriden) of the
    /// [Bulma icon element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::icon::{Icon, IconText},
    ///     helpers::color::TextColor
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <IconText color={TextColor::Primary}>
    ///             <Icon
    ///                 icon={html! {
    ///                     <i class="fas fa-home"></i>
    ///                 }}
    ///                 text="Home" />
    ///         </IconText>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/icon/#colors
    #[prop_or_default]
    pub color: Option<TextColor>,
    /// The list of elements found inside the [content element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma content element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/icon/
    pub children: ChildrenWithProps<Icon>,
}

/// Yew helper for the [Bulma icon text element][bd].
///
/// Yew helepr for the icon text element, based on the specification found in
/// the [Bulma icon text element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::icon::{Icon, IconText};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <IconText>
///             <Icon
///                 icon={html! {
///                     <i class="fas fa-home"></i>
///                 }}
///                 text="Home" />
///         </IconText>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/icon/#icon-text
#[function_component(IconText)]
pub fn icon_text(props: &IconTextProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("icon-text")
        .with_text_color(props.color)
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag={if props.flex { "div" } else { "span" }} {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma icon element][bd].
///
/// Defines the properties of the icon element, based on the specification
/// found in the [Bulma icon element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::icon::Icon;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Icon
///             icon={html! {
///                 <i class="fas fa-home"></i>
///             }} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/icon/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct IconProperties {
    /// Sets the text that should be displayed with the [icon element][bd].
    ///
    /// Sets the text that should be displayed alongside the
    /// [Bulma icon element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::icon::{Icon, IconText};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <IconText>
    ///             <Icon
    ///                 icon={html! {
    ///                     <i class="fas fa-home"></i>
    ///                 }}
    ///                 text="Home" />
    ///         </IconText>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/icon/
    #[prop_or_default]
    pub text: AttrValue,
    /// Sets the color of the [Bulma icon element][bd].
    ///
    /// Sets the color of the [Bulma icon element][bd] which will receive these
    /// properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::icon::Icon,
    ///     helpers::color::TextColor
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Icon
    ///             icon={html! {
    ///                 <i class="fas fa-home"></i>
    ///             }}
    ///             color={TextColor::Primary} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/icon/#colors
    #[prop_or_default]
    pub color: Option<TextColor>,
    /// Sets the size of the [Bulma icon element][bd].
    ///
    /// Sets the size of the [Bulma icon element][bd] which will receive these
    /// properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::icon::Icon,
    ///     utils::size::Size
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Icon
    ///             icon={html! {
    ///                 <i class="fas fa-home"></i>
    ///             }}
    ///             size={Size::Large} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/icon/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
    /// Sets the framework specific HTML used in the [Bulma image element][bd].
    ///
    /// Sets the framework specific HTML to be encapsulated inside the
    /// [Bulma image element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::icon::Icon;
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Icon
    ///             icon={html! {
    ///                 <i class="fas fa-home"></i>
    ///             }} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/icon/
    pub icon: Html,
}

/// Yew implementation of the [Bulma icon element][bd].
///
/// Yew implementation of the icon element, based on the specification found in
/// the [Bulma icon element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::icon::Icon;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Icon
///             icon={html! {
///                 <i class="fas fa-home"></i>
///             }} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/icon/
#[function_component(Icon)]
pub fn icon(props: &IconProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| {
            if Size::Normal == *size {
                "".to_owned()
            } else {
                format!("{IS_PREFIX}-{size}")
            }
        })
        .unwrap_or("".to_owned());
    let class = ClassBuilder::default()
        .with_custom_class("icon")
        .with_text_color(props.color)
        .with_custom_class(&size)
        .with_custom_class(&props.class.to_string())
        .build();

    let icon_html = html! {
        <BaseComponent tag="span" {class} ..props.into()>
            { props.icon.clone() }
        </BaseComponent>
    };
    html! {
        <>
        {icon_html}
        if !props.text.is_empty() {
            <span>{ &props.text }</span>
        }
        </>
    }
}
