use crate::dom::{impl_shadow_host_for_element, impl_try_from_element};
use crate::html::{impl_extendable_element, impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlSpanElement {
    inner: web_sys::HtmlSpanElement,
}

impl From<web_sys::HtmlSpanElement> for HtmlSpanElement {
    fn from(inner: web_sys::HtmlSpanElement) -> Self {
        HtmlSpanElement { inner }
    }
}

impl AsRef<web_sys::HtmlSpanElement> for HtmlSpanElement {
    fn as_ref(&self) -> &web_sys::HtmlSpanElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlSpanElement);
impl_try_from_element!(HtmlSpanElement);
impl_known_element!(HtmlSpanElement, "SPAN");
impl_extendable_element!(HtmlSpanElement, "span");
impl_shadow_host_for_element!(HtmlSpanElement);
