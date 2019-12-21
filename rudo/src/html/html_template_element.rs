use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{
    DocumentFragment, Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node,
};

#[derive(Clone)]
pub struct HtmlTemplateElement {
    inner: web_sys::HtmlTemplateElement,
}

impl HtmlTemplateElement {
    pub fn content(&self) -> DocumentFragment {
        DocumentFragment::from(self.inner.content())
    }
}

impl_html_common_traits!(HtmlTemplateElement);
