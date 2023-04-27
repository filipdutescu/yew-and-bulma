use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, Html, Properties,
};
use yew::{html, AttrValue, ChildrenWithProps};
use yew_and_bulma_macros::{base_component_properties, TypedChildren};

use crate::elements::button::Button;
use crate::utils::{class::ClassBuilder, constants::IS_PREFIX, BaseComponent};

/// Defines the possible alignment of a [Bulma dropdown component][bd].
///
/// Defines the possible alignment of a [Bulma dropdown content element][bd],
/// inside its parent.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Align, Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu,
///     DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown align={Align::Right}>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/#right-aligned
#[derive(PartialEq, Clone, Copy)]
pub enum Align {
    // TODO: use #[default] when updating the MSRV
    Left,
    Right,
}

impl From<Align> for String {
    fn from(value: Align) -> Self {
        match value {
            Align::Left => "".to_owned(),
            Align::Right => format!("{IS_PREFIX}-right"),
        }
    }
}

/// Defines the properties of the [Bulma dropdown component][bd].
///
/// Defines the properties of the dropdown component, based on the
/// specification found in the [Bulma dropdown component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct DropdownProperties {
    /// Whether or not the [Bulma dropdown element][bd] should be active.
    ///
    /// Whether or not the [Bulma dropdown element][bd], which will receive these
    /// properties, will be active.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::components::dropdown::{
    ///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Dropdown active=true>
    ///             <DropdownTrigger>
    ///                 <Button>
    ///                     <span>{"Dropdown toggle"}</span>
    ///                 </Button>
    ///             </DropdownTrigger>
    ///
    ///             <DropdownMenu>
    ///                 <DropdownContent>
    ///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
    ///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
    ///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
    ///
    ///                     <DropdownDivider />
    ///
    ///                     <DropdownItem>{"A separate item"}</DropdownItem>
    ///                 </DropdownContent>
    ///             </DropdownMenu>
    ///         </Dropdown>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/dropdown/#hoverable-or-toggable
    #[prop_or_default]
    pub active: bool,
    /// Whether or not the [Bulma dropdown element][bd] should be hoverable.
    ///
    /// Whether or not the [Bulma dropdown element][bd], which will receive these
    /// properties, will be hoverable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::components::dropdown::{
    ///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Dropdown hoverable=true>
    ///             <DropdownTrigger>
    ///                 <Button>
    ///                     <span>{"Dropdown toggle"}</span>
    ///                 </Button>
    ///             </DropdownTrigger>
    ///
    ///             <DropdownMenu>
    ///                 <DropdownContent>
    ///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
    ///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
    ///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
    ///
    ///                     <DropdownDivider />
    ///
    ///                     <DropdownItem>{"A separate item"}</DropdownItem>
    ///                 </DropdownContent>
    ///             </DropdownMenu>
    ///         </Dropdown>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/dropdown/#hoverable-or-toggable
    #[prop_or_default]
    pub hoverable: bool,
    /// Sets the alignment of a [Bulma dropdown component][bd].
    ///
    /// Sets the alignment of a [Bulma dropdown content element][bd], which will
    /// receive these properties, inside its parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::components::dropdown::{
    ///     Align, Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu,
    ///     DropdownTrigger,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Dropdown align={Align::Right}>
    ///             <DropdownTrigger>
    ///                 <Button>
    ///                     <span>{"Dropdown toggle"}</span>
    ///                 </Button>
    ///             </DropdownTrigger>
    ///
    ///             <DropdownMenu>
    ///                 <DropdownContent>
    ///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
    ///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
    ///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
    ///
    ///                     <DropdownDivider />
    ///
    ///                     <DropdownItem>{"A separate item"}</DropdownItem>
    ///                 </DropdownContent>
    ///             </DropdownMenu>
    ///         </Dropdown>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/dropdown/#right-aligned
    #[prop_or(Align::Left)]
    pub align: Align,
    /// Whether or not the [dropdown element][bd] should be appear above the trigger.
    ///
    /// Whether or not the [Bulma dropdown element][bd], which will receive these
    /// properties, will be above the trigger.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::components::dropdown::{
    ///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Dropdown up=true>
    ///             <DropdownTrigger>
    ///                 <Button>
    ///                     <span>{"Dropdown toggle"}</span>
    ///                 </Button>
    ///             </DropdownTrigger>
    ///
    ///             <DropdownMenu>
    ///                 <DropdownContent>
    ///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
    ///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
    ///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
    ///
    ///                     <DropdownDivider />
    ///
    ///                     <DropdownItem>{"A separate item"}</DropdownItem>
    ///                 </DropdownContent>
    ///             </DropdownMenu>
    ///         </Dropdown>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/elements/dropdown/#dropdup
    #[prop_or_default]
    pub up: bool,
    /// The list of elements found inside the [dropdown component][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma dropdown component][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/dropdown/
    pub children: ChildrenRenderer<DropdownElement>,
}

/// Yew implementation of the [Bulma dropdown component][bd].
///
/// Yew implementation of the dropdown component, based on the specification
/// found in the [Bulma dropdown component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProperties) -> Html {
    let active = if props.active { "is-active" } else { "" };
    let hoverable = if props.hoverable { "is-hoverable" } else { "" };
    let up = if props.up { "is-up" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("dropdown")
        .with_custom_class(&props.class.to_string())
        .with_custom_class(active)
        .with_custom_class(hoverable)
        .with_custom_class(&String::from(props.align))
        .with_custom_class(up)
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the possible types of children from a [Bulma dropdown component][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma dropdown component][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[derive(Clone, PartialEq, TypedChildren)]
pub enum DropdownElement {
    DropdownMenu(VChild<DropdownMenu>),
    DropdownTrigger(VChild<DropdownTrigger>),
}

/// Defines the properties of the [Bulma dropdown menu element][bd].
///
/// Defines the properties of the dropdown menu element, based on the
/// specification found in the [Bulma dropdown component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct DropdownMenuProperties {
    /// The list of elements found inside the [dropdown menu element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma dropdown menu element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/dropdown/
    pub children: ChildrenWithProps<DropdownContent>,
}

/// Yew implementation of the [Bulma dropdown menu component][bd].
///
/// Yew implementation of the dropdown menu component, based on the specification
/// found in the [Bulma dropdown menu component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[function_component(DropdownMenu)]
pub fn dropdown_menu(props: &DropdownMenuProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("dropdown-menu")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma dropdown trigger element][bd].
///
/// Defines the properties of the dropdown trigger element, based on the
/// specification found in the [Bulma dropdown component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct DropdownTriggerProperties {
    /// The list of elements found inside the [dropdown trigger element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma dropdown trigger element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/dropdown/
    pub children: ChildrenWithProps<Button>,
}

/// Yew implementation of the [Bulma dropdown trigger component][bd].
///
/// Yew implementation of the dropdown trigger component, based on the specification
/// found in the [Bulma dropdown trigger component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[function_component(DropdownTrigger)]
pub fn dropdown_trigger(props: &DropdownTriggerProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("dropdown-trigger")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma dropdown content element][bd].
///
/// Defines the properties of the dropdown content element, based on the
/// specification found in the [Bulma dropdown component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct DropdownContentProperties {
    /// The list of elements found inside the [dropdown content element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma dropdown content element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/dropdown/
    pub children: Children,
}

/// Yew implementation of the [Bulma dropdown content component][bd].
///
/// Yew implementation of the dropdown content component, based on the specification
/// found in the [Bulma dropdown content component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[function_component(DropdownContent)]
pub fn dropdown_content(props: &DropdownContentProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("dropdown-content")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the possible types of children from a [Bulma dropdown content element][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma dropdown content element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[derive(Clone, PartialEq, TypedChildren)]
pub enum DropdownContentItem {
    DropdownItem(VChild<DropdownItem>),
    DropdownDivider(VChild<DropdownDivider>),
}

/// Defines the possible types of a [Bulma dropdown item element][bd].
///
/// Defines the possible types that a [Bulma dropdown item element][bd] can
/// take and their respective properties.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/tabs/#alignment
#[derive(PartialEq, Clone)]
pub enum DropdownItemTag {
    // TODO: use #[default] when updating the MSRV
    Anchor { href: Option<AttrValue> },
    Div,
}

impl From<DropdownItemTag> for AttrValue {
    fn from(value: DropdownItemTag) -> Self {
        match value {
            DropdownItemTag::Anchor { href: _ } => AttrValue::from("a"),
            DropdownItemTag::Div => AttrValue::from("div"),
        }
    }
}

/// Defines the properties of the [Bulma dropdown item element][bd].
///
/// Defines the properties of the dropdown item element, based on the
/// specification found in the [Bulma dropdown component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct DropdownItemProperties {
    /// The [HTML tag][tag] name to be used for the [dropdown component][bd].
    ///
    /// Specifies what [HTML tag][tag] name should be used for the base
    /// component.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::components::dropdown::{
    ///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Dropdown>
    ///             <DropdownTrigger>
    ///                 <Button>
    ///                     <span>{"Dropdown toggle"}</span>
    ///                 </Button>
    ///             </DropdownTrigger>
    ///
    ///             <DropdownMenu>
    ///                 <DropdownContent>
    ///                     <DropdownItem tag="div"><p>{"One dropdown item"}</p></DropdownItem>
    ///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
    ///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
    ///
    ///                     <DropdownDivider />
    ///
    ///                     <DropdownItem>{"A separate item"}</DropdownItem>
    ///                 </DropdownContent>
    ///             </DropdownMenu>
    ///         </Dropdown>
    ///     }
    /// }
    /// ```
    ///
    /// [tag]: https://developer.mozilla.org/en-US/docs/Glossary/Tag
    /// [bd]: https://bulma.io/documentation/components/dropdown/
    #[prop_or(DropdownItemTag::Anchor { href: None })]
    pub tag: DropdownItemTag,
    /// Whether or not the [Bulma dropdown item element][bd] should be active.
    ///
    /// Whether or not the [Bulma dropdown item element][bd], which will receive these
    /// properties, will be active.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::components::dropdown::{
    ///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Dropdown>
    ///             <DropdownTrigger>
    ///                 <Button>
    ///                     <span>{"Dropdown toggle"}</span>
    ///                 </Button>
    ///             </DropdownTrigger>
    ///
    ///             <DropdownMenu>
    ///                 <DropdownContent>
    ///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
    ///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
    ///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
    ///
    ///                     <DropdownDivider />
    ///
    ///                     <DropdownItem>{"A separate item"}</DropdownItem>
    ///                 </DropdownContent>
    ///             </DropdownMenu>
    ///         </Dropdown>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/dropdown/
    #[prop_or_default]
    pub active: bool,
    /// The list of elements found inside the [dropdown item element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma dropdown item element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/dropdown/
    pub children: Children,
}

/// Yew implementation of the [Bulma dropdown item component][bd].
///
/// Yew implementation of the dropdown item component, based on the specification
/// found in the [Bulma dropdown item component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[function_component(DropdownItem)]
pub fn dropdown_item(props: &DropdownItemProperties) -> Html {
    let active = if props.active { "is-active" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("dropdown-item")
        .with_custom_class(active)
        .with_custom_class(&props.class.to_string())
        .build();
    let mut attrs = props.attrs.clone();
    let tag: AttrValue = props.tag.clone().into();
    if let DropdownItemTag::Anchor { href: Some(href) } = &props.tag {
        attrs.insert("href", href.clone());
    }

    html! {
        <BaseComponent {tag} {attrs} {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma dropdown divider element][bd].
///
/// Defines the properties of the dropdown divider element, based on the
/// specification found in the [Bulma dropdown component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct DropdownDividerProperties {}

/// Yew implementation of the [Bulma dropdown divider component][bd].
///
/// Yew implementation of the dropdown divider component, based on the specification
/// found in the [Bulma dropdown divider component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::components::dropdown::{
///     Dropdown, DropdownContent, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Dropdown>
///             <DropdownTrigger>
///                 <Button>
///                     <span>{"Dropdown toggle"}</span>
///                 </Button>
///             </DropdownTrigger>
///
///             <DropdownMenu>
///                 <DropdownContent>
///                     <DropdownItem>{"One dropdown item"}</DropdownItem>
///                     <DropdownItem active=true>{"A different dropdown item"}</DropdownItem>
///                     <DropdownItem>{"Another dropdown item"}</DropdownItem>
///
///                     <DropdownDivider />
///
///                     <DropdownItem>{"A separate item"}</DropdownItem>
///                 </DropdownContent>
///             </DropdownMenu>
///         </Dropdown>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/dropdown/
#[function_component(DropdownDivider)]
pub fn dropdown_divider(props: &DropdownDividerProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("dropdown-divider")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="hr" {class} ..props.into() />
    }
}
