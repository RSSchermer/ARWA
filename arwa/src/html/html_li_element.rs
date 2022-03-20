use delegate::delegate;

use crate::dom::impl_try_from_element;
use crate::html::{impl_extendable_element, impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlLiElement {
    inner: web_sys::HtmlLiElement,
}

impl HtmlLiElement {
    delegate! {
        target self.inner {
            pub fn value(&self) -> i32;

            pub fn set_value(&self, value: i32);
        }
    }
}

impl From<web_sys::HtmlLiElement> for HtmlLiElement {
    fn from(inner: web_sys::HtmlLiElement) -> Self {
        HtmlLiElement { inner }
    }
}

impl AsRef<web_sys::HtmlLiElement> for HtmlLiElement {
    fn as_ref(&self) -> &web_sys::HtmlLiElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlLiElement);
impl_try_from_element!(HtmlLiElement);
impl_known_element!(HtmlLiElement, "LI");
impl_extendable_element!(HtmlLiElement, "li");
