use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::error::RangeError;
use crate::html::{
    GenericHtmlElement, HtmlElement, HtmlTableCaptionElement, HtmlTableRowElement,
    HtmlTableSectionElement,
};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlTableElement {
    inner: web_sys::HtmlTableElement,
}

impl HtmlTableElement {
    delegate! {
        target self.inner {
        }
    }

    pub fn caption(&self) -> Option<HtmlTableCaptionElement> {
        self.inner.caption().map(|c| c.into())
    }

    pub fn set_caption(&self, caption: Option<&HtmlTableCaptionElement>) {
        self.inner.set_caption(caption.map(|e| e.as_ref()));
    }

    pub fn get_or_create_caption(&self) -> HtmlTableCaptionElement {
        let caption: web_sys::HtmlTableCaptionElement =
            self.inner.create_caption().unchecked_into();

        caption.into()
    }

    pub fn remove_caption(&self) {
        self.inner.delete_caption();
    }

    pub fn table_head(&self) -> Option<HtmlTableSectionElement> {
        self.inner.t_head().map(|s| s.into())
    }

    pub fn set_table_head(&self, thead: Option<&HtmlTableSectionElement>) {
        self.inner.set_t_head(thead.map(|e| e.as_ref()));
    }

    pub fn get_or_create_table_head(&self) -> HtmlTableSectionElement {
        let head: web_sys::HtmlTableSectionElement = self.inner.create_t_head().unchecked_into();

        head.into()
    }

    pub fn remove_table_head(&self) {
        self.inner.delete_t_head();
    }

    pub fn get_or_create_table_body(&self) -> HtmlTableSectionElement {
        let body: web_sys::HtmlTableSectionElement = self.inner.create_t_body().unchecked_into();

        body.into()
    }

    pub fn table_foot(&self) -> Option<HtmlTableSectionElement> {
        self.inner.t_foot().map(|s| s.into())
    }

    pub fn set_table_foot(&self, tfoot: Option<&HtmlTableSectionElement>) {
        self.inner.set_t_foot(tfoot.map(|e| e.as_ref()));
    }

    pub fn get_or_create_table_foot(&self) -> HtmlTableSectionElement {
        let foot: web_sys::HtmlTableSectionElement = self.inner.create_t_foot().unchecked_into();

        foot.into()
    }

    pub fn remove_table_foot(&self) {
        self.inner.delete_t_foot();
    }

    pub fn table_bodies(&self) -> TableBodies {
        TableBodies {
            inner: self.inner.t_bodies(),
        }
    }

    pub fn rows(&self) -> TableRows {
        TableRows {
            table: &self.inner,
            rows: self.inner.rows(),
        }
    }
}

impl_html_common_traits!(HtmlTableElement);

pub struct TableBodies {
    inner: web_sys::HtmlCollection,
}

impl TableBodies {
    pub fn get(&self, index: usize) -> Option<HtmlTableSectionElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get_with_index(index))
            .map(|e| {
                let e: web_sys::HtmlTableSectionElement = e.unchecked_into();

                e.into()
            })
    }

    pub fn find_by_id(&self, id: &str) -> Option<HtmlTableSectionElement> {
        self.inner.get_with_name(id).map(|e| {
            let e: web_sys::HtmlTableSectionElement = e.unchecked_into();

            e.into()
        })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<HtmlTableSectionElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<HtmlTableSectionElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> TableBodiesIter {
        TableBodiesIter {
            table_bodies: self,
            current: 0,
        }
    }
}

impl Write for TableBodies {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl IntoIterator for TableBodies {
    type Item = HtmlTableSectionElement;
    type IntoIter = TableBodiesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        TableBodiesIntoIter {
            table_bodies: self,
            current: 0,
        }
    }
}

pub struct TableBodiesIter<'a> {
    table_bodies: &'a TableBodies,
    current: usize,
}

impl<'a> Iterator for TableBodiesIter<'a> {
    type Item = HtmlTableSectionElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.table_bodies.get(current)
    }
}

pub struct TableBodiesIntoIter {
    table_bodies: TableBodies,
    current: usize,
}

impl Iterator for TableBodiesIntoIter {
    type Item = HtmlTableSectionElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.table_bodies.get(current)
    }
}

pub struct TableRows<'a> {
    table: &'a web_sys::HtmlTableElement,
    rows: web_sys::HtmlCollection,
}

impl<'a> TableRows<'a> {
    pub fn get(&self, index: usize) -> Option<HtmlTableRowElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.rows.get_with_index(index))
            .map(|e| {
                let e: web_sys::HtmlTableRowElement = e.unchecked_into();

                e.into()
            })
    }

    pub fn find_by_id(&self, id: &str) -> Option<HtmlTableRowElement> {
        self.rows.get_with_name(id).map(|e| {
            let e: web_sys::HtmlTableRowElement = e.unchecked_into();

            e.into()
        })
    }

    pub fn len(&self) -> usize {
        self.rows.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<HtmlTableRowElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<HtmlTableRowElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn push_new(&self) -> HtmlTableRowElement {
        let cell: web_sys::HtmlTableRowElement = self.table.insert_row().unwrap().unchecked_into();

        HtmlTableRowElement::from(cell)
    }

    pub fn insert_new(&self, index: usize) -> Result<HtmlTableRowElement, RangeError> {
        self.table
            .insert_row_with_index(index as i32)
            .map(|row| {
                let row: web_sys::HtmlTableRowElement = row.unchecked_into();

                row.into()
            })
            .map_err(|e| {
                let e: web_sys::DomException = e.unchecked_into();

                RangeError::new(e)
            })
    }

    pub fn remove(&self, index: usize) {
        // TODO: decide: panic to match std behaviour for e.g. Vec, or Result?
        self.table
            .delete_row(index as i32)
            .expect("Index out of bounds");
    }

    pub fn iter(&self) -> TableRowsIter {
        TableRowsIter {
            table_rows: self,
            current: 0,
        }
    }
}

impl<'a> Write for TableRows<'a> {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.rows.as_ref());
    }
}

impl<'a> IntoIterator for TableRows<'a> {
    type Item = HtmlTableRowElement;
    type IntoIter = TableRowsIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TableRowsIntoIter {
            table_rows: self,
            current: 0,
        }
    }
}

pub struct TableRowsIter<'a> {
    table_rows: &'a TableRows<'a>,
    current: usize,
}

impl<'a> Iterator for TableRowsIter<'a> {
    type Item = HtmlTableRowElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.table_rows.get(current)
    }
}

pub struct TableRowsIntoIter<'a> {
    table_rows: TableRows<'a>,
    current: usize,
}

impl<'a> Iterator for TableRowsIntoIter<'a> {
    type Item = HtmlTableRowElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.table_rows.get(current)
    }
}
