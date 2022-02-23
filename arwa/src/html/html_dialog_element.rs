use delegate::delegate;

use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlDialogElement {
    inner: web_sys::HtmlDialogElement,
}

impl HtmlDialogElement {
    delegate! {
        target self.inner {
            pub fn return_value(&self) -> String;

            pub fn set_return_value(&self, return_value: &str);

            pub fn show(&self);

            pub fn close(&self);
        }
    }

    pub fn is_open(&self) -> bool {
        self.inner.open()
    }

    pub fn close_with(&self, return_value: &str) {
        self.inner.close_with_return_value(return_value);
    }
}

impl From<web_sys::HtmlDialogElement> for HtmlDialogElement {
    fn from(inner: web_sys::HtmlDialogElement) -> Self {
        HtmlDialogElement { inner }
    }
}

impl AsRef<web_sys::HtmlDialogElement> for HtmlDialogElement {
    fn as_ref(&self) -> &web_sys::HtmlDialogElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlDialogElement);
impl_try_from_element!(HtmlDialogElement);
impl_known_element!(HtmlDialogElement, "DIALOG");
