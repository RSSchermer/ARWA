use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement, Labels};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

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

    pub fn labels(&self) -> Labels {
        Labels::new(self.inner.labels())
    }
}

impl_html_common_traits!(HtmlProgressElement);
