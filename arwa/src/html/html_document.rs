use crate::html::{
    slot_change_event_target_seal, HtmlBodyElement, HtmlHeadElement, SlotChangeEventTarget,
};
use std::convert::TryFrom;

pub(crate) mod known_element_seal {
    pub trait Seal {}
}

/// Trait implemented by [HtmlElement] types that are specified in the
/// [HTML Standard](https://html.spec.whatwg.org/multipage/#toc-semantics).
pub trait KnownElement: Sized + known_element_seal::Seal {
    /// Creates a new instance of the element that is owned by the given `owner_document`.
    fn create(owner_document: &HtmlDocument) -> Self;
}

macro_rules! impl_known_element {
    ($tpe:ident, $web_sys_tpe:ident, $tag_name:literal) => {
        impl $crate::html::known_element_seal::Seal for $tpe {}

        impl $crate::html::KnownElement for $tpe {
            fn create(document: &$crate::html::HtmlDocument) -> Self {
                let web_sys_document: web_sys::Document = document.as_ref();

                // Tag name is guaranteed to be valid, safe to unwrap.
                let element = web_sys_document.create_element($tag_name).unwrap();

                let typed_element: $web_sys_tpe = element.unchecked_into();

                typed_element.into()
            }
        }
    }
    ($tpe:ident, $tag_name:literal) => {
        $crate::html::impl_known_element!($tpe, web_sys::$tpe, $tag_name);
    }
}

pub(crate) use impl_known_element;

pub struct HtmlDocument {
    inner: web_sys::HtmlDocument,
}

impl HtmlDocument {
    pub fn create_known_element<T: KnownElement>(&self) -> T {
        T::create(self)
    }

    pub fn design_mode_enabled(&self) -> bool {
        match &*self.inner.design_mode() {
            "on" => true,
            _ => false,
        }
    }

    pub fn set_design_mode_enabled(&self, design_mode_enabled: bool) {
        let design_mode = if design_mode_enabled { "on" } else { "off" };

        self.inner.set_design_mode(design_mode);
    }

    pub fn head(&self) -> Option<HtmlHeadElement> {
        self.as_ref().head().map(|h| h.into())
    }

    pub fn body(&self) -> Option<HtmlBodyElement> {
        // Disregard deprecated frameset element
        self.as_ref()
            .body()
            .and_then(|e| e.dyn_into::<web_sys::HtmlBodyElement>().ok())
            .map(|body| body.into())
    }

    // Note: not including `forms`, `links`, `embeds`, `plugins`, `scripts` and `images` for now.
    // These all return live `HtmlCollection`s, which introduces headaches re. iterability.
    // `query_selector_all` covers the most of the functionality, except in that it does not return
    // live collections, which I feel is actually strictly a positive difference.
}

impl slot_change_event_target_seal::Seal for HtmlDocument {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.as_web_sys_html_element().as_ref()
    }
}

impl SlotChangeEventTarget for HtmlDocument {}

impl From<web_sys::HtmlDocument> for HtmlDocument {
    fn from(inner: web_sys::HtmlDocument) -> Self {
        HtmlDocument { inner }
    }
}

impl AsRef<web_sys::HtmlDocument> for HtmlDocument {
    fn as_ref(&self) -> &web_sys::HtmlDocument {
        &self.inner
    }
}

impl_document_traits!(HtmlDocument);
