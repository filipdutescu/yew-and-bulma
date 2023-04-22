use yew::{function_component, html, AttrValue, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

use crate::utils::{
    class::ClassBuilder,
    constants::{HAS_PREFIX, IS_PREFIX},
    size::Size,
    BaseComponent,
};

/// Defines the possible alignment of a [Bulma breadcrumb component][bd].
///
/// Defines the possible alignment of a [Bulma breadcrumb element][bd], inside
/// its parent.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::breadcrumb::{Align, Breadcrumb, Crumb};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let crumbs = vec![
///         Crumb(AttrValue::from("#"), html!{ {"Trail"} }),
///         Crumb(AttrValue::from("#"), html!{ {"of"} }),
///         Crumb(AttrValue::from("#"), html!{ {"breadcrumbs"} }),
///     ];
///
///     html! {
///         <Breadcrumb align={Align::Center} {crumbs} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/breadcrumb/#alignment
#[derive(PartialEq)]
pub enum Align {
    // TODO: use #[default] when updating the MSRV
    Left,
    Center,
    Right,
}

impl From<&Align> for String {
    fn from(value: &Align) -> Self {
        match value {
            Align::Left => "".to_owned(),
            Align::Center => format!("{IS_PREFIX}-centered"),
            Align::Right => format!("{IS_PREFIX}-right"),
        }
    }
}

/// Defines the possible separators of a [Bulma breadcrumb component][bd].
///
/// Defines the possible separators of a [Bulma breadcrumb element][bd], shown
/// between each element.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::breadcrumb::{Breadcrumb, Crumb, Separator};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let crumbs = vec![
///         Crumb(AttrValue::from("#"), html!{ {"Trail"} }),
///         Crumb(AttrValue::from("#"), html!{ {"of"} }),
///         Crumb(AttrValue::from("#"), html!{ {"breadcrumbs"} }),
///     ];
///
///     html! {
///         <Breadcrumb separator={Separator::Bullet} {crumbs} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/breadcrumb/#alternative-separators
#[derive(PartialEq)]
pub enum Separator {
    // TODO: use #[default] when updating the MSRV
    Div,
    Arrow,
    Bullet,
    Dot,
    Succeeds,
}

impl From<&Separator> for String {
    fn from(value: &Separator) -> Self {
        match value {
            Separator::Div => "".to_owned(),
            Separator::Arrow => format!("{HAS_PREFIX}-arrow-separator"),
            Separator::Bullet => format!("{HAS_PREFIX}-bullet-separator"),
            Separator::Dot => format!("{HAS_PREFIX}-dot-separator"),
            Separator::Succeeds => format!("{HAS_PREFIX}-succeeds-separator"),
        }
    }
}

/// A wrapper for a [Bulma breadcrumb component][bd] inner element.
///
/// A wrapper for a [Bulma breadcrumb component][bd] inner element, in which the
/// first element is the [`href` HTML attribute][href] value and the second is
/// the inner HTML element that should be displayed inside the breadcrumb.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::breadcrumb::{Breadcrumb, Crumb};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let crumbs = vec![
///         Crumb(AttrValue::from("#"), html!{ {"Trail"} }),
///         Crumb(AttrValue::from("#"), html!{ {"of"} }),
///         Crumb(AttrValue::from("#"), html!{ {"breadcrumbs"} }),
///     ];
///
///     html! {
///         <Breadcrumb {crumbs} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/breadcrumb/
/// [href]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#href
#[derive(PartialEq, Clone)]
pub struct Crumb(
    /// The [`href` HTML attribute][href] value that the crumb points to.
    ///
    /// [href]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#href
    pub AttrValue,
    /// The inner HTML of the crumb.
    pub Html,
);

/// Defines the properties of the [Bulma breadcrumb component][bd].
///
/// Defines the properties of the breadcrumb component, based on the
/// specification found in the [Bulma breadcrumb component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::breadcrumb::{Breadcrumb, Crumb};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let crumbs = vec![
///         Crumb(AttrValue::from("#"), html!{ {"Trail"} }),
///         Crumb(AttrValue::from("#"), html!{ {"of"} }),
///         Crumb(AttrValue::from("#"), html!{ {"breadcrumbs"} }),
///     ];
///
///     html! {
///         <Breadcrumb {crumbs} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/breadcrumb/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct BreadcrumbProperties {
    /// Sets the size of the [breadcrumb component][bd].
    ///
    /// Sets the size of the [Bulma breadcrumb component][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     components::breadcrumb::{Breadcrumb, Crumb},
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     let crumbs = vec![
    ///         Crumb(AttrValue::from("#"), html!{ {"Trail"} }),
    ///         Crumb(AttrValue::from("#"), html!{ {"of"} }),
    ///         Crumb(AttrValue::from("#"), html!{ {"breadcrumbs"} }),
    ///     ];
    ///
    ///     html! {
    ///         <Breadcrumb size={Size::Large} {crumbs} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/breadcrumb/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
    /// Sets the alignment of a [Bulma breadcrumb component][bd].
    ///
    /// Sets the alignment of a [Bulma breadcrumb component][bd], which will
    /// receive these properties, inside its parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::components::breadcrumb::{Align, Breadcrumb, Crumb};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     let crumbs = vec![
    ///         Crumb(AttrValue::from("#"), html!{ {"Trail"} }),
    ///         Crumb(AttrValue::from("#"), html!{ {"of"} }),
    ///         Crumb(AttrValue::from("#"), html!{ {"breadcrumbs"} }),
    ///     ];
    ///
    ///     html! {
    ///         <Breadcrumb align={Align::Center} {crumbs} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/breadcrumb/#alignment
    #[prop_or(Align::Left)]
    pub align: Align,
    /// Sets the separator of a [Bulma breadcrumb component][bd].
    ///
    /// Sets the separator of a [Bulma breadcrumb component][bd], which will
    /// receive these properties, shown between each element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::components::breadcrumb::{Breadcrumb, Crumb, Separator};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     let crumbs = vec![
    ///         Crumb(AttrValue::from("#"), html!{ {"Trail"} }),
    ///         Crumb(AttrValue::from("#"), html!{ {"of"} }),
    ///         Crumb(AttrValue::from("#"), html!{ {"breadcrumbs"} }),
    ///     ];
    ///
    ///     html! {
    ///         <Breadcrumb separator={Separator::Bullet} {crumbs} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/breadcrumb/#alternative-separator
    #[prop_or(Separator::Div)]
    pub separator: Separator,
    /// The list of elements found inside the [breadcrumb component][bd].
    ///
    /// Defines the elements and their `href`s that will be found inside the
    /// [Bulma breadcrumb component][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/breadcrumb/
    pub crumbs: Vec<Crumb>,
}

/// Yew implementation of the [Bulma breadcrumb component][bd].
///
/// Yew implementation of the breadcrumb component, based on the specification
/// found in the [Bulma breadcrumb component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::breadcrumb::{Breadcrumb, Crumb};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let crumbs = vec![
///         Crumb(AttrValue::from("#"), html!{ {"Trail"} }),
///         Crumb(AttrValue::from("#"), html!{ {"of"} }),
///         Crumb(AttrValue::from("#"), html!{ {"breadcrumbs"} }),
///     ];
///
///     html! {
///         <Breadcrumb {crumbs} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/breadcrumb/
#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &BreadcrumbProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| {
            if *size == Size::Normal {
                "".to_owned()
            } else {
                format!("{IS_PREFIX}-{size}")
            }
        })
        .unwrap_or("".to_owned());
    let class = ClassBuilder::default()
        .with_custom_class("breadcrumb")
        .with_custom_class(&size)
        .with_custom_class(&String::from(&props.align))
        .with_custom_class(&String::from(&props.separator))
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    let no_children = props.crumbs.len();
    let mut crumbs = Vec::with_capacity(no_children);
    for (i, c) in props.crumbs.iter().enumerate() {
        let (href, elem) = (c.0.clone(), c.1.clone());
        let (class, aria_current) = if i < no_children - 1 {
            (Some("is-active"), Some("page"))
        } else {
            (None, None)
        };

        crumbs.push(html! {
            <li {class}>
                <a {href} aria-current={aria_current}>{elem}</a>
            </li>
        });
    }

    html! {
        <BaseComponent tag="nav" {class} ..props.into()>
            <ul>
                { for crumbs.into_iter() }
            </ul>
        </BaseComponent>
    }
}
