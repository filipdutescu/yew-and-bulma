use yew::{function_component, html, Children, Classes, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

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

impl From<&Align> for String {
    fn from(value: &Align) -> Self {
        match value {
            Align::Left => "".to_owned(),
            Align::Center => format!("{IS_PREFIX}-centered"),
            Align::Right => format!("{IS_PREFIX}-right"),
        }
    }
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
#[base_component_properties]
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
/// [bd]: https://bulma.io/documentation/elements/button/
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
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <div id={props.id.clone()} {class}
            onclick={props.onclick.clone()} onwheel={props.onwheel.clone()} onscroll={props.onscroll.clone()}
            onmousedown={props.onmousedown.clone()} onmousemove={props.onmousemove.clone()} onmouseout={props.onmouseout.clone()} onmouseover={props.onmouseover.clone()} onmouseup={props.onmouseup.clone()}
            ondrag={props.ondrag.clone()} ondragend={props.ondragend.clone()} ondragenter={props.ondragenter.clone()} ondragleave={props.ondragleave.clone()} ondragover={props.ondragover.clone()} ondragstart={props.ondragstart.clone()} ondrop={props.ondrop.clone()}
            oncopy={props.oncopy.clone()} oncut={props.oncut.clone()} onpaste={props.onpaste.clone()}
            onkeydown={props.onkeydown.clone()} onkeypress={props.onkeypress.clone()} onkeyup={props.onkeyup.clone()}
            onblur={props.onblur.clone()} onchange={props.onchange.clone()} oncontextmenu={props.oncontextmenu.clone()} onfocus={props.onfocus.clone()} oninput={props.oninput.clone()} oninvalid={props.oninvalid.clone()} onreset={props.onreset.clone()} onselect={props.onselect.clone()} onsubmit={props.onsubmit.clone()}
            onabort={props.onabort.clone()} oncanplay={props.oncanplay.clone()} oncanplaythrough={props.oncanplaythrough.clone()} oncuechange={props.oncuechange.clone()}
            ondurationchange={props.ondurationchange.clone()} onemptied={props.onemptied.clone()} onended={props.onended.clone()} onerror={props.onerror.clone()}
            onloadeddata={props.onloadeddata.clone()} onloadedmetadata={props.onloadedmetadata.clone()} onloadstart={props.onloadstart.clone()} onpause={props.onpause.clone()}
            onplay={props.onplay.clone()} onplaying={props.onplaying.clone()} onprogress={props.onprogress.clone()} onratechange={props.onratechange.clone()}
            onseeked={props.onseeked.clone()} onseeking={props.onseeking.clone()} onstalled={props.onstalled.clone()} onsuspend={props.onsuspend.clone()}
            ontimeupdate={props.ontimeupdate.clone()} onvolumechange={props.onvolumechange.clone()} onwaiting={props.onwaiting.clone()}>
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
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ButtonProperties {
    /// Sets the color of the [Bulma button element][bd].
    ///
    /// Sets the color of the [Bulma button element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::button::Button,
    ///     helpers::color::Color,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Button color={Color::Primary}>{"Button"}</Button>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/button/#colors
    #[prop_or_default]
    pub color: Option<Color>,
    /// Whether or not the color of the [button element][bd] should be light.
    ///
    /// Whether or not the color of the [Bulma button element][bd], which will
    /// receive these properties, should be of the light variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::button::Button,
    ///     helpers::color::Color,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Button light=true color={Color::Primary}>{"Button"}</Button>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/button/#colors
    #[prop_or_default]
    pub light: Option<bool>,
    /// Sets the size of the [Bulma button element][bd].
    ///
    /// Sets the size of the [Bulma button element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::button::Button,
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Button size={Size::Large}>{"Button"}</Button>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/button/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
    /// Whether the size of the [button element][bd] should be responsive.
    ///
    /// Whether or not the size of the [Bulma button element][bd], which will
    /// receive these properties, will be responsive.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     elements::button::Button,
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Button responsive=true size={Size::Large}>{"Button"}</Button>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/button/#responsive-sizes
    #[prop_or_default]
    pub responsive: bool,
    /// Whether the [button element][bd] should have the width of its parent.
    ///
    /// Whether or not the [Bulma button element][bd], which will receive these
    /// properties, will have the same width as its parent.
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
    ///         <Button fullwidth=true>{"Button"}</Button>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/button/#displays
    #[prop_or_default]
    pub fullwidth: bool,
    /// Sets the style of the [Bulma button element][bd].
    ///
    /// Sets the style of the [Bulma button element][bd] which will receive
    /// these properties.
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
    /// [bd]: https://bulma.io/documentation/elements/button/#styles
    #[prop_or_default]
    pub style: Option<Style>,
    /// Sets the state of the [Bulma button element][bd].
    ///
    /// Sets the state of the [Bulma button element][bd] which will receive
    /// these properties.
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
    ///         <Button state={State::Loading}>
    ///             {"This label will be a spinner"}
    ///         </Button>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/button/#states
    #[prop_or_default]
    pub state: Option<State>,
    /// Whether or not the [Bulma button element][bd] should be disabled.
    ///
    /// Whether or not the [Bulma button element][bd], which will receive these
    /// properties, will be disabled. This means it will have the *HTML
    /// attribute* `disabled` set.
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
    ///         <Button disabled=true>{"Button"}</Button>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/button/#displays
    #[prop_or_default]
    pub disabled: bool,
    /// The list of elements found inside the [button element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma button element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/button/
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
            .with_custom_class(
                &value
                    .class
                    .as_ref()
                    .map(|c| c.to_string())
                    .unwrap_or("".to_owned()),
            )
            .build()
    }
}

/// Yew implementation of the [Bulma button element][bd].
///
/// Yew implementation of the button element, based on the specification found
/// in the [Bulma button element documentation][bd].
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
///         <Button>{"A button"}</Button>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/button/
#[function_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let class: Classes = props.into();

    html! {
        <button id={props.id.clone()} {class} disabled={props.disabled}
            onclick={props.onclick.clone()} onwheel={props.onwheel.clone()} onscroll={props.onscroll.clone()}
            onmousedown={props.onmousedown.clone()} onmousemove={props.onmousemove.clone()} onmouseout={props.onmouseout.clone()} onmouseover={props.onmouseover.clone()} onmouseup={props.onmouseup.clone()}
            ondrag={props.ondrag.clone()} ondragend={props.ondragend.clone()} ondragenter={props.ondragenter.clone()} ondragleave={props.ondragleave.clone()} ondragover={props.ondragover.clone()} ondragstart={props.ondragstart.clone()} ondrop={props.ondrop.clone()}
            oncopy={props.oncopy.clone()} oncut={props.oncut.clone()} onpaste={props.onpaste.clone()}
            onkeydown={props.onkeydown.clone()} onkeypress={props.onkeypress.clone()} onkeyup={props.onkeyup.clone()}
            onblur={props.onblur.clone()} onchange={props.onchange.clone()} oncontextmenu={props.oncontextmenu.clone()} onfocus={props.onfocus.clone()} oninput={props.oninput.clone()} oninvalid={props.oninvalid.clone()} onreset={props.onreset.clone()} onselect={props.onselect.clone()} onsubmit={props.onsubmit.clone()}
            onabort={props.onabort.clone()} oncanplay={props.oncanplay.clone()} oncanplaythrough={props.oncanplaythrough.clone()} oncuechange={props.oncuechange.clone()}
            ondurationchange={props.ondurationchange.clone()} onemptied={props.onemptied.clone()} onended={props.onended.clone()} onerror={props.onerror.clone()}
            onloadeddata={props.onloadeddata.clone()} onloadedmetadata={props.onloadedmetadata.clone()} onloadstart={props.onloadstart.clone()} onpause={props.onpause.clone()}
            onplay={props.onplay.clone()} onplaying={props.onplaying.clone()} onprogress={props.onprogress.clone()} onratechange={props.onratechange.clone()}
            onseeked={props.onseeked.clone()} onseeking={props.onseeking.clone()} onstalled={props.onstalled.clone()} onsuspend={props.onsuspend.clone()}
            ontimeupdate={props.ontimeupdate.clone()} onvolumechange={props.onvolumechange.clone()} onwaiting={props.onwaiting.clone()}>
            { for props.children.iter() }
        </button>
    }
}
