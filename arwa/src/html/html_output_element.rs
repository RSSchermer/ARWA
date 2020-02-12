use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement, HtmlFormElement, Labels};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

pub use web_sys::ValidityState;

#[derive(Clone)]
pub struct HtmlOutputElement {
    inner: web_sys::HtmlOutputElement,
}

impl HtmlOutputElement {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, name: &str);

            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);

            pub fn default_value(&self) -> String;

            pub fn set_default_value(&self, default_value: &str);

            pub fn check_validity(&self) -> bool;

            pub fn report_validity(&self) -> bool;

            pub fn will_validate(&self) -> bool;

            pub fn validity(&self) -> ValidityState;

            pub fn set_custom_validity(&self, error: &str);
        }
    }

    // TODO: decide what to do about `type`, which is a readonly attribute that can only be
    // "output". This may make sense in a dynamic language, but may not in Rust.

    pub fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| form.into())
    }

    pub fn validation_message(&self) -> String {
        // There's no indication in the spec that this can actually fail, unwrap for now.
        self.inner.validation_message().unwrap()
    }

    pub fn labels(&self) -> Labels {
        Labels::new(self.inner.labels())
    }
}

impl_html_common_traits!(HtmlOutputElement);
