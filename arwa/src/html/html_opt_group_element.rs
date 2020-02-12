use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlOptGroupElement {
    inner: web_sys::HtmlOptGroupElement,
}

impl HtmlOptGroupElement {
    delegate! {
        target self.inner {
            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);

            pub fn label(&self) -> String;

            pub fn set_label(&self, label: &str);
        }
    }
}

impl_html_common_traits!(HtmlOptGroupElement);
