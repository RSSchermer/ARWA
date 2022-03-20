use wasm_bindgen::JsCast;

use crate::collection::{Collection, Sequence};
use crate::dom::impl_try_from_element;
use crate::html::{
    impl_extendable_element, impl_html_element_traits, impl_known_element, DynamicTableCellElement,
};

#[derive(Clone)]
pub struct HtmlTrElement {
    inner: web_sys::HtmlTableRowElement,
}

impl HtmlTrElement {
    pub fn row_index(&self) -> Option<usize> {
        let index = self.inner.row_index();

        if index > 0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn cells(&self) -> TableRowCells {
        TableRowCells {
            inner: self.inner.cells(),
        }
    }
}

impl From<web_sys::HtmlTableRowElement> for HtmlTrElement {
    fn from(inner: web_sys::HtmlTableRowElement) -> Self {
        HtmlTrElement { inner }
    }
}

impl AsRef<web_sys::HtmlTableRowElement> for HtmlTrElement {
    fn as_ref(&self) -> &web_sys::HtmlTableRowElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlTrElement);
impl_try_from_element!(HtmlTrElement, HtmlTableRowElement);
impl_known_element!(HtmlTrElement, HtmlTableRowElement, "TR");
impl_extendable_element!(HtmlTrElement, "tr");

pub struct TableRowCells {
    inner: web_sys::HtmlCollection,
}

impl Collection for TableRowCells {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for TableRowCells {
    type Item = DynamicTableCellElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get_with_index(index).map(|e| {
            DynamicTableCellElement::from(e.unchecked_into::<web_sys::HtmlTableCellElement>())
        })
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}
