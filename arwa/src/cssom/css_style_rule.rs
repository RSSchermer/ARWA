use crate::cssom::{css_rule_seal, CssRule};
use web_sys::CssStyleDeclaration;

#[derive(Clone)]
pub struct CssStyleRule {
    inner: web_sys::CssStyleRule,
}

impl CssStyleRule {
    delegate! {
        to self.inner {
            pub fn selector_text(&self) -> String;

            pub fn set_selector_text(&self, value: &str);
        }
    }

    pub fn style(&self) -> CssStyleDeclaration {
        self.inner.style().into()
    }
}

impl css_rule_seal::Seal for CssStyleRule {
    fn as_web_sys_css_rule(&self) -> &web_sys::CssRule {
        self.inner.as_ref()
    }
}

impl CssRule for CssStyleRule {}

impl From<web_sys::CssStyleRule> for CssStyleRule {
    fn from(inner: web_sys::CssStyleRule) -> Self {
        CssStyleRule { inner }
    }
}

impl AsRef<web_sys::CssStyleRule> for CssStyleRule {
    fn as_ref(&self) -> &web_sys::CssStyleRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssStyleRule, web_sys::CssStyleRule);
