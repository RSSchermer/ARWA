use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_val, JsCast, UnwrapThrowExt};

use crate::collection::{Collection, Sequence};
use crate::dom::{
    node_seal, ChildNode, Fragment, DynamicChildNode, DynamicElement, DynamicNode,
    HierarchyRequestError, Node, Selector,
};
use crate::event::impl_event_target_traits;

pub(crate) mod parent_node_seal {
    use crate::dom::JsParentNode;

    pub trait Seal {
        #[doc(hidden)]
        fn from_web_sys_node_unchecked(node: web_sys::Node) -> Self;

        #[doc(hidden)]
        fn as_js_parent_node(&self) -> &JsParentNode;
    }
}

/// Implemented for node types that can have an ordered list of child nodes.
pub trait ParentNode: parent_node_seal::Seal + Sized {
    /// Returns `true` if the given `node` is a descendant of this parent node, `false` otherwise.
    ///
    /// Note that the `node` does not have to be a direct child of this parent node for this
    /// function to return `true`, it may also be a child-of-a-child, a child-of-a-child-of-a-child,
    /// etc.
    fn contains<T>(&self, node: &T) -> bool
    where
        T: ChildNode,
    {
        self.as_js_parent_node()
            .contains(Some(node.as_js_child_node()))
    }

    fn query_selector(&self, selector: &Selector) -> Option<DynamicElement> {
        self.as_js_parent_node()
            .query_selector(selector.as_ref())
            .map(|e| e.into())
    }

    fn query_selector_all(&self, selector: &Selector) -> QuerySelectorAll {
        let node_list = self
            .as_js_parent_node()
            .query_selector_all(selector.as_ref());

        QuerySelectorAll::new(node_list)
    }

    /// Creates a new node that is a deep copy of the `source` node.
    ///
    /// Recursively copies the source node's entire sub-tree.
    ///
    /// See also [duplicate].
    fn duplicate_deep(source: &Self) -> Self {
        let cloned_node = source
            .as_js_parent_node()
            .clone_node_with_deep(true)
            .unwrap_throw();

        Self::from_web_sys_node_unchecked(cloned_node)
    }

    fn normalize(&self) {
        self.as_js_parent_node().normalize()
    }

    /// Returns a live collection of the all nodes that are direct children of this parent node.
    ///
    /// See also [child_elements] to retrieve only the subset of nodes that implement [Element].
    fn child_nodes(&self) -> ChildNodes {
        ChildNodes::new(self.as_js_parent_node().child_nodes())
    }

    /// Returns a live collection of only the [Element nodes that are direct children of this parent node.
    ///
    /// See also [child_elements] to retrieve only the subset of nodes that implement [Element].
    fn child_elements(&self) -> ChildElements {
        let children = self.as_js_parent_node().children();

        ChildElements::new(children)
    }

    fn prepend_child<T>(&self, node: &T)
    where
        T: ChildNode,
    {
        self.as_js_parent_node()
            .prepend(node.as_js_child_node())
            .unwrap_throw()
    }

    fn try_prepend_child<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode,
    {
        self.as_js_parent_node()
            .prepend(node.as_js_child_node())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }

    /// Prepends the document structure in the given `document_fragment` before the first child
    /// element in this node.
    ///
    /// This moves the nodes from the document fragment, leaving the document fragment empty.
    fn prepend_fragment<T>(&self, document_fragment: &T)
    where
        T: Fragment,
    {
        // Note: this should never cause a hierarchy request error.
        self.as_js_parent_node()
            .prepend(document_fragment.as_web_sys_document_fragment().as_ref())
            .unwrap_throw()
    }

    fn append_child<T>(&self, node: &T)
    where
        T: ChildNode,
    {
        if let Err(err) = self
            .as_js_parent_node()
            .append_child(node.as_js_child_node())
        {
            throw_val(err)
        }
    }

    fn try_append_child<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode,
    {
        self.as_js_parent_node()
            .append_child(node.as_js_child_node())
            .map(|_| ())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }

    /// Append the document structure in the given `document_fragment` after the last child element
    /// in this node.
    ///
    /// This moves the nodes from the document fragment, leaving the document fragment empty.
    fn append_fragment<T>(&self, document_fragment: &T)
    where
        T: Fragment,
    {
        // Note: this should never cause a hierarchy request error.
        self.as_js_parent_node()
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

impl ChildNodes {
    pub(crate) fn new(inner: web_sys::NodeList) -> Self {
        ChildNodes { inner }
    }
}

impl Collection for ChildNodes {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for ChildNodes {
    type Item = DynamicChildNode;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get(index).map(|n| DynamicChildNode::new(n))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
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
}

impl Collection for ChildElements {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for ChildElements {
    type Item = DynamicElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.item(index).map(|n| n.into())
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

pub struct QuerySelectorAll {
    inner: web_sys::NodeList,
}

impl QuerySelectorAll {
    pub(crate) fn new(inner: web_sys::NodeList) -> Self {
        QuerySelectorAll { inner }
    }
}

impl Collection for QuerySelectorAll {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for QuerySelectorAll {
    type Item = DynamicElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner
            .get(index)
            .map(|n| DynamicElement::from(n.unchecked_into::<web_sys::Element>()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

/// A [Node] that can be used as a [ChildNode], but for which a specific type is not statically
/// known.
///
/// You may try to resolve a value of this type to a more specific type using [TryFrom]. All Arwa
/// types that implement [ChildNode] also implement `TryFrom<DynamicChildNode>`.
#[derive(Clone, PartialEq)]
pub struct DynamicParentNode {
    inner: web_sys::Node,
}

impl DynamicParentNode {
    pub(crate) fn new(inner: web_sys::Node) -> Self {
        DynamicParentNode { inner }
    }
}

impl node_seal::Seal for DynamicParentNode {
    fn from_web_sys_node_unchecked(inner: web_sys::Node) -> Self {
        DynamicParentNode { inner }
    }

    fn as_web_sys_node(&self) -> &web_sys::Node {
        &self.inner
    }
}

impl Node for DynamicParentNode {}

impl parent_node_seal::Seal for DynamicParentNode {
    fn from_web_sys_node_unchecked(inner: web_sys::Node) -> Self {
        DynamicParentNode { inner }
    }

    fn as_js_parent_node(&self) -> &JsParentNode {
        self.inner.unchecked_ref()
    }
}

impl ParentNode for DynamicParentNode {}

impl AsRef<web_sys::Node> for DynamicParentNode {
    fn as_ref(&self) -> &web_sys::Node {
        &self.inner
    }
}

impl From<DynamicParentNode> for DynamicNode {
    fn from(value: DynamicParentNode) -> Self {
        DynamicNode::from(value.inner)
    }
}

impl From<DynamicParentNode> for web_sys::Node {
    fn from(value: DynamicParentNode) -> Self {
        value.inner
    }
}

impl_event_target_traits!(DynamicParentNode);

macro_rules! impl_parent_node {
    ($tpe:ident) => {
        impl $crate::dom::parent_node_seal::Seal for $tpe {
            fn from_web_sys_node_unchecked(node: web_sys::Node) -> Self {
                use wasm_bindgen::JsCast;

                $tpe {
                    inner: node.unchecked_into(),
                }
            }

            fn as_js_parent_node(&self) -> &$crate::dom::JsParentNode {
                use wasm_bindgen::JsCast;

                self.inner.unchecked_ref()
            }
        }

        impl $crate::dom::ParentNode for $tpe {}
    };
}

pub(crate) use impl_parent_node;

macro_rules! impl_try_from_parent_node {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl TryFrom<$crate::dom::DynamicParentNode> for $tpe {
            type Error = $crate::InvalidCast<$crate::dom::DynamicParentNode, $tpe>;

            fn try_from(value: $crate::dom::DynamicParentNode) -> Result<Self, Self::Error> {
                use wasm_bindgen::JsCast;

                let value: web_sys::Node = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|e| e.into())
                    .map_err(|e| $crate::InvalidCast::new($crate::dom::DynamicParentNode::new(e)))
            }
        }
    };
    ($tpe:ident) => {
        $crate::dom::impl_try_from_parent_node!($tpe, $tpe);
    };
}

pub(crate) use impl_try_from_parent_node;

#[wasm_bindgen]
extern "C" {
    #[doc(hidden)]
    #[wasm_bindgen(extends = web_sys::Node)]
    pub type JsParentNode;

    #[wasm_bindgen(method, js_name=querySelector)]
    pub(crate) fn query_selector(this: &JsParentNode, selector: &str) -> Option<web_sys::Element>;

    #[wasm_bindgen(method, js_name=querySelectorAll)]
    pub(crate) fn query_selector_all(this: &JsParentNode, selector: &str) -> web_sys::NodeList;

    #[wasm_bindgen(method)]
    pub(crate) fn children(this: &JsParentNode) -> web_sys::HtmlCollection;

    #[wasm_bindgen(catch, method)]
    pub(crate) fn prepend(this: &JsParentNode, node: &web_sys::Node) -> Result<(), JsValue>;
}
