use std::convert::TryFrom;
use std::str::FromStr;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement, HtmlFormElement};
use crate::{
    DynamicElement, DynamicNode, Element, GenericDocument, GlobalEventHandlers, InvalidCast, Node,
};

pub use web_sys::ValidityState;

#[derive(Clone)]
pub struct HtmlObjectElement {
    inner: web_sys::HtmlObjectElement,
}

impl HtmlObjectElement {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, name: &str);

            pub fn data(&self) -> String;

            pub fn set_data(&self, data: &str);

            pub fn type_must_match(&self) -> bool;

            pub fn set_type_must_match(&self, type_must_match: bool);

            pub fn use_map(&self) -> String;

            pub fn set_use_map(&self, use_map: &str);

            pub fn check_validity(&self) -> bool;

            pub fn report_validity(&self) -> bool;

            pub fn will_validate(&self) -> bool;

            pub fn validity(&self) -> ValidityState;

            pub fn set_custom_validity(&self, error: &str);
        }
    }

    pub fn mime_type(&self) -> String {
        self.inner.type_()
    }

    pub fn set_mime_type(&self, mime_type: &str) {
        self.inner.set_type(mime_type);
    }

    pub fn width(&self) -> u32 {
        u32::from_str(&self.inner.width()).unwrap_or(0)
    }

    pub fn set_width(&self, width: u32) {
        self.inner.set_width(&width.to_string());
    }

    pub fn height(&self) -> u32 {
        u32::from_str(&self.inner.height()).unwrap_or(0)
    }

    pub fn set_height(&self, height: u32) {
        self.inner.set_height(&height.to_string());
    }

    pub fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| form.into())
    }

    pub fn content_document(&self) -> Option<GenericDocument> {
        self.inner
            .content_document()
            .map(|document| document.into())
    }

    // TODO: content_window once Window has been figured out.

    pub fn validation_message(&self) -> String {
        // There's no indication in the spec that this can actually fail, unwrap for now.
        self.inner.validation_message().unwrap()
    }
}

impl_html_common_traits!(HtmlObjectElement);
