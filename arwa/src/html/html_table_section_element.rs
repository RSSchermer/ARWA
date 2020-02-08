use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::error::RangeError;
use crate::html::{GenericHtmlElement, HtmlElement, HtmlTableRowElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlTableSectionElement {
    inner: web_sys::HtmlTableSectionElement,
}

impl HtmlTableSectionElement {
    pub fn rows(&self) -> TableSectionRows {
        TableSectionRows {
            table_section: &self.inner,
            rows: self.inner.rows(),
        }
    }
}

impl_html_common_traits!(HtmlTableSectionElement);

pub struct TableSectionRows<'a> {
    table_section: &'a web_sys::HtmlTableSectionElement,
    rows: web_sys::HtmlCollection,
}

impl<'a> TableSectionRows<'a> {
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
        let row: web_sys::HtmlTableRowElement =
            self.table_section.insert_row().unwrap().unchecked_into();

        row.into()
    }

    pub fn insert_new(&self, index: usize) -> Result<HtmlTableRowElement, RangeError> {
        self.table_section
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
        self.table_section
            .delete_row(index as i32)
            .expect("Index out of bounds");
    }

    pub fn iter(&self) -> TableSectionRowsIter {
        TableSectionRowsIter {
            table_section_rows: self,
            current: 0,
        }
    }
}

impl<'a> Write for TableSectionRows<'a> {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.rows.as_ref());
    }
}

impl<'a> IntoIterator for TableSectionRows<'a> {
    type Item = HtmlTableRowElement;
    type IntoIter = TableSectionRowsIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TableSectionRowsIntoIter {
            table_section_rows: self,
            current: 0,
        }
    }
}

pub struct TableSectionRowsIter<'a> {
    table_section_rows: &'a TableSectionRows<'a>,
    current: usize,
}

impl<'a> Iterator for TableSectionRowsIter<'a> {
    type Item = HtmlTableRowElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.table_section_rows.get(current)
    }
}

pub struct TableSectionRowsIntoIter<'a> {
    table_section_rows: TableSectionRows<'a>,
    current: usize,
}

impl<'a> Iterator for TableSectionRowsIntoIter<'a> {
    type Item = HtmlTableRowElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.table_section_rows.get(current)
    }
}
