use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{DynamicElement, DynamicNode, Element, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlLiElement {
    inner: web_sys::HtmlLiElement,
}

impl HtmlLiElement {
    delegate! {
        target self.inner {
            pub fn value(&self) -> i32;

            pub fn set_value(&self, value: i32);
        }
    }
}

impl_html_common_traits!(HtmlLiElement);
