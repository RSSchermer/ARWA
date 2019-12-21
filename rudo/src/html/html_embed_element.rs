use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlEmbedElement {
    inner: web_sys::HtmlEmbedElement,
}

impl HtmlEmbedElement {
    delegate! {
        target self.inner {
            pub fn width(&self) -> String;

            pub fn set_width(&self, width: &str);

            pub fn height(&self) -> String;

            pub fn set_height(&self, height: &str);

            pub fn src(&self) -> String;

            pub fn set_src(&self, src: &str);
        }
    }

    pub fn mime_type(&self) -> String {
        self.inner.type_()
    }

    pub fn set_mime_type(&self, mime_type: &str) {
        self.inner.set_type(mime_type);
    }
}

impl_html_common_traits!(HtmlEmbedElement);
