use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, Html, Properties,
};
use yew_and_bulma_macros::{base_component_properties, TypedChildren};

use crate::helpers::color::Color;
use crate::utils::{class::ClassBuilder, BaseComponent};

use super::tabs::Tab;

/// Defines the properties of the [Bulma panel component][bd].
///
/// Defines the properties of the panel component, based on the
/// specification found in the [Bulma panel component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     component::{
///         panel::{Panel, PanelBlock, PanelHeading, PanelTabs},
///         tabs::Tab,
///     },
///     elements::button::Button,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Panel>
///             <PanelHeading>{"Repositories"}</PanelHeading>
///
///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
///
///             <PanelBlock active=true>{"yew-and-bulma"}</PanelBlock>
///             <PanelBlock>{"bulma"}</PanelBlock>
///             <PanelBlock>{"yew"}</PanelBlock>
///             <PanelBlock>
///                 <Button fullwidth=true>{"Reset filters"}</Button>
///             </PanelBlock>
///         </Panel>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/panel/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct PanelProperties {
    /// Sets the color of the [Bulma panel component][bd].
    ///
    /// Sets the color of the [Bulma panel component][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     component::{
    ///         panel::{Panel, PanelBlock, PanelHeading, PanelTabs},
    ///         tabs::Tab,
    ///     },
    ///     elements::button::Button,
    ///     helpers::color::Color,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Panel color={Color::Danger}>
    ///             <PanelHeading>{"Repositories"}</PanelHeading>
    ///
    ///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
    ///
    ///             <PanelBlock active=true>{"yew-and-bulma"}</PanelBlock>
    ///             <PanelBlock>{"bulma"}</PanelBlock>
    ///             <PanelBlock>{"yew"}</PanelBlock>
    ///             <PanelBlock>
    ///                 <Button fullwidth=true>{"Reset filters"}</Button>
    ///             </PanelBlock>
    ///         </Panel>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/panel/#colors
    #[prop_or_default]
    pub color: Option<Color>,
    /// The list of elements found inside the [panel component][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma panel component][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/panel/
    pub children: ChildrenRenderer<PanelItem>,
}

/// Yew implementation of the [Bulma panel component][bd].
///
/// Yew implementation of the panel component, based on the specification
/// found in the [Bulma panel component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     component::{
///         panel::{Panel, PanelBlock, PanelHeading, PanelTabs},
///         tabs::Tab,
///     },
///     elements::button::Button,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Panel>
///             <PanelHeading>{"Repositories"}</PanelHeading>
///
///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
///
///             <PanelBlock active=true>{"yew-and-bulma"}</PanelBlock>
///             <PanelBlock>{"bulma"}</PanelBlock>
///             <PanelBlock>{"yew"}</PanelBlock>
///             <PanelBlock>
///                 <Button fullwidth=true>{"Reset filters"}</Button>
///             </PanelBlock>
///         </Panel>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/panel/
#[function_component(Panel)]
pub fn panel(props: &PanelProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("panel")
        .with_color(props.color)
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="nav" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the possible types of children from a [Bulma panel component][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma panel component][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     component::{
///         panel::{Panel, PanelBlock, PanelHeading, PanelTabs},
///         tabs::Tab,
///     },
///     elements::button::Button,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Panel>
///             <PanelHeading>{"Repositories"}</PanelHeading>
///
///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
///
///             <PanelBlock active=true>{"yew-and-bulma"}</PanelBlock>
///             <PanelBlock>{"bulma"}</PanelBlock>
///             <PanelBlock>{"yew"}</PanelBlock>
///             <PanelBlock>
///                 <Button fullwidth=true>{"Reset filters"}</Button>
///             </PanelBlock>
///         </Panel>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/panel/
#[derive(Clone, PartialEq, TypedChildren)]
pub enum PanelItem {
    PanelBlock(VChild<PanelBlock>),
    PanelHeading(VChild<PanelHeading>),
    PanelTabs(VChild<PanelTabs>),
}

/// Defines the properties of the [Bulma panel block element][bd].
///
/// Defines the properties of the panel block element, based on the
/// specification found in the [Bulma panel block element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     component::{
///         panel::{Panel, PanelBlock, PanelHeading, PanelTabs},
///         tabs::Tab,
///     },
///     elements::button::Button,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Panel>
///             <PanelHeading>{"Repositories"}</PanelHeading>
///
///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
///
///             <PanelBlock active=true>{"yew-and-bulma"}</PanelBlock>
///             <PanelBlock>{"bulma"}</PanelBlock>
///             <PanelBlock>{"yew"}</PanelBlock>
///             <PanelBlock>
///                 <Button fullwidth=true>{"Reset filters"}</Button>
///             </PanelBlock>
///         </Panel>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/panel/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct PanelBlockProperties {
    /// Whether or not the [panel block element][bd] should be marked as active.
    ///
    /// Whether or not the [Bulma panel block element][bd], which will receive
    /// these properties, will be marked as active.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     component::{
    ///         panel::{Panel, PanelBlock, PanelHeading, PanelTabs},
    ///         tabs::Tab,
    ///     },
    ///     elements::button::Button,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Panel>
    ///             <PanelHeading>{"Repositories"}</PanelHeading>
    ///
    ///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
    ///
    ///             <PanelBlock active=true>{"yew-and-bulma"}</PanelBlock>
    ///             <PanelBlock>{"bulma"}</PanelBlock>
    ///             <PanelBlock>{"yew"}</PanelBlock>
    ///             <PanelBlock>
    ///                 <Button fullwidth=true>{"Reset filters"}</Button>
    ///             </PanelBlock>
    ///         </Panel>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/panel/
    #[prop_or_default]
    pub active: bool,
    /// The list of elements found inside the [panel block element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma panel block element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/panel/
    pub children: Children,
}

/// Yew implementation of the [Bulma panel block element][bd].
///
/// Yew implementation of the panel block element, based on the specification
/// found in the [Bulma panel block element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     component::{
///         panel::{Panel, PanelBlock, PanelHeading, PanelTabs},
///         tabs::Tab,
///     },
///     elements::button::Button,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Panel>
///             <PanelHeading>{"Repositories"}</PanelHeading>
///
///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
///
///             <PanelBlock active=true>{"yew-and-bulma"}</PanelBlock>
///             <PanelBlock>{"bulma"}</PanelBlock>
///             <PanelBlock>{"yew"}</PanelBlock>
///             <PanelBlock>
///                 <Button fullwidth=true>{"Reset filters"}</Button>
///             </PanelBlock>
///         </Panel>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/panel/
#[function_component(PanelBlock)]
pub fn panel_block(props: &PanelBlockProperties) -> Html {
    let active = if props.active { "is-active" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("panel-block")
        .with_custom_class(active)
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma panel heading element][bd].
///
/// Defines the properties of the panel heading element, based on the
/// specification found in the [Bulma panel heading element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     component::{
///         panel::{Panel, PanelBlock, PanelHeading, PanelTabs},
///         tabs::Tab,
///     },
///     elements::button::Button,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Panel>
///             <PanelHeading>{"Repositories"}</PanelHeading>
///
///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
///
///             <PanelBlock active=true>{"yew-and-bulma"}</PanelBlock>
///             <PanelBlock>{"bulma"}</PanelBlock>
///             <PanelBlock>{"yew"}</PanelBlock>
///             <PanelBlock>
///                 <Button fullwidth=true>{"Reset filters"}</Button>
///             </PanelBlock>
///         </Panel>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/panel/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct PanelHeadingProperties {
    /// The list of elements found inside the [panel heading element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma panel heading element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/panel/
    pub children: Children,
}

/// Yew implementation of the [Bulma panel heading element][bd].
///
/// Yew implementation of the panel heading element, based on the specification
/// found in the [Bulma panel heading element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     component::{
///         panel::{Panel, PanelBlock, PanelHeading, PanelTabs},
///         tabs::Tab,
///     },
///     elements::button::Button,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Panel>
///             <PanelHeading>{"Repositories"}</PanelHeading>
///
///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
///
///             <PanelBlock active=true>{"yew-and-bulma"}</PanelBlock>
///             <PanelBlock>{"bulma"}</PanelBlock>
///             <PanelBlock>{"yew"}</PanelBlock>
///             <PanelBlock>
///                 <Button fullwidth=true>{"Reset filters"}</Button>
///             </PanelBlock>
///         </Panel>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/panel/
#[function_component(PanelHeading)]
pub fn panel_heading(props: &PanelHeadingProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("panel-heading")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="p" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma panel tabs element][bd].
///
/// Defines the properties of the panel tabs element, based on the
/// specification found in the [Bulma panel tabs element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     component::{
///         panel::{Panel, PanelBlock, PanelHeading, PanelTabs},
///         tabs::Tab,
///     },
///     elements::button::Button,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Panel>
///             <PanelHeading>{"Repositories"}</PanelHeading>
///
///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
///
///             <PanelBlock active=true>{"yew-and-bulma"}</PanelBlock>
///             <PanelBlock>{"bulma"}</PanelBlock>
///             <PanelBlock>{"yew"}</PanelBlock>
///             <PanelBlock>
///                 <Button fullwidth=true>{"Reset filters"}</Button>
///             </PanelBlock>
///         </Panel>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/panel/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct PanelTabsProperties {
    /// The list of elements found inside the [panel tabs element][bd].
    ///
    /// Defines the elements and their active state that will be found inside the
    /// [Bulma panel tabs element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/panel/
    pub tabs: Vec<Tab>,
}

/// Yew implementation of the [Bulma panel tabs element][bd].
///
/// Yew implementation of the panel tabs element, based on the specification
/// found in the [Bulma panel tabs element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     component::{
///         panel::{Panel, PanelBlock, PanelHeading, PanelTabs},
///         tabs::Tab,
///     },
///     elements::button::Button,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Panel>
///             <PanelHeading>{"Repositories"}</PanelHeading>
///
///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
///
///             <PanelBlock active=true>{"yew-and-bulma"}</PanelBlock>
///             <PanelBlock>{"bulma"}</PanelBlock>
///             <PanelBlock>{"yew"}</PanelBlock>
///             <PanelBlock>
///                 <Button fullwidth=true>{"Reset filters"}</Button>
///             </PanelBlock>
///         </Panel>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/panel/
#[function_component(PanelTabs)]
pub fn panel_tabs(props: &PanelTabsProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("panel-tabs")
        .with_custom_class(&props.class.to_string())
        .build();

    let no_children = props.tabs.len();
    let mut tabs = Vec::with_capacity(no_children);
    for t in props.tabs.iter() {
        let (elem, is_active) = (t.0.clone(), t.1);
        let class = is_active.then_some("is-active");

        tabs.push(html! { <a {class}>{elem}</a> });
    }

    html! {
        <BaseComponent tag="p" {class} ..props.into()>
            { for tabs.into_iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma panel icon element][bd].
///
/// Defines the properties of the panel icon element, based on the
/// specification found in the [Bulma panel icon element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     component::{
///         panel::{Panel, PanelBlock, PanelHeading, PanelTabs},
///         tabs::Tab,
///     },
///     elements::button::Button,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Panel>
///             <PanelHeading>{"Repositories"}</PanelHeading>
///
///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
///
///             <PanelBlock active=true>{"yew-and-bulma"}</PanelBlock>
///             <PanelBlock>{"bulma"}</PanelBlock>
///             <PanelBlock>{"yew"}</PanelBlock>
///             <PanelBlock>
///                 <Button fullwidth=true>{"Reset filters"}</Button>
///             </PanelBlock>
///         </Panel>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/panel/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct PanelIconProperties {
    /// The list of elements found inside the [panel icon element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma panel icon element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/panel/
    pub children: Children,
}

/// Yew implementation of the [Bulma panel icon element][bd].
///
/// Yew implementation of the panel icon element, based on the specification
/// found in the [Bulma panel icon element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     component::{
///         panel::{Panel, PanelBlock, PanelHeading, PanelIcon, PanelTabs},
///         tabs::Tab,
///     },
///     elements::button::Button,
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Panel>
///             <PanelHeading>{"Repositories"}</PanelHeading>
///
///             <PanelTabs tabs={vec![Tab("All".into(), true), Tab("Public".into(), false), Tab("Private".into(), false)]}>
///
///             <PanelBlock active=true>
///                 <PanelIcon>
///                     <i class="fas fa-book" aria-hidden="true"></i>
///                 </PanelIcon>
///                 {"yew-and-bulma"}
///             </PanelBlock>
///             <PanelBlock>
///                 <PanelIcon>
///                     <i class="fas fa-book" aria-hidden="true"></i>
///                 </PanelIcon>
///                 {"bulma"}
///             </PanelBlock>
///             <PanelBlock>
///                 <PanelIcon>
///                     <i class="fas fa-book" aria-hidden="true"></i>
///                 </PanelIcon>
///                 {"yew"}
///             </PanelBlock>
///             <PanelBlock>
///                 <Button fullwidth=true>{"Reset filters"}</Button>
///             </PanelBlock>
///         </Panel>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/panel/
#[function_component(PanelIcon)]
pub fn panel_icon(props: &PanelIconProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("panel-icon")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="span" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}
