use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlAreaElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlMapElement {
    inner: web_sys::HtmlMapElement,
}

impl HtmlMapElement {
    pub fn areas(&self) -> MapAreas {
        MapAreas {
            inner: self.inner.areas(),
        }
    }
}

impl_html_common_traits!(HtmlMapElement);

pub struct MapAreas {
    inner: web_sys::HtmlCollection,
}

impl MapAreas {
    pub fn get(&self, index: usize) -> Option<HtmlAreaElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.item(index))
            .map(|area| {
                let area: web_sys::HtmlAreaElement = area.unchecked_into();

                area.into()
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

    pub fn first(&self) -> Option<HtmlAreaElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<HtmlAreaElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn find_by_id(&self, id: &str) -> Option<HtmlAreaElement> {
        self.inner.get_with_name(id).map(|area| {
            let area: web_sys::HtmlAreaElement = area.unchecked_into();

            area.into()
        })
    }

    pub fn iter(&self) -> MapAreasIter {
        MapAreasIter {
            map_areas: self,
            current: 0,
        }
    }
}

impl Write for MapAreas {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl IntoIterator for MapAreas {
    type Item = HtmlAreaElement;
    type IntoIter = MapAreasIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        MapAreasIntoIter {
            map_areas: self,
            current: 0,
        }
    }
}

pub struct MapAreasIter<'a> {
    map_areas: &'a MapAreas,
    current: usize,
}

impl<'a> Iterator for MapAreasIter<'a> {
    type Item = HtmlAreaElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.map_areas.get(current)
    }
}

pub struct MapAreasIntoIter {
    map_areas: MapAreas,
    current: usize,
}

impl Iterator for MapAreasIntoIter {
    type Item = HtmlAreaElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.map_areas.get(current)
    }
}
