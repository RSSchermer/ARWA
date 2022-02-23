use crate::cssom::{
    css_condition_rule_seal, css_grouping_rule_seal, impl_css_rule_traits, CssConditionRule,
    CssGroupingRule,
};

#[derive(Clone)]
pub struct CssSupportsRule {
    inner: web_sys::CssSupportsRule,
}

impl css_grouping_rule_seal::Seal for CssSupportsRule {
    fn as_web_sys_css_grouping_rule(&self) -> &web_sys::CssGroupingRule {
        self.inner.as_ref()
    }
}

impl CssGroupingRule for CssSupportsRule {}

impl css_condition_rule_seal::Seal for CssSupportsRule {
    fn as_web_sys_css_condition_rule(&self) -> &web_sys::CssConditionRule {
        self.inner.as_ref()
    }
}

impl CssConditionRule for CssSupportsRule {}

impl From<web_sys::CssSupportsRule> for CssSupportsRule {
    fn from(inner: web_sys::CssSupportsRule) -> Self {
        CssSupportsRule { inner }
    }
}

impl AsRef<web_sys::CssSupportsRule> for CssSupportsRule {
    fn as_ref(&self) -> &web_sys::CssSupportsRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssSupportsRule, CssSupportsRule);
