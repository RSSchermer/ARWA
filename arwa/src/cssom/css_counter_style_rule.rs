use delegate::delegate;

use crate::cssom::impl_css_rule_traits;

#[derive(Clone)]
pub struct CssCounterStyleRule {
    inner: web_sys::CssCounterStyleRule,
}

impl CssCounterStyleRule {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, value: &str);

            pub fn system(&self) -> String;

            pub fn set_system(&self, value: &str);

            pub fn symbols(&self) -> String;

            pub fn set_symbols(&self, value: &str);

            pub fn additive_symbols(&self) -> String;

            pub fn set_additive_symbols(&self, value: &str);

            pub fn negative(&self) -> String;

            pub fn set_negative(&self, value: &str);

            pub fn prefix(&self) -> String;

            pub fn set_prefix(&self, value: &str);

            pub fn suffix(&self) -> String;

            pub fn set_suffix(&self, value: &str);

            pub fn range(&self) -> String;

            pub fn set_range(&self, value: &str);

            pub fn pad(&self) -> String;

            pub fn set_pad(&self, value: &str);

            pub fn speak_as(&self) -> String;

            pub fn set_speak_as(&self, value: &str);

            pub fn fallback(&self) -> String;

            pub fn set_fallback(&self, value: &str);
        }
    }
}

impl From<web_sys::CssCounterStyleRule> for CssCounterStyleRule {
    fn from(inner: web_sys::CssCounterStyleRule) -> Self {
        CssCounterStyleRule { inner }
    }
}

impl AsRef<web_sys::CssCounterStyleRule> for CssCounterStyleRule {
    fn as_ref(&self) -> &web_sys::CssCounterStyleRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssCounterStyleRule, CssCounterStyleRule);
