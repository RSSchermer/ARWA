use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::connection::{connection_event_target_seal, ConnectionEventTarget};
use crate::cssom::{styled_seal, StyleSheets, Styled};
use crate::dom::{
    document_seal::Seal, impl_node_traits, impl_parent_node_for_document, impl_try_from_node,
    Attribute, CDATAError, CDataSection, Comment, DocumentType, DynamicElement,
    FullscreenChangeEvent, FullscreenErrorEvent, GenericDocumentFragment, LiveRange, Name,
    OwnedNode, PointerLockChangeEvent, PointerLockErrorEvent, ProcessingInstruction,
    ProcessingInstructionError, QualifiedName, ReadyStateChangeEvent, SelectionChangeEvent, Text,
    TextDirectionality, VisibilityChangeEvent,
};
use crate::dom_exception_wrapper;
use crate::event::typed_event_iterator;
use crate::ui::{ui_event_target_seal, UiEventTarget};
use crate::window::Window;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DocumentReadyState {
    Loading,
    Interactive,
    Complete,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VisibilityState {
    Visible,
    Hidden,
}

pub(crate) mod document_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_document(&self) -> &web_sys::Document;
    }
}

pub trait Document: document_seal::Seal + Sized {
    fn character_set(&self) -> String {
        self.as_web_sys_document().character_set()
    }

    fn text_directionality(&self) -> TextDirectionality {
        match &*self.as_web_sys_document().dir().to_lowercase() {
            "ltr" => TextDirectionality::LeftToRight,
            "rtl" => TextDirectionality::RightToLeft,
            _ => TextDirectionality::Auto,
        }
    }

    fn set_text_directionality(&self, dir: TextDirectionality) {
        let text_directionality = match dir {
            TextDirectionality::Auto => "auto",
            TextDirectionality::LeftToRight => "ltr",
            TextDirectionality::RightToLeft => "rtl",
        };

        self.as_web_sys_document().set_dir(text_directionality);
    }

    fn doctype(&self) -> Option<DocumentType> {
        self.as_web_sys_document().doctype().map(|d| d.into())
    }

    fn document_element(&self) -> Option<DynamicElement> {
        self.as_web_sys_document()
            .document_element()
            .map(|e| e.into())
    }

    fn document_uri(&self) -> String {
        // No indication in the WHATWG spec that this can actually fail, unwrap for now.
        self.as_web_sys_document().document_uri().unwrap_throw()
    }

    fn has_focus(&self) -> bool {
        // No indication in the spec that this can actually fail, unwrap for now.
        self.as_web_sys_document().has_focus().unwrap_throw()
    }

    fn hidden(&self) -> bool {
        self.as_web_sys_document().hidden()
    }

    fn last_modified(&self) -> String {
        self.as_web_sys_document().last_modified()
    }

    fn default_view(&self) -> Option<Window> {
        self.as_web_sys_document().default_view().map(|w| w.into())
    }

    // Note: ignoring `location` here, seems more consistent to force access to the current browsing
    // contexts Location object to go through `default_view`.

    fn referrer(&self) -> String {
        self.as_web_sys_document().referrer()
    }

    fn ready_state(&self) -> DocumentReadyState {
        match self.as_web_sys_document().ready_state().as_ref() {
            "loading" => DocumentReadyState::Loading,
            "interactive" => DocumentReadyState::Interactive,
            "complete" => DocumentReadyState::Complete,
            _ => unreachable!(),
        }
    }

    fn visibility_state(&self) -> VisibilityState {
        match self.as_web_sys_document().ready_state().as_ref() {
            "visible" => VisibilityState::Visible,
            "hidden" => VisibilityState::Hidden,
            _ => unreachable!(),
        }
    }

    fn title(&self) -> String {
        self.as_web_sys_document().title()
    }

    fn set_title(&self, title: &str) {
        self.as_web_sys_document().set_title(title);
    }

    fn fullscreen_enabled(&self) -> bool {
        self.as_web_sys_document().fullscreen_enabled()
    }

    fn fullscreen_element(&self) -> Option<DynamicElement> {
        self.as_web_sys_document()
            .fullscreen_element()
            .map(|e| e.into())
    }

    fn exit_fullscreen(&self) {
        self.as_web_sys_document().exit_fullscreen();
    }

    fn pointer_lock_element(&self) -> Option<DynamicElement> {
        self.as_web_sys_document()
            .pointer_lock_element()
            .map(|e| e.into())
    }

    fn exit_pointer_lock(&self) {
        self.as_web_sys_document().exit_pointer_lock();
    }

    fn release_capture(&self) {
        self.as_web_sys_document().release_capture()
    }

    fn adopt_node<T>(&self, node: &T)
    where
        T: OwnedNode,
    {
        // Note: the OwnedNode constraint is not implemented for Document or ShadowRoot which should
        // make this operation infallible.
        self.as_web_sys_document()
            .adopt_node(node.as_web_sys_node())
            .unwrap_throw();
    }

    // Note: ignoring importNode for now. Prefer an explicit 2 step process where the target node
    // is first duplicated, then adopted.

    fn create_document_fragment(&self) -> GenericDocumentFragment {
        GenericDocumentFragment::from(self.as_web_sys_document().create_document_fragment())
    }

    fn create_attribute(&self, name: &Name) -> Attribute {
        self.as_web_sys_document()
            .create_attribute(name.as_ref())
            .unwrap_throw()
            .into()
    }

    fn create_attribute_namespaced(
        &self,
        qualified_name: &QualifiedName,
        namespace: &str,
    ) -> Attribute {
        self.as_web_sys_document()
            .create_attribute_ns(Some(namespace), qualified_name.as_ref())
            .unwrap_throw()
            .into()
    }

    fn try_create_attribute_namespaced(
        &self,
        qualified_name: &QualifiedName,
        namespace: &str,
    ) -> Result<Attribute, NamespaceError> {
        self.as_web_sys_document()
            .create_attribute_ns(Some(namespace), qualified_name.as_ref())
            .map(|a| a.into())
            .map_err(|err| NamespaceError::new(err.unchecked_into()))
    }

    fn create_element(&self, name: &Name) -> DynamicElement {
        self.as_web_sys_document()
            .create_element(name.as_ref())
            .unwrap_throw()
            .into()
    }

    fn create_element_namespaced(
        &self,
        qualified_name: &QualifiedName,
        namespace: &str,
    ) -> DynamicElement {
        self.as_web_sys_document()
            .create_element_ns(Some(namespace), qualified_name.as_ref())
            .unwrap_throw()
            .into()
    }

    fn try_create_element_namespaced(
        &self,
        qualified_name: &QualifiedName,
        namespace: &str,
    ) -> Result<DynamicElement, NamespaceError> {
        self.as_web_sys_document()
            .create_element_ns(Some(namespace), qualified_name.as_ref())
            .map(|a| a.into())
            .map_err(|err| NamespaceError::new(err.unchecked_into()))
    }

    #[allow(non_snake_case)]
    fn create_CDATA_section(&self, data: &str) -> CDataSection {
        self.as_web_sys_document()
            .create_cdata_section(data)
            .unwrap_throw()
            .into()
    }

    #[allow(non_snake_case)]
    fn try_create_CDATA_section(&self, data: &str) -> Result<CDataSection, CDATAError> {
        self.as_web_sys_document()
            .create_cdata_section(data)
            .map(|s| s.into())
            .map_err(|err| CDATAError::new(err.unchecked_into()))
    }

    fn create_comment(&self, data: &str) -> Comment {
        self.as_web_sys_document().create_comment(data).into()
    }

    fn create_processing_instruction(&self, target: &str, data: &str) -> ProcessingInstruction {
        self.as_web_sys_document()
            .create_processing_instruction(target, data)
            .unwrap_throw()
            .into()
    }

    fn try_create_processing_instruction(
        &self,
        target: &str,
        data: &str,
    ) -> Result<ProcessingInstruction, ProcessingInstructionError> {
        self.as_web_sys_document()
            .create_processing_instruction(target, data)
            .map(|i| i.into())
            .map_err(|err| ProcessingInstructionError::new(err.unchecked_into()))
    }

    fn create_text(&self, data: &str) -> Text {
        self.as_web_sys_document().create_text_node(data).into()
    }

    fn create_range(&self) -> LiveRange {
        self.as_web_sys_document()
            .create_range()
            .unwrap_throw()
            .into()
    }

    fn element_from_point(&self, x: f32, y: f32) -> Option<DynamicElement> {
        self.as_web_sys_document()
            .element_from_point(x, y)
            .map(|e| e.into())
    }

    fn on_fullscreen_change(&self) -> OnFullscreenChange<Self> {
        OnFullscreenChange::new(self.as_web_sys_document())
    }

    fn on_fullscreen_error(&self) -> OnFullscreenError<Self> {
        OnFullscreenError::new(self.as_web_sys_document())
    }

    fn on_pointer_lock_change(&self) -> OnPointerLockChange<Self> {
        OnPointerLockChange::new(self.as_web_sys_document())
    }

    fn on_pointer_lock_error(&self) -> OnPointerLockError<Self> {
        OnPointerLockError::new(self.as_web_sys_document())
    }

    fn on_ready_state_change(&self) -> OnReadyStateChange<Self> {
        OnReadyStateChange::new(self.as_web_sys_document())
    }

    fn on_selection_change(&self) -> OnSelectionChange<Self> {
        OnSelectionChange::new(self.as_web_sys_document())
    }

    fn on_visibility_change(&self) -> OnVisibilityChange<Self> {
        OnVisibilityChange::new(self.as_web_sys_document())
    }

    // TODO: `implementation`

    // TODO: picture-in-picture

    // TODO: has_storage_access and request_storage_access seem to be missing from web_sys.

    // TODO: XPath?

    // TODO: tree walker?

    // TODO: node iterator?
}

dom_exception_wrapper!(NamespaceError);

pub struct DynamicDocument {
    inner: web_sys::Document,
}

impl document_seal::Seal for DynamicDocument {
    fn as_web_sys_document(&self) -> &web_sys::Document {
        &self.inner
    }
}

impl Document for DynamicDocument {}

impl connection_event_target_seal::Seal for DynamicDocument {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.as_web_sys_document().as_ref()
    }
}

impl ConnectionEventTarget for DynamicDocument {}

impl ui_event_target_seal::Seal for DynamicDocument {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl UiEventTarget for DynamicDocument {}

impl styled_seal::Seal for DynamicDocument {}
impl Styled for DynamicDocument {
    fn style_sheets(&self) -> StyleSheets {
        StyleSheets::new(self.inner.style_sheets())
    }
}

impl From<web_sys::Document> for DynamicDocument {
    fn from(inner: web_sys::Document) -> Self {
        DynamicDocument { inner }
    }
}

impl From<DynamicDocument> for web_sys::Document {
    fn from(value: DynamicDocument) -> Self {
        value.inner
    }
}

impl AsRef<web_sys::Document> for DynamicDocument {
    fn as_ref(&self) -> &web_sys::Document {
        &self.inner
    }
}

impl_node_traits!(DynamicDocument);
impl_try_from_node!(DynamicDocument, Document);
impl_parent_node_for_document!(DynamicDocument);

typed_event_iterator!(
    OnFullscreenChange,
    OnFullscreenChangeWithOptions,
    FullscreenChangeEvent,
    "fullscreenchange"
);
typed_event_iterator!(
    OnFullscreenError,
    OnFullscreenErrorWithOptions,
    FullscreenErrorEvent,
    "fullscreenerror"
);
typed_event_iterator!(
    OnPointerLockChange,
    OnPointerLockChangeWithOptions,
    PointerLockChangeEvent,
    "pointerlockchange"
);
typed_event_iterator!(
    OnPointerLockError,
    OnPointerLockErrorWithOptions,
    PointerLockErrorEvent,
    "pointerlockerror"
);
typed_event_iterator!(
    OnReadyStateChange,
    OnReadyStateChangeWithOptions,
    ReadyStateChangeEvent,
    "readystatechange"
);
typed_event_iterator!(
    OnSelectionChange,
    OnSelectionChangeWithOptions,
    SelectionChangeEvent,
    "selectionchange"
);
typed_event_iterator!(
    OnVisibilityChange,
    OnVisibilityChangeWithOptions,
    VisibilityChangeEvent,
    "visibilitychange"
);

macro_rules! impl_document_traits {
    ($document:ident, $web_sys_tpe:ident) => {
        impl $crate::dom::document_seal::Seal for $document {
            fn as_web_sys_document(&self) -> &web_sys::Document {
                self.inner.as_ref()
            }
        }

        impl $crate::dom::Document for $document {}

        impl $crate::connection::connection_event_target_seal::Seal for $document {
            fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
                use crate::dom::document_seal::Seal;

                self.as_web_sys_document().as_ref()
            }
        }

        impl $crate::connection::ConnectionEventTarget for $document {}

        impl $crate::ui::ui_event_target_seal::Seal for $document {
            fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
                use crate::dom::document_seal::Seal;

                self.as_web_sys_document().as_ref()
            }
        }

        impl $crate::ui::UiEventTarget for $document {}

        impl $crate::cssom::styled_seal::Seal for $document {}
        impl $crate::cssom::Styled for $document {
            fn style_sheets(&self) -> $crate::cssom::StyleSheets {
                use crate::dom::document_seal::Seal;

                $crate::cssom::StyleSheets::new(self.as_web_sys_document().style_sheets())
            }
        }

        impl AsRef<web_sys::Document> for $document {
            fn as_ref(&self) -> &web_sys::Document {
                use crate::dom::document_seal::Seal;

                self.as_web_sys_document()
            }
        }

        impl TryFrom<$crate::dom::DynamicDocument> for $document {
            type Error = $crate::InvalidCast<$crate::dom::DynamicDocument, $document>;

            fn try_from(value: $crate::dom::DynamicDocument) -> Result<Self, Self::Error> {
                let value: web_sys::Document = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|e| e.into())
                    .map_err(|e| $crate::InvalidCast::new(e.into()))
            }
        }

        $crate::dom::impl_node_traits!($document);
        $crate::dom::impl_try_from_node!($document, $web_sys_tpe);
        $crate::dom::impl_parent_node_for_document!($document);
    };
    ($tpe:ident) => {
        $crate::dom::impl_document_traits!($tpe, $tpe);
    };
}

pub(crate) use impl_document_traits;
