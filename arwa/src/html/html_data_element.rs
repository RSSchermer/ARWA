use delegate::delegate;

use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlDataElement {
    inner: web_sys::HtmlDataElement,
}

impl HtmlDataElement {
    delegate! {
        target self.inner {
            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);
        }
    }
}

impl From<web_sys::HtmlDataElement> for HtmlDataElement {
    fn from(inner: web_sys::HtmlDataElement) -> Self {
        HtmlDataElement { inner }
    }
}

impl AsRef<web_sys::HtmlDataElement> for HtmlDataElement {
    fn as_ref(&self) -> &web_sys::HtmlDataElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlDataElement);
impl_try_from_element!(HtmlDataElement);
impl_known_element!(HtmlDataElement, "DATA");
