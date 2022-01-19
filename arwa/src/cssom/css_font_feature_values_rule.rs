use crate::cssom::{css_rule_seal, CssRule};

#[derive(Clone)]
pub struct CssFontFeatureValuesRule {
    inner: web_sys::CssFontFeatureValuesRule,
}

impl CssFontFeatureValuesRule {
    delegate! {
        to self.inner {
            pub fn font_family(&self) -> String;

            pub fn set_font_family(&self, value: &str);

            pub fn value_text(&self) -> String;

            pub fn set_value_text(&self, value: &str);
        }
    }
}

impl css_rule_seal::Seal for CssFontFeatureValuesRule {
    fn as_web_sys_css_rule(&self) -> &web_sys::CssRule {
        self.inner.as_ref()
    }
}

impl CssRule for CssFontFeatureValuesRule {}

impl From<web_sys::CssFontFeatureValuesRule> for CssFontFeatureValuesRule {
    fn from(inner: web_sys::CssFontFeatureValuesRule) -> Self {
        CssFontFeatureValuesRule { inner }
    }
}

impl AsRef<web_sys::CssFontFeatureValuesRule> for CssFontFeatureValuesRule {
    fn as_ref(&self) -> &web_sys::CssFontFeatureValuesRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssFontFeatureValuesRule, web_sys::CssFontFeatureValuesRule);
