use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::utils::{class::ClassBuilder, constants::IS_PREFIX};

/// Defines the properties of the [Bulma image element][bd].
///
/// Defines the properties of the image element, based on the specification
/// found in the [Bulma image element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::image::{Figure, Image};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Figure>
///             <Image src={"media/images/img.png"} />
///         </Figure>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/image/
#[derive(Properties, PartialEq)]
pub struct ImageProperties {
    /// Sets the [HTML id attribute][id] of the element.
    ///
    /// Sets the [HTML id attrbiute][id] of the element which will receive
    /// these properties.
    ///
    /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Sets the classes to be appended to the [HTML classattribute][class].
    ///
    /// Sets the classes to be appended to [HTML class attrbiute][class] of the
    /// element which will receive these properties.
    ///
    /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class
    #[prop_or_default]
    pub class: Option<Classes>,
    /// Whether or not the [image element][bd] should have the width of its parent.
    ///
    /// Whether or not the [Bulma image element][bd], which will receive these
    /// properties, will have the same width as its parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::image::{Figure, Image};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Figure>
    ///             <Image fullwidth=true src={"media/images/img.png"} />
    ///         </Figure>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/image/#responsive-images-with-ratios
    #[prop_or_default]
    pub fullwidth: bool,
    /// Whether or not the [Bulma image element][bd] should be rounded.
    ///
    /// Whether or not the [Bulma image element][bd], which will receive these
    /// properties, will be rounded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::image::{Figure, Image};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Figure>
    ///             <Image rounded=true src={"media/images/img.png"} />
    ///         </Figure>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/image/#rounded-images
    #[prop_or_default]
    pub rounded: bool,
    /// Sets the source of the [Bulma image element][bd].
    ///
    /// Sets the source of the [Bulma image element][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::image::{Figure, Image};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Figure>
    ///             <Image src={"media/images/img.png"} />
    ///         </Figure>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/image/
    pub src: AttrValue,
}

/// Yew helper for the [Bulma image element][bd].
///
/// Yew helepr for the image element, based on the specification found in
/// the [Bulma image text element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::image::{Figure, Image};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Figure>
///             <Image src={"media/images/img.png"} />
///         </Figure>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/image/
#[function_component(Image)]
pub fn image(props: &ImageProperties) -> Html {
    let fullwidth = if props.fullwidth { "is-fullwidth" } else { "" };
    let rounded = if props.rounded { "is-rounded" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class(fullwidth)
        .with_custom_class(rounded)
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <img id={props.id.clone()} {class} src={props.src.clone()} />
    }
}

/// Defines the possible sizes of a [Bulma image element][bd].
///
/// Defines the possible sizes of a [Bulma image element][bd]. Those include
/// the [fixed image sized][f-sizes] as well as the
/// [responsive image sizes][r-sizes]
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::image::{Figure, Image, Size};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Figure size={Size::Pixels128x128}>
///             <Image src={"media/images/img.png"} />
///         </Figure>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/image/
/// [f-sizes]: https://bulma.io/documentation/elements/image/#fixed-square-images
/// [r-sizes]: https://bulma.io/documentation/elements/image/#responsive-images-with-ratios
#[derive(PartialEq)]
pub enum Size {
    Pixels16x16,
    Pixels24x24,
    Pixels32x32,
    Pixels48x48,
    Pixels64x64,
    Pixels96x96,
    Pixels128x128,
    RatioSquare,
    Ratio1x1,
    Ratio5x4,
    Ratio4x3,
    Ratio3x2,
    Ratio5x3,
    Ratio16x9,
    Ratio2x1,
    Ratio3x1,
    Ratio4x5,
    Ratio3x4,
    Ratio2x3,
    Ratio3x5,
    Ratio9x16,
    Ratio1x2,
    Ratio1x3,
}

impl From<&Size> for String {
    fn from(value: &Size) -> Self {
        match value {
            Size::Pixels16x16 => format!("{IS_PREFIX}-16x16"),
            Size::Pixels24x24 => format!("{IS_PREFIX}-24x24"),
            Size::Pixels32x32 => format!("{IS_PREFIX}-32x32"),
            Size::Pixels48x48 => format!("{IS_PREFIX}-48x48"),
            Size::Pixels64x64 => format!("{IS_PREFIX}-64x64"),
            Size::Pixels96x96 => format!("{IS_PREFIX}-96x96"),
            Size::Pixels128x128 => format!("{IS_PREFIX}-128x128"),
            Size::RatioSquare => format!("{IS_PREFIX}-square"),
            Size::Ratio1x1 => format!("{IS_PREFIX}-1by1"),
            Size::Ratio5x4 => format!("{IS_PREFIX}-5by4"),
            Size::Ratio4x3 => format!("{IS_PREFIX}-4by3"),
            Size::Ratio3x2 => format!("{IS_PREFIX}-3by2"),
            Size::Ratio5x3 => format!("{IS_PREFIX}-5by3"),
            Size::Ratio16x9 => format!("{IS_PREFIX}-16by9"),
            Size::Ratio2x1 => format!("{IS_PREFIX}-2by1"),
            Size::Ratio3x1 => format!("{IS_PREFIX}-3by1"),
            Size::Ratio4x5 => format!("{IS_PREFIX}-4by5"),
            Size::Ratio3x4 => format!("{IS_PREFIX}-3by4"),
            Size::Ratio2x3 => format!("{IS_PREFIX}-2by3"),
            Size::Ratio3x5 => format!("{IS_PREFIX}-3by5"),
            Size::Ratio9x16 => format!("{IS_PREFIX}-9by16"),
            Size::Ratio1x2 => format!("{IS_PREFIX}-1by2"),
            Size::Ratio1x3 => format!("{IS_PREFIX}-1by3"),
        }
    }
}

/// Defines the properties of the [Bulma figure element][bd].
///
/// Defines the properties of the figure element, based on the specification
/// found in the [Bulma figure element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::image::{Figure, Image};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Figure>
///             <Image src={"media/images/img.png"} />
///         </Figure>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/image/
#[derive(Properties, PartialEq)]
pub struct FigureProperties {
    /// Sets the [HTML id attribute][id] of the element.
    ///
    /// Sets the [HTML id attrbiute][id] of the element which will receive
    /// these properties.
    ///
    /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Sets the classes to be appended to the [HTML classattribute][class].
    ///
    /// Sets the classes to be appended to [HTML class attrbiute][class] of the
    /// element which will receive these properties.
    ///
    /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class
    #[prop_or_default]
    pub class: Option<Classes>,
    /// Sets the size of the [Bulma figure element][bd].
    ///
    /// Sets the size of the [Bulma figure element][bd] which will receive these
    /// properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::elements::image::{Figure, Image, Size};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Figure size={Size::Pixels128x128}>
    ///             <Image src={"media/images/img.png"} />
    ///         </Figure>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/image/
    #[prop_or_default]
    pub size: Option<Size>,
    /// The list of elements found inside the [image element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma image element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/elements/image/
    pub children: Children,
}

/// Yew helper for the [Bulma figure element][bd].
///
/// Yew helepr for the figure element, based on the specification found in the
/// [Bulma figure element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::elements::image::{Figure, Image};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Figure>
///             <Image src={"media/images/img.png"} />
///         </Figure>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/elements/image/
#[function_component(Figure)]
pub fn figure(props: &FigureProperties) -> Html {
    let size = props
        .size
        .as_ref()
        .map(String::from)
        .unwrap_or("".to_owned());
    let class = ClassBuilder::default()
        .with_custom_class("image")
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
        <figure id={props.id.clone()} {class}>
            { for props.children.iter() }
        </figure>
    }
}
