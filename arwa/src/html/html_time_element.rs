use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlTimeElement {
    inner: web_sys::HtmlTimeElement,
}

impl HtmlTimeElement {
    delegate! {
        target self.inner {
            pub fn date_time(&self) -> String;

            pub fn set_date_time(&self, date_time: &str);
        }
    }
}

impl_html_common_traits!(HtmlTimeElement);
