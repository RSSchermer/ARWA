use delegate::delegate;
use web_sys::CssStyleDeclaration;

use crate::cssom::impl_css_rule_traits;

#[derive(Clone)]
pub struct CssStyleRule {
    inner: web_sys::CssStyleRule,
}

impl CssStyleRule {
    delegate! {
        target self.inner {
            pub fn selector_text(&self) -> String;

            pub fn set_selector_text(&self, value: &str);
        }
    }

    pub fn style(&self) -> CssStyleDeclaration {
        self.inner.style().into()
    }
}

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

impl_css_rule_traits!(CssStyleRule, CssStyleRule);
