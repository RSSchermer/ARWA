use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::error::RangeError;
use crate::html::{GenericHtmlElement, HtmlElement, HtmlTableCellElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlTableRowElement {
    inner: web_sys::HtmlTableRowElement,
}

impl HtmlTableRowElement {
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
            table_row: &self.inner,
            cells: self.inner.cells(),
        }
    }
}

impl_html_common_traits!(HtmlTableRowElement);

pub struct TableRowCells<'a> {
    table_row: &'a web_sys::HtmlTableRowElement,
    cells: web_sys::HtmlCollection,
}

impl<'a> TableRowCells<'a> {
    pub fn get(&self, index: usize) -> Option<HtmlTableCellElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.cells.get_with_index(index))
            .map(|e| {
                let e: web_sys::HtmlTableCellElement = e.unchecked_into();

                HtmlTableCellElement::from(e)
            })
    }

    pub fn find_by_id(&self, id: &str) -> Option<HtmlTableCellElement> {
        self.cells.get_with_name(id).map(|e| {
            let e: web_sys::HtmlTableCellElement = e.unchecked_into();

            HtmlTableCellElement::from(e)
        })
    }

    pub fn len(&self) -> usize {
        self.cells.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<HtmlTableCellElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<HtmlTableCellElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn push_new(&self) -> HtmlTableCellElement {
        let cell: web_sys::HtmlTableCellElement =
            self.table_row.insert_cell().unwrap().unchecked_into();

        HtmlTableCellElement::from(cell)
    }

    pub fn insert_new(&self, index: usize) -> Result<HtmlTableCellElement, RangeError> {
        self.table_row
            .insert_cell_with_index(index as i32)
            .map(|cell| {
                let cell: web_sys::HtmlTableCellElement = cell.unchecked_into();

                HtmlTableCellElement::from(cell)
            })
            .map_err(|e| {
                let e: web_sys::DomException = e.unchecked_into();

                RangeError::new(e)
            })
    }

    pub fn remove(&self, index: usize) {
        // TODO: decide: panic to match std behaviour for e.g. Vec, or Result?
        self.table_row
            .delete_cell(index as i32)
            .expect("Index out of bounds");
    }

    pub fn iter(&self) -> TableRowCellsIter {
        TableRowCellsIter {
            table_row_cells: self,
            current: 0,
        }
    }
}

impl<'a> IntoIterator for TableRowCells<'a> {
    type Item = HtmlTableCellElement;
    type IntoIter = TableRowCellsIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TableRowCellsIntoIter {
            table_row_cells: self,
            current: 0,
        }
    }
}

pub struct TableRowCellsIter<'a> {
    table_row_cells: &'a TableRowCells<'a>,
    current: usize,
}

impl<'a> Iterator for TableRowCellsIter<'a> {
    type Item = HtmlTableCellElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.table_row_cells.get(current)
    }
}

pub struct TableRowCellsIntoIter<'a> {
    table_row_cells: TableRowCells<'a>,
    current: usize,
}

impl<'a> Iterator for TableRowCellsIntoIter<'a> {
    type Item = HtmlTableCellElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.table_row_cells.get(current)
    }
}
