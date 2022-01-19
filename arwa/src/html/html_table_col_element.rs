use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{DynamicElement, DynamicNode, Element, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlTableColElement {
    inner: web_sys::HtmlTableColElement,
}

impl HtmlTableColElement {
    delegate! {
        target self.inner {
            pub fn span(&self) -> u32;

            pub fn set_span(&self, span: u32);
        }
    }
}

impl_html_common_traits!(HtmlTableColElement);
