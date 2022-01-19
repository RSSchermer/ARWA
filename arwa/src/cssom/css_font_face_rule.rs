use crate::cssom::{css_rule_seal, CssRule};
use web_sys::CssStyleDeclaration;

#[derive(Clone)]
pub struct CssFontFaceRule {
    inner: web_sys::CssFontFaceRule,
}

impl CssFontFaceRule {
    pub fn style(&self) -> CssStyleDeclaration {
        self.inner.style().into()
    }
}

impl css_rule_seal::Seal for CssFontFaceRule {
    fn as_web_sys_css_rule(&self) -> &web_sys::CssRule {
        self.inner.as_ref()
    }
}

impl CssRule for CssFontFaceRule {}

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

impl_css_rule_traits!(CssFontFaceRule, web_sys::CssFontFaceRule);
