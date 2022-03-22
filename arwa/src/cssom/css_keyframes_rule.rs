use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::collection::{Collection, Sequence};
use crate::cssom::{impl_css_rule_traits, CssKeyframeRule};

#[derive(Clone)]
pub struct CssKeyframesRule {
    inner: web_sys::CssKeyframesRule,
}

impl CssKeyframesRule {
    delegate! {
        to self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, value: &str);
        }
    }

    pub fn css_rules(&self) -> CssKeyframesRules {
        CssKeyframesRules {
            keyframes_rule: self.inner.clone(),
            rules: self.inner.css_rules(),
        }
    }
}

impl From<web_sys::CssKeyframesRule> for CssKeyframesRule {
    fn from(inner: web_sys::CssKeyframesRule) -> Self {
        CssKeyframesRule { inner }
    }
}

impl AsRef<web_sys::CssKeyframesRule> for CssKeyframesRule {
    fn as_ref(&self) -> &web_sys::CssKeyframesRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssKeyframesRule, CssKeyframesRule);

pub struct CssKeyframesRules {
    keyframes_rule: web_sys::CssKeyframesRule,
    rules: web_sys::CssRuleList,
}

impl CssKeyframesRules {
    pub fn lookup(&self, selector: &str) -> Option<CssKeyframeRule> {
        self.keyframes_rule.find_rule(selector).map(|r| r.into())
    }

    pub fn push(&self, rule: &str) {
        self.keyframes_rule.append_rule(rule);
    }

    pub fn remove(&self, selector: &str) {
        self.keyframes_rule.delete_rule(selector);
    }
}

impl Collection for CssKeyframesRules {
    fn len(&self) -> u32 {
        self.rules.length()
    }
}

impl Sequence for CssKeyframesRules {
    type Item = CssKeyframeRule;

    fn get(&self, index: u32) -> Option<Self::Item> {
        // Experimentation suggests that a KeyframesRule can only contain KeyframeRule rules. You
        // can attempt to add other rule kinds (either in the base stylesheet css code or via
        // appendRule), but those will simply be silently ignored. That means that the rule should
        // always successfully cast to a keyframe-rule.
        self.rules
            .get(index)
            .map(|r| CssKeyframeRule::from(r.unchecked_into::<web_sys::CssKeyframeRule>()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.rules.as_ref())
    }
}
