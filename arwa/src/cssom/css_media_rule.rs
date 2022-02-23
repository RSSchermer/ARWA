use crate::cssom::{
    css_condition_rule_seal, css_grouping_rule_seal, impl_css_rule_traits, CssConditionRule,
    CssGroupingRule,
};

#[derive(Clone)]
pub struct CssMediaRule {
    inner: web_sys::CssMediaRule,
}

impl CssMediaRule {
    // TODO: media
}

impl css_grouping_rule_seal::Seal for CssMediaRule {
    fn as_web_sys_css_grouping_rule(&self) -> &web_sys::CssGroupingRule {
        self.inner.as_ref()
    }
}

impl CssGroupingRule for CssMediaRule {}

impl css_condition_rule_seal::Seal for CssMediaRule {
    fn as_web_sys_css_condition_rule(&self) -> &web_sys::CssConditionRule {
        self.inner.as_ref()
    }
}

impl CssConditionRule for CssMediaRule {}

impl From<web_sys::CssMediaRule> for CssMediaRule {
    fn from(inner: web_sys::CssMediaRule) -> Self {
        CssMediaRule { inner }
    }
}

impl AsRef<web_sys::CssMediaRule> for CssMediaRule {
    fn as_ref(&self) -> &web_sys::CssMediaRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssMediaRule, CssMediaRule);
