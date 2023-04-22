use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, Html, Properties,
};
use yew_and_bulma_macros::{base_component_properties, TypedChildren};

use crate::utils::BaseComponent;
use crate::{
    elements::delete::Delete,
    utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size},
};

/// Defines the properties of the [Bulma modal component][bd].
///
/// Defines the properties of the modal component, based on the
/// specification found in the [Bulma modal component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::modal::{Modal, ModalBackground, ModalClose, ModalContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalContent>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </ModalContent>
///
///             <ModalClose />
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ModalProperties {
    /// Whether or not the [Bulma modal element][bd] should be active.
    ///
    /// Whether or not the [Bulma modal element][bd], which will receive these
    /// properties, will be active.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::modal::{Modal, ModalBackground, ModalClose, ModalContent};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Modal active=true>
    ///             <ModalBackground />
    ///
    ///             <ModalContent>
    ///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
    ///             </ModalContent>
    ///
    ///             <ModalClose />
    ///         </Modal>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/modal/
    #[prop_or_default]
    pub active: bool,
    /// The list of elements found inside the [modal component][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma modal component][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/modal/
    pub children: ChildrenRenderer<ModalItem>,
}

/// Yew implementation of the [Bulma modal component][bd].
///
/// Yew implementation of the modal component, based on the specification
/// found in the [Bulma modal component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::modal::{Modal, ModalBackground, ModalClose, ModalContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalContent>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </ModalContent>
///
///             <ModalClose />
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/
#[function_component(Modal)]
pub fn modal(props: &ModalProperties) -> Html {
    let active = if props.active { "is-active" } else { "" };
    let class = ClassBuilder::default()
        .with_custom_class("modal")
        .with_custom_class(active)
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the possible types of children from a [Bulma modal component][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma modal component][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::modal::{Modal, ModalBackground, ModalClose, ModalContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalContent>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </ModalContent>
///
///             <ModalClose />
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/
#[derive(Clone, PartialEq, TypedChildren)]
pub enum ModalItem {
    ModalBackground(VChild<ModalBackground>),
    ModalCard(VChild<ModalCard>),
    ModalClose(VChild<ModalClose>),
    ModalContent(VChild<ModalContent>),
}

/// Defines the properties of the [Bulma modal background element][bd].
///
/// Defines the properties of the modal background element, based on the
/// specification found in the [Bulma modal component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::modal::{Modal, ModalBackground, ModalClose, ModalContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalContent>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </ModalContent>
///
///             <ModalClose />
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ModalBackgroundProperties {}

/// Yew implementation of the [Bulma modal background element][bd].
///
/// Yew implementation of the modal background element, based on the
/// specification found in the [Bulma modal component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::modal::{Modal, ModalBackground, ModalClose, ModalContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalContent>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </ModalContent>
///
///             <ModalClose />
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/
#[function_component(ModalBackground)]
pub fn modal_background(props: &ModalBackgroundProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("modal-background")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into() />
    }
}

/// Defines the properties of the [Bulma modal close element][bd].
///
/// Defines the properties of the modal close element, based on the
/// specification found in the [Bulma modal component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::modal::{Modal, ModalBackground, ModalClose, ModalContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalContent>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </ModalContent>
///
///             <ModalClose />
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ModalCloseProperties {
    /// Sets the size of the [Bulma modal close component][bd].
    ///
    /// Sets the size of the [Bulma modal close component][bd] which will
    /// receive these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     layout::modal::{Modal, ModalBackground, ModalClose, ModalContent},
    ///     utils::size::Size,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Modal>
    ///             <ModalBackground />
    ///
    ///             <ModalContent>
    ///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
    ///             </ModalContent>
    ///
    ///             <ModalClose size={Size::Large} />
    ///         </Modal>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/modal/
    #[prop_or_default]
    pub size: Option<Size>,
}

/// Yew implementation of the [Bulma modal close element][bd].
///
/// Yew implementation of the modal close element, based on the
/// specification found in the [Bulma modal component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::modal::{Modal, ModalBackground, ModalClose, ModalContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalContent>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </ModalContent>
///
///             <ModalClose />
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/
#[function_component(ModalClose)]
pub fn modal_close(props: &ModalCloseProperties) -> Html {
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
        .with_custom_class("modal-close")
        .with_custom_class(&size)
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="button" {class} ..props.into() />
    }
}

/// Defines the properties of the [Bulma modal content element][bd].
///
/// Defines the properties of the modal content element, based on the
/// specification found in the [Bulma modal component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::modal::{Modal, ModalBackground, ModalClose, ModalContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalContent>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </ModalContent>
///
///             <ModalClose />
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ModalContentProperties {
    /// The list of elements found inside the [modal content element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma modal content element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/modal/
    pub children: Children,
}

/// Yew implementation of the [Bulma modal content element][bd].
///
/// Yew implementation of the modal content element, based on the
/// specification found in the [Bulma modal component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::modal::{Modal, ModalBackground, ModalClose, ModalContent};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalContent>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </ModalContent>
///
///             <ModalClose />
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/
#[function_component(ModalContent)]
pub fn modal_content(props: &ModalContentProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("modal-content")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma modal card element][bd].
///
/// Defines the properties of the modal card element, based on the
/// specification found in the [Bulma modal card element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     elements::{button::Button, delete::Delete},
///     layout::modal::{
///         Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
///         ModalCardTitle,
///     },
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalCard>
///                 <ModalCardHead>
///                     <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
///                     <Delete />
///                 </ModalCardHead>
///
///                 <ModalCardBody>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}</ModalCardBody>
///
///                 <ModalCardFoot>
///                     <Button>{"Confirm"}</Button>
///                     <Button>{"Cancel"}</Button>
///                 </ModalCardFoot>
///             </ModalCard>
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/#modal-card
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ModalCardProperties {
    /// The list of elements found inside the [modal card element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma modal card element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/modal/#modal-card
    pub children: ChildrenRenderer<ModalCardItem>,
}

/// Yew implementation of the [Bulma modal card element][bd].
///
/// Yew implementation of the modal card element, based on the specification
/// found in the [Bulma modal card element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     elements::{button::Button, delete::Delete},
///     layout::modal::{
///         Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
///         ModalCardTitle,
///     },
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalCard>
///                 <ModalCardHead>
///                     <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
///                     <Delete />
///                 </ModalCardHead>
///
///                 <ModalCardBody>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}</ModalCardBody>
///
///                 <ModalCardFoot>
///                     <Button>{"Confirm"}</Button>
///                     <Button>{"Cancel"}</Button>
///                 </ModalCardFoot>
///             </ModalCard>
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/#modal-card
#[function_component(ModalCard)]
pub fn modal_card(props: &ModalCardProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("modal-card")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the possible types of children from a [Bulma modal card element][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma modal card element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     elements::{button::Button, delete::Delete},
///     layout::modal::{
///         Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
///         ModalCardTitle,
///     },
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalCard>
///                 <ModalCardHead>
///                     <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
///                     <Delete />
///                 </ModalCardHead>
///
///                 <ModalCardBody>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}</ModalCardBody>
///
///                 <ModalCardFoot>
///                     <Button>{"Confirm"}</Button>
///                     <Button>{"Cancel"}</Button>
///                 </ModalCardFoot>
///             </ModalCard>
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/#modal-card
#[derive(Clone, PartialEq, TypedChildren)]
pub enum ModalCardItem {
    ModalCardBody(VChild<ModalCardBody>),
    ModalCardFoot(VChild<ModalCardFoot>),
    ModalCardHead(VChild<ModalCardHead>),
    ModalCardTitle(VChild<ModalCardTitle>),
}

/// Defines the properties of the [Bulma modal card body element][bd].
///
/// Defines the properties of the modal card body element, based on the
/// specification found in the [Bulma modal card element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     elements::{button::Button, delete::Delete},
///     layout::modal::{
///         Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
///         ModalCardTitle,
///     },
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalCard>
///                 <ModalCardHead>
///                     <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
///                     <Delete />
///                 </ModalCardHead>
///
///                 <ModalCardBody>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}</ModalCardBody>
///
///                 <ModalCardFoot>
///                     <Button>{"Confirm"}</Button>
///                     <Button>{"Cancel"}</Button>
///                 </ModalCardFoot>
///             </ModalCard>
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/#modal-card
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ModalCardBodyProperties {
    /// The list of elements found inside the [modal card body element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma modal card body element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/modal/#modal-card
    pub children: Children,
}

/// Yew implementation of the [Bulma modal card body element][bd].
///
/// Yew implementation of the modal card body element, based on the
/// specification found in the [Bulma modal card element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     elements::{button::Button, delete::Delete},
///     layout::modal::{
///         Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
///         ModalCardTitle,
///     },
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalCard>
///                 <ModalCardHead>
///                     <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
///                     <Delete />
///                 </ModalCardHead>
///
///                 <ModalCardBody>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}</ModalCardBody>
///
///                 <ModalCardFoot>
///                     <Button>{"Confirm"}</Button>
///                     <Button>{"Cancel"}</Button>
///                 </ModalCardFoot>
///             </ModalCard>
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/#modal-card
#[function_component(ModalCardBody)]
pub fn modal_card_body(props: &ModalCardBodyProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("modal-card-body")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="section" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma modal card foot element][bd].
///
/// Defines the properties of the modal card foot element, based on the
/// specification found in the [Bulma modal card element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     elements::{button::Button, delete::Delete},
///     layout::modal::{
///         Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
///         ModalCardTitle,
///     },
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalCard>
///                 <ModalCardHead>
///                     <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
///                     <Delete />
///                 </ModalCardHead>
///
///                 <ModalCardBody>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}</ModalCardBody>
///
///                 <ModalCardFoot>
///                     <Button>{"Confirm"}</Button>
///                     <Button>{"Cancel"}</Button>
///                 </ModalCardFoot>
///             </ModalCard>
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/#modal-card
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ModalCardFootProperties {
    /// The list of elements found inside the [modal card foot element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma modal card foot element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/modal/#modal-card
    pub children: Children,
}

/// Yew implementation of the [Bulma modal card foot element][bd].
///
/// Yew implementation of the modal card foot element, based on the
/// specification found in the [Bulma modal card element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     elements::{button::Button, delete::Delete},
///     layout::modal::{
///         Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
///         ModalCardTitle,
///     },
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalCard>
///                 <ModalCardHead>
///                     <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
///                     <Delete />
///                 </ModalCardHead>
///
///                 <ModalCardBody>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}</ModalCardBody>
///
///                 <ModalCardFoot>
///                     <Button>{"Confirm"}</Button>
///                     <Button>{"Cancel"}</Button>
///                 </ModalCardFoot>
///             </ModalCard>
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/#modal-card
#[function_component(ModalCardFoot)]
pub fn modal_card_foot(props: &ModalCardFootProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("modal-card-foot")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="footer" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma modal card head element][bd].
///
/// Defines the properties of the modal card head element, based on the
/// specification found in the [Bulma modal card element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     elements::{button::Button, delete::Delete},
///     layout::modal::{
///         Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
///         ModalCardTitle,
///     },
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalCard>
///                 <ModalCardHead>
///                     <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
///                     <Delete />
///                 </ModalCardHead>
///
///                 <ModalCardBody>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}</ModalCardBody>
///
///                 <ModalCardFoot>
///                     <Button>{"Confirm"}</Button>
///                     <Button>{"Cancel"}</Button>
///                 </ModalCardFoot>
///             </ModalCard>
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/#modal-card
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ModalCardHeadProperties {
    /// The list of elements found inside the [modal card head element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma modal card head element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/modal/#modal-card
    pub children: ChildrenRenderer<ModalCardHeadItem>,
}

/// Yew implementation of the [Bulma modal card head element][bd].
///
/// Yew implementation of the modal card head element, based on the
/// specification found in the [Bulma modal card element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     elements::{button::Button, delete::Delete},
///     layout::modal::{
///         Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
///         ModalCardTitle,
///     },
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalCard>
///                 <ModalCardHead>
///                     <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
///                     <Delete />
///                 </ModalCardHead>
///
///                 <ModalCardBody>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}</ModalCardBody>
///
///                 <ModalCardFoot>
///                     <Button>{"Confirm"}</Button>
///                     <Button>{"Cancel"}</Button>
///                 </ModalCardFoot>
///             </ModalCard>
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/#modal-card
#[function_component(ModalCardHead)]
pub fn modal_card_head(props: &ModalCardHeadProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("modal-card-head")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="header" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the possible types of children from a [Bulma modal card element][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma modal card element][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     elements::{button::Button, delete::Delete},
///     layout::modal::{
///         Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
///         ModalCardTitle,
///     },
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalCard>
///                 <ModalCardHead>
///                     <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
///                     <Delete />
///                 </ModalCardHead>
///
///                 <ModalCardBody>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}</ModalCardBody>
///
///                 <ModalCardFoot>
///                     <Button>{"Confirm"}</Button>
///                     <Button>{"Cancel"}</Button>
///                 </ModalCardFoot>
///             </ModalCard>
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/#modal-card
#[derive(Clone, PartialEq, TypedChildren)]
pub enum ModalCardHeadItem {
    ModalCardTitle(VChild<ModalCardTitle>),
    Delete(VChild<Delete>),
}

/// Defines the properties of the [Bulma modal card title element][bd].
///
/// Defines the properties of the modal card title element, based on the
/// specification found in the [Bulma modal card element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     elements::{button::Button, delete::Delete},
///     layout::modal::{
///         Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
///         ModalCardTitle,
///     },
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalCard>
///                 <ModalCardHead>
///                     <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
///                     <Delete />
///                 </ModalCardHead>
///
///                 <ModalCardBody>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}</ModalCardBody>
///
///                 <ModalCardFoot>
///                     <Button>{"Confirm"}</Button>
///                     <Button>{"Cancel"}</Button>
///                 </ModalCardFoot>
///             </ModalCard>
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/#modal-card
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct ModalCardTitleProperties {
    /// The list of elements found inside the [modal card title element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma modal card title element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/modal/#modal-card
    pub children: Children,
}

/// Yew implementation of the [Bulma modal card title element][bd].
///
/// Yew implementation of the modal card title element, based on the
/// specification found in the [Bulma modal card element documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     elements::{button::Button, delete::Delete},
///     layout::modal::{
///         Modal, ModalBackground, ModalCard, ModalCardBody, ModalCardFoot, ModalCardHead,
///         ModalCardTitle,
///     },
/// };
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Modal>
///             <ModalBackground />
///
///             <ModalCard>
///                 <ModalCardHead>
///                     <ModalCardTitle>{"Card modal example"}</ModalCardTitle>
///                     <Delete />
///                 </ModalCardHead>
///
///                 <ModalCardBody>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}</ModalCardBody>
///
///                 <ModalCardFoot>
///                     <Button>{"Confirm"}</Button>
///                     <Button>{"Cancel"}</Button>
///                 </ModalCardFoot>
///             </ModalCard>
///         </Modal>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/modal/#modal-card
#[function_component(ModalCardTitle)]
pub fn modal_card_title(props: &ModalCardTitleProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("modal-card-title")
        .with_custom_class(&props.class.to_string())
        .build();

    html! {
        <BaseComponent tag="p" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}
