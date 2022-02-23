use wasm_bindgen::JsCast;

use crate::collection::{Collection, Sequence};
use crate::cssom::CssStyleSheet;

pub(crate) mod styled_seal {
    pub trait Seal {}
}

pub trait Styled: styled_seal::Seal {
    fn style_sheets(&self) -> StyleSheets;
}

pub struct StyleSheets {
    inner: web_sys::StyleSheetList,
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
        // web_sys returns `StyleSheet`, but spec says `CssStyleSheet` so we cast.
        self.inner
            .get(index)
            .map(|s| CssStyleSheet::from(s.unchecked_into::<web_sys::CssStyleSheet>()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}
