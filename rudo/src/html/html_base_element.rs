use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlBaseElement {
    inner: web_sys::HtmlBaseElement,
}

impl HtmlBaseElement {
    delegate! {
        target self.inner {
            pub fn href(&self) -> String;

            pub fn set_href(&self, href: &str);

            pub fn target(&self) -> String;

            pub fn set_target(&self, target: &str);
        }
    }
}

impl_html_common_traits!(HtmlBaseElement);
