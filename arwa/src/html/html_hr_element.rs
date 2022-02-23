use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlHrElement {
    inner: web_sys::HtmlHrElement,
}

impl From<web_sys::HtmlHrElement> for HtmlHrElement {
    fn from(inner: web_sys::HtmlHrElement) -> Self {
        HtmlHrElement { inner }
    }
}

impl AsRef<web_sys::HtmlHrElement> for HtmlHrElement {
    fn as_ref(&self) -> &web_sys::HtmlHrElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlHrElement);
impl_try_from_element!(HtmlHrElement);
impl_known_element!(HtmlHrElement, "HR");
