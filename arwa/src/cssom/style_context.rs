use crate::collection::{Collection, Sequence};
use crate::cssom::CssStyleSheet;

pub(crate) mod style_context_seal {
    pub trait Seal {}
}

pub trait StyleContext: style_context_seal::Seal {
    fn style_sheets(&self) -> StyleSheets;
}

pub struct StyleSheets {
    inner: web_sys::ShyleSheetList,
}

impl StyleSheets {
    pub(crate) fn new(inner: web_sys::StyleSheetList) -> Self {
        StyleSheets { inner }
    }
}

impl Collection for StyleSheets {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for StyleSheets {
    type Item = CssStyleSheet;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get(index).map(|_| s.into())
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}
