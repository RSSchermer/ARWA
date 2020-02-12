use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlDataElement {
    inner: web_sys::HtmlDataElement,
}

impl HtmlDataElement {
    delegate! {
        target self.inner {
            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);
        }
    }
}

impl_html_common_traits!(HtmlDataElement);
