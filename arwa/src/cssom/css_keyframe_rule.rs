use crate::cssom::{css_rule_seal, CssRule, CssStyleDeclaration};

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

impl css_rule_seal::Seal for CssKeyframeRule {
    fn as_web_sys_css_rule(&self) -> &web_sys::CssRule {
        self.inner.as_ref()
    }
}

impl CssRule for CssKeyframeRule {}

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

impl_css_rule_traits!(CssKeyframeRule, web_sys::CssKeyframeRule);
