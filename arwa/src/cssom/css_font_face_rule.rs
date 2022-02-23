use web_sys::CssStyleDeclaration;

use crate::cssom::impl_css_rule_traits;

#[derive(Clone)]
pub struct CssFontFaceRule {
    inner: web_sys::CssFontFaceRule,
}

impl CssFontFaceRule {
    pub fn style(&self) -> CssStyleDeclaration {
        self.inner.style().into()
    }
}

impl From<web_sys::CssFontFaceRule> for CssFontFaceRule {
    fn from(inner: web_sys::CssFontFaceRule) -> Self {
        CssFontFaceRule { inner }
    }
}

impl AsRef<web_sys::CssFontFaceRule> for CssFontFaceRule {
    fn as_ref(&self) -> &web_sys::CssFontFaceRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssFontFaceRule, CssFontFaceRule);
