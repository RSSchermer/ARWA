use delegate::delegate;

use crate::dom::impl_try_from_element;
use crate::html::{impl_extendable_element, impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlOptgroupElement {
    inner: web_sys::HtmlOptGroupElement,
}

impl HtmlOptgroupElement {
    delegate! {
        target self.inner {
            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);

            pub fn label(&self) -> String;

            pub fn set_label(&self, label: &str);
        }
    }
}

impl From<web_sys::HtmlOptGroupElement> for HtmlOptgroupElement {
    fn from(inner: web_sys::HtmlOptGroupElement) -> Self {
        HtmlOptgroupElement { inner }
    }
}

impl AsRef<web_sys::HtmlOptGroupElement> for HtmlOptgroupElement {
    fn as_ref(&self) -> &web_sys::HtmlOptGroupElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlOptgroupElement);
impl_try_from_element!(HtmlOptgroupElement, HtmlOptGroupElement);
impl_known_element!(HtmlOptgroupElement, HtmlOptGroupElement, "OPTGROUP");
impl_extendable_element!(HtmlOptgroupElement, "optgroup");
