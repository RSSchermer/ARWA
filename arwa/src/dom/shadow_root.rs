use crate::cssom::{style_context_seal, StyleContext, StyleSheets};
use crate::dom::selector::CompiledSelector;
use crate::dom::{
    document_fragment_seal, parent_node_seal, ChildElements, ChildNode, DocumentFragment,
    DynamicElement, HierarchyRequestError, ParentNode, QuerySelectorAll,
};

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

        let mut init = web_sys::ShadowRootInit::new();

        match mode {
            ShadowRootMode::Open => init.mode(web_sys::ShadowRootMode::Open),
            ShadowRootMode::Closed => init.mode(web_sys::ShadowRootMode::Closed),
        }

        init
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
        self.as_web_sys_element()
            .attach_shadow(options.into_web_sys())
            .unwrap_throw()
            .into()
    }

    fn try_attach_shadow(
        &self,
        options: ShadowRootOptions,
    ) -> Result<ShadowRoot, AttachShadowError> {
        self.as_web_sys_element()
            .attach_shadow(options.into_web_sys())
            .map(|r| r.into())
            .map_err(|err| AttachShadowError::new(err.unchecked_into()))
    }

    fn shadow_root(&self) -> Option<ShadowRoot> {
        self.as_web_sys_element().shadow_root().map(|r| r.into())
    }
}

#[derive(Clone)]
pub struct AttachShadowError {
    inner: web_sys::DomException,
}

impl AttachShadowError {
    fn new(inner: web_sys::DomException) -> Self {
        AttachShadowError { inner }
    }
}

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

impl style_context_seal::Seal for ShadowRoot {}
impl StyleContext for ShadowRoot {
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

impl_document_fragment_traits!(ShadowRoot);

macro_rules! impl_shadow_host_for_element {
    ($tpe:ident) => {
        impl shadow_host_seal::Seal for HtmlArticleElement {
            fn as_web_sys_element(&self) -> &web_sys::Element {
                self.inner.as_ref()
            }
        }

        impl ShadowHost for HtmlArticleElement {}
    }
}

pub(crate) use impl_shadow_host_for_element;