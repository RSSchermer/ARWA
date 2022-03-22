use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::dom::{impl_try_from_element, DynamicElement};
use crate::html::{
    impl_extendable_element, impl_html_element_traits, impl_known_element, HtmlFormElement,
};

#[derive(Clone)]
pub struct HtmlLabelElement {
    inner: web_sys::HtmlLabelElement,
}

impl HtmlLabelElement {
    delegate! {
        to self.inner {
            pub fn html_for(&self) -> String;

            pub fn set_html_for(&self, html_for: &str);
        }
    }

    pub fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| form.into())
    }

    pub fn control(&self) -> Option<DynamicElement> {
        self.inner
            .control()
            .map(|e| DynamicElement::from(e.unchecked_into::<web_sys::Element>()))
    }
}

impl From<web_sys::HtmlLabelElement> for HtmlLabelElement {
    fn from(inner: web_sys::HtmlLabelElement) -> Self {
        HtmlLabelElement { inner }
    }
}

impl AsRef<web_sys::HtmlLabelElement> for HtmlLabelElement {
    fn as_ref(&self) -> &web_sys::HtmlLabelElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlLabelElement);
impl_try_from_element!(HtmlLabelElement);
impl_known_element!(HtmlLabelElement, "LABEL");
impl_extendable_element!(HtmlLabelElement, "label");
