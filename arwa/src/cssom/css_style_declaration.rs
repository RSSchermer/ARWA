use crate::cssom::DynamicCssRule;
use arwa::console::{Write, Writer};

mod css_style_declaration_read_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_css_style_declaration(&self) -> &web_sys::CssStyleDeclaration;
    }
}

pub trait CssStyleDeclarationRead: css_style_declaration_read_seal::Seal {
    fn len(&self) -> u32 {
        self.as_web_sys_css_style_declaration().length()
    }

    fn parent_rule(&self) -> Option<DynamicCssRule> {
        self.as_web_sys_css_style_declaration()
            .parent_rule()
            .map(|r| r.into())
    }

    fn property_at(&self, index: u32) -> Option<String> {
        self.as_web_sys_css_style_declaration().get(index)
    }

    fn property_value(&self, property_name: &str) -> Option<String> {
        self.as_web_sys_css_style_declaration()
            .get_property_value(property_name)
    }

    fn property_priority(&self, property_name: &str) -> Option<String> {
        self.as_web_sys_css_style_declaration()
            .get_property_priority(property_name)
    }
}

mod css_style_declaration_write_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_css_style_declaration(&self) -> &web_sys::CssStyleDeclaration;
    }
}

pub trait CssStyleDeclarationWrite: css_style_declaration_write_seal::Seal {
    fn set_property(&self, name: &str, value: &str) {
        self.as_web_sys_css_style_declaration()
            .set_property(name, value)
            .unwrap_throw()
    }

    fn set_property_with_priority(&self, name: &str, value: &str, priority: &str) {
        self.as_web_sys_css_style_declaration()
            .set_property_with_priority(name, value, priority)
            .unwrap_throw()
    }

    fn remove_property(&self, property_name: &str) -> String {
        self.as_web_sys_css_style_declaration()
            .remove_property(property_name)
            .unwrap_throw()
    }
}

#[derive(Clone)]
pub struct CssStyleDeclaration {
    inner: web_sys::CssStyleDeclaration,
}

impl css_style_declaration_read_seal::Seal for CssStyleDeclaration {
    fn as_web_sys_css_style_declaration(&self) -> &web_sys::CssStyleDeclaration {
        &self.inner
    }
}

impl CssStyleDeclarationRead for CssStyleDeclaration {}

impl css_style_declaration_write_seal::Seal for CssStyleDeclaration {
    fn as_web_sys_css_style_declaration(&self) -> &web_sys::CssStyleDeclaration {
        &self.inner
    }
}

impl CssStyleDeclarationWrite for CssStyleDeclaration {}

impl From<web_sys::CssStyleDeclaration> for CssStyleDeclaration {
    fn from(inner: web_sys::CssStyleDeclaration) -> Self {
        CssStyleDeclaration { inner }
    }
}

impl AsRef<web_sys::CssStyleDeclaration> for CssStyleDeclaration {
    fn as_ref(&self) -> &web_sys::CssStyleDeclaration {
        &self.inner
    }
}

impl_common_wrapper_traits!(CssStyleDeclaration);

#[derive(Clone)]
pub struct CssReadOnlyStyleDeclaration {
    inner: web_sys::CssStyleDeclaration,
}

impl css_style_declaration_read_seal::Seal for CssReadOnlyStyleDeclaration {
    fn as_web_sys_css_style_declaration(&self) -> &web_sys::CssStyleDeclaration {
        &self.inner
    }
}

impl CssStyleDeclarationRead for CssReadOnlyStyleDeclaration {}

impl From<web_sys::CssStyleDeclaration> for CssReadOnlyStyleDeclaration {
    fn from(inner: web_sys::CssStyleDeclaration) -> Self {
        CssReadOnlyStyleDeclaration { inner }
    }
}

impl AsRef<web_sys::CssStyleDeclaration> for CssReadOnlyStyleDeclaration {
    fn as_ref(&self) -> &web_sys::CssStyleDeclaration {
        &self.inner
    }
}

impl_common_wrapper_traits!(CssReadOnlyStyleDeclaration);
