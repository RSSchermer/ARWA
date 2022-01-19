use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{
    DocumentFragment, DynamicElement, DynamicNode, Element, GlobalEventHandlers, InvalidCast, Node,
};

#[derive(Clone)]
pub struct HtmlTemplateElement {
    inner: web_sys::HtmlTemplateElement,
}

impl HtmlTemplateElement {
    pub fn content(&self) -> DocumentFragment {
        self.inner.content().into()
    }
}

impl_html_common_traits!(HtmlTemplateElement);
