use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OListType {
    Decimal,
    LowerAlpha,
    UpperAlpha,
    LowerRoman,
    UpperRoman,
}

impl Default for OListType {
    fn default() -> Self {
        OListType::Decimal
    }
}

#[derive(Clone)]
pub struct HtmlOListElement {
    inner: web_sys::HtmlOListElement,
}

impl HtmlOListElement {
    delegate! {
        target self.inner {
            pub fn reversed(&self) -> bool;

            pub fn set_reversed(&self, reversed: bool);

            pub fn start(&self) -> i32;

            pub fn set_start(&self, start: i32);
        }
    }

    pub fn list_type(&self) -> OListType {
        match &*self.inner.type_() {
            "a" => OListType::LowerAlpha,
            "A" => OListType::UpperAlpha,
            "i" => OListType::LowerRoman,
            "I" => OListType::UpperRoman,
            _ => OListType::Decimal,
        }
    }

    pub fn set_list_type(&self, list_type: OListType) {
        let list_type = match list_type {
            OListType::Decimal => "1",
            OListType::LowerAlpha => "a",
            OListType::UpperAlpha => "A",
            OListType::LowerRoman => "i",
            OListType::UpperRoman => "I",
        };

        self.inner.set_type(list_type);
    }
}

impl_html_common_traits!(HtmlOListElement);
