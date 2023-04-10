use std::fmt::Display;

use yew::{function_component, html, Children, Html, Properties};
use yew_and_bulma_macros::base_component_properties;

use crate::utils::{class::ClassBuilder, constants::IS_PREFIX};

/// Defines the relation that a [tile element][bd] has with its siblings.
///
/// Defines the relation that a [Bulma tile element][bd] has with its siblings.
/// This allows for complex patterns that can be made, by combining the various
/// values among themselves.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::tile::{Tile, Relation};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Tile relation={Relation::Ancestor}>
///             <Tile relation={Relation::Child}>
///                 {"This is some text in a tile."}
///             </Tile>
///
///             <Tile relation={Relation::Child}>
///                 {"This is some text in a tile."}
///             </Tile>
///
///             <Tile relation={Relation::Child}>
///                 {"This is some text in a tile."}
///             </Tile>
///         </Tile>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(PartialEq)]
pub enum Relation {
    Ancestor,
    Parent,
    Child,
}

impl Display for Relation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let relation = match self {
            Relation::Ancestor => "ancestor",
            Relation::Parent => "parent",
            Relation::Child => "child",
        };

        write!(f, "{relation}")
    }
}

/// Defines the possible sizes of a [Bulma tile element][bd].
///
/// Defines the possible sizes of a [Bulma tile element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::tile::{Tile, Size};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Tile size={Size::Five}>{"This is some text inside a tile."}</Tile>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(PartialEq)]
pub enum Size {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Size::One => "1",
            Size::Two => "2",
            Size::Three => "3",
            Size::Four => "4",
            Size::Five => "5",
            Size::Six => "6",
            Size::Seven => "7",
            Size::Eight => "8",
            Size::Nine => "9",
            Size::Ten => "10",
            Size::Eleven => "11",
            Size::Twelve => "12",
        };

        write!(f, "{size}")
    }
}

/// Defines the properties of the [Bulma tile element][bd].
///
/// Defines the properties of the tile element, based on the specification
/// found in the [Bulma tile element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::tile::Tile;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Tile>{"This is some text in a tile."}</Tile>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/tiles/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct TileProperties {
    /// Sets how the [tile element][bd] should relate to its siblings.
    ///
    /// Sets how the [Bulma tile element][bd], which will receive these
    /// properties, should relate to its siblings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::tile::{Tile, Relation};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Tile relation={Relation::Ancestor}>
    ///             <Tile relation={Relation::Child}>
    ///                 {"This is some text in a tile."}
    ///             </Tile>
    ///
    ///             <Tile relation={Relation::Child}>
    ///                 {"This is some text in a tile."}
    ///             </Tile>
    ///
    ///             <Tile relation={Relation::Child}>
    ///                 {"This is some text in a tile."}
    ///             </Tile>
    ///         </Tile>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub relation: Option<Relation>,
    /// Whether to stack the child [tile elements][bd] vertically.
    ///
    /// Whether or not to vertically stack the tiles found inside the
    /// [Bulma tile element][bd] which will receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::tile::Tile;
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Tile vertical={true}>
    ///             <Tile>{"This is some text in a tile."}</Tile>
    ///             <Tile>{"This is some text in a tile."}</Tile>
    ///             <Tile>{"This is some text in a tile."}</Tile>
    ///         </Tile>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub vertical: bool,
    /// Sets the size of the [tile element][bd].
    ///
    /// Sets how the [Bulma container element][bd], which will receive these
    /// properties, should relate to its siblings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::tile::{Tile, Relation, Size};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Tile relation={Relation::Ancestor}>
    ///             <Tile size={Size::Four}>
    ///                 {"This is some text in a tile."}
    ///             </Tile>
    ///
    ///             <Tile size={Size::Four}>
    ///                 {"This is some text in a tile."}
    ///             </Tile>
    ///
    ///             <Tile size={Size::Four}>
    ///                 {"This is some text in a tile."}
    ///             </Tile>
    ///         </Tile>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub size: Option<Size>,
    /// The list of elements found inside the [tile element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma tile element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/layout/tiles/
    pub children: Children,
}

/// Yew implementation of the [Bulma tile element][bd].
///
/// Yew implementation of the tile element, based on the specification
/// found in the [Bulma tile element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::tile::Tile;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Tile>{"This is some text in a tile."}</Tile>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/layout/tiles/
#[function_component(Tile)]
pub fn tile(props: &TileProperties) -> Html {
    let relation = props
        .relation
        .as_ref()
        .map(|relation| format!("{IS_PREFIX}-{relation}"))
        .unwrap_or("".to_string());
    let vertical = if props.vertical { "is-vertical" } else { "" };
    let size = props
        .size
        .as_ref()
        .map(|size| format!("{IS_PREFIX}-{size}"))
        .unwrap_or("".to_string());
    let class = ClassBuilder::default()
        .with_custom_class("tile")
        .with_custom_class(&relation)
        .with_custom_class(vertical)
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
        <div id={&props.id} {class}
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
