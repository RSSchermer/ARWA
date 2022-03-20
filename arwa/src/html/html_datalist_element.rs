use wasm_bindgen::JsCast;

use crate::collection::{Collection, Sequence};
use crate::dom::impl_try_from_element;
use crate::html::{
    impl_extendable_element, impl_html_element_traits, impl_known_element, HtmlOptionElement,
};

#[derive(Clone)]
pub struct HtmlDatalistElement {
    inner: web_sys::HtmlDataListElement,
}

impl HtmlDatalistElement {
    pub fn options(&self) -> DatalistOptions {
        DatalistOptions {
            inner: self.inner.options(),
        }
    }
}

impl From<web_sys::HtmlDataListElement> for HtmlDatalistElement {
    fn from(inner: web_sys::HtmlDataListElement) -> Self {
        HtmlDatalistElement { inner }
    }
}

impl AsRef<web_sys::HtmlDataListElement> for HtmlDatalistElement {
    fn as_ref(&self) -> &web_sys::HtmlDataListElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlDatalistElement);
impl_try_from_element!(HtmlDatalistElement, HtmlDataListElement);
impl_known_element!(HtmlDatalistElement, HtmlDataListElement, "DATALIST");
impl_extendable_element!(HtmlDatalistElement, "datalist");

pub struct DatalistOptions {
    inner: web_sys::HtmlCollection,
}

impl Collection for DatalistOptions {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for DatalistOptions {
    type Item = HtmlOptionElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get_with_index(index).map(|e| {
            let option: web_sys::HtmlOptionElement = e.unchecked_into();

            option.into()
        })
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}
