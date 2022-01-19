use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{DynamicElement, DynamicNode, Element, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlModElement {
    inner: web_sys::HtmlModElement,
}

impl HtmlModElement {
    delegate! {
        target self.inner {
            pub fn cite(&self) -> String;

            pub fn set_cite(&self, cite: &str);

            pub fn date_time(&self) -> String;

            pub fn set_date_time(&self, date_time: &str);
        }
    }
}

impl_html_common_traits!(HtmlModElement);
