use crate::dom::impl_try_from_element;
use crate::html::{impl_extendable_element, impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlCaptionElement {
    inner: web_sys::HtmlTableCaptionElement,
}

impl From<web_sys::HtmlTableCaptionElement> for HtmlCaptionElement {
    fn from(inner: web_sys::HtmlTableCaptionElement) -> Self {
        HtmlCaptionElement { inner }
    }
}

impl AsRef<web_sys::HtmlTableCaptionElement> for HtmlCaptionElement {
    fn as_ref(&self) -> &web_sys::HtmlTableCaptionElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlCaptionElement);
impl_try_from_element!(HtmlCaptionElement, HtmlTableCaptionElement);
impl_known_element!(HtmlCaptionElement, HtmlTableCaptionElement, "CAPTION");
impl_extendable_element!(HtmlCaptionElement, "caption");
