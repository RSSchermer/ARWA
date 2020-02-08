use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::GenericElement;

pub struct QuerySelectorAll {
    inner: web_sys::NodeList,
}

impl QuerySelectorAll {
    pub(crate) fn new(inner: web_sys::NodeList) -> Self {
        QuerySelectorAll { inner }
    }

    pub fn get(&self, index: usize) -> Option<GenericElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get(index))
            .map(|node| {
                let element: web_sys::Element = node.unchecked_into();

                element.into()
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

    pub fn first(&self) -> Option<GenericElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<GenericElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> QuerySelectorAllIter {
        QuerySelectorAllIter {
            query_selector_all: self,
            current: 0,
        }
    }
}

impl Write for QuerySelectorAll {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

impl IntoIterator for QuerySelectorAll {
    type Item = GenericElement;
    type IntoIter = QuerySelectorAllIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        QuerySelectorAllIntoIter {
            query_selector_all: self,
            current: 0,
        }
    }
}

pub struct QuerySelectorAllIter<'a> {
    query_selector_all: &'a QuerySelectorAll,
    current: usize,
}

impl<'a> Iterator for QuerySelectorAllIter<'a> {
    type Item = GenericElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.query_selector_all.get(current)
    }
}

pub struct QuerySelectorAllIntoIter {
    query_selector_all: QuerySelectorAll,
    current: usize,
}

impl Iterator for QuerySelectorAllIntoIter {
    type Item = GenericElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.query_selector_all.get(current)
    }
}
