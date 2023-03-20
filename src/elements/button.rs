use yew::{function_component, html, Children, Classes, Html, Properties};

use crate::{
    helpers::color::Color,
    utils::size::Size,
    utils::{
        class::ClassBuilder,
        constants::{ARE_PREFIX, IS_PREFIX},
    },
};

/// Defines the possible alignment of buttons from a [buttons element][bd].
///
/// Defines the possible alignment of buttons found inside a
/// [Bulma buttons element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::button::{Button, Buttons, Align};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Buttons align={Align::Center}>
///             <Button>{"Button"}</Button>
///             <Button>{"Button"}</Button>
///             <Button>{"Button"}</Button>
///         </Buttons>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/button/#list-of-buttons
#[derive(PartialEq)]
pub enum Align {
    // TODO: use #[default] when updating the MSRV
    Left,
    Center,
    Right,
}

/// Defines the properties of the [Bulma buttons element][bd].
///
/// Defines the properties of the buttons element, based on the specification
/// found in the [Bulma buttons element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::button::{Button, Buttons};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Buttons>
///             <Button>{"Button"}</Button>
///         </Buttons>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/button/#list-of-buttons
#[derive(Properties, PartialEq)]
pub struct ButtonsProperties {
    /// Sets the size of the buttons found inside the [buttons element][bd].
    ///
    /// Sets the size of the buttons that will be found inside the
    /// [Bulma buttons element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::button::{Button, Buttons},
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Buttons size={Size::Large}>
    ///             <Button>{"Button"}</Button>
    ///             <Button>{"Button"}</Button>
    ///             <Button>{"Button"}</Button>
    ///         </Buttons>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/button/#list-of-buttons
    #[prop_or_default]
    pub size: Option<Size>,
    /// Whether to attach the buttons found inside the [buttons element][bd].
    ///
    /// Whether or not to attach the buttons that will be found inside the
    /// [Bulma buttons element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::button::{Button, Buttons},
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Buttons addons=true>
    ///             <Button>{"Button"}</Button>
    ///             <Button>{"Button"}</Button>
    ///             <Button>{"Button"}</Button>
    ///         </Buttons>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/button/#list-of-buttons
    #[prop_or_default]
    pub addons: bool,
    /// Sets the alignment of buttons from a [buttons element][bd].
    ///
    /// Sets the alignment of buttons found inside a
    /// [Bulma buttons element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::button::{Button, Buttons, Align};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Buttons align={Align::Center}>
    ///             <Button>{"Button"}</Button>
    ///             <Button>{"Button"}</Button>
    ///             <Button>{"Button"}</Button>
    ///         </Buttons>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/button/#list-of-buttons
    #[prop_or(Align::Left)]
    pub align: Align,
    /// The list of elements found inside the [buttons element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma buttons element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/button/#list-of-buttons
    pub children: Children,
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

/// Yew implementation of the [Bulma buttons element][bd].
///
/// Yew implementation of the buttons element, based on the specification found
/// in the [Bulma buttons element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::button::{Button, Buttons};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Buttons>
///             <Button>{"A button"}</Button>
///             <Button>{"Another button"}</Button>
///         </Buttons>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/buttons/
#[function_component(Buttons)]
pub fn buttons(props: &ButtonsProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(|size| {
            if Size::Normal == *size {
                "".to_owned()
            } else {
                format!("{ARE_PREFIX}-{size}")
            }
        })
        .unwrap_or("".to_owned());
    let addons = if props.addons { "has-addons" } else { "" }.to_owned();
    let class = ClassBuilder::default()
        .with_custom_class("buttons")
        .with_custom_class(&size)
        .with_custom_class(&addons)
        .with_custom_class(&String::from(&props.align))
        .build();

    html! {
        <div {class}>
            { for props.children.iter() }
        </div>
    }
}

/// Defines the possible states of a [button element][bd].
///
/// Defines the possible states of a [Bulma button element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::button::{Button, State};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Button state={State::Loading}>{"Button"}</Button>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/button/#states
#[derive(PartialEq)]
pub enum State {
    Normal,
    Hover,
    Focus,
    Active,
    Loading,
    Static,
}

impl From<&State> for String {
    fn from(value: &State) -> Self {
        let state = match value {
            State::Normal => "normal",
            State::Hover => "hover",
            State::Focus => "focus",
            State::Active => "active",
            State::Loading => "loading",
            State::Static => "static",
        };

        format!("{IS_PREFIX}-{state}")
    }
}

/// Defines the possible style of a [button element][bd].
///
/// Defines the possible style of a [Bulma button element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::button::{Button, Style};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Button style={Style::Outlined}>{"Button"}</Button>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/button/#style
#[derive(PartialEq)]
pub enum Style {
    Outlined,
    Inverted,
    InvertedOutlined,
    Rounded,
}

impl From<&Style> for String {
    fn from(value: &Style) -> Self {
        match value {
            Style::Outlined => format!("{IS_PREFIX}-outlined"),
            Style::Inverted => format!("{IS_PREFIX}-inverted"),
            Style::InvertedOutlined => format!("{IS_PREFIX}-inverted {IS_PREFIX}-outlined"),
            Style::Rounded => format!("{IS_PREFIX}-rounded"),
        }
    }
}

/// Defines the properties of the [Bulma button element][bd].
///
/// Defines the properties of the button element, based on the specification
/// found in the [Bulma button element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::button::Button;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Button>{"Button"}</Button>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/button/
#[derive(Properties, PartialEq)]
pub struct ButtonProperties {
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub light: Option<bool>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub responsive: bool,
    #[prop_or_default]
    pub fullwidth: bool,
    #[prop_or_default]
    pub style: Option<Style>,
    #[prop_or_default]
    pub state: Option<State>,
    #[prop_or_default]
    pub disabled: bool,
    pub children: Children,
}

impl From<&ButtonProperties> for Classes {
    fn from(value: &ButtonProperties) -> Self {
        let size = value
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
        let style = value
            .style
            .as_ref()
            .map(String::from)
            .unwrap_or("".to_string());
        let fullwidth = if value.fullwidth {
            format!("{IS_PREFIX}-fullwidth")
        } else {
            "".to_owned()
        };
        let responsive = if value.responsive {
            format!("{IS_PREFIX}-responsive")
        } else {
            "".to_string()
        };
        let state = value
            .state
            .as_ref()
            .map(String::from)
            .unwrap_or("".to_owned());

        ClassBuilder::default()
            .with_custom_class("button")
            .with_color(value.color)
            .is_light(value.light)
            .with_custom_class(&size)
            .with_custom_class(&responsive)
            .with_custom_class(&fullwidth)
            .with_custom_class(&style)
            .with_custom_class(&state)
            .build()
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let class: Classes = props.into();

    html! {
        <button {class} disabled={props.disabled}>{ for props.children.iter() }</button>
    }
}
