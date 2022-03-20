use delegate::delegate;

use crate::dom::impl_try_from_element;
use crate::html::{
    impl_extendable_element, impl_html_element_traits, impl_known_element, labelable_element_seal,
    LabelableElement, Labels,
};

#[derive(Clone)]
pub struct HtmlProgressElement {
    inner: web_sys::HtmlProgressElement,
}

impl HtmlProgressElement {
    delegate! {
        target self.inner {
            pub fn value(&self) -> f64;

            pub fn set_value(&self, value: f64);

            pub fn max(&self) -> f64;

            pub fn set_max(&self, max: f64);
        }
    }

    pub fn position(&self) -> Option<f64> {
        let position = self.inner.position();

        if position >= 0.0 {
            Some(position)
        } else {
            None
        }
    }
}

impl labelable_element_seal::Seal for HtmlProgressElement {}

impl LabelableElement for HtmlProgressElement {
    fn labels(&self) -> Labels {
        Labels::new(Some(self.inner.labels()))
    }
}

impl From<web_sys::HtmlProgressElement> for HtmlProgressElement {
    fn from(inner: web_sys::HtmlProgressElement) -> Self {
        HtmlProgressElement { inner }
    }
}

impl AsRef<web_sys::HtmlProgressElement> for HtmlProgressElement {
    fn as_ref(&self) -> &web_sys::HtmlProgressElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlProgressElement);
impl_try_from_element!(HtmlProgressElement);
impl_known_element!(HtmlProgressElement, "PROGRESS");
impl_extendable_element!(HtmlProgressElement, "progress");
