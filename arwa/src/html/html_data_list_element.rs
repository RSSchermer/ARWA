use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::html::{GenericHtmlElement, HtmlElement, HtmlOptionElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlDataListElement {
    inner: web_sys::HtmlDataListElement,
}

impl HtmlDataListElement {
    pub fn options(&self) -> DataListOptions {
        DataListOptions {
            inner: self.inner.options(),
        }
    }
}

impl_html_common_traits!(HtmlDataListElement);

pub struct DataListOptions {
    inner: web_sys::HtmlCollection,
}

impl DataListOptions {
    pub fn get(&self, index: usize) -> Option<HtmlOptionElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get_with_index(index))
            .map(|e| {
                let option: web_sys::HtmlOptionElement = e.unchecked_into();

                option.into()
            })
    }

    pub fn find_by_id(&self, id: &str) -> Option<HtmlOptionElement> {
        self.inner.get_with_name(id).map(|e| {
            let option: web_sys::HtmlOptionElement = e.unchecked_into();

            option.into()
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

    pub fn first(&self) -> Option<HtmlOptionElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<HtmlOptionElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> DataListOptionsIter {
        DataListOptionsIter {
            data_list_options: self,
            current: 0,
        }
    }
}

impl Write for DataListOptions {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl IntoIterator for DataListOptions {
    type Item = HtmlOptionElement;
    type IntoIter = DataListOptionsIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        DataListOptionsIntoIter {
            data_list_options: self,
            current: 0,
        }
    }
}

pub struct DataListOptionsIter<'a> {
    data_list_options: &'a DataListOptions,
    current: usize,
}

impl<'a> Iterator for DataListOptionsIter<'a> {
    type Item = HtmlOptionElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.data_list_options.get(current)
    }
}

pub struct DataListOptionsIntoIter {
    data_list_options: DataListOptions,
    current: usize,
}

impl Iterator for DataListOptionsIntoIter {
    type Item = HtmlOptionElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.data_list_options.get(current)
    }
}
