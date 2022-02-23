use delegate::delegate;

use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlDetailsElement {
    inner: web_sys::HtmlDetailsElement,
}

impl HtmlDetailsElement {
    delegate! {
        target self.inner {
            pub fn open(&self) -> bool;

            pub fn set_open(&self, open: bool);
        }
    }
}

impl From<web_sys::HtmlDetailsElement> for HtmlDetailsElement {
    fn from(inner: web_sys::HtmlDetailsElement) -> Self {
        HtmlDetailsElement { inner }
    }
}

impl AsRef<web_sys::HtmlDetailsElement> for HtmlDetailsElement {
    fn as_ref(&self) -> &web_sys::HtmlDetailsElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlDetailsElement);
impl_try_from_element!(HtmlDetailsElement);
impl_known_element!(HtmlDetailsElement, "DETAILS");
