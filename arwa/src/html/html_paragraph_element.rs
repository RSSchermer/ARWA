use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlParagraphElement {
    inner: web_sys::HtmlParagraphElement,
}

impl_html_common_traits!(HtmlParagraphElement);
