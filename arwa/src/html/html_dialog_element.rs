use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlDialogElement {
    inner: web_sys::HtmlDialogElement,
}

impl HtmlDialogElement {
    delegate! {
        target self.inner {
            pub fn open(&self) -> bool;

            pub fn set_open(&self, open: bool);

            pub fn return_value(&self) -> String;

            pub fn set_return_value(&self, return_value: &str);

            pub fn show(&self);

            pub fn close(&self);
        }
    }

    pub fn close_with(&self, return_value: &str) {
        self.inner.close_with_return_value(return_value);
    }
}

impl_html_common_traits!(HtmlDialogElement);
