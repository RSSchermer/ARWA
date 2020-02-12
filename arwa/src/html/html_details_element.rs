use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlDetailsElement {
    inner: web_sys::HtmlDetailsElement,
}

impl HtmlDetailsElement {
    delegate! {
        target self.inner {
            pub fn open(&self) -> bool;

            pub fn set_open(&self, open: bool);
        }
    }
}

impl_html_common_traits!(HtmlDetailsElement);
