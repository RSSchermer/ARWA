use delegate::delegate;

use crate::cssom::{impl_css_rule_traits, CssStyleDeclaration};

#[derive(Clone)]
pub struct CssKeyframeRule {
    inner: web_sys::CssKeyframeRule,
}

impl CssKeyframeRule {
    delegate! {
        to self.inner {
            pub fn key_text(&self) -> String;

            pub fn set_key_text(&self, value: &str);
        }
    }

    pub fn style(&self) -> CssStyleDeclaration {
        self.inner.style().into()
    }
}

impl From<web_sys::CssKeyframeRule> for CssKeyframeRule {
    fn from(inner: web_sys::CssKeyframeRule) -> Self {
        CssKeyframeRule { inner }
    }
}

impl AsRef<web_sys::CssKeyframeRule> for CssKeyframeRule {
    fn as_ref(&self) -> &web_sys::CssKeyframeRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssKeyframeRule, CssKeyframeRule);
