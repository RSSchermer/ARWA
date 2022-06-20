use wasm_bindgen::{throw_val, JsCast};

use crate::cssom::{styled_seal, StyleSheets, Styled};
use crate::dom::{impl_fragment_traits, DynamicElement};
use crate::dom_exception_wrapper;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShadowRootMode {
    Open,
    Closed,
}

impl Default for ShadowRootMode {
    fn default() -> Self {
        ShadowRootMode::Open
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ShadowRootOptions {
    pub mode: ShadowRootMode,
    pub delegates_focus: bool,
}

impl ShadowRootOptions {
    fn into_web_sys(self) -> web_sys::ShadowRootInit {
        let ShadowRootOptions {
            mode,
            delegates_focus,
        } = self;

        let mode = match mode {
            ShadowRootMode::Open => web_sys::ShadowRootMode::Open,
            ShadowRootMode::Closed => web_sys::ShadowRootMode::Closed,
        };

        if delegates_focus {
            todo!("Missing from web-sys")
        }

        web_sys::ShadowRootInit::new(mode)
    }
}

impl Default for ShadowRootOptions {
    fn default() -> Self {
        ShadowRootOptions {
            mode: ShadowRootMode::default(),
            delegates_focus: false,
        }
    }
}

pub(crate) mod shadow_host_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_element(&self) -> &web_sys::Element;
    }
}

pub trait ShadowHost: shadow_host_seal::Seal {
    fn attach_shadow(&self, options: ShadowRootOptions) -> ShadowRoot {
        match self
            .as_web_sys_element()
            .attach_shadow(&options.into_web_sys())
        {
            Ok(shadow) => shadow.into(),
            Err(err) => throw_val(err),
        }
    }

    fn try_attach_shadow(
        &self,
        options: ShadowRootOptions,
    ) -> Result<ShadowRoot, AttachShadowError> {
        self.as_web_sys_element()
            .attach_shadow(&options.into_web_sys())
            .map(|r| r.into())
            .map_err(|err| AttachShadowError::new(err.unchecked_into()))
    }

    fn shadow_root(&self) -> Option<ShadowRoot> {
        self.as_web_sys_element().shadow_root().map(|r| r.into())
    }
}

dom_exception_wrapper!(AttachShadowError);

#[derive(Clone)]
pub struct ShadowRoot {
    inner: web_sys::ShadowRoot,
}

impl ShadowRoot {
    pub fn host(&self) -> DynamicElement {
        self.inner.host().into()
    }

    pub fn mode(&self) -> ShadowRootMode {
        match self.inner.mode() {
            web_sys::ShadowRootMode::Open => ShadowRootMode::Open,
            web_sys::ShadowRootMode::Closed => ShadowRootMode::Closed,
            _ => unreachable!(),
        }
    }

    pub fn active_element(&self) -> Option<DynamicElement> {
        self.inner.active_element().map(|e| e.into())
    }

    pub fn fullscreen_element(&self) -> Option<DynamicElement> {
        self.inner.fullscreen_element().map(|e| e.into())
    }

    pub fn pointer_lock_element(&self) -> Option<DynamicElement> {
        self.inner.pointer_lock_element().map(|e| e.into())
    }
}

impl styled_seal::Seal for ShadowRoot {}
impl Styled for ShadowRoot {
    fn style_sheets(&self) -> StyleSheets {
        StyleSheets::new(self.inner.style_sheets())
    }
}

impl From<web_sys::ShadowRoot> for ShadowRoot {
    fn from(inner: web_sys::ShadowRoot) -> Self {
        ShadowRoot { inner }
    }
}

impl AsRef<web_sys::ShadowRoot> for ShadowRoot {
    fn as_ref(&self) -> &web_sys::ShadowRoot {
        &self.inner
    }
}

impl_fragment_traits!(ShadowRoot);

macro_rules! impl_shadow_host_for_element {
    ($tpe:ident) => {
        impl $crate::dom::shadow_host_seal::Seal for $tpe {
            fn as_web_sys_element(&self) -> &web_sys::Element {
                self.inner.as_ref()
            }
        }

        impl $crate::dom::ShadowHost for $tpe {}
    };
}

pub(crate) use impl_shadow_host_for_element;
