use wasm_bindgen::{throw_val, JsCast, UnwrapThrowExt};

use crate::connection::{connection_event_target_seal, ConnectionEventTarget};
use crate::cssom::{styled_seal, StyleSheets, Styled};
use crate::dom::{document_seal::Seal, impl_node_traits, impl_parent_node, impl_try_from_node, impl_try_from_parent_node, Attribute,  Comment, DocumentType, DynamicElement, FullscreenChangeEvent, FullscreenErrorEvent, DocumentFragment, LiveRange, Name, PointerLockChangeEvent, PointerLockErrorEvent,   QualifiedName, ReadyStateChangeEvent, SelectionChangeEvent, Text, TextDirectionality, VisibilityChangeEvent, OwnedNode};
use crate::dom_exception_wrapper;
use crate::event::typed_event_iterator;
use crate::ui::{ui_event_target_seal, UiEventTarget};
use crate::url::Url;
use crate::window::Window;

/// Enumerates the ready states a [Document] can be in.
///
/// See [Document::ready_state].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DocumentReadyState {
    /// The document is still loading.
    Loading,

    /// The document has finished loading and the document has been parsed, but sub-resources such
    /// as scripts, images, stylesheets and frames are still loading.
    Interactive,

    /// The document and all sub-resources have finished loading.
    Complete,
}

/// Enumerates the visibility states a [Document] can be in.
///
/// See [Document::visibility_state].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VisibilityState {
    /// The document may be at least partially visible.
    ///
    /// This typically means that the document window is in a foreground tab that is not minimized.
    Visible,

    /// The document is not visible to the user.
    Hidden,
}

pub(crate) mod document_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_document(&self) -> &web_sys::Document;
    }
}

/// Implemented by document node types.
///
/// A document acts as a top level container for a DOM tree. Though it is possible to create DOM
/// trees that are not part of a document hierarchy (e.g. disconnected nodes, or [DocumentFragment]
/// nodes), such trees will not be displayed until they are connected to a document, be it the
/// window associated document (see [Window::document]), or a document embedded in the window
/// associated document.
///
/// All node types except [Document] node types implement [OwnedNode] and always have an associated
/// owner document (even if they were not created through an operation on a document; in this case
/// they are owned by the document associated with the current global scope, typically the window
/// associated document). Nodes that belong to another document should first be adopted with
/// [Document::adopt_node] before they can be connected to the target document. This changes the
/// owner document for that node, and for its entire sub-tree, to the target document.
///
/// Documents implement the [ParentNode] trait, but may only contain a single [Element] type node.
/// This element node is referred to as the "document element" (see [Document::document_element])
/// and acts as the root node for the document's node tree. If a document already has a document
/// element, then operations that would result in additional element type nodes becoming child nodes
/// of the document will result in an error.
pub trait Document: document_seal::Seal + Sized {
    /// Returns a string describing the character encoding the document is being rendered with.
    fn character_set(&self) -> String {
        self.as_web_sys_document().character_set()
    }

    /// The fallback text direction the document is being rendered with.
    ///
    /// CSS stylesheets or inline style may override this for both the entire document and
    /// individual elements.
    fn text_directionality(&self) -> TextDirectionality {
        match &*self.as_web_sys_document().dir().to_lowercase() {
            "ltr" => TextDirectionality::LeftToRight,
            "rtl" => TextDirectionality::RightToLeft,
            _ => TextDirectionality::Auto,
        }
    }

    /// Sets the documents fallback [text_directionality] to the given direction.
    fn set_text_directionality(&self, dir: TextDirectionality) {
        let text_directionality = match dir {
            TextDirectionality::Auto => "auto",
            TextDirectionality::LeftToRight => "ltr",
            TextDirectionality::RightToLeft => "rtl",
        };

        self.as_web_sys_document().set_dir(text_directionality);
    }

    /// Returns the [DocumentType] node associated with this document, or `None` if the document
    /// is not associated with a [DocumentType] node.
    fn doctype(&self) -> Option<DocumentType> {
        self.as_web_sys_document().doctype().map(|d| d.into())
    }

    /// Returns the element that acts as the root node for this document's document tree, or `None`
    /// if no elements are connected to the document.
    fn document_element(&self) -> Option<DynamicElement> {
        self.as_web_sys_document()
            .document_element()
            .map(|e| e.into())
    }

    /// Returns the document's URL.
    ///
    /// Note that a document always has a URL, though the URL may be the `about:blank` fallback URL.
    fn url(&self) -> Url {
        // Some experimentation in Chromium and Firefox suggests all document seem to have a valid
        // URL, defaults to `about:blank`.
        Url::parse(
            self.as_web_sys_document()
                .document_uri()
                .unwrap_throw()
                .as_ref(),
        )
        .unwrap_throw()
    }

    /// Returns `true` if the document or any element in the document has focus, `false` otherwise.
    fn has_focus(&self) -> bool {
        // No indication in the spec that this can actually fail, unwrap for now.
        self.as_web_sys_document().has_focus().unwrap_throw()
    }

    // Note: ignoring `hidden`, seems redundant with `visiblity_state`.

    // TODO: last_modified as a Date?
    // fn last_modified(&self) -> String {
    //     self.as_web_sys_document().last_modified()
    // }

    /// Returns the [Window] with which this document is associated, or `None` if this document is
    /// not associated with a [Window].
    fn default_view(&self) -> Option<Window> {
        self.as_web_sys_document().default_view().map(|w| w.into())
    }

    // Note: ignoring `location` here, seems more consistent to force access to the current browsing
    // context's `Location` object to go through `default_view`.

    /// The URL of the document from which the user navigated to this document, or `None` if it was
    /// blocked or there is no such document.
    ///
    /// The [noreferrer](https://html.spec.whatwg.org/multipage/links.html#link-type-noreferrer)
    /// link type may be used to block referrer information.
    ///
    /// For embedded documents the referrer is typically the embedding document.
    fn referrer(&self) -> Option<Url> {
        let referrer = self.as_web_sys_document().referrer();

        if referrer.is_empty() {
            return None;
        }

        Url::parse(&referrer).ok()
    }

    /// Returns the current ready state of the document.
    ///
    /// Will have one of the following values:
    ///
    /// - [Loading](DocumentReadyState::Loading): the document is still loading.
    /// - [Interactive](DocumentReadyState::Interactive): the document has finished loading and the
    ///   document has been parsed, but sub-resources such as scripts, images, stylesheets and
    ///   frames are still loading.
    /// - [Complete](DocumentReadyState::Complete): the document and all sub-resources have finished
    ///   loading.
    ///
    /// The document emits a [ReadyStateChangeEvent] whenever the ready state changes, see
    /// [on_ready_state_change].
    fn ready_state(&self) -> DocumentReadyState {
        match self.as_web_sys_document().ready_state().as_ref() {
            "loading" => DocumentReadyState::Loading,
            "interactive" => DocumentReadyState::Interactive,
            "complete" => DocumentReadyState::Complete,
            _ => unreachable!(),
        }
    }

    /// Returns the current visibility state of the document.
    ///
    /// Will have one of the following values:
    ///
    /// - [Visible](VisibilityState::Visible): the document may be at least partially visible. This
    ///   typically means that the document window is in a foreground tab that is not minimized.
    /// - [Hidden](VisibilityState::Hidden): the document is not visible to the user.
    ///
    /// The document emits a [VisibilityChangeEvent] whenever the visibility state changes, see
    /// [on_visibility_change].
    fn visibility_state(&self) -> VisibilityState {
        match self.as_web_sys_document().ready_state().as_ref() {
            "visible" => VisibilityState::Visible,
            "hidden" => VisibilityState::Hidden,
            _ => unreachable!(),
        }
    }

    /// Returns the title of the document.
    ///
    /// May be displayed in e.g. the title bar of the window or tab.
    fn title(&self) -> String {
        self.as_web_sys_document().title()
    }

    /// Sets the [title] of the document to the given string.
    ///
    /// This may also affect the DOM of the document (e.g. the text content of a `<title>` element
    /// in an HTML document).
    fn set_title(&self, title: &str) {
        self.as_web_sys_document().set_title(title);
    }

    /// Returns `true` if fullscreen mode is available, `false` otherwise.
    ///
    /// Fullscreen mode is supported if there is no previously-established user preference, security
    /// risk, or platform limitation. Fullscreen mode can be requested for an element by calling
    /// [Element::request_fullscreen].
    ///
    /// If [fullscreen_enabled] returns `true`, then the future returned by
    /// [Element::request_fullscreen] can complete successfully, otherwise it will always complete
    /// with an error.
    fn fullscreen_enabled(&self) -> bool {
        self.as_web_sys_document().fullscreen_enabled()
    }

    /// Returns the currently fullscreened element, or `None` if no element owned by this document
    /// is in fullscreen mode.
    ///
    /// See [Element::request_fullscreen] for details on entering fullscreen mode, and
    /// [exit_fullscreen] for details on exiting fullscreen mode.
    fn fullscreen_element(&self) -> Option<DynamicElement> {
        self.as_web_sys_document()
            .fullscreen_element()
            .map(|e| e.into())
    }

    /// Exits fullscreen mode.
    ///
    /// Does nothing if no element owned by this document is in fullscreen mode.
    ///
    /// See [Element::request_fullscreen] for details on entering fullscreen mode, and
    /// [fullscreen_element] for obtaining a reference to the currently fullscreened element (if
    /// any).
    fn exit_fullscreen(&self) {
        self.as_web_sys_document().exit_fullscreen();
    }

    /// Returns the element that currently has pointer lock, or `None` if no element currently has
    /// pointer lock.
    ///
    /// See [Element::request_pointer_lock] for details entering pointer lock, and see
    /// [exit_pointer_lock] for details on exiting pointer lock.
    fn pointer_lock_element(&self) -> Option<DynamicElement> {
        self.as_web_sys_document()
            .pointer_lock_element()
            .map(|e| e.into())
    }

    /// Asynchronously releases the current pointer lock.
    ///
    /// Does nothing if no element currently has pointer lock.
    ///
    /// The document will emit a [PointerLockChangeEvent] when pointer lock is exited successfully
    /// and a [PointerLockErrorEvent] when it failed to exit pointer lock; see
    /// [on_pointer_lock_change] and [on_pointer_lock_error] respectively for details on obtaining
    /// async iterators over these events.
    ///
    /// See [Element::request_pointer_lock] for details entering pointer lock, and see
    /// [pointer_lock_element] for obtaining a reference to the element that currently has pointer
    /// lock (if any).
    fn exit_pointer_lock(&self) {
        self.as_web_sys_document().exit_pointer_lock();
    }

    /// Adopts the given `node` and all nodes in its sub-tree so that this document becomes their
    /// owner document.
    ///
    /// Does nothing if this document is already the `node`'s owner document. Does nothing if the
    /// `node` is a [DocumentFragment] node that is associated with a
    /// [host](https://dom.spec.whatwg.org/#concept-documentfragment-host) (this occurs when a
    /// document fragment is set as the content of an [HtmlTemplateElement]; in this case, adopt
    /// the template element instead).
    fn adopt_node<T>(&self, node: &T)
    where
        T: OwnedNode,
    {
        // Note: the OwnedNode constraint is not implemented for Document or ShadowRoot which should
        // make this operation infallible.
        // Note 2: I really don't like the behavior around document fragments associated with a
        // "host". As far as I know there's not even a way to determine if a document fragment
        // currently has a host, otherwise document fragments could be handled by a separate
        // adopt_fragment function that may error.
        self.as_web_sys_document()
            .adopt_node(node.as_web_sys_node())
            .unwrap_throw();
    }

    // Note: ignoring importNode for now. Prefer an explicit 2 step process where the target node
    // is first duplicated, then adopted.

    /// Creates a new empty [GenericDocumentFragment].
    fn create_document_fragment(&self) -> DocumentFragment {
        DocumentFragment::from(self.as_web_sys_document().create_document_fragment())
    }

    /// Creates a new attribute node with the given `name`.
    ///
    /// See [Name] for details on valid names.
    ///
    /// # Example
    ///
    /// ```
    /// # let document = arwa::window::window().document();
    /// use arwa::dom::{name, Document};
    ///
    /// let attribute = document.create_attribute(&name!("my-attribute"));
    /// ```
    fn create_attribute(&self, name: &Name) -> Attribute {
        self.as_web_sys_document()
            .create_attribute(name.as_ref())
            .unwrap_throw()
            .into()
    }

    /// Creates a new attribute node with the given `qualified_name` and `namespace`.
    ///
    /// See [QualifiedName] for details on valid qualified names. Note that though namespace is
    /// generally expected to be a [URI](https://datatracker.ietf.org/doc/html/rfc2396), any string
    /// is accepted.
    ///
    /// # Panics
    ///
    /// - Panics if the [QualifiedName::prefix] of `qualified_name` is not `None` and namespace if
    ///   the empty string.
    /// - Panics if the [QualifiedName::prefix] of `qualified_name` is `xml` and
    ///   `namespace` is not the [XML namespace](https://infra.spec.whatwg.org/#xml-namespace).
    /// - Panics if the [QualifiedName::prefix] of `qualified_name` is `xmlns` and
    ///   `namespace` is not the [XMLNS namespace](https://infra.spec.whatwg.org/#xmlns-namespace).
    /// - Panics if `namespace` is the [XMLNS namespace](https://infra.spec.whatwg.org/#xmlns-namespace)
    ///   and `qualified_name` is not either a prefix-less qualified name with a
    ///   [QualifiedName::local_name] of `xmlns`, or a qualified name with a [QualifiedName::prefix]
    ///   of `xmlns`.
    ///
    /// See [try_create_attribute_namespaced] for an alternative that returns ah error instead of
    /// panicking.
    fn create_attribute_namespaced(
        &self,
        qualified_name: &QualifiedName,
        namespace: &str,
    ) -> Attribute {
        match self
            .as_web_sys_document()
            .create_attribute_ns(Some(namespace), qualified_name.as_ref())
        {
            Ok(attr) => attr.into(),
            Err(err) => throw_val(err),
        }
    }

    /// Creates a new attribute node with the given `qualified_name` and `namespace`.
    ///
    /// See [QualifiedName] for details on valid qualified names. Note that though namespace is
    /// generally expected to be a [URI](https://datatracker.ietf.org/doc/html/rfc2396), any string
    /// is accepted.
    ///
    /// Returns a [NamespaceError] in any of the following cases:
    ///
    /// - Panics if the [QualifiedName::prefix] of `qualified_name` is not `None` and namespace if
    ///   the empty string.
    /// - Panics if the [QualifiedName::prefix] of `qualified_name` is `xml` and
    ///   `namespace` is not the [XML namespace](https://infra.spec.whatwg.org/#xml-namespace).
    /// - Panics if the [QualifiedName::prefix] of `qualified_name` is `xmlns` and
    ///   `namespace` is not the [XMLNS namespace](https://infra.spec.whatwg.org/#xmlns-namespace).
    /// - Panics if `namespace` is the [XMLNS namespace](https://infra.spec.whatwg.org/#xmlns-namespace)
    ///   and `qualified_name` is not either a prefix-less qualified name with a
    ///   [QualifiedName::local_name] of `xmlns`, or a qualified name with a [QualifiedName::prefix]
    ///   of `xmlns`.
    ///
    /// See [create_attribute_namespaced] for an alternative that panics instead of returning an
    /// error.
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

    /// Creates a new element node with the given `name`.
    ///
    /// See [Name] for details on valid element names.
    ///
    /// # Example
    ///
    /// ```
    /// # let document = arwa::window::window().document();
    /// use arwa::dom::{name, Document};
    ///
    /// let element = document.create_element(&name!("my-element"));
    /// ```
    ///
    /// Note that the concrete type of the returned [DynamicElement] depends on the concrete
    /// type of the document:
    ///
    /// ```
    /// # let document = arwa::window::window().document();
    /// use arwa::dom::{name, Document};
    /// use arwa::html::{HtmlDivElement, HtmlDocument};
    ///
    /// // The document is an HTML document
    /// let document: HtmlDocument = document.try_into().unwrap();
    ///
    /// let element = document.create_element(&name!("div"));
    ///
    /// let div: HtmlDivElement = element.try_into().expect("this will not fail");
    /// ```
    fn create_element(&self, name: &Name) -> DynamicElement {
        self.as_web_sys_document()
            .create_element(name.as_ref())
            .unwrap_throw()
            .into()
    }

    /// Creates a new element node with the given `qualified_name` and `namespace`.
    ///
    /// See [QualifiedName] for details on valid qualified names. Note that though
    /// namespace is generally expected to be a [URI](https://datatracker.ietf.org/doc/html/rfc2396),
    /// any string is accepted.
    ///
    /// # Panics
    ///
    /// - Panics if the [QualifiedName::prefix] of `qualified_name` is not `None` and namespace if
    ///   the empty string.
    /// - Panics if the [QualifiedName::prefix] of `qualified_name` is `xml` and
    ///   `namespace` is not the [XML namespace](https://infra.spec.whatwg.org/#xml-namespace).
    /// - Panics if the [QualifiedName::prefix] of `qualified_name` is `xmlns` and
    ///   `namespace` is not the [XMLNS namespace](https://infra.spec.whatwg.org/#xmlns-namespace).
    /// - Panics if `namespace` is the [XMLNS namespace](https://infra.spec.whatwg.org/#xmlns-namespace)
    ///   and `qualified_name` is not either a prefix-less qualified name with a
    ///   [QualifiedName::local_name] of `xmlns`, or a qualified name with a [QualifiedName::prefix]
    ///   of `xmlns`.
    ///
    /// See [try_create_element_namespaced] for an alternative that returns ah error instead of
    /// panicking.
    fn create_element_namespaced(
        &self,
        qualified_name: &QualifiedName,
        namespace: &str,
    ) -> DynamicElement {
        match self
            .as_web_sys_document()
            .create_element_ns(Some(namespace), qualified_name.as_ref())
        {
            Ok(element) => element.into(),
            Err(err) => throw_val(err),
        }
    }

    /// Creates a new element node with the given `qualified_name` and `namespace`.
    ///
    /// See [QualifiedName] for details on valid qualified names. Note that though namespace is
    /// generally expected to be a [URI](https://datatracker.ietf.org/doc/html/rfc2396), any string
    /// is accepted.
    ///
    /// Returns a [NamespaceError] in any of the following cases:
    ///
    /// - Panics if the [QualifiedName::prefix] of `qualified_name` is not `None` and namespace if
    ///   the empty string.
    /// - Panics if the [QualifiedName::prefix] of `qualified_name` is `xml` and
    ///   `namespace` is not the [XML namespace](https://infra.spec.whatwg.org/#xml-namespace).
    /// - Panics if the [QualifiedName::prefix] of `qualified_name` is `xmlns` and
    ///   `namespace` is not the [XMLNS namespace](https://infra.spec.whatwg.org/#xmlns-namespace).
    /// - Panics if `namespace` is the [XMLNS namespace](https://infra.spec.whatwg.org/#xmlns-namespace)
    ///   and `qualified_name` is not either a prefix-less qualified name with a
    ///   [QualifiedName::local_name] of `xmlns`, or a qualified name with a [QualifiedName::prefix]
    ///   of `xmlns`.
    ///
    /// See [create_element_namespaced] for an alternative that panics instead of returning an
    /// error.
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

    /// Creates a new comment node that contains a copy of the given `data` string.
    ///
    /// # Example
    ///
    /// ```
    /// # let document = arwa::window::window().document();
    /// use arwa::dom::Document;
    ///
    /// let comment = document.create_comment("A comment!");
    /// ```
    fn create_comment(&self, data: &str) -> Comment {
        self.as_web_sys_document().create_comment(data).into()
    }

    /// Creates a new text node that contains a copy of the given `data` string.
    ///
    /// # Example
    ///
    /// ```
    /// # let document = arwa::window::window().document();
    /// use arwa::dom::Document;
    ///
    /// let text = document.create_text("Some text!");
    /// ```
    fn create_text(&self, data: &str) -> Text {
        self.as_web_sys_document().create_text_node(data).into()
    }

    /// Creates a new empty live range.
    ///
    /// Live ranges may modify their boundaries after creation, see [LiveRange] for details.
    fn create_range(&self) -> LiveRange {
        self.as_web_sys_document()
            .create_range()
            .unwrap_throw()
            .into()
    }

    /// Returns the top-most element in a the document at the specified pixel coordinates relative
    /// to the viewport.
    ///
    /// If the top-most element belongs to an embedded document, then the embedding element is
    /// returned. If the element the coordinates point is an anonymous element or
    /// [XBL](https://www.w3.org/TR/xbl/) generated content (e.g. a [<textarea>](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)'s
    /// scroll bars), then the first non-anonymous ancestor element (the [<textarea>](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea))
    /// is returned.
    fn element_from_point(&self, x: f32, y: f32) -> Option<DynamicElement> {
        self.as_web_sys_document()
            .element_from_point(x, y)
            .map(|e| e.into())
    }

    /// Async iterator over [FullscreenChangeEvent]s emitted by this document.
    fn on_fullscreen_change(&self) -> OnFullscreenChange<Self> {
        OnFullscreenChange::new(self.as_web_sys_document())
    }

    /// Async iterator over [FullscreenErrorEvent]s emitted by this document.
    fn on_fullscreen_error(&self) -> OnFullscreenError<Self> {
        OnFullscreenError::new(self.as_web_sys_document())
    }

    /// Async iterator over [PointerLockChangeEvent]s emitted by this document.
    fn on_pointer_lock_change(&self) -> OnPointerLockChange<Self> {
        OnPointerLockChange::new(self.as_web_sys_document())
    }

    /// Async iterator over [PointerLockErrorEvent]s emitted by this document.
    fn on_pointer_lock_error(&self) -> OnPointerLockError<Self> {
        OnPointerLockError::new(self.as_web_sys_document())
    }

    /// Async iterator over [ReadyStateChangeEvent]s emitted by this document.
    fn on_ready_state_change(&self) -> OnReadyStateChange<Self> {
        OnReadyStateChange::new(self.as_web_sys_document())
    }

    /// Async iterator over [SelectionChangeEvent]s emitted by this document.
    fn on_selection_change(&self) -> OnSelectionChange<Self> {
        OnSelectionChange::new(self.as_web_sys_document())
    }

    /// Async iterator over [VisibilityChangeEvent]s emitted by this document.
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

/// Represents a value that can be used as a [Document], but for which a specific type is not
/// statically known.
///
/// You may try to resolve a value of this type to a concrete type using [TryFrom] conversion. All
/// Arwa types that implement [Document] also implement `TryFrom<DynamicDocument>`.
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
impl_parent_node!(DynamicDocument);
impl_try_from_parent_node!(DynamicDocument, Document);

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
        $crate::dom::impl_parent_node!($document);
        $crate::dom::impl_try_from_parent_node!($document, $web_sys_tpe);
    };
    ($tpe:ident) => {
        $crate::dom::impl_document_traits!($tpe, $tpe);
    };
}

pub(crate) use impl_document_traits;
