use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement, HtmlFormElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlLegendElement {
    inner: web_sys::HtmlLegendElement,
}

impl HtmlLegendElement {
    pub fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| HtmlFormElement::from(form))
    }
}

impl_html_common_traits!(HtmlLegendElement);
