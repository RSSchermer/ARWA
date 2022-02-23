use wasm_bindgen::UnwrapThrowExt;

use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlTitleElement {
    inner: web_sys::HtmlTitleElement,
}

impl HtmlTitleElement {
    // Note: text differs from Node::text_content in that `text` returns the concatenation of
    // direct child Text nodes, and `Node::text_content` returns the concatenation of all descendant
    // text nodes.

    pub fn text(&self) -> String {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.text().unwrap_throw()
    }

    pub fn set_text(&self, text: &str) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.set_text(text).unwrap_throw();
    }
}

impl From<web_sys::HtmlTitleElement> for HtmlTitleElement {
    fn from(inner: web_sys::HtmlTitleElement) -> Self {
        HtmlTitleElement { inner }
    }
}

impl AsRef<web_sys::HtmlTitleElement> for HtmlTitleElement {
    fn as_ref(&self) -> &web_sys::HtmlTitleElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlTitleElement);
impl_try_from_element!(HtmlTitleElement);
impl_known_element!(HtmlTitleElement, "TITLE");
