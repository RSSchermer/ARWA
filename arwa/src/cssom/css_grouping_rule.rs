use crate::cssom::{DynamicCssRule, InsertRuleError};

pub(crate) mod css_grouping_rule_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_css_grouping_rule(&self) -> web_sys::CssGroupingRule;
    }
}

pub trait CssGroupingRule: css_grouping_rule_seal::Seal {
    fn css_rules(&self) -> CssGroupedRules {
        let as_web_sys = self.as_web_sys_css_grouping_rule();

        CssGroupedRules {
            grouping_rule: as_web_sys.clone(),
            rules: as_web_sys.css_rules(),
        }
    }
}

#[derive(Clone)]
pub struct CssGroupedRules {
    grouping_rule: web_sys::CssGroupingRule,
    rules: web_sys::CssRuleList,
}

impl CssGroupedRules {
    pub fn get(&self, index: u32) -> Option<DynamicCssRule> {
        self.rules.get(index).map(|r| r.into())
    }

    pub fn len(&self) -> u32 {
        self.rules.length()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<DynamicCssRule> {
        self.get(0)
    }

    pub fn last(&self) -> Option<DynamicCssRule> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn insert(&self, index: u32, rule: &str) {
        self.grouping_rule
            .insert_rule_with_index(rule, index)
            .unwrap_throw();
    }

    pub fn try_insert(&self, index: u32, rule: &str) -> Result<(), InsertRuleError> {
        self.grouping_rule
            .insert_rule_with_index(rule, index)
            .map_err(|err| InsertRuleError::new(err.unchecked_into()))
    }

    pub fn remove(&self, index: u32) {
        self.grouping_rule.delete_rule(index).unwrap_throw();
    }

    pub fn snapshot(&self) -> CssGroupedRulesSnapshot {
        CssGroupedRulesSnapshot::new(js_sys::Array::from(self.rules.as_ref()))
    }
}

unchecked_cast_array_wrapper!(
    DynamicCssRule,
    web_sys::DynamicCssRule,
    CssGroupedRulesSnapshot,
    CssGroupedRulesSnapshotIter
);
