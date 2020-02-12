use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlParamElement {
    inner: web_sys::HtmlParamElement,
}

impl HtmlParamElement {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, name: &str);

            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);
        }
    }
}

impl_html_common_traits!(HtmlParamElement);
