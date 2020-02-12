use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlTitleElement {
    inner: web_sys::HtmlTitleElement,
}

impl HtmlTitleElement {
    pub fn text(&self) -> String {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.text().unwrap()
    }

    pub fn set_text(&self, text: &str) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.set_text(text).unwrap();
    }
}

impl_html_common_traits!(HtmlTitleElement);
