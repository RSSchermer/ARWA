use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement, Labels};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlMeterElement {
    inner: web_sys::HtmlMeterElement,
}

impl HtmlMeterElement {
    delegate! {
        target self.inner {
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

    pub fn labels(&self) -> Labels {
        Labels::new(self.inner.labels())
    }
}

impl_html_common_traits!(HtmlMeterElement);
