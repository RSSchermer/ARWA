use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlPreElement {
    inner: web_sys::HtmlPreElement,
}

impl From<web_sys::HtmlPreElement> for HtmlPreElement {
    fn from(inner: web_sys::HtmlPreElement) -> Self {
        HtmlPreElement { inner }
    }
}

impl AsRef<web_sys::HtmlPreElement> for HtmlPreElement {
    fn as_ref(&self) -> &web_sys::HtmlPreElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlPreElement);
impl_try_from_element!(HtmlPreElement);
impl_known_element!(HtmlPreElement, "PRE");
