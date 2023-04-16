use yew::html;
use yew::{
    function_component, html::ChildrenRenderer, virtual_dom::VChild, Children, Html, Properties,
};
use yew_and_bulma_macros::base_component_properties;

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
        <article id={&props.id} {class}
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
        </article>
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
#[derive(Clone, PartialEq)]
pub enum MessageItem {
    MessageBody(VChild<MessageBody>),
    MessageHeader(VChild<MessageHeader>),
}

impl From<VChild<MessageBody>> for MessageItem {
    fn from(value: VChild<MessageBody>) -> Self {
        MessageItem::MessageBody(value)
    }
}

impl From<VChild<MessageHeader>> for MessageItem {
    fn from(value: VChild<MessageHeader>) -> Self {
        MessageItem::MessageHeader(value)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Html> for MessageItem {
    fn into(self) -> Html {
        match self {
            MessageItem::MessageBody(mb) => mb.into(),
            MessageItem::MessageHeader(mh) => mh.into(),
        }
    }
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
            <p>
                { for props.children.iter() }
            </p>

            if props.delete {
                <Delete />
            }
        </div>
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
