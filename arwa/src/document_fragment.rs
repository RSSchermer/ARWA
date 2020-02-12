use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::error::SyntaxError;
use crate::{Element, GenericElement, Node, QuerySelectorAll, InvalidCast};
use crate::event::GenericEventTarget;

pub struct DocumentFragment {
    inner: web_sys::DocumentFragment,
}

impl DocumentFragment {
    pub fn query_id(&self, id: &str) -> Option<GenericElement> {
        self.inner.get_element_by_id(id).map(|e| e.into())
    }

    pub fn query_selector_first(
        &self,
        selector: &str,
    ) -> Result<Option<GenericElement>, SyntaxError> {
        self.inner
            .query_selector(selector)
            .map(|ok| ok.map(|e| e.into()))
            .map_err(|err| SyntaxError::new(err.unchecked_into()))
    }

    pub fn query_selector_all(&self, selector: &str) -> Result<QuerySelectorAll, SyntaxError> {
        self.inner
            .query_selector_all(selector)
            .map(|inner| QuerySelectorAll::new(inner))
            .map_err(|err| SyntaxError::new(err.unchecked_into()))
    }

    pub fn child_elements(&self) -> DocumentFragmentChildElements {
        DocumentFragmentChildElements {
            document_fragment: &self.inner,
            children: self.inner.children(),
        }
    }
}

impl From<web_sys::DocumentFragment> for DocumentFragment {
    fn from(inner: web_sys::DocumentFragment) -> Self {
        DocumentFragment { inner }
    }
}

impl TryFrom<GenericEventTarget> for DocumentFragment {
    type Error = InvalidCast<GenericEventTarget>;

    fn try_from(value: GenericEventTarget) -> Result<Self, Self::Error> {
        let value: web_sys::EventTarget = value.into();

        value
            .dyn_into::<web_sys::DocumentFragment>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl AsRef<web_sys::DocumentFragment> for DocumentFragment {
    fn as_ref(&self) -> &web_sys::DocumentFragment {
        &self.inner
    }
}

impl AsRef<web_sys::Node> for DocumentFragment {
    fn as_ref(&self) -> &web_sys::Node {
        self.inner.as_ref()
    }
}

impl Write for DocumentFragment {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

impl Node for DocumentFragment {}

pub struct DocumentFragmentChildElements<'a> {
    document_fragment: &'a web_sys::DocumentFragment,
    children: web_sys::HtmlCollection,
}

impl<'a> DocumentFragmentChildElements<'a> {
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
        self.document_fragment.child_element_count() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<GenericElement> {
        self.document_fragment
            .first_element_child()
            .map(|e| e.into())
    }

    pub fn last(&self) -> Option<GenericElement> {
        self.document_fragment
            .last_element_child()
            .map(|e| e.into())
    }

    pub fn append<C>(&self, child: &C)
    where
        C: DocumentFragmentChild,
    {
        child.append_to(self);
    }

    pub fn prepend<C>(&self, child: &C)
    where
        C: DocumentFragmentChild,
    {
        child.prepend_to(self);
    }

    pub fn iter(&self) -> DocumentFragmentChildElementsIter {
        DocumentFragmentChildElementsIter {
            document_fragment_child_elements: self,
            current: 0,
        }
    }
}

impl<'a> Write for DocumentFragmentChildElements<'a> {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.children.as_ref())
    }
}

impl<'a> IntoIterator for DocumentFragmentChildElements<'a> {
    type Item = GenericElement;
    type IntoIter = DocumentFragmentChildElementsIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DocumentFragmentChildElementsIntoIter {
            document_fragment_child_elements: self,
            current: 0,
        }
    }
}

pub trait DocumentFragmentChild: document_fragment_child_seal::Sealed {
    // Note, this construct *should* be locked down enough to avoid any "HierarchyRequestError" and
    // thus these operations should never fail.

    fn prepend_to(&self, document_fragment_children: &DocumentFragmentChildElements);

    fn append_to(&self, document_fragment_children: &DocumentFragmentChildElements);
}

impl<T> DocumentFragmentChild for T
where
    T: Element,
{
    // TODO: decide on panic vs error on append/prepend when trying to insert an element into an
    // element that is a descendant. May be argued to be in the same class of errors as indexing
    // error, std handles these with panics (e.g. Vec::insert).

    fn prepend_to(&self, document_fragment_children: &DocumentFragmentChildElements) {
        document_fragment_children
            .document_fragment
            .prepend_with_node_1(self.as_ref())
            .expect(
                "Element cannot be an ancestor of the element into which it is being inserted.",
            );
    }

    fn append_to(&self, document_fragment_children: &DocumentFragmentChildElements) {
        document_fragment_children
            .document_fragment
            .append_with_node_1(self.as_ref())
            .expect(
                "Element cannot be an ancestor of the element into which it is being inserted.",
            );
    }
}

impl DocumentFragmentChild for str {
    fn prepend_to(&self, document_fragment_children: &DocumentFragmentChildElements) {
        document_fragment_children
            .document_fragment
            .prepend_with_str_1(self)
            .unwrap();
    }

    fn append_to(&self, document_fragment_children: &DocumentFragmentChildElements) {
        document_fragment_children
            .document_fragment
            .append_with_str_1(self)
            .unwrap();
    }
}

mod document_fragment_child_seal {
    use super::*;

    pub trait Sealed {}

    impl<T> Sealed for T where T: Element {}
    impl Sealed for str {}
}

pub struct DocumentFragmentChildElementsIter<'a> {
    document_fragment_child_elements: &'a DocumentFragmentChildElements<'a>,
    current: usize,
}

impl<'a> Iterator for DocumentFragmentChildElementsIter<'a> {
    type Item = GenericElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.document_fragment_child_elements.get(current)
    }
}

pub struct DocumentFragmentChildElementsIntoIter<'a> {
    document_fragment_child_elements: DocumentFragmentChildElements<'a>,
    current: usize,
}

impl<'a> Iterator for DocumentFragmentChildElementsIntoIter<'a> {
    type Item = GenericElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.document_fragment_child_elements.get(current)
    }
}
