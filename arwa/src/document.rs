use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::error::{AdoptNodeError, HierarchyRequestError, NotSupportedError, SyntaxError};
use crate::event::{OnFullscreenChange, OnFullscreenError, OnPointerLockChange, OnPointerLockError, OnReadyStateChange, OnVisibilityChange, GenericEventTarget};
use crate::html::{
    GenericHtmlElement, HtmlBodyElement, HtmlFormElement, HtmlHeadElement, HtmlImageElement,
};
use crate::{DocumentFragment, DocumentType, Element, GenericElement, GenericNode, GlobalEventHandlers, Location, Node, QuerySelectorAll, TextDirectionality, InvalidCast};

pub trait Document: AsRef<web_sys::Document> {
    // TODO: implement new() for concrete implementations rather than trait.

    // TODO: all create element methods

    fn body(&self) -> Option<HtmlBodyElement> {
        // Disregard deprecated frameset element
        self.as_ref()
            .body()
            .and_then(|e| e.dyn_into::<web_sys::HtmlBodyElement>().ok())
            .map(|body| body.into())
    }

    fn set_body(&self, body: Option<&HtmlBodyElement>) {
        self.as_ref().set_body(body.map(|b| b.as_ref()));
    }

    fn character_set(&self) -> String {
        self.as_ref().character_set()
    }

    fn child_elements(&self) -> DocumentChildElements {
        DocumentChildElements {
            document: self.as_ref(),
            children: self.as_ref().children(),
        }
    }

    fn dir(&self) -> TextDirectionality {
        match &*self.as_ref().dir().to_lowercase() {
            "ltr" => TextDirectionality::LeftToRight,
            "rtl" => TextDirectionality::RightToLeft,
            _ => TextDirectionality::Auto,
        }
    }

    fn set_dir(&self, dir: TextDirectionality) {
        let text_directionality = match dir {
            TextDirectionality::Auto => "auto",
            TextDirectionality::LeftToRight => "ltr",
            TextDirectionality::RightToLeft => "rtl",
        };

        self.as_ref().set_dir(text_directionality);
    }

    fn doctype(&self) -> Option<DocumentType> {
        self.as_ref().doctype().map(|d| d.into())
    }

    fn document_element(&self) -> Option<GenericElement> {
        self.as_ref().document_element().map(|e| e.into())
    }

    fn document_uri(&self) -> String {
        // No indication in the WHATWG spec that this can actually fail, unwrap for now.
        self.as_ref().document_uri().unwrap()
    }

    // TODO: embeds. Unclear what the exact element is/can be. WHATWG implies only HtmlEmbedElement,
    // MDN implies HTMLObjectElement. Do we need to defensively use GenericHtmlElement?

    fn forms(&self) -> DocumentForms {
        DocumentForms {
            inner: self.as_ref().forms(),
        }
    }

    fn fullscreen_enabled(&self) -> bool {
        self.as_ref().fullscreen_enabled()
    }

    fn has_focus(&self) -> bool {
        // No indication in the spec that this can actually fail, unwrap for now.
        self.as_ref().has_focus().unwrap()
    }

    fn head(&self) -> Option<HtmlHeadElement> {
        self.as_ref().head().map(|h| h.into())
    }

    fn hidden(&self) -> bool {
        self.as_ref().hidden()
    }

    fn images(&self) -> DocumentImages {
        DocumentImages {
            inner: self.as_ref().images(),
        }
    }

    fn last_modified(&self) -> String {
        self.as_ref().last_modified()
    }

    fn links(&self) -> DocumentLinks {
        DocumentLinks {
            inner: self.as_ref().links(),
        }
    }

    fn location(&self) -> Option<Location> {
        self.as_ref().location().map(|l| l.into())
    }

    fn referrer(&self) -> String {
        self.as_ref().referrer()
    }

    fn title(&self) -> String {
        self.as_ref().title()
    }

    fn set_title(&self, title: &str) {
        self.as_ref().set_title(title);
    }

    fn pointer_lock_element(&self) -> Option<GenericElement> {
        self.as_ref().pointer_lock_element().map(|e| e.into())
    }

    // TODO: default_view when Window has been figured out

    fn on_fullscreen_change(&self) -> OnFullscreenChange {
        OnFullscreenChange::new(self.as_ref().clone().into())
    }

    fn on_fullscreen_error(&self) -> OnFullscreenError {
        OnFullscreenError::new(self.as_ref().clone().into())
    }

    fn on_pointer_lock_change(&self) -> OnPointerLockChange {
        OnPointerLockChange::new(self.as_ref().clone().into())
    }

    fn on_pointer_lock_error(&self) -> OnPointerLockError {
        OnPointerLockError::new(self.as_ref().clone().into())
    }

    fn on_ready_state_change(&self) -> OnReadyStateChange {
        OnReadyStateChange::new(self.as_ref().clone().into())
    }

    fn on_visibility_change(&self) -> OnVisibilityChange {
        OnVisibilityChange::new(self.as_ref().clone().into())
    }

    fn adopt_node<T>(&self, node: &T) -> Result<GenericNode, AdoptNodeError>
    where
        T: Node,
    {
        self.as_ref()
            .adopt_node(node.as_ref())
            .map(|ok| ok.into())
            .map_err(|err| {
                let err: web_sys::DomException = err.unchecked_into();

                match &*err.name() {
                    "NotSupportedError" => NotSupportedError::new(err).into(),
                    "HierarchyRequestError" => HierarchyRequestError::new(err).into(),
                    _ => unreachable!(),
                }
            })
    }

    fn import_node<T>(&self, node: &T) -> Result<GenericNode, NotSupportedError>
    where
        T: Node,
    {
        self.as_ref()
            .import_node(node.as_ref())
            .map(|ok| ok.into())
            .map_err(|err| NotSupportedError::new(err.unchecked_into()))
    }

    fn import_node_deep<T>(&self, node: &T) -> Result<GenericNode, NotSupportedError>
    where
        T: Node,
    {
        self.as_ref()
            .import_node_with_deep(node.as_ref(), true)
            .map(|ok| ok.into())
            .map_err(|err| NotSupportedError::new(err.unchecked_into()))
    }

    // TODO: modify `query_id` or add additional method that incorporates the cast for convenience?
    // E.g.: fn query_id<T>(&self, id: &str) -> Option<T> where T: TryFrom<GenericElement>;

    fn query_id(&self, id: &str) -> Option<GenericElement> {
        self.as_ref().get_element_by_id(id).map(|e| e.into())
    }

    fn query_selector_first(&self, selector: &str) -> Result<Option<GenericElement>, SyntaxError> {
        self.as_ref()
            .query_selector(selector)
            .map(|ok| ok.map(|e| e.into()))
            .map_err(|err| SyntaxError::new(err.unchecked_into()))
    }

    fn query_selector_all(&self, selector: &str) -> Result<QuerySelectorAll, SyntaxError> {
        self.as_ref()
            .query_selector_all(selector)
            .map(|inner| QuerySelectorAll::new(inner))
            .map_err(|err| SyntaxError::new(err.unchecked_into()))
    }

    // TODO: decide on get_elements_by_tag_name and get_elements_by_class_name, see comment in
    // `Element`.

    fn enable_style_sheets_for_set(&self, set_name: Option<&str>) {
        self.as_ref().enable_style_sheets_for_set(set_name);
    }

    // TODO: has_storage_access and request_storage_access seem to be missing from web_sys.

    // TODO: XPath?

    fn create_document_fragment(&self) -> DocumentFragment {
        DocumentFragment::from(self.as_ref().create_document_fragment())
    }

    fn element_from_point(&self, x: f32, y: f32) -> Option<GenericElement> {
        self.as_ref().element_from_point(x, y).map(|e| e.into())
    }

    fn exit_pointer_lock(&self) {
        self.as_ref().exit_pointer_lock();
    }
}

pub struct GenericDocument {
    inner: web_sys::Document,
}

impl From<web_sys::Document> for GenericDocument {
    fn from(inner: web_sys::Document) -> Self {
        GenericDocument { inner }
    }
}

impl From<GenericDocument> for web_sys::Document {
    fn from(value: GenericDocument) -> Self {
        value.inner
    }
}

impl TryFrom<GenericEventTarget> for GenericDocument {
    type Error = InvalidCast<GenericEventTarget>;

    fn try_from(value: GenericEventTarget) -> Result<Self, Self::Error> {
        let value: web_sys::EventTarget = value.into();

        value
            .dyn_into::<web_sys::Document>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl TryFrom<GenericNode> for GenericDocument {
    type Error = InvalidCast<GenericNode>;

    fn try_from(value: GenericNode) -> Result<Self, Self::Error> {
        let value: web_sys::Node = value.into();

        value
            .dyn_into::<web_sys::Document>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl AsRef<web_sys::Document> for GenericDocument {
    fn as_ref(&self) -> &web_sys::Document {
        &self.inner
    }
}

impl AsRef<web_sys::Node> for GenericDocument {
    fn as_ref(&self) -> &web_sys::Node {
        self.inner.as_ref()
    }
}

impl AsRef<web_sys::EventTarget> for GenericDocument {
    fn as_ref(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl Write for GenericDocument {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl GlobalEventHandlers for GenericDocument {}
impl Node for GenericDocument {}
impl Document for GenericDocument {}

pub struct DocumentChildElements<'a> {
    document: &'a web_sys::Document,
    children: web_sys::HtmlCollection,
}

impl<'a> DocumentChildElements<'a> {
    pub fn get(&self, index: usize) -> Option<GenericElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.children.get_with_index(index))
            .map(|e| e.into())
    }

    pub fn find_by_id(&self, id: &str) -> Option<GenericElement> {
        self.children.get_with_name(id).map(|e| e.into())
    }

    pub fn len(&self) -> usize {
        self.document.child_element_count() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<GenericElement> {
        self.document.first_element_child().map(|e| e.into())
    }

    pub fn last(&self) -> Option<GenericElement> {
        self.document.last_element_child().map(|e| e.into())
    }

    pub fn append<C>(&self, child: &C)
    where
        C: DocumentChild,
    {
        child.append_to(self);
    }

    pub fn prepend<C>(&self, child: &C)
    where
        C: DocumentChild,
    {
        child.prepend_to(self);
    }

    pub fn iter(&self) -> DocumentChildElementsIter {
        DocumentChildElementsIter {
            document_child_elements: self,
            current: 0,
        }
    }
}

impl<'a> Write for DocumentChildElements<'a> {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.children.as_ref());
    }
}

impl<'a> IntoIterator for DocumentChildElements<'a> {
    type Item = GenericElement;
    type IntoIter = DocumentChildElementsIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DocumentChildElementsIntoIter {
            document_child_elements: self,
            current: 0,
        }
    }
}

pub trait DocumentChild: document_child_seal::Sealed {
    // Note, this construct *should* be locked down enough to avoid any "HierarchyRequestError" and
    // thus these operations should never fail.

    fn prepend_to(&self, document_children: &DocumentChildElements);

    fn append_to(&self, document_children: &DocumentChildElements);
}

impl<T> DocumentChild for T
where
    T: Element,
{
    // TODO: decide on panic vs error on append/prepend when trying to insert an element into an
    // element that is a descendant. May be argued to be in the same class of errors as indexing
    // error, std handles these with panics (e.g. Vec::insert).

    fn prepend_to(&self, document_children: &DocumentChildElements) {
        document_children
            .document
            .prepend_with_node_1(self.as_ref())
            .expect(
                "Element cannot be an ancestor of the element into which it is being inserted.",
            );
    }

    fn append_to(&self, document_children: &DocumentChildElements) {
        document_children
            .document
            .append_with_node_1(self.as_ref())
            .expect(
                "Element cannot be an ancestor of the element into which it is being inserted.",
            );
    }
}

impl DocumentChild for str {
    fn prepend_to(&self, document_children: &DocumentChildElements) {
        document_children.document.prepend_with_str_1(self).unwrap();
    }

    fn append_to(&self, document_children: &DocumentChildElements) {
        document_children.document.append_with_str_1(self).unwrap();
    }
}

mod document_child_seal {
    use super::*;

    pub trait Sealed {}

    impl<T> Sealed for T where T: Element {}
    impl Sealed for str {}
}

pub struct DocumentChildElementsIter<'a> {
    document_child_elements: &'a DocumentChildElements<'a>,
    current: usize,
}

impl<'a> Iterator for DocumentChildElementsIter<'a> {
    type Item = GenericElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.document_child_elements.get(current)
    }
}

pub struct DocumentChildElementsIntoIter<'a> {
    document_child_elements: DocumentChildElements<'a>,
    current: usize,
}

impl<'a> Iterator for DocumentChildElementsIntoIter<'a> {
    type Item = GenericElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.document_child_elements.get(current)
    }
}

pub struct DocumentForms {
    inner: web_sys::HtmlCollection,
}

impl DocumentForms {
    pub fn get(&self, index: usize) -> Option<HtmlFormElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get_with_index(index))
            .map(|e| {
                let e: web_sys::HtmlFormElement = e.unchecked_into();

                e.into()
            })
    }

    pub fn find_by_id(&self, id: &str) -> Option<HtmlFormElement> {
        self.inner.get_with_name(id).map(|e| {
            let e: web_sys::HtmlFormElement = e.unchecked_into();

            e.into()
        })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<HtmlFormElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<HtmlFormElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> DocumentFormsIter {
        DocumentFormsIter {
            document_forms: self,
            current: 0,
        }
    }
}

impl Write for DocumentForms {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl IntoIterator for DocumentForms {
    type Item = HtmlFormElement;
    type IntoIter = DocumentFormsIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        DocumentFormsIntoIter {
            document_forms: self,
            current: 0,
        }
    }
}

pub struct DocumentFormsIter<'a> {
    document_forms: &'a DocumentForms,
    current: usize,
}

impl<'a> Iterator for DocumentFormsIter<'a> {
    type Item = HtmlFormElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.document_forms.get(current)
    }
}

pub struct DocumentFormsIntoIter {
    document_forms: DocumentForms,
    current: usize,
}

impl Iterator for DocumentFormsIntoIter {
    type Item = HtmlFormElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.document_forms.get(current)
    }
}

pub struct DocumentImages {
    inner: web_sys::HtmlCollection,
}

impl DocumentImages {
    pub fn get(&self, index: usize) -> Option<HtmlImageElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get_with_index(index))
            .map(|e| {
                let e: web_sys::HtmlImageElement = e.unchecked_into();

                e.into()
            })
    }

    pub fn find_by_id(&self, id: &str) -> Option<HtmlImageElement> {
        self.inner.get_with_name(id).map(|e| {
            let e: web_sys::HtmlImageElement = e.unchecked_into();

            e.into()
        })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<HtmlImageElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<HtmlImageElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> DocumentImagesIter {
        DocumentImagesIter {
            document_images: self,
            current: 0,
        }
    }
}

impl Write for DocumentImages {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl IntoIterator for DocumentImages {
    type Item = HtmlImageElement;
    type IntoIter = DocumentImagesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        DocumentImagesIntoIter {
            document_images: self,
            current: 0,
        }
    }
}

pub struct DocumentImagesIter<'a> {
    document_images: &'a DocumentImages,
    current: usize,
}

impl<'a> Iterator for DocumentImagesIter<'a> {
    type Item = HtmlImageElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.document_images.get(current)
    }
}

pub struct DocumentImagesIntoIter {
    document_images: DocumentImages,
    current: usize,
}

impl Iterator for DocumentImagesIntoIter {
    type Item = HtmlImageElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.document_images.get(current)
    }
}

pub struct DocumentLinks {
    inner: web_sys::HtmlCollection,
}

impl DocumentLinks {
    pub fn get(&self, index: usize) -> Option<GenericHtmlElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get_with_index(index))
            .map(|e| {
                let e: web_sys::HtmlElement = e.unchecked_into();

                e.into()
            })
    }

    pub fn find_by_id(&self, id: &str) -> Option<GenericHtmlElement> {
        self.inner.get_with_name(id).map(|e| {
            let e: web_sys::HtmlElement = e.unchecked_into();

            e.into()
        })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<GenericHtmlElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<GenericHtmlElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> DocumentLinksIter {
        DocumentLinksIter {
            document_links: self,
            current: 0,
        }
    }
}

impl Write for DocumentLinks {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl IntoIterator for DocumentLinks {
    type Item = GenericHtmlElement;
    type IntoIter = DocumentLinksIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        DocumentLinksIntoIter {
            document_links: self,
            current: 0,
        }
    }
}

pub struct DocumentLinksIter<'a> {
    document_links: &'a DocumentLinks,
    current: usize,
}

impl<'a> Iterator for DocumentLinksIter<'a> {
    type Item = GenericHtmlElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.document_links.get(current)
    }
}

pub struct DocumentLinksIntoIter {
    document_links: DocumentLinks,
    current: usize,
}

impl Iterator for DocumentLinksIntoIter {
    type Item = GenericHtmlElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.document_links.get(current)
    }
}
