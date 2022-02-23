use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlLabelElement;

use crate::collection::{Collection, Sequence};

pub(crate) mod labelable_element_seal {
    pub trait Seal {}
}

pub trait LabelableElement: labelable_element_seal::Seal {
    fn labels(&self) -> Labels;
}

pub struct Labels {
    inner: Option<web_sys::NodeList>,
}

impl Labels {
    pub(crate) fn new(inner: Option<web_sys::NodeList>) -> Self {
        Labels { inner }
    }
}

impl Collection for Labels {
    fn len(&self) -> u32 {
        self.inner.as_ref().map(|i| i.length()).unwrap_or(0)
    }
}

impl Sequence for Labels {
    type Item = HtmlLabelElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner
            .as_ref()
            .and_then(|i| i.get(index))
            .map(|e| HtmlLabelElement::from(e.unchecked_into::<web_sys::HtmlLabelElement>()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(
            self.inner
                .as_ref()
                .map(|v| v.as_ref())
                .unwrap_or(&JsValue::null()),
        )
    }
}
