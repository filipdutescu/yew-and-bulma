use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, Html, Properties,
};
use yew_and_bulma_macros::{base_component_properties, TypedChildren};

use crate::utils::{class::ClassBuilder, BaseComponent};

/// Defines the properties of the [Bulma menu component][bd].
///
/// Defines the properties of the menu component, based on the
/// specification found in the [Bulma menu component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::component::menu::{Menu, MenuLabel, MenuList};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Menu>
///             <MenuLabel>{"General"}</MenuLabel>
///             <MenuList>
///                 <a class="is-active">{"Dashboard"}</a>
///                 <a>{"About"}</a>
///             </MenuList>
///
///             <MenuLabel>{"Project"}</MenuLabel>
///             <MenuList>
///                 <a>{"Team"}</a>
///                 <a>{"Technologies"}</a>
///                 <a>{"Blog"}</a>
///             </MenuList>
///         </Menu>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/menu/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MenuProperties {
    /// The list of elements found inside the [menu component][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma menu component][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/menu/
    pub children: ChildrenRenderer<MenuItem>,
}

/// Yew implementation of the [Bulma menu component][bd].
///
/// Yew implementation of the menu component, based on the specification
/// found in the [Bulma menu component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::component::menu::{Menu, MenuLabel, MenuList};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Menu>
///             <MenuLabel>{"General"}</MenuLabel>
///             <MenuList>
///                 <a class="is-active">{"Dashboard"}</a>
///                 <a>{"About"}</a>
///             </MenuList>
///
///             <MenuLabel>{"Project"}</MenuLabel>
///             <MenuList>
///                 <a>{"Team"}</a>
///                 <a>{"Technologies"}</a>
///                 <a>{"Blog"}</a>
///             </MenuList>
///         </Menu>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/menu/
#[function_component(Menu)]
pub fn menu(props: &MenuProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("menu")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="aside" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the possible types of children from a [Bulma menu component][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma menu component][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::component::menu::{Menu, MenuLabel, MenuList};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Menu>
///             <MenuLabel>{"General"}</MenuLabel>
///             <MenuList>
///                 <a class="is-active">{"Dashboard"}</a>
///                 <a>{"About"}</a>
///             </MenuList>
///
///             <MenuLabel>{"Project"}</MenuLabel>
///             <MenuList>
///                 <a>{"Team"}</a>
///                 <a>{"Technologies"}</a>
///                 <a>{"Blog"}</a>
///             </MenuList>
///         </Menu>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/menu/
#[derive(Clone, PartialEq, TypedChildren)]
pub enum MenuItem {
    MenuLabel(VChild<MenuLabel>),
    MenuList(VChild<MenuList>),
}

/// Defines the properties of the [Bulma menu label component][bd].
///
/// Defines the properties of the menu label component, based on the
/// specification found in the [Bulma menu label component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::component::menu::{Menu, MenuLabel, MenuList};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Menu>
///             <MenuLabel>{"General"}</MenuLabel>
///             <MenuList>
///                 <a class="is-active">{"Dashboard"}</a>
///                 <a>{"About"}</a>
///             </MenuList>
///
///             <MenuLabel>{"Project"}</MenuLabel>
///             <MenuList>
///                 <a>{"Team"}</a>
///                 <a>{"Technologies"}</a>
///                 <a>{"Blog"}</a>
///             </MenuList>
///         </Menu>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/menu/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MenuLabelProperties {
    /// The list of elements found inside the [menu label component][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma menu label component][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/menu/
    pub children: Children,
}

/// Yew implementation of the [Bulma menu label component][bd].
///
/// Yew implementation of the menu label component, based on the specification
/// found in the [Bulma menu label component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::component::menu::{Menu, MenuLabel, MenuList};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Menu>
///             <MenuLabel>{"General"}</MenuLabel>
///             <MenuList>
///                 <a class="is-active">{"Dashboard"}</a>
///                 <a>{"About"}</a>
///             </MenuList>
///
///             <MenuLabel>{"Project"}</MenuLabel>
///             <MenuList>
///                 <a>{"Team"}</a>
///                 <a>{"Technologies"}</a>
///                 <a>{"Blog"}</a>
///             </MenuList>
///         </Menu>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/menu/
#[function_component(MenuLabel)]
pub fn menu_label(props: &MenuLabelProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("menu-label")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="p" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma menu list component][bd].
///
/// Defines the properties of the menu list component, based on the
/// specification found in the [Bulma menu list component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::component::menu::{Menu, MenuLabel, MenuList};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Menu>
///             <MenuLabel>{"General"}</MenuLabel>
///             <MenuList>
///                 <a class="is-active">{"Dashboard"}</a>
///                 <a>{"About"}</a>
///             </MenuList>
///
///             <MenuLabel>{"Project"}</MenuLabel>
///             <MenuList>
///                 <a>{"Team"}</a>
///                 <a>{"Technologies"}</a>
///                 <a>{"Blog"}</a>
///             </MenuList>
///         </Menu>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/menu/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MenuListProperties {
    /// The list of elements found inside the [menu list component][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma menu list component][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/menu/
    pub children: Children,
}

/// Yew implementation of the [Bulma menu label component][bd].
///
/// Yew implementation of the menu label component, based on the specification
/// found in the [Bulma menu label component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::component::menu::{Menu, MenuLabel, MenuList};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Menu>
///             <MenuLabel>{"General"}</MenuLabel>
///             <MenuList>
///                 <a class="is-active">{"Dashboard"}</a>
///                 <a>{"About"}</a>
///             </MenuList>
///
///             <MenuLabel>{"Project"}</MenuLabel>
///             <MenuList>
///                 <a>{"Team"}</a>
///                 <a>{"Technologies"}</a>
///                 <a>{"Blog"}</a>
///             </MenuList>
///         </Menu>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/menu/
#[function_component(MenuList)]
pub fn menu_list(props: &MenuListProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("menu-list")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="ul" {class} ..props.into()>
            { for props.children.iter().map(|c| html! { <li>{c}</li> }) }
        </BaseComponent>
    }
}
