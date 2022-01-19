use crate::cssom::{css_rule_seal, CssRule};

#[derive(Clone)]
pub struct CssNamespaceRule {
    inner: web_sys::CssNamespaceRule,
}

impl CssNamespaceRule {
    delegate! {
        to self.inner {
            pub fn namespace_uri(&self) -> String;

            pub fn prefix(&self) -> String;
        }
    }
}

impl css_rule_seal::Seal for CssNamespaceRule {
    fn as_web_sys_css_rule(&self) -> &web_sys::CssRule {
        self.inner.as_ref()
    }
}

impl CssRule for CssNamespaceRule {}

impl From<web_sys::CssNamespaceRule> for CssNamespaceRule {
    fn from(inner: web_sys::CssNamespaceRule) -> Self {
        CssNamespaceRule { inner }
    }
}

impl AsRef<web_sys::CssNamespaceRule> for CssNamespaceRule {
    fn as_ref(&self) -> &web_sys::CssNamespaceRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssNamespaceRule, web_sys::CssNamespaceRule);
