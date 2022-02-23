use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlDlElement {
    inner: web_sys::HtmlDListElement,
}

impl From<web_sys::HtmlDListElement> for HtmlDlElement {
    fn from(inner: web_sys::HtmlDListElement) -> Self {
        HtmlDlElement { inner }
    }
}

impl AsRef<web_sys::HtmlDListElement> for HtmlDlElement {
    fn as_ref(&self) -> &web_sys::HtmlDListElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlDlElement);
impl_try_from_element!(HtmlDlElement, HtmlDListElement);
impl_known_element!(HtmlDlElement, HtmlDListElement, "DL");
