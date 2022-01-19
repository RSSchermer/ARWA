use crate::dom::child_node::ChildNode;
use crate::dom::hierarchy_request_error::HierarchyRequestError;
use crate::dom::selector::{CompiledSelector, Selector};
use crate::dom::{DocumentFragment, DynamicElement, DynamicNode};
use crate::{DynamicElement, DynamicNode};
use std::convert::TryFrom;

pub(crate) mod parent_node_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_node(&self) -> &web_sys::Node;
    }
}

#[derive(Clone, PartialEq)]
pub struct QuerySelectorSyntaxError {
    inner: js_sys::SyntaxError,
}

pub trait ParentNode: parent_node_seal::Seal + Sized {
    fn contains<T>(&self, node: &T) -> bool
    where
        T: ChildNode,
    {
        self.as_web_sys_node().contains(Some(node.as_ref()))
    }

    fn query_selector_first(&self, selector: &CompiledSelector) -> Option<DynamicElement>;

    fn query_selector_all(&self, selector: &CompiledSelector) -> QuerySelectorAll;

    /// Creates a new node that is a deep copy of the `source` node.
    ///
    /// Recursively copies the source node's entire sub-tree.
    ///
    /// See also [duplicate].
    fn duplicate_deep(source: Self) -> Self {
        let cloned_node = source
            .as_web_sys_node()
            .clone_node_with_deep(true)
            .unwrap_throw();

        Self::from_web_sys_node_unchecked(cloned_node)
    }

    fn normalize(&self) {
        self.as_web_sys_node().normalize()
    }

    /// Returns a live collection of the all nodes that are direct children of this parent node.
    ///
    /// See also [child_elements] to retrieve only the subset of nodes that implement [Element].
    fn child_nodes(&self) -> ChildNodes {
        ChildNodes {
            inner: self.as_web_sys_node().children(),
        }
    }

    /// Returns a live collection of only the [Element nodes that are direct children of this parent node.
    ///
    /// See also [child_elements] to retrieve only the subset of nodes that implement [Element].
    fn child_elements(&self) -> ChildElements;

    fn prepend_child<T>(&self, node: &T)
    where
        T: ChildNode;

    fn try_prepend_child<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode;

    /// Prepends the document structure in the given `document_fragment` before the first child
    /// element in this node.
    ///
    /// This moves the nodes from the document fragment, leaving the document fragment empty.
    fn prepend_fragment<T>(&self, document_fragment: &T)
    where
        T: DocumentFragment;

    fn append_child<T>(&self, node: &T)
    where
        T: ChildNode,
    {
        self.as_web_sys_node()
            .append_child(node.as_web_sys_node())
            .unwrap_throw();
    }

    fn try_append_child<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode,
    {
        self.as_web_sys_node()
            .append_child(node.as_web_sys_node())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }

    /// Append the document structure in the given `document_fragment` after the last child element
    /// in this node.
    ///
    /// This moves the nodes from the document fragment, leaving the document fragment empty.
    fn append_fragment<T>(&self, document_fragment: &T)
    where
        T: DocumentFragment,
    {
        // Note: this should never cause a hierarchy request error.
        self.as_web_sys_node()
            .append_child(document_fragment.as_web_sys_document_fragment().as_ref())
            .unwrap_throw();
    }
}

/// A live collection of all nodes that are direct children of the parent node.
///
/// Obtained by calling [child_nodes] on a [ParentNode] type.
pub struct ChildNodes {
    inner: web_sys::NodeList,
}

impl Collection for ChildNodes {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for ChildNodes {}

impl ChildNodes {
    pub fn get(&self, index: u32) -> Option<DynamicNode> {
        self.inner.get(index).map(|n| n.into())
    }

    pub fn first(&self) -> Option<DynamicNode> {
        self.get(0)
    }

    pub fn last(&self) -> Option<DynamicNode> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn len(&self) -> u32 {
        self.inner.length()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn snapshot(&self) -> ChildNodesSnapshot {
        ChildNodesSnapshot::new(Array::from(self.inner.as_ref()))
    }
}

/// A live collection of all nodes that are direct children of the parent node.
///
/// Obtained by calling [child_nodes] on a [ParentNode] type.
pub struct ChildElements {
    inner: web_sys::HtmlCollection,
}

impl ChildElements {
    pub(crate) fn new(inner: web_sys::HtmlCollection) -> Self {
        ChildElements { inner }
    }

    pub fn get(&self, index: u32) -> Option<DynamicElement> {
        self.inner.item(index).map(|n| n.into())
    }

    pub fn first(&self) -> Option<DynamicElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<DynamicElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn len(&self) -> u32 {
        self.inner.length()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn snapshot(&self) -> ChildElementsSnapshot {
        ChildElementsSnapshot::new(Array::from(self.inner.as_ref()))
    }
}

unchecked_cast_array_wrapper!(
    DynamicNode,
    web_sys::Node,
    ChildElementsSnapshot,
    ChildElementsSnapshotIter
);

pub struct QuerySelectorAll {
    inner: web_sys::NodeList,
}

impl QuerySelectorAll {
    pub(crate) fn new(inner: web_sys::NodeList) -> Self {
        QuerySelectorAll { inner }
    }

    pub fn get(&self, index: usize) -> Option<DynamicElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get(index))
            .map(|node| {
                let element: web_sys::Element = node.unchecked_into();

                element.into()
            })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<DynamicElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<DynamicElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> QuerySelectorAllIter {
        QuerySelectorAllIter {
            query_selector_all: self,
            current: 0,
        }
    }
}

pub struct QuerySelectorAllIter<'a> {
    query_selector_all: &'a QuerySelectorAll,
    current: usize,
}

impl<'a> Iterator for QuerySelectorAllIter<'a> {
    type Item = DynamicElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.query_selector_all.get(current)
    }
}

macro_rules! impl_parent_node_for_element {
    ($element:ident) => {
        impl $crate::dom::parent_node_seal::Seal for $element {
            fn as_web_sys_node(&self) -> &web_sys::Node {
                self.as_web_sys_element().as_ref()
            }
        }

        use $crate::dom::{
            ChildElements, ChildNode, CompiledSelector, DocumentFragment, DynamicElement,
            HierarchyRequestError, ParentNode, QuerySelectorAll,
        };

        impl ParentNode for $element {
            fn query_selector_first(&self, selector: &CompiledSelector) -> Option<DynamicElement> {
                self.as_web_sys_element()
                    .query_selector_first(selector.as_ref())
                    .unwrap_throw()
                    .map(|e| e.into())
            }

            fn query_selector_all(&self, selector: &CompiledSelector) -> QuerySelectorAll {
                QuerySelectorAll::new(
                    self.as_web_sys_element()
                        .query_selector_all(selector.as_ref())
                        .unwrap_trhwo(),
                )
            }

            fn child_elements(&self) -> ChildElements {
                ChildElements::new(self.as_web_sys_element().children())
            }

            fn prepend_child<T>(&self, node: &T) -> T
            where
                T: ChildNode,
            {
                self.as_web_sys_element()
                    .prepend_with_node_1(node.as_web_sys_node())
                    .unwrap_throw();
            }

            fn try_prepend_child<T>(&self, node: &T) -> Result<T, HierarchyRequestError>
            where
                T: ChildNode,
            {
                self.as_web_sys_element()
                    .prepend_with_node_1(node.as_web_sys_node())
                    .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
            }

            fn prepend_fragment<T>(&self, document_fragment: T)
            where
                T: DocumentFragment,
            {
                self.as_web_sys_element()
                    .prepend_with_node_1(node.as_web_sys_document_fragment().as_ref())
                    .unwrap_throw();
            }
        }
    };
}

pub(crate) use impl_parent_node_for_element;

macro_rules! impl_parent_node_for_document {
    ($document:ident) => {
        impl $crate::dom::parent_node_seal::Seal for $document {
            fn as_web_sys_node(&self) -> &web_sys::Node {
                self.as_web_sys_document().as_ref()
            }
        }

        use $crate::dom::{
            ChildElements, ChildNode, CompiledSelector, DocumentFragment, DynamicElement,
            HierarchyRequestError, ParentNode, QuerySelectorAll,
        };

        impl ParentNode for $document {
            fn query_selector_first(&self, selector: &CompiledSelector) -> Option<DynamicElement> {
                self.as_web_sys_document()
                    .query_selector_first(selector.as_ref())
                    .unwrap_throw()
                    .map(|e| e.into())
            }

            fn query_selector_all(&self, selector: &CompiledSelector) -> QuerySelectorAll {
                QuerySelectorAll::new(
                    self.as_web_sys_document()
                        .query_selector_all(selector.as_ref())
                        .unwrap_trhwo(),
                )
            }

            fn child_documents(&self) -> ChildElements {
                ChildElements::new(self.as_web_sys_document().children())
            }

            fn prepend_child<T>(&self, node: &T) -> T
            where
                T: ChildNode,
            {
                self.as_web_sys_document()
                    .prepend_with_node_1(node.as_web_sys_node())
                    .unwrap_throw();
            }

            fn try_prepend_child<T>(&self, node: &T) -> Result<T, HierarchyRequestError>
            where
                T: ChildNode,
            {
                self.as_web_sys_document()
                    .prepend_with_node_1(node.as_web_sys_node())
                    .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
            }

            fn prepend_fragment<T>(&self, document_fragment: T)
            where
                T: DocumentFragment,
            {
                self.as_web_sys_document()
                    .prepend_with_node_1(node.as_web_sys_document_fragment().as_ref())
                    .unwrap_throw();
            }
        }
    };
}

use crate::collection::{Collection, Sequence};
pub(crate) use impl_parent_node_for_document;
