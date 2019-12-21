use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlBrElement {
    inner: web_sys::HtmlBrElement,
}

impl_html_common_traits!(HtmlBrElement);
