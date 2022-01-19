use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::html::HtmlLabelElement;

pub struct Labels {
    inner: web_sys::NodeList,
}

impl Labels {
    pub(crate) fn new(inner: web_sys::NodeList) -> Self {
        Labels { inner }
    }

    pub fn get(&self, index: usize) -> Option<HtmlLabelElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get(index))
            .map(|n| {
                let label: web_sys::HtmlLabelElement = n.unchecked_into();

                HtmlLabelElement::from(label)
            })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<HtmlLabelElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<HtmlLabelElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> LabelsIter {
        LabelsIter {
            labels: self,
            current: 0,
        }
    }
}

impl Write for Labels {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl IntoIterator for Labels {
    type Item = HtmlLabelElement;
    type IntoIter = LabelsIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        LabelsIntoIter {
            labels: self,
            current: 0,
        }
    }
}

pub struct LabelsIter<'a> {
    labels: &'a Labels,
    current: usize,
}

impl<'a> Iterator for LabelsIter<'a> {
    type Item = HtmlLabelElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.labels.get(current)
    }
}

pub struct LabelsIntoIter {
    labels: Labels,
    current: usize,
}

impl Iterator for LabelsIntoIter {
    type Item = HtmlLabelElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.labels.get(current)
    }
}
