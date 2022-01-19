use crate::cssom::{css_rule_seal, CssRule, CssStyleSheet, Media};

#[derive(Clone)]
pub struct CssImportRule {
    inner: web_sys::CssImportRule,
}

impl CssImportRule {
    delegate! {
        to self.inner {
            pub fn href(&self) -> String;
        }
    }

    pub fn media(&self) -> Media {
        // Spec does not indicate this is nullable.
        Media::new(self.inner.media().unwrap())
    }

    pub fn stylesheet(&self) -> CssStyleSheet {
        // Spec says import rule always has an associated stylesheet
        self.inner.stylesheet().unwrap().into()
    }
}

impl css_rule_seal::Seal for CssImportRule {
    fn as_web_sys_css_rule(&self) -> &web_sys::CssRule {
        self.inner.as_ref()
    }
}

impl CssRule for CssImportRule {}

impl From<web_sys::CssImportRule> for CssImportRule {
    fn from(inner: web_sys::CssImportRule) -> Self {
        CssImportRule { inner }
    }
}

impl AsRef<web_sys::CssImportRule> for CssImportRule {
    fn as_ref(&self) -> &web_sys::CssImportRule {
        &self.inner
    }
}

impl_css_rule_traits!(CssImportRule, web_sys::CssImportRule);
