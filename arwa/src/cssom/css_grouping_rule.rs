use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::collection::{Collection, Sequence};
use crate::cssom::{DynamicCssRule, InsertRuleError, RemoveRuleError};

pub(crate) mod css_grouping_rule_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_css_grouping_rule(&self) -> &web_sys::CssGroupingRule;
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
    pub fn insert(&self, index: u32, rule: &str) -> u32 {
        self.grouping_rule
            .insert_rule_with_index(rule, index)
            .unwrap_throw()
    }

    pub fn try_insert(&self, index: u32, rule: &str) -> Result<u32, InsertRuleError> {
        self.grouping_rule
            .insert_rule_with_index(rule, index)
            .map_err(|err| InsertRuleError::new(err.unchecked_into()))
    }

    pub fn remove(&self, index: u32) {
        self.grouping_rule.delete_rule(index).unwrap_throw();
    }

    pub fn try_remove(&self, index: u32) -> Result<(), RemoveRuleError> {
        self.grouping_rule
            .delete_rule(index)
            .map_err(|err| RemoveRuleError::new(err.unchecked_into()))
    }
}

impl Collection for CssGroupedRules {
    fn len(&self) -> u32 {
        self.rules.length()
    }
}

impl Sequence for CssGroupedRules {
    type Item = DynamicCssRule;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.rules.get(index).map(|r| r.into())
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.rules.as_ref())
    }
}
