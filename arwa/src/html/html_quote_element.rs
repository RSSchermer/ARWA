use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{DynamicElement, DynamicNode, Element, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlQuoteElement {
    inner: web_sys::HtmlQuoteElement,
}

impl HtmlQuoteElement {
    delegate! {
        target self.inner {
            pub fn cite(&self) -> String;

            pub fn set_cite(&self, cite: &str);
        }
    }
}

impl_html_common_traits!(HtmlQuoteElement);
