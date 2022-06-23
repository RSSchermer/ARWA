use wasm_bindgen::{throw_val, JsCast};

use crate::collection::{Collection, Sequence};
use crate::cssom::{CssImportRule, DynamicCssRule, InsertRuleError, RemoveRuleError};
use crate::dom::DynamicNode;
use crate::media_type::MediaType;
use crate::security::SecurityError;
use crate::url::Url;
use crate::{impl_common_wrapper_traits, impl_js_cast};

pub struct CssStyleSheet {
    inner: web_sys::CssStyleSheet,
}

impl CssStyleSheet {
    pub fn is_disabled(&self) -> bool {
        self.inner.disabled()
    }

    pub fn disable(&self) {
        self.inner.set_disabled(true);
    }

    pub fn enable(&self) {
        self.inner.set_disabled(false);
    }

    pub fn title(&self) -> Option<String> {
        self.inner.title()
    }

    pub fn href(&self) -> Option<Url> {
        self.inner
            .href()
            .ok()
            .flatten()
            .and_then(|href| Url::parse(&href).ok())
    }

    pub fn media_type(&self) -> Option<MediaType> {
        MediaType::parse(self.inner.type_().as_ref()).ok()
    }

    pub fn owner_node(&self) -> Option<DynamicNode> {
        self.inner.owner_node().map(|n| n.into())
    }

    pub fn parent_style_sheet(&self) -> Option<CssStyleSheet> {
        // web_sys returns `StyleSheet`, but the specs says this should always be `CssStyleSheet` so
        // we just cast.
        self.inner
            .parent_style_sheet()
            .map(|s| CssStyleSheet::from(s.unchecked_into::<web_sys::CssStyleSheet>()))
    }

    pub fn owner_rule(&self) -> Option<CssImportRule> {
        self.inner
            .owner_rule()
            .map(|r| CssImportRule::from(r.unchecked_into::<web_sys::CssImportRule>()))
    }

    pub fn resolve_rules(&self) -> CssStyleSheetRules {
        let rules = match self.inner.css_rules() {
            Ok(rules) => rules,
            Err(err) => throw_val(err),
        };

        CssStyleSheetRules {
            style_sheet: self.inner.clone(),
            rules,
        }
    }

    pub fn try_resolve_rules(&self) -> Result<CssStyleSheetRules, SecurityError> {
        self.inner
            .css_rules()
            .map(|rules| CssStyleSheetRules {
                style_sheet: self.inner.clone(),
                rules,
            })
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    // TODO: media
    // TODO: replace / replace_sync
}

impl AsRef<web_sys::CssStyleSheet> for CssStyleSheet {
    fn as_ref(&self) -> &web_sys::CssStyleSheet {
        &self.inner
    }
}

impl From<web_sys::CssStyleSheet> for CssStyleSheet {
    fn from(inner: web_sys::CssStyleSheet) -> Self {
        CssStyleSheet { inner }
    }
}

impl_common_wrapper_traits!(CssStyleSheet);
impl_js_cast!(CssStyleSheet);

pub struct CssStyleSheetRules {
    style_sheet: web_sys::CssStyleSheet,
    rules: web_sys::CssRuleList,
}

impl CssStyleSheetRules {
    pub fn insert(&self, index: u32, rule: &str) {
        if let Err(err) = self.style_sheet.insert_rule_with_index(rule, index) {
            throw_val(err)
        }
    }

    pub fn try_insert(&self, index: u32, rule: &str) -> Result<u32, InsertRuleError> {
        self.style_sheet
            .insert_rule_with_index(rule, index)
            .map_err(|err| InsertRuleError::new(err.unchecked_into()))
    }

    pub fn remove(&self, index: u32) {
        if let Err(err) = self.style_sheet.delete_rule(index) {
            throw_val(err)
        }
    }

    pub fn try_remove(&self, index: u32) -> Result<(), RemoveRuleError> {
        self.style_sheet
            .delete_rule(index)
            .map_err(|err| RemoveRuleError::new(err.unchecked_into()))
    }
}

impl Collection for CssStyleSheetRules {
    fn len(&self) -> u32 {
        self.rules.length()
    }
}

impl Sequence for CssStyleSheetRules {
    type Item = DynamicCssRule;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.rules.get(index).map(|r| r.into())
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.rules.as_ref())
    }
}
