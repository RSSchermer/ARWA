use wasm_bindgen::UnwrapThrowExt;

use crate::cssom::{impl_css_rule_traits, CssStyleSheet};

#[derive(Clone)]
pub struct CssImportRule {
    inner: web_sys::CssImportRule,
}

impl CssImportRule {
    // TODO: decide what to do about `href`. It returns a raw string, whereas
    // `import_rule.style_sheet().href()` returns the resolved URL. Perhaps call it `href_text`, or
    // ignore it altogether?

    pub fn style_sheet(&self) -> CssStyleSheet {
        // Spec says import rule always has an associated stylesheet
        self.inner.style_sheet().unwrap_throw().into()
    }

    // todo: media
}

impl From<web_sys::CssImportRule> for CssImportRule {
    fn from(inner: web_sys::CssImportRule) -> Self {
        CssImportRule { inner }
    }
}

impl AsRef<web_sys::CssImportRule> for CssImportRule {
    fn as_ref(&self) -> &web_sys::CssImportRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssImportRule, CssImportRule);
