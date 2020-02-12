use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{
    Element, GenericDocument, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node,
};
use std::str::FromStr;

#[derive(Clone)]
pub struct HtmlIFrameElement {
    inner: web_sys::HtmlIFrameElement,
}

impl HtmlIFrameElement {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, name: &str);

            pub fn src(&self) -> String;

            pub fn set_src(&self, src: &str);

            pub fn srcdoc(&self) -> String;

            pub fn set_srcdoc(&self, srcdoc: &str);

            pub fn allow_payment_request(&self) -> bool;

            pub fn set_allow_payment_request(&self, allow_payment_request: bool);
        }
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

    pub fn content_document(&self) -> Option<GenericDocument> {
        self.inner
            .content_document()
            .map(|document| document.into())
    }

    // TODO: content_window once Window has been figured out.
}

impl_html_common_traits!(HtmlIFrameElement);
