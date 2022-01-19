use crate::console::{Write, Writer};
use crate::cssom::{CssImportRule, DynamicCssRule, InsertRuleError, Media};
use crate::dom::DynamicNode;
use crate::DynamicNode;
use delegate::delegate;
use std::convert::TryFrom;

pub(crate) mod style_sheet_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_style_sheet(&self) -> &web_sys::StyleSheet;
    }
}

pub trait StyleSheet: style_sheet_seal::Seal {
    fn is_disabled(&self) -> bool {
        self.as_web_sys_style_sheet().disabled()
    }

    fn disable(&self) {
        self.as_web_sys_style_sheet().set_disabled(true);
    }

    fn enable(&self) {
        self.as_web_sys_style_sheet().set_disabled(false);
    }

    fn title(&self) -> Option<String> {
        self.as_web_sys_style_sheet().title()
    }

    fn href(&self) -> Option<String> {
        self.as_web_sys_style_sheet().href().ok().flatten()
    }

    fn format_type(&self) -> String {
        self.as_web_sys_style_sheet().type_()
    }

    fn owner_node(&self) -> Option<DynamicNode> {
        self.as_web_sys_style_sheet().owner_node().map(|n| n.into())
    }

    fn parent_style_sheet(&self) -> Option<Self>
    where
        Self: Sized,
    {
        self.as_web_sys_style_sheet()
            .parent_style_sheet()
            .map(|s| s.into())
    }

    fn media(&self) -> Media {
        Media::new(self.as_web_sys_style_sheet().media())
    }
}

pub struct CssStyleSheet {
    inner: web_sys::CssStyleSheet,
}

impl CssStyleSheet {
    pub fn owner_rule(&self) -> Option<CssImportRule> {
        self.inner
            .owner_rule()
            .map(|r| CssImportRule::from(r.unchecked_into()))
    }

    pub fn css_rules(&self) -> CssStyleSheetRules {
        CssStyleSheetRules {
            style_sheet: self.inner.clone(),
            rules: self.inner.css_rules(),
        }
    }

    // TODO: replace / replace_sync
}

impl style_sheet_seal::Seal for CssStyleSheet {
    fn as_web_sys_style_sheet(&self) -> &web_sys::StyleSheet {
        self.inner.as_ref()
    }
}

impl StyleSheet for CssStyleSheet {}

impl AsRef<web_sys::CssStyleSheet> for CssStyleSheet {
    fn as_ref(&self) -> &web_sys::StyleSheet {
        &self.inner
    }
}

impl From<web_sys::CssStyleSheet> for CssStyleSheet {
    fn from(inner: web_sys::CssStyleSheet) -> Self {
        CssStyleSheet { inner }
    }
}

impl_common_wrapper_traits!(CssStyleSheet);

pub struct CssStyleSheetRules {
    style_sheet: web_sys::CssStyleSheet,
    rules: web_sys::CssRuleList,
}

impl CssStyleSheetRules {
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
        self.style_sheet
            .insert_rule_with_index(rule, index)
            .unwrap_throw();
    }

    pub fn try_insert(&self, index: u32, rule: &str) -> Result<(), InsertRuleError> {
        self.style_sheet
            .insert_rule_with_index(rule, index)
            .map_err(|err| InsertRuleError::new(err.unchecked_into()))
    }

    pub fn remove(&self, index: u32) {
        self.style_sheet.delete_rule(index).unwrap_throw();
    }

    pub fn snapshot(&self) -> CssStyleSheetRulesSnapshot {
        CssStyleSheetRulesSnapshot::new(js_sys::Array::from(self.rules.as_ref()))
    }
}

unchecked_cast_array_wrapper!(
    DynamicCssRule,
    web_sys::DynamicCssRule,
    CssStyleSheetRulesSnapshot,
    CssStyleSheetRulesSnapshotIter
);
