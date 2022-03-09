use delegate::delegate;
use web_sys::CssStyleDeclaration;

use crate::cssom::impl_css_rule_traits;

#[derive(Clone)]
pub struct CssStyleRule {
    inner: web_sys::CssStyleRule,
}

impl CssStyleRule {
    // Note: these are always valid css selectors (if a stylesheet contains syntactically invalid
    // selectors, these are ignored by the parser). However, we cannot trust this as a valid
    // dom::Selector, because namespaced type selectors are valid in CSS, but invalid when used in
    // dom queries. Could create a separate cssom::Selector type, with internal state that tracks
    // whether it can be cast to a dom::Selector? Note that conversion from dom::Selector ->
    // cssom::Selector would always be valid.

    delegate! {
        target self.inner {
            pub fn selector_text(&self) -> String;

            pub fn set_selector_text(&self, value: &str);
        }
    }

    pub fn style(&self) -> CssStyleDeclaration {
        self.inner.style().into()
    }
}

impl From<web_sys::CssStyleRule> for CssStyleRule {
    fn from(inner: web_sys::CssStyleRule) -> Self {
        CssStyleRule { inner }
    }
}

impl AsRef<web_sys::CssStyleRule> for CssStyleRule {
    fn as_ref(&self) -> &web_sys::CssStyleRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssStyleRule, CssStyleRule);
