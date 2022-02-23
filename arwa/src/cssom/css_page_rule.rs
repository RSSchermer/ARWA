use crate::cssom::{
    css_grouping_rule_seal, impl_css_rule_traits, CssGroupingRule, CssStyleDeclaration,
};

#[derive(Clone)]
pub struct CssPageRule {
    inner: web_sys::CssPageRule,
}

impl CssPageRule {
    pub fn selector_text(&self) -> String {
        todo!("Missing in web-sys")
    }

    pub fn set_selector_text(&self, _value: &str) {
        todo!("Missing in web-sys")
    }

    pub fn style(&self) -> CssStyleDeclaration {
        self.inner.style().into()
    }
}

impl css_grouping_rule_seal::Seal for CssPageRule {
    fn as_web_sys_css_grouping_rule(&self) -> &web_sys::CssGroupingRule {
        todo!("Missing in web-sys")
    }
}

impl CssGroupingRule for CssPageRule {}

impl From<web_sys::CssPageRule> for CssPageRule {
    fn from(inner: web_sys::CssPageRule) -> Self {
        CssPageRule { inner }
    }
}

impl AsRef<web_sys::CssPageRule> for CssPageRule {
    fn as_ref(&self) -> &web_sys::CssPageRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssPageRule, CssPageRule);
