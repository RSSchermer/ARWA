use delegate::delegate;

use crate::dom::impl_try_from_element;
use crate::html::{
    impl_extendable_element, impl_html_element_traits, impl_known_element, labelable_element_seal,
    LabelableElement, Labels,
};

#[derive(Clone)]
pub struct HtmlMeterElement {
    inner: web_sys::HtmlMeterElement,
}

impl HtmlMeterElement {
    delegate! {
        to self.inner {
            pub fn value(&self) -> f64;

            pub fn set_value(&self, value: f64);

            pub fn min(&self) -> f64;

            pub fn set_min(&self, min: f64);

            pub fn max(&self) -> f64;

            pub fn set_max(&self, max: f64);

            pub fn low(&self) -> f64;

            pub fn set_low(&self, low: f64);

            pub fn high(&self) -> f64;

            pub fn set_high(&self, high: f64);

            pub fn optimum(&self) -> f64;

            pub fn set_optimum(&self, optimum: f64);
        }
    }
}

impl labelable_element_seal::Seal for HtmlMeterElement {}

impl LabelableElement for HtmlMeterElement {
    fn labels(&self) -> Labels {
        Labels::new(Some(self.inner.labels()))
    }
}

impl From<web_sys::HtmlMeterElement> for HtmlMeterElement {
    fn from(inner: web_sys::HtmlMeterElement) -> Self {
        HtmlMeterElement { inner }
    }
}

impl AsRef<web_sys::HtmlMeterElement> for HtmlMeterElement {
    fn as_ref(&self) -> &web_sys::HtmlMeterElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlMeterElement);
impl_try_from_element!(HtmlMeterElement);
impl_known_element!(HtmlMeterElement, "METER");
impl_extendable_element!(HtmlMeterElement, "meter");
