use crate::collection::{Collection, Sequence};
use web_sys::HtmlLabelElement;

pub(crate) mod labelable_element_seal {
    pub trait Seal {}
}

pub trait LabelableElement: labelable_element_seal::Seal {
    fn labels(&self) -> Labels;
}

pub struct Labels {
    inner: web_sys::NodeList,
}

impl Labels {
    pub(crate) fn new(inner: web_sys::NodeList) -> Self {
        Labels { inner }
    }
}

impl Collection for Labels {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for Labels {
    type Item = HtmlLabelElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner
            .get(index)
            .map(|e| HtmlLabelElement::from(e.unchecked_into()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}
