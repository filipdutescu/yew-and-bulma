use yew::html;
use yew::{function_component, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

use crate::utils::BaseComponent;
use crate::utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size};

/// Defines the possible alignment of a [Bulma tabs component][bd].
///
/// Defines the possible alignment of a [Bulma tabs element][bd], inside
/// its parent.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::tabs::{Align, Tab, Tabs};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let tabs = vec![
///         Tab(html!{ {"Tab 1"} }, true),
///         Tab(html!{ {"Tab 2"} }, false),
///         Tab(html!{ {"Tab 3"} }, false),
///     ];
///
///     html! {
///         <Tabs align={Align::Center} {tabs} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/tabs/#alignment
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

/// Defines the possible style of a [tabs element][bd].
///
/// Defines the possible style of a [Bulma tabs element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::tabs::{Tab, Tabs, Style};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let tabs = vec![
///         Tab(html!{ {"Tab 1"} }, true),
///         Tab(html!{ {"Tab 2"} }, false),
///         Tab(html!{ {"Tab 3"} }, false),
///     ];
///
///     html! {
///         <Tabs style={Style::Boxed} {tabs} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/tabs/#styles
#[derive(PartialEq)]
pub enum Style {
    Boxed,
    Toggle,
    ToggleRounded,
}

impl From<&Style> for String {
    fn from(value: &Style) -> Self {
        match value {
            Style::Boxed => format!("{IS_PREFIX}-boxed"),
            Style::Toggle => format!("{IS_PREFIX}-toggle"),
            Style::ToggleRounded => format!("{IS_PREFIX}-toggle {IS_PREFIX}-toggle-rounded"),
        }
    }
}

/// A wrapper for a [Bulma tabs component][bd] inner element.
///
/// A wrapper for a [Bulma tabs component][bd] inner element, in which the
/// first element is the inner HTML element that should be displayed inside the
/// tabs and the second element determines whether or not the tab is active.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::tabs::{Tab, Tabs};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let tabs = vec![
///         Tab(html!{ {"Tab 1"} }, true),
///         Tab(html!{ {"Tab 2"} }, false),
///         Tab(html!{ {"Tab 3"} }, false),
///     ];
///
///     html! {
///         <Tabs {tabs} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/tabs/
#[derive(PartialEq, Clone)]
pub struct Tab(
    /// The inner HTML of the tab.
    pub Html,
    /// Whether or not this tab is active.
    pub bool,
);

/// Defines the properties of the [Bulma tabs component][bd].
///
/// Defines the properties of the tabs component, based on the
/// specification found in the [Bulma tabs component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::tabs::{Tab, Tabs};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let tabs = vec![
///         Tab(html!{ {"Tab 1"} }, true),
///         Tab(html!{ {"Tab 2"} }, false),
///         Tab(html!{ {"Tab 3"} }, false),
///     ];
///
///     html! {
///         <Tabs {tabs} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/tabs/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct TabsProperties {
    /// Sets the size of the [tabs component][bd].
    ///
    /// Sets the size of the [Bulma tabs component][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     components::tabs::{Tab, Tabs},
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     let tabs = vec![
    ///         Tab(html!{ {"Tab 1"} }, true),
    ///         Tab(html!{ {"Tab 2"} }, false),
    ///         Tab(html!{ {"Tab 3"} }, false),
    ///     ];
    ///
    ///     html! {
    ///         <Tabs size={Size::Large} {tabs} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/tabs/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
    /// Sets the alignment of a [Bulma tabs component][bd].
    ///
    /// Sets the alignment of a [Bulma tabs component][bd], which will
    /// receive these properties, inside its parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::components::tabs::{Align, Tab, Tabs};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     let tabs = vec![
    ///         Tab(html!{ {"Tab 1"} }, true),
    ///         Tab(html!{ {"Tab 2"} }, false),
    ///         Tab(html!{ {"Tab 3"} }, false),
    ///     ];
    ///
    ///     html! {
    ///         <Tabs align={Align::Center} {tabs} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/tabs/#alignment
    #[prop_or(Align::Left)]
    pub align: Align,
    /// Whether the [tabs element][bd] should have the width of its parent.
    ///
    /// Whether or not the [Bulma tabs element][bd], which will receive these
    /// properties, will have the same width as its parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::tabs::{Tab, Tabs};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     let tabs = vec![
    ///         Tab(html!{ {"Tab 1"} }, true),
    ///         Tab(html!{ {"Tab 2"} }, false),
    ///         Tab(html!{ {"Tab 3"} }, false),
    ///     ];
    ///
    ///     html! {
    ///         <Tabs fullwidth=true {tabs} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/tabs/#styles
    #[prop_or_default]
    pub fullwidth: bool,
    /// Sets the style of the [Bulma tabs element][bd].
    ///
    /// Sets the style of the [Bulma tabs element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::tabs::{Tab, Tabs, Style};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     let tabs = vec![
    ///         Tab(html!{ {"Tab 1"} }, true),
    ///         Tab(html!{ {"Tab 2"} }, false),
    ///         Tab(html!{ {"Tab 3"} }, false),
    ///     ];
    ///
    ///     html! {
    ///         <Tabs style={Style::Boxed} {tabs} />
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/tabs/#styles
    #[prop_or_default]
    pub style: Option<Style>,
    /// The list of elements found inside the [tabs component][bd].
    ///
    /// Defines the elements and their active state that will be found inside the
    /// [Bulma tabs component][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/tabs/
    pub tabs: Vec<Tab>,
}

/// Yew implementation of the [Bulma tabs component][bd].
///
/// Yew implementation of the tabs component, based on the specification
/// found in the [Bulma tabs component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::tabs::{Tab, Tabs};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let tabs = vec![
///         Tab(html!{ {"Tab 1"} }, true),
///         Tab(html!{ {"Tab 2"} }, false),
///         Tab(html!{ {"Tab 3"} }, false),
///     ];
///
///     html! {
///         <Tabs {tabs} />
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/tabs/
#[function_component(Tabs)]
pub fn tabs(props: &TabsProperties) -> Html {
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
    let fullwidth = if props.fullwidth {
        format!("{IS_PREFIX}-fullwidth")
    } else {
        "".to_owned()
    };
    let style = props
        .style
        .as_ref()
        .map(String::from)
        .unwrap_or("".to_string());
    let class = ClassBuilder::default()
        .with_custom_class("tabs")
        .with_custom_class(&size)
        .with_custom_class(&String::from(&props.align))
        .with_custom_class(&fullwidth)
        .with_custom_class(&style)
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    let no_children = props.tabs.len();
    let mut tabs = Vec::with_capacity(no_children);
    for t in props.tabs.iter() {
        let (elem, is_active) = (t.0.clone(), t.1);
        let class = is_active.then_some("is-active");

        tabs.push(html! {
            <li {class}>
                <a>{elem}</a>
            </li>
        });
    }

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for tabs.into_iter() }
        </BaseComponent>
    }
}
