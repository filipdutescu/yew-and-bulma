use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, Html, Properties,
};
use yew_and_bulma_macros::{base_component_properties, TypedChildren};

use crate::utils::BaseComponent;
use crate::{
    elements::delete::Delete,
    helpers::color::Color,
    utils::{class::ClassBuilder, constants::IS_PREFIX, size::Size},
};

/// Defines the properties of the [Bulma message component][bd].
///
/// Defines the properties of the message component, based on the
/// specification found in the [Bulma message component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::message::{Message, MessageBody, MessageHeader};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Message>
///             <MessageHeader>{"Hello!"}</MessageHeader>
///
///             <MessageBody>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </MessageBody>
///         </Message>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/message/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MessageProperties {
    /// Sets the size of the [message component][bd].
    ///
    /// Sets the size of the [Bulma message component][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     layout::message::{Message, MessageBody, MessageHeader};
    ///     utils::size::Size,
    ///};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Message size={Size::Large}>
    ///             <MessageHeader>{"Hello!"}</MessageHeader>
    ///
    ///             <MessageBody>
    ///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
    ///             </MessageBody>
    ///         </Message>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/message/#sizes
    #[prop_or_default]
    pub size: Option<Size>,
    /// Sets the color of the [Bulma message component][bd].
    ///
    /// Sets the color of the [Bulma message component][bd] which will receive
    /// these properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::{
    ///     components::message::{Message, MessageBody, MessageHeader},
    ///     helpers::color::Color,
    /// };
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Message color={Color::Primary}>
    ///             <MessageHeader>{"Hello!"}</MessageHeader>
    ///
    ///             <MessageBody>
    ///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
    ///             </MessageBody>
    ///         </Message>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/message/#colors
    #[prop_or_default]
    pub color: Option<Color>,
    /// The list of elements found inside the [message component][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma message component][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/message/
    pub children: ChildrenRenderer<MessageItem>,
}

/// Yew implementation of the [Bulma message component][bd].
///
/// Yew implementation of the message component, based on the specification
/// found in the [Bulma message component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::message::{Message, MessageBody, MessageHeader};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Message>
///             <MessageHeader>{"Hello!"}</MessageHeader>
///
///             <MessageBody>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </MessageBody>
///         </Message>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/message/
#[function_component(Message)]
pub fn message(props: &MessageProperties) -> Html {
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
        .with_custom_class("message")
        .with_custom_class(&size)
        .with_color(props.color)
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="article" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}

/// Defines the possible types of children from a [Bulma message component][bd].
///
/// Defines the possible types of children found inside a
/// [Bulma message component][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::message::{Message, MessageBody, MessageHeader};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Message>
///             <MessageHeader>{"Hello!"}</MessageHeader>
///
///             <MessageBody>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </MessageBody>
///         </Message>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/message/
#[derive(Clone, PartialEq, TypedChildren)]
pub enum MessageItem {
    MessageBody(VChild<MessageBody>),
    MessageHeader(VChild<MessageHeader>),
}

/// Defines the properties of the [Bulma message header element][bd].
///
/// Defines the properties of the message header element, based on the
/// specification found in the [Bulma message component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::message::{Message, MessageBody, MessageHeader};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Message>
///             <MessageHeader>{"Hello!"}</MessageHeader>
///
///             <MessageBody>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </MessageBody>
///         </Message>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/message/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MessageHeaderProperties {
    /// Whether or not the [message header element][bd] should have a delete element.
    ///
    /// Whether or not the [Bulma message header element][bd], which will receive
    /// these properties, will have a delete element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use yew_and_bulma::layout::message::{Message, MessageBody, MessageHeader};
    ///
    /// #[function_component(App)]
    /// fn app() -> Html {
    ///     html! {
    ///         <Message>
    ///             <MessageHeader delete=false>{"Hello!"}</MessageHeader>
    ///
    ///             <MessageBody>
    ///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
    ///             </MessageBody>
    ///         </Message>
    ///     }
    /// }
    /// ```
    ///
    /// [bd]: https://bulma.io/documentation/components/message/
    #[prop_or(true)]
    pub delete: bool,
    /// The list of elements found inside the [message header element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma message header element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/message/
    pub children: Children,
}

/// Yew implementation of the [Bulma message header element][bd].
///
/// Yew implementation of the message header element, based on the
/// specification found in the [Bulma message component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::message::{Message, MessageBody, MessageHeader};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Message>
///             <MessageHeader>{"Hello!"}</MessageHeader>
///
///             <MessageBody>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </MessageBody>
///         </Message>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/message/
#[function_component(MessageHeader)]
pub fn message_header(props: &MessageHeaderProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("message-header")
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            <p>{ for props.children.iter() }</p>

            if props.delete {
                <Delete />
            }
        </BaseComponent>
    }
}

/// Defines the properties of the [Bulma message body element][bd].
///
/// Defines the properties of the message body element, based on the
/// specification found in the [Bulma message component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::message::{Message, MessageBody, MessageHeader};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Message>
///             <MessageHeader>{"Hello!"}</MessageHeader>
///
///             <MessageBody>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </MessageBody>
///         </Message>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/message/
#[base_component_properties]
#[derive(Properties, PartialEq)]
pub struct MessageBodyProperties {
    /// The list of elements found inside the [message body element][bd].
    ///
    /// Defines the elements that will be found inside the
    /// [Bulma message body element][bd] which will receive these properties.
    ///
    /// [bd]: https://bulma.io/documentation/components/message/
    pub children: Children,
}

/// Yew implementation of the [Bulma message body element][bd].
///
/// Yew implementation of the message body element, based on the
/// specification found in the [Bulma message component documentation][bd].
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::layout::message::{Message, MessageBody, MessageHeader};
///
/// #[function_component(App)]
/// fn app() -> Html {
///     html! {
///         <Message>
///             <MessageHeader>{"Hello!"}</MessageHeader>
///
///             <MessageBody>
///                 {"Lorem ipsum dolor sit amet, consectetur adipiscing elit..."}
///             </MessageBody>
///         </Message>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/components/message/
#[function_component(MessageBody)]
pub fn message_body(props: &MessageBodyProperties) -> Html {
    let class = ClassBuilder::default()
        .with_custom_class("message-body")
        .with_custom_class(
            &props
                .class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .build();

    html! {
        <BaseComponent tag="div" {class} ..props.into()>
            { for props.children.iter() }
        </BaseComponent>
    }
}
