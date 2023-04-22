use quote::quote;
use syn::{parse::Parser, Field};

/// Provides all HTML attributes which should be added to properties.
///
/// Provides definitions for all HTML attributes that should be found on
/// [Yew component properties][yew].
///
/// [yew]: https://yew.rs/docs/concepts/function-components/properties
pub(crate) struct BaseAttributes {
    attributes: Vec<Field>,
}

impl BaseAttributes {
    pub fn attributes(self) -> Vec<Field> {
        self.attributes
    }
}

impl Default for BaseAttributes {
    fn default() -> Self {
        let attributes: Vec<_> = vec![
            quote! {
                /// Sets the [HTML id attribute][id] of the element.
                ///
                /// Sets the [HTML id attrbiute][id] of the element which will receive
                /// these properties.
                ///
                /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id
                #[prop_or_default]
                pub id: Option<yew::AttrValue>
            },
            quote! {
                /// Sets the classes to be appended to the [HTML class attribute][class].
                ///
                /// Sets the classes to be appended to [HTML class attribute][class] of the
                /// element which will receive these properties.
                ///
                /// [class]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class
                #[prop_or_default]
                pub class: Option<yew::Classes>
            },
            quote! {
                /// Sets the extra attributes that the component should have set.
                ///
                /// Sets the extra attributes that the component which will receive these
                /// properties should have set.
                #[prop_or_default]
                pub attrs: std::collections::HashMap<&'static str, yew::AttrValue>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onclick attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onclick attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/click_event
                #[prop_or_default]
                pub onclick: Option<yew::Callback<yew::MouseEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onmousedown attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onmousedown attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event
                #[prop_or_default]
                pub onmousedown: Option<yew::Callback<yew::MouseEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onmousemove attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onmousemove attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event
                #[prop_or_default]
                pub onmousemove: Option<yew::Callback<yew::MouseEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onmouseout attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onmouseout attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event
                #[prop_or_default]
                pub onmouseout: Option<yew::Callback<yew::MouseEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onmouseover attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onmouseover attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event
                #[prop_or_default]
                pub onmouseover: Option<yew::Callback<yew::MouseEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onmouseup attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onmouseup attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event
                #[prop_or_default]
                pub onmouseup: Option<yew::Callback<yew::MouseEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onwheel attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onwheel attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/wheel_event
                #[prop_or_default]
                pub onwheel: Option<yew::Callback<yew::WheelEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML ondrag attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML ondrag attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/drag_event
                #[prop_or_default]
                pub ondrag: Option<yew::Callback<yew::DragEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML ondragend attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML ondragend attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/dragend_event
                #[prop_or_default]
                pub ondragend: Option<yew::Callback<yew::DragEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML ondragenter attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML ondragenter attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/dragenter_event
                #[prop_or_default]
                pub ondragenter: Option<yew::Callback<yew::DragEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML ondragleave attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML ondragleave attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/dragleave_event
                #[prop_or_default]
                pub ondragleave: Option<yew::Callback<yew::DragEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML ondragover attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML ondragover attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/dragover_event
                #[prop_or_default]
                pub ondragover: Option<yew::Callback<yew::DragEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML ondragstart attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML ondragstart attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/dragstart_event
                #[prop_or_default]
                pub ondragstart: Option<yew::Callback<yew::DragEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML ondrop attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML ondrop attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/drop_event
                #[prop_or_default]
                pub ondrop: Option<yew::Callback<yew::DragEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onscroll attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onscroll attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll_event
                #[prop_or_default]
                pub onscroll: Option<yew::Callback<yew::html::onscroll::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML oncopy attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML oncopy attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/copy_event
                #[prop_or_default]
                pub oncopy: Option<yew::Callback<yew::html::oncopy::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML oncut attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML oncut attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/cut_event
                #[prop_or_default]
                pub oncut: Option<yew::Callback<yew::html::oncut::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onpaste attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onpaste attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/paste_event
                #[prop_or_default]
                pub onpaste: Option<yew::Callback<yew::html::onpaste::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onkeydown attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onkeydown attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
                #[prop_or_default]
                pub onkeydown: Option<yew::Callback<yew::KeyboardEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onkeypress attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onkeypress attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keypress_event
                #[prop_or_default]
                pub onkeypress: Option<yew::Callback<yew::KeyboardEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onkeyup attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onkeyup attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event
                #[prop_or_default]
                pub onkeyup: Option<yew::Callback<yew::KeyboardEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onblur attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onblur attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event
                #[prop_or_default]
                pub onblur: Option<yew::Callback<yew::FocusEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onchange attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onchange attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/change_event
                #[prop_or_default]
                pub onchange: Option<yew::Callback<yew::html::onchange::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML oncontextmenu attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML oncontextmenu attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/contextmenu_event
                #[prop_or_default]
                pub oncontextmenu: Option<yew::Callback<yew::html::oncontextmenu::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onfocus attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onfocus attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event
                #[prop_or_default]
                pub onfocus: Option<yew::Callback<yew::FocusEvent>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML oninput attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML oninput attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/input_event
                #[prop_or_default]
                pub oninput: Option<yew::Callback<yew::html::oninput::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML oninvalid attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML oninvalid attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/invalid_event
                #[prop_or_default]
                pub oninvalid: Option<yew::Callback<yew::html::oninvalid::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onreset attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onreset attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/reset_event
                #[prop_or_default]
                pub onreset: Option<yew::Callback<yew::html::onreset::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onselect attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onselect attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/select_event
                #[prop_or_default]
                pub onselect: Option<yew::Callback<yew::html::onselect::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onsubmit attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onsubmit attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/submit_event
                #[prop_or_default]
                pub onsubmit: Option<yew::Callback<yew::html::onsubmit::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onabort attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onabort attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/abort_event
                #[prop_or_default]
                pub onabort: Option<yew::Callback<yew::html::onabort::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML oncanplay attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML oncanplay attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/canplay_event
                #[prop_or_default]
                pub oncanplay: Option<yew::Callback<yew::html::oncanplay::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML oncanplaythrough attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML oncanplaythrough attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/canplaythrough_event
                #[prop_or_default]
                pub oncanplaythrough: Option<yew::Callback<yew::html::oncanplaythrough::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML oncuechange attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML oncuechange attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/cuechange_event
                #[prop_or_default]
                pub oncuechange: Option<yew::Callback<yew::html::oncuechange::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML ondurationchange attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML ondurationchange attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/durationchange_event
                #[prop_or_default]
                pub ondurationchange: Option<yew::Callback<yew::html::ondurationchange::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onemptied attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onemptied attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/emptied_event
                #[prop_or_default]
                pub onemptied: Option<yew::Callback<yew::html::onemptied::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onended attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onended attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/ended_event
                #[prop_or_default]
                pub onended: Option<yew::Callback<yew::html::onended::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onerror attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onerror attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/error_event
                #[prop_or_default]
                pub onerror: Option<yew::Callback<yew::html::onerror::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onloadeddata attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onloadeddata attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/loadeddata_event
                #[prop_or_default]
                pub onloadeddata: Option<yew::Callback<yew::html::onloadeddata::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onloadedmetadata attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onloadedmetadata attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/loadedmetadata_event
                #[prop_or_default]
                pub onloadedmetadata: Option<yew::Callback<yew::html::onloadedmetadata::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onloadstart attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onloadstart attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/loadstart_event
                #[prop_or_default]
                pub onloadstart: Option<yew::Callback<yew::html::onloadstart::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onpause attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onpause attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/pause_event
                #[prop_or_default]
                pub onpause: Option<yew::Callback<yew::html::onpause::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onplay attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onplay attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/play_event
                #[prop_or_default]
                pub onplay: Option<yew::Callback<yew::html::onplay::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onplaying attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onplaying attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/playing_event
                #[prop_or_default]
                pub onplaying: Option<yew::Callback<yew::html::onplaying::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onprogress attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onprogress attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/progress_event
                #[prop_or_default]
                pub onprogress: Option<yew::Callback<yew::html::onprogress::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onratechange attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onratechange attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/ratechange_event
                #[prop_or_default]
                pub onratechange: Option<yew::Callback<yew::html::onratechange::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onseeked attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onseeked attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/seeked_event
                #[prop_or_default]
                pub onseeked: Option<yew::Callback<yew::html::onseeked::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onseeking attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onseeking attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/seeking_event
                #[prop_or_default]
                pub onseeking: Option<yew::Callback<yew::html::onseeking::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onstalled attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onstalled attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/stalled_event
                #[prop_or_default]
                pub onstalled: Option<yew::Callback<yew::html::onstalled::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onsuspend attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onsuspend attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/suspend_event
                #[prop_or_default]
                pub onsuspend: Option<yew::Callback<yew::html::onsuspend::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML ontimeupdate attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML ontimeupdate attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/timeupdate_event
                #[prop_or_default]
                pub ontimeupdate: Option<yew::Callback<yew::html::ontimeupdate::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onvolumechange attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onvolumechange attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/volumechange_event
                #[prop_or_default]
                pub onvolumechange: Option<yew::Callback<yew::html::onvolumechange::Event>>
            },
            quote! {
                /// Sets the callback to be used for the [HTML onwaiting attribute][ev].
                ///
                /// Sets the callback to be used for the [HTML onwaiting attribute][ev] of the
                /// element which will receive these properties.
                ///
                /// [ev]: https://developer.mozilla.org/en-US/docs/Web/API/Element/waiting_event
                #[prop_or_default]
                pub onwaiting: Option<yew::Callback<yew::html::onwaiting::Event>>
            },
            quote! {
                /// Sets the [HTML title attribute][title] of the element.
                ///
                /// Sets the [HTML title attrbiute][title] of the element which will receive
                /// these properties.
                ///
                /// [alable]:https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/title
                #[prop_or_default]
                pub title: Option<yew::AttrValue>
            },
            quote! {
                /// Sets the [HTML role attribute][role] of the element.
                ///
                /// Sets the [HTML role attrbiute][role] of the element which will receive
                /// these properties.
                ///
                /// [role]: https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles
                #[prop_or_default]
                pub role: Option<yew::AttrValue>
            },
            quote! {
                /// Sets the [HTML aria-label attribute][alabel] of the element.
                ///
                /// Sets the [HTML aria-label attrbiute][alabel] of the element which will receive
                /// these properties.
                ///
                /// [alabel]: https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-label
                #[prop_or_default]
                pub aria_label: Option<yew::AttrValue>
            },
            quote! {
                /// Sets the [HTML aria-label attribute][acurr] of the element.
                ///
                /// Sets the [HTML aria-label attrbiute][acurr] of the element which will receive
                /// these properties.
                ///
                /// [acurr]: https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-label
                #[prop_or_default]
                pub aria_current: Option<yew::AttrValue>
            },
        ]
        .into_iter()
        .map(|q| Field::parse_named.parse2(q).unwrap())
        .collect();

        Self { attributes }
    }
}
