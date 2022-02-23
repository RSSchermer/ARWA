use crate::dom::{impl_shadow_host_for_element, impl_try_from_element};
use crate::html::{impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlPElement {
    inner: web_sys::HtmlParagraphElement,
}

impl From<web_sys::HtmlParagraphElement> for HtmlPElement {
    fn from(inner: web_sys::HtmlParagraphElement) -> Self {
        HtmlPElement { inner }
    }
}

impl AsRef<web_sys::HtmlParagraphElement> for HtmlPElement {
    fn as_ref(&self) -> &web_sys::HtmlParagraphElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlPElement);
impl_try_from_element!(HtmlPElement, HtmlParagraphElement);
impl_known_element!(HtmlPElement, HtmlParagraphElement, "P");
impl_shadow_host_for_element!(HtmlPElement);
