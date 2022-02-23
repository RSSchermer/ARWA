use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlHtmlElement {
    inner: web_sys::HtmlHtmlElement,
}

impl From<web_sys::HtmlHtmlElement> for HtmlHtmlElement {
    fn from(inner: web_sys::HtmlHtmlElement) -> Self {
        HtmlHtmlElement { inner }
    }
}

impl AsRef<web_sys::HtmlHtmlElement> for HtmlHtmlElement {
    fn as_ref(&self) -> &web_sys::HtmlHtmlElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlHtmlElement);
impl_try_from_element!(HtmlHtmlElement);
impl_known_element!(HtmlHtmlElement, "HTML");
