use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{DynamicElement, DynamicNode, Element, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlSourceElement {
    inner: web_sys::HtmlSourceElement,
}

impl HtmlSourceElement {
    delegate! {
        target self.inner {
            pub fn src(&self) -> String;

            pub fn set_src(&self, src: &str);

            pub fn media(&self) -> String;

            pub fn set_media(&self, media: &str);
        }
    }

    pub fn mime_type(&self) -> String {
        self.inner.type_()
    }

    pub fn set_mime_type(&self, mime_type: &str) {
        self.inner.set_type(mime_type);
    }
}

impl_html_common_traits!(HtmlSourceElement);
