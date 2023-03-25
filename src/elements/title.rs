use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::utils::{class::ClassBuilder, constants::IS_PREFIX};

/// Defines the possible sizes of a [Bulma title element][bd].
///
/// Defines the possible sizes of a [Bulma title element][bd] and of a
/// [Bulma subtitle element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::title::{Title, Size};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Title size={Size::One}>{"Hello, world!"}</Title>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/title/#sizes
#[derive(PartialEq)]
pub enum Size {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl From<&Size> for String {
    fn from(value: &Size) -> Self {
        match value {
            Size::One => "1",
            Size::Two => "2",
            Size::Three => "3",
            Size::Four => "4",
            Size::Five => "5",
            Size::Six => "6",
        }
        .to_owned()
    }
}

/// Defines the properties of the [Bulma title element][bd].
///
/// Defines the properties of the title element, based on the specification
/// found in the [Bulma title element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::title::Title;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Title>{"Hello, world!"}</Title>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/title/
#[derive(Properties, PartialEq)]
pub struct TitleProperties {
    /// Sets the [HTML id attribute][id] of the element.
    ///
    /// Sets the [HTML id attrbiute][id] of the element which will receive
    /// these properties.
    ///
    /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Sets the classes to be appended to the [HTML class attribute][class].
    ///
    /// Sets the classes to be appended to [HTML class attrbiute][class] of the
    /// element which will receive these properties.
    ///
    /// [class]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class
    #[prop_or_default]
    pub class: Option<Classes>,
    /// Sets the size of the [Bulma title element][bd].
    ///
    /// Sets the size of the [Bulma title element][bd] which will receive
    /// these properties. By default, it is
    /// [`crate::elements::title::Size::Three`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::title::{Title, Size};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Title size={Size::One}>{"Hello, world!"}</Title>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/title/#sizes
    #[prop_or(Size::Three)]
    pub size: Size,
    /// Whether the [title element][bd] should have increased space below.
    ///
    /// Whether or not the [Bulma title element][bd], which will receive these
    /// properties, should have more space between it and the next element than
    /// usual.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::title::Title;
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Title spaced=true>{"Hello, world!"}</Title>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/title/
    #[prop_or_default]
    pub spaced: bool,
    /// The list of elements found inside the [title element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma title element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/title/
    pub children: Children,
}

impl From<&TitleProperties> for String {
    fn from(value: &TitleProperties) -> Self {
        let mut modifier_classes = String::with_capacity(if value.spaced { 20 } else { 10 });

        modifier_classes.push_str("title");
        modifier_classes.push_str(&format!(" {IS_PREFIX}-{}", String::from(&value.size)));
        if value.spaced {
            modifier_classes.push_str(" is-spaced");
        }

        modifier_classes
    }
}

/// Yew implementation of the [Bulma title element][bd].
///
/// Yew implementation of the title element, based on the specification found
/// in the [Bulma title element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::title::Title;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Title>{"Hello, world!"}</Title>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/title/
#[function_component(Title)]
pub fn title(props: &TitleProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class(&String::from(props))
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <@{format!("h{}", String::from(&props.size))} id={props.id.clone()} {class}>{ for props.children.iter() }</@>
    }
}

/// Defines the properties of the [Bulma subtitle element][bd].
///
/// Defines the properties of the subtitle element, based on the specification
/// found in the [Bulma subtitle element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::title::Subtitle;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Subtitle>{"Hello, world!"}</Subtitle>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/title/
#[derive(Properties, PartialEq)]
pub struct SubtitleProperties {
    /// Sets the [HTML id attribute][id] of the element.
    ///
    /// Sets the [HTML id attrbiute][id] of the element which will receive
    /// these properties.
    ///
    /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Sets the classes to be appended to the [HTML class attribute][class].
    ///
    /// Sets the classes to be appended to [HTML class attrbiute][class] of the
    /// element which will receive these properties.
    ///
    /// [class]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class
    #[prop_or_default]
    pub class: Option<Classes>,
    /// Sets the size of the [Bulma subtitle element][bd].
    ///
    /// Sets the size of the [Bulma subtitle element][bd] which will receive
    /// these properties. By default, it is
    /// [`crate::elements::title::Size::Five`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::title::{Size, Subtitle};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Subtitle size={Size::Three}>{"Hello, world!"}</Subtitle>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/title/#sizes
    #[prop_or(Size::Five)]
    pub size: Size,
    /// Whether the [subtitle element][bd] should have increased space below.
    ///
    /// Whether or not the [Bulma subtitle element][bd], which will receive
    /// these properties, should have more space between it and the next element
    /// than usual.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::title::Subtitle;
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Subtitle spaced=true>{"Hello, world!"}</Subtitle>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/title/
    #[prop_or_default]
    pub spaced: bool,
    /// The list of elements found inside the [subtitle element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma subtitle element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/title/
    pub children: Children,
}

impl From<&SubtitleProperties> for String {
    fn from(value: &SubtitleProperties) -> Self {
        let mut modifier_classes = String::with_capacity(if value.spaced { 23 } else { 13 });

        modifier_classes.push_str("subtitle");
        modifier_classes.push_str(&format!(" {IS_PREFIX}-{}", String::from(&value.size)));
        if value.spaced {
            modifier_classes.push_str(" is-spaced");
        }

        modifier_classes
    }
}

/// Yew implementation of the [Bulma subtitle element][bd].
///
/// Yew implementation of the subtitle element, based on the specification found
/// in the [Bulma subtitle element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::title::Subtitle;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Subtitle>{"Hello, world!"}</Subtitle>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/title/
#[function_component(Subtitle)]
pub fn subtitle(props: &SubtitleProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class(&String::from(props))
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <@{format!("h{}", String::from(&props.size))} id={props.id.clone()} {class}>{ for props.children.iter() }</@>
    }
}
