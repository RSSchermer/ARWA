use delegate::delegate;

use crate::dom::impl_try_from_element;
use crate::html::{impl_extendable_element, impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlQuoteElement {
    inner: web_sys::HtmlQuoteElement,
}

impl HtmlQuoteElement {
    delegate! {
        to self.inner {
            pub fn cite(&self) -> String;

            pub fn set_cite(&self, cite: &str);
        }
    }
}

impl From<web_sys::HtmlQuoteElement> for HtmlQuoteElement {
    fn from(inner: web_sys::HtmlQuoteElement) -> Self {
        HtmlQuoteElement { inner }
    }
}

impl AsRef<web_sys::HtmlQuoteElement> for HtmlQuoteElement {
    fn as_ref(&self) -> &web_sys::HtmlQuoteElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlQuoteElement);
impl_try_from_element!(HtmlQuoteElement);
impl_known_element!(HtmlQuoteElement, "QUOTE");
impl_extendable_element!(HtmlQuoteElement, "quote");
