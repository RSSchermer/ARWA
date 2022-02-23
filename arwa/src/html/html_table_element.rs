use wasm_bindgen::JsCast;

use crate::collection::{Collection, Sequence};
use crate::dom::impl_try_from_element;
use crate::html::{
    impl_html_element_traits, impl_known_element, HtmlCaptionElement, HtmlTbodyElement,
    HtmlTfootElement, HtmlTheadElement, HtmlTrElement,
};

#[derive(Clone)]
pub struct HtmlTableElement {
    inner: web_sys::HtmlTableElement,
}

impl HtmlTableElement {
    pub fn caption(&self) -> Option<HtmlCaptionElement> {
        self.inner.caption().map(|c| c.into())
    }

    pub fn table_head(&self) -> Option<HtmlTheadElement> {
        self.inner.t_head().map(|s| HtmlTheadElement::new(s))
    }

    pub fn table_foot(&self) -> Option<HtmlTfootElement> {
        self.inner.t_foot().map(|s| HtmlTfootElement::new(s))
    }

    pub fn table_bodies(&self) -> TableBodies {
        TableBodies {
            inner: self.inner.t_bodies(),
        }
    }

    pub fn table_rows(&self) -> TableRows {
        TableRows {
            inner: self.inner.rows(),
        }
    }

    // TODO: unsure whether to include below convenience methods for modifying a table.
    //
    // pub fn set_caption(&self, caption: Option<&HtmlCaptionElement>) {
    //     self.inner.set_caption(caption.map(|e| e.as_ref()));
    // }
    //
    // pub fn get_or_create_caption(&self) -> HtmlCaptionElement {
    //     let caption: web_sys::HtmlTableCaptionElement =
    //         self.inner.create_caption().unchecked_into();
    //
    //     caption.into()
    // }
    //
    // pub fn remove_caption(&self) {
    //     self.inner.delete_caption();
    // }
    //
    // pub fn set_table_head(&self, thead: Option<&HtmlTableSectionElement>) {
    //     self.inner.set_t_head(thead.map(|e| e.as_ref()));
    // }
    //
    // pub fn get_or_create_table_head(&self) -> HtmlTableSectionElement {
    //     let head: web_sys::HtmlTableSectionElement = self.inner.create_t_head().unchecked_into();
    //
    //     head.into()
    // }
    //
    // pub fn remove_table_head(&self) {
    //     self.inner.delete_t_head();
    // }
    //
    // pub fn create_table_body(&self) -> HtmlTableSectionElement {
    //     let body: web_sys::HtmlTableSectionElement = self.inner.create_t_body().unchecked_into();
    //
    //     body.into()
    // }
    //
    // pub fn set_table_foot(&self, tfoot: Option<&HtmlTableSectionElement>) {
    //     self.inner.set_t_foot(tfoot.map(|e| e.as_ref()));
    // }
    //
    // pub fn get_or_create_table_foot(&self) -> HtmlTableSectionElement {
    //     let foot: web_sys::HtmlTableSectionElement = self.inner.create_t_foot().unchecked_into();
    //
    //     foot.into()
    // }
    //
    // pub fn remove_table_foot(&self) {
    //     self.inner.delete_t_foot();
    // }
}

impl From<web_sys::HtmlTableElement> for HtmlTableElement {
    fn from(inner: web_sys::HtmlTableElement) -> Self {
        HtmlTableElement { inner }
    }
}

impl AsRef<web_sys::HtmlTableElement> for HtmlTableElement {
    fn as_ref(&self) -> &web_sys::HtmlTableElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlTableElement);
impl_try_from_element!(HtmlTableElement);
impl_known_element!(HtmlTableElement, "TABLE");

pub struct TableBodies {
    inner: web_sys::HtmlCollection,
}

impl Collection for TableBodies {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for TableBodies {
    type Item = HtmlTbodyElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner
            .get_with_index(index)
            .map(|e| HtmlTbodyElement::new(e.unchecked_into()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

pub struct TableRows {
    inner: web_sys::HtmlCollection,
}

impl Collection for TableRows {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for TableRows {
    type Item = HtmlTrElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner
            .get_with_index(index)
            .map(|e| HtmlTrElement::from(e.unchecked_into::<web_sys::HtmlTableRowElement>()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}
