use wasm_bindgen::JsCast;
use web_sys::HtmlAreaElement;

use crate::collection::{Collection, Sequence};
use crate::dom::impl_try_from_element;
use crate::html::{impl_extendable_element, impl_html_element_traits, impl_known_element};

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

impl From<web_sys::HtmlMapElement> for HtmlMapElement {
    fn from(inner: web_sys::HtmlMapElement) -> Self {
        HtmlMapElement { inner }
    }
}

impl AsRef<web_sys::HtmlMapElement> for HtmlMapElement {
    fn as_ref(&self) -> &web_sys::HtmlMapElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlMapElement);
impl_try_from_element!(HtmlMapElement);
impl_known_element!(HtmlMapElement, "MAP");
impl_extendable_element!(HtmlMapElement, "map");

pub struct MapAreas {
    inner: web_sys::HtmlCollection,
}

impl Collection for MapAreas {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for MapAreas {
    type Item = HtmlAreaElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner
            .item(index)
            .map(|e| HtmlAreaElement::from(e.unchecked_into::<web_sys::HtmlAreaElement>()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}
