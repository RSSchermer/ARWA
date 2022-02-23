use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlTimeElement {
    inner: web_sys::HtmlTimeElement,
}

impl HtmlTimeElement {
    // TODO: date_time
}

impl From<web_sys::HtmlTimeElement> for HtmlTimeElement {
    fn from(inner: web_sys::HtmlTimeElement) -> Self {
        HtmlTimeElement { inner }
    }
}

impl AsRef<web_sys::HtmlTimeElement> for HtmlTimeElement {
    fn as_ref(&self) -> &web_sys::HtmlTimeElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlTimeElement);
impl_try_from_element!(HtmlTimeElement);
impl_known_element!(HtmlTimeElement, "TIME");
