use delegate::delegate;

use crate::cssom::impl_css_rule_traits;

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

impl_css_rule_traits!(CssNamespaceRule, CssNamespaceRule);
