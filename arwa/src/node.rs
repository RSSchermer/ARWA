use std::convert::TryFrom;

use bitflags::bitflags;
use wasm_bindgen::JsCast;

use crate::element::GenericElement;
use crate::error::HierarchyRequestError;

bitflags! {
    pub struct CompareDocumentPosition: u16 {
        const DISCONNECTED = 0b00000001;
        const PRECEDING = 0b00000010;
        const FOLLOWING = 0b00000100;
        const CONTAINS = 0b00001000;
        const CONTAINED_BY = 0b000010000;
        const IMPLEMENTATION_SPECIFIC = 0b00100000;
    }
}

#[repr(u16)]
pub enum NodeType {
    Element = 1,
    Attribute = 2,
    Text = 3,
    CDataSection = 4,
    EntityReference = 5,
    Entity = 6,
    ProcessingInstruction = 7,
    Comment = 8,
    Document = 9,
    DocumentType = 10,
    DocumentFragment = 11,
    Notation = 12,
}

pub trait Node: AsRef<web_sys::Node> {
    fn base_uri(&self) -> Option<String> {
        self.as_ref().base_uri().ok().flatten()
    }

    fn is_connected(&self) -> bool {
        self.as_ref().is_connected()
    }

    fn node_type(&self) -> NodeType {
        match self.as_ref().node_type() {
            1 => NodeType::Element,
            2 => NodeType::Attribute,
            3 => NodeType::Text,
            4 => NodeType::CDataSection,
            5 => NodeType::EntityReference,
            6 => NodeType::Entity,
            7 => NodeType::ProcessingInstruction,
            8 => NodeType::Comment,
            9 => NodeType::Document,
            10 => NodeType::DocumentType,
            11 => NodeType::DocumentFragment,
            12 => NodeType::Notation,
            _ => unreachable!(),
        }
    }

    fn node_value(&self) -> Option<String> {
        self.as_ref().node_value()
    }

    fn set_node_value(&self, value: Option<&str>) {
        self.as_ref().set_node_value(value)
    }

    fn text_content(&self) -> Option<String> {
        self.as_ref().text_content()
    }

    fn set_text_content(&self, content: Option<&str>) {
        self.as_ref().set_text_content(content)
    }

    fn compare_document_position<T>(&self, other: &T) -> CompareDocumentPosition
    where
        T: Node,
    {
        let pos = self.as_ref().compare_document_position(other.as_ref());

        CompareDocumentPosition::from_bits_truncate(pos)
    }

    fn is_default_namespace(&self, namespace: &str) -> bool {
        self.as_ref().is_default_namespace(Some(namespace))
    }

    fn lookup_namespace_uri(&self, prefix: Option<&str>) -> Option<String> {
        self.as_ref().lookup_namespace_uri(prefix)
    }

    fn lookup_prefix(&self, namespace: &str) -> Option<String> {
        self.as_ref().lookup_prefix(Some(namespace))
    }

    fn normalize(&self) {
        self.as_ref().normalize()
    }

    fn root_node(&self) -> GenericNode {
        self.as_ref().get_root_node().into()
    }

    fn root_node_composed(&self) -> GenericNode {
        let mut options = web_sys::GetRootNodeOptions::new();

        options.composed(true);

        self.as_ref().get_root_node_with_options(&options).into()
    }

    fn parent_node(&self) -> Option<GenericNode> {
        self.as_ref().parent_node().map(|n| n.into())
    }

    fn parent_element(&self) -> Option<GenericElement> {
        self.as_ref().parent_element().map(|e| e.into())
    }

    fn previous_sibling_node(&self) -> Option<GenericNode> {
        self.as_ref().previous_sibling().map(|n| n.into())
    }

    fn next_sibling_node(&self) -> Option<GenericNode> {
        self.as_ref().next_sibling().map(|n| n.into())
    }

    fn child_nodes(&self) -> ChildNodes {
        ChildNodes {
            parent: self.as_ref(),
            children: self.as_ref().child_nodes(),
        }
    }

    // TODO:
    // - insert before/after
    // - owner_document
    // - figure out what to do about is_equal, is_same with regards to PartialEq
    // - figure out what to do about actually cloning nodes (rather than cloning a node handle with
    //   the `Clone` trait). Perhaps a `create_from`/`clone_from` method?
}

#[derive(Clone)]
pub struct ChildNodes<'a> {
    parent: &'a web_sys::Node,
    children: web_sys::NodeList,
}

impl<'a> ChildNodes<'a> {
    pub fn get(&self, index: usize) -> Option<GenericNode> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.children.get(index).map(|n| n.into()))
    }

    pub fn first(&self) -> Option<GenericNode> {
        self.get(0)
    }

    pub fn last(&self) -> Option<GenericNode> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn contains<T>(&self, node: &T) -> bool
    where
        T: Node,
    {
        self.parent.contains(Some(node.as_ref()))
    }

    pub fn append<T>(&self, node: &T)
    where
        T: Node,
    {
        // TODO: error or panic?
        self.parent
            .append_child(node.as_ref())
            .map_err(|err| {
                // The only error that should be able to occur here is a "HierarchyRequestError".
                let err: web_sys::DomException = err.unchecked_into();

                HierarchyRequestError::new(err)
            })
            .unwrap();
    }

    // TODO: prepend missing from web_sys. Emulate with `insert_before` with `first`?

    pub fn remove<T>(&self, node: &T)
    where
        T: Node,
    {
        // TODO: error or panic?
        self.parent
            .remove_child(node.as_ref())
            .expect("The node to be removed is not a child of this node.");
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn len(&self) -> usize {
        self.children.length() as usize
    }

    pub fn iter(&self) -> ChildNodesIter {
        ChildNodesIter {
            child_nodes: self.clone(),
            current: 0,
        }
    }
}

impl<'a> IntoIterator for ChildNodes<'a> {
    type Item = GenericNode;
    type IntoIter = ChildNodesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChildNodesIter {
            child_nodes: self,
            current: 0,
        }
    }
}

pub struct ChildNodesIter<'a> {
    child_nodes: ChildNodes<'a>,
    current: usize,
}

impl<'a> Iterator for ChildNodesIter<'a> {
    type Item = GenericNode;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.child_nodes.get(current)
    }
}

pub struct GenericNode {
    inner: web_sys::Node,
}

impl AsRef<web_sys::Node> for GenericNode {
    fn as_ref(&self) -> &web_sys::Node {
        &self.inner
    }
}

impl From<web_sys::Node> for GenericNode {
    fn from(inner: web_sys::Node) -> Self {
        GenericNode { inner }
    }
}

impl From<GenericNode> for web_sys::Node {
    fn from(value: GenericNode) -> Self {
        value.inner
    }
}

impl Node for GenericNode {}
