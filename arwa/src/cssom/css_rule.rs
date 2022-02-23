use crate::cssom::{css_rule_seal::Seal, CssStyleSheet};
use crate::impl_common_wrapper_traits;

pub(crate) mod css_rule_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_css_rule(&self) -> &web_sys::CssRule;
    }
}

pub trait CssRule: css_rule_seal::Seal {
    fn css_text(&self) -> String {
        self.as_web_sys_css_rule().css_text()
    }

    // Note: setting `css_text` is now specced to do nothing at all. Omitting.

    fn parent_rule(&self) -> Option<DynamicCssRule> {
        self.as_web_sys_css_rule().parent_rule().map(|r| r.into())
    }

    fn parent_style_sheet(&self) -> Option<CssStyleSheet> {
        self.as_web_sys_css_rule()
            .parent_style_sheet()
            .map(|r| r.into())
    }
}

#[derive(Clone)]
pub struct DynamicCssRule {
    inner: web_sys::CssRule,
}

impl css_rule_seal::Seal for DynamicCssRule {
    fn as_web_sys_css_rule(&self) -> &web_sys::CssRule {
        &self.inner
    }
}

impl CssRule for DynamicCssRule {}

impl From<web_sys::CssRule> for DynamicCssRule {
    fn from(inner: web_sys::CssRule) -> Self {
        DynamicCssRule { inner }
    }
}

impl From<DynamicCssRule> for web_sys::CssRule {
    fn from(rule: DynamicCssRule) -> Self {
        rule.inner
    }
}

impl AsRef<web_sys::CssRule> for DynamicCssRule {
    fn as_ref(&self) -> &web_sys::CssRule {
        self.as_web_sys_css_rule()
    }
}

impl_common_wrapper_traits!(DynamicCssRule);

macro_rules! impl_css_rule_traits {
    ($rule:ident, $web_sys_tpe:ident) => {
        impl $crate::cssom::css_rule_seal::Seal for $rule {
            fn as_web_sys_css_rule(&self) -> &web_sys::CssRule {
                &self.inner
            }
        }

        impl $crate::cssom::CssRule for $rule {}

        impl AsRef<web_sys::CssRule> for $rule {
            fn as_ref(&self) -> &web_sys::CssRule {
                use crate::cssom::css_rule_seal::Seal;

                self.as_web_sys_css_rule()
            }
        }

        impl std::convert::TryFrom<$crate::cssom::DynamicCssRule> for $rule {
            type Error = $crate::InvalidCast<$crate::cssom::DynamicCssRule, $rule>;

            fn try_from(value: $crate::cssom::DynamicCssRule) -> Result<Self, Self::Error> {
                use wasm_bindgen::JsCast;

                let value: web_sys::CssRule = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|e| e.into())
                    .map_err(|e| $crate::InvalidCast::new(e.into()))
            }
        }

        $crate::impl_common_wrapper_traits!($rule);
    };
}

pub(crate) use impl_css_rule_traits;
