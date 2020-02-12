use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlTableCellElement {
    inner: web_sys::HtmlTableCellElement,
}

impl HtmlTableCellElement {
    delegate! {
        target self.inner {
            pub fn col_span(&self) -> u32;

            pub fn set_col_span(&self, col_span: u32);

            pub fn row_span(&self) -> u32;

            pub fn set_row_span(&self, row_span: u32);

            pub fn headers(&self) -> String;

            pub fn set_headers(&self, headers: &str);
        }
    }

    pub fn cell_index(&self) -> Option<usize> {
        let index = self.inner.cell_index();

        if index > 0 {
            Some(index as usize)
        } else {
            None
        }
    }

    // TODO: `scope` seems absent in web_sys.
}

impl_html_common_traits!(HtmlTableCellElement);
