use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{DynamicElement, DynamicNode, Element, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlMetaElement {
    inner: web_sys::HtmlMetaElement,
}

impl HtmlMetaElement {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, name: &str);

            pub fn http_equiv(&self) -> String;

            pub fn set_http_equiv(&self, http_equiv: &str);

            pub fn content(&self) -> String;

            pub fn set_content(&self, content: &str);
        }
    }
}

impl_html_common_traits!(HtmlMetaElement);
