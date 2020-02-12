use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{
    Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node, StyleSheet,
};

#[derive(Clone)]
pub struct HtmlStyleElement {
    inner: web_sys::HtmlStyleElement,
}

impl HtmlStyleElement {
    delegate! {
        target self.inner {
            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);

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

    pub fn sheet(&self) -> Option<StyleSheet> {
        self.inner.sheet().map(|s| s.into())
    }
}

impl_html_common_traits!(HtmlStyleElement);
