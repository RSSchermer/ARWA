use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement, HtmlFormElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlOptionElement {
    inner: web_sys::HtmlOptionElement,
}

impl HtmlOptionElement {
    delegate! {
        target self.inner {
            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);

            pub fn label(&self) -> String;

            pub fn set_label(&self, label: &str);

            pub fn default_selected(&self) -> bool;

            pub fn set_default_selected(&self, default_selected: bool);

            pub fn selected(&self) -> bool;

            pub fn set_selected(&self, selected: bool);

            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);

            pub fn text(&self) -> String;

            pub fn set_text(&self, text: &str);
        }
    }

    pub fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| HtmlFormElement::from(form))
    }

    pub fn index(&self) -> usize {
        self.inner.index() as usize
    }
}

impl_html_common_traits!(HtmlOptionElement);
