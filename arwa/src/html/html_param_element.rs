use delegate::delegate;

use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlParamElement {
    inner: web_sys::HtmlParamElement,
}

impl HtmlParamElement {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, name: &str);

            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);
        }
    }
}

impl From<web_sys::HtmlParamElement> for HtmlParamElement {
    fn from(inner: web_sys::HtmlParamElement) -> Self {
        HtmlParamElement { inner }
    }
}

impl AsRef<web_sys::HtmlParamElement> for HtmlParamElement {
    fn as_ref(&self) -> &web_sys::HtmlParamElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlParamElement);
impl_try_from_element!(HtmlParamElement);
impl_known_element!(HtmlParamElement, "PARAM");
