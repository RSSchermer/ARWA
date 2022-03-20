use crate::dom::{impl_shadow_host_for_element, impl_try_from_element};
use crate::html::{impl_extendable_element, impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlDivElement {
    inner: web_sys::HtmlDivElement,
}

impl From<web_sys::HtmlDivElement> for HtmlDivElement {
    fn from(inner: web_sys::HtmlDivElement) -> Self {
        HtmlDivElement { inner }
    }
}

impl AsRef<web_sys::HtmlDivElement> for HtmlDivElement {
    fn as_ref(&self) -> &web_sys::HtmlDivElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlDivElement);
impl_try_from_element!(HtmlDivElement);
impl_known_element!(HtmlDivElement, "DIV");
impl_extendable_element!(HtmlDivElement, "div");
impl_shadow_host_for_element!(HtmlDivElement);
