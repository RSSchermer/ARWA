use crate::cssom::{css_rule_seal, CssRule, DynamicCssRule};

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

    pub fn css_rules(&self) -> CssKeyframeRules {
        CssKeyframeRules {
            keyframes_rule: self.inner.clone(),
            rules: self.inner.css_rules(),
        }
    }
}

impl css_rule_seal::Seal for CssKeyframesRule {
    fn as_web_sys_css_rule(&self) -> &web_sys::CssRule {
        self.inner.as_ref()
    }
}

impl CssRule for CssKeyframesRule {}

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

impl_css_rule_traits!(CssKeyframesRule, web_sys::CssKeyframesRule);

pub struct CssKeyframeRules {
    keyframes_rule: web_sys::CssKeyframesRule,
    rules: web_sys::CssRuleList,
}

impl CssKeyframeRules {
    pub fn get(&self, index: u32) -> Option<CssKeyframeRule> {
        // Experimentation suggests that a KeyframesRule can only contain KeyframeRule rules. You
        // can attempt to add other rule kinds (either in the base stylesheet css code or via
        // appendRule), but those will simply be silently ignored. That means that the rule should
        // always successfully cast to a keyframe-rule.
        self.rules
            .get(index)
            .map(|r| CssKeyFrameRule::new(r.unchecked_into()))
    }

    pub fn find(&self, selector: &str) -> Option<CssKeyframeRule> {
        self.keyframes_rule.find_rule(selector).map(|r| r.into())
    }

    pub fn len(&self) -> u32 {
        self.rules.length()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<CssKeyframeRule> {
        self.get(0)
    }

    pub fn last(&self) -> Option<CssKeyframeRule> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn push(&self, rule: &str) {
        self.keyframes_rule.append_rule(rule);
    }

    pub fn remove(&self, selector: &str) {
        self.keyframes_rule.delete_rule(selector);
    }

    pub fn snapshot(&self) -> CssKeyframeRulesSnapshot {
        CssKeyframeRulesSnapshot::new(js_sys::Array::from(self.rules.as_ref()))
    }
}

unchecked_cast_array_wrapper!(
    CssKeyframeRule,
    web_sys::CssKeyframeRule,
    CssKeyframeRulesSnapshot,
    CssKeyframeRulesSnapshotIter
);
