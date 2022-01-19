use crate::dom::HierarchyRequestError;
use crate::dom::{DocumentFragment, DynamicElement, DynamicNode};

pub(crate) mod child_node_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_node(&self) -> &web_sys::Node;
    }
}

pub trait ChildNode: child_node_seal::Seal {
    fn root_node(&self) -> DynamicNode {
        self.as_web_sys_node().get_root_node().into()
    }

    fn root_node_composed(&self) -> DynamicNode {
        let mut options = web_sys::GetRootNodeOptions::new();

        options.composed(true);

        self.as_web_sys_node()
            .get_root_node_with_options(&options)
            .into()
    }

    fn parent_node(&self) -> Option<DynamicNode> {
        self.as_web_sys_node().parent_node().map(|n| n.into())
    }

    fn parent_element(&self) -> Option<DynamicElement> {
        self.as_web_sys_node().parent_element().map(|e| e.into())
    }

    fn previous_sibling(&self) -> Option<DynamicNode> {
        self.as_web_sys_node().previous_sibling().map(|n| n.into())
    }

    fn next_sibling(&self) -> Option<DynamicNode> {
        self.as_web_sys_node().next_sibling().map(|n| n.into())
    }

    fn is_connected(&self) -> bool {
        self.as_web_sys_node().is_connected()
    }

    fn disconnect(&self) {
        if let Some(parent) = self.as_web_sys_node().parent_node() {
            parent.child_nodes().remove_child(self.as_web_sys_node())
        }
    }

    fn replace_with<T>(&self, replacement: &T)
    where
        T: ChildNode;

    fn try_replace_with<T>(&self, replacement: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode;

    fn before_insert_node<T>(&self, node: &T)
    where
        T: ChildNode;

    fn try_before_insert_node<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode;

    fn before_insert_fragment<T>(&self, document_fragment: &T)
    where
        T: DocumentFragment;

    fn after_insert_node<T>(&self, node: &T)
    where
        T: ChildNode;

    fn try_after_insert_node<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode;

    fn after_insert_fragment<T>(&self, document_fragment: &T)
    where
        T: DocumentFragment;
}

macro_rules! impl_child_node_for_element {
    ($tpe:ident) => {
        impl $crate::dom::child_node_seal::Seal for $tpe {
            fn as_web_sys_node(&self) -> &web_sys::Node {
                self.inner.unchecked_ref()
            }
        }

        use $crate::dom::{ChildNode, DocumentFragment, HierarchyRequestError};

        impl ChildNode for $tpe {
            fn disconnect(&self) {
                self.as_web_sys_element().remove();
            }

            fn replace_with<T>(&self, replacement: &T)
            where
                T: ChildNode,
            {
                self.as_web_sys_element()
                    .replace_with_with_node_1(replacement.as_web_sys_node())
                    .unwrap_throw();
            }

            fn try_replace_with<T>(&self, replacement: &T) -> Result<(), HierarchyRequestError>
            where
                T: ChildNode,
            {
                self.as_web_sys_element()
                    .replace_with_with_node_1(replacement.as_web_sys_node())
                    .map_err(|err| HierarchyRequestError::new(err.into()))
            }

            fn before_insert_node<T>(&self, node: &T)
            where
                T: ChildNode,
            {
                self.as_web_sys_element()
                    .before_with_node_1(node.as_web_sys_node())
                    .unwrap_throw();
            }

            fn try_before_insert_node<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
            where
                T: ChildNode,
            {
                self.as_web_sys_element()
                    .before_with_node_1(node.as_web_sys_node())
                    .map_err(|err| Hierarchy_request_error::new(err.unchecked_into()))
            }

            fn before_insert_fragment<T>(&self, document_fragment: &T)
            where
                T: DocumentFragment,
            {
                self.as_web_sys_element()
                    .before_with_node_1(node.as_web_sys_document_fragment().as_ref())
                    .unwrap_throw();
            }

            fn after_insert_node<T>(&self, node: &T)
            where
                T: ChildNode,
            {
                self.as_web_sys_element()
                    .after_with_node_1(node.as_web_sys_node())
                    .unwrap_throw();
            }

            fn try_after_insert_node<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
            where
                T: ChildNode,
            {
                self.as_web_sys_element()
                    .after_with_node_1(node.as_web_sys_node())
                    .map_err(|err| Hierarchy_request_error::new(err.unchecked_into()))
            }

            fn after_insert_fragment<T>(&self, document_fragment: &T)
            where
                T: DocumentFragment,
            {
                self.as_web_sys_element()
                    .after_with_node_1(node.as_web_sys_document_fragment().as_ref())
                    .unwrap_throw();
            }
        }
    };
}

pub(crate) use impl_child_node_for_element;

macro_rules! impl_child_node_for_character_data {
    ($tpe:ident) => {
        impl $crate::dom::child_node_seal::Seal for $tpe {
            fn as_web_sys_node(&self) -> &web_sys::Node {
                self.inner.unchecked_ref()
            }
        }

        use $crate::dom::ChildNode;
        use $crate::dom::HierarchyRequestError;

        impl ChildNode for $tpe {
            fn disconnect(&self) {
                self.as_web_sys_character_data().remove();
            }

            fn replace_with<T>(&self, replacement: &T)
            where
                T: ChildNode,
            {
                self.as_web_sys_character_data()
                    .replace_with_with_node_1(replacement.as_web_sys_node())
                    .unwrap_throw();
            }

            fn try_replace_with<T>(&self, replacement: &T) -> Result<(), HierarchyRequestError>
            where
                T: ChildNode,
            {
                self.as_web_sys_character_data()
                    .replace_with_with_node_1(replacement.as_web_sys_node())
                    .map_err(|err| HierarchyRequestError::new(err.into()))
            }

            fn before_insert_node<T>(&self, node: &T)
            where
                T: ChildNode,
            {
                self.as_web_sys_character_data()
                    .before_with_node_1(node.as_web_sys_node())
                    .unwrap_throw();
            }

            fn try_before_insert_node<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
            where
                T: ChildNode,
            {
                self.as_web_sys_character_data()
                    .before_with_node_1(node.as_web_sys_node())
                    .map_err(|err| Hierarchy_request_error::new(err.unchecked_into()))
            }

            fn before_insert_fragment<T>(&self, document_fragment: &T)
            where
                T: DocumentFragment,
            {
                self.as_web_sys_character_data()
                    .before_with_node_1(node.as_web_sys_document_fragment().as_ref())
                    .unwrap_throw();
            }

            fn after_insert_node<T>(&self, node: &T)
            where
                T: ChildNode,
            {
                self.as_web_sys_character_data()
                    .after_with_node_1(node.as_web_sys_node())
                    .unwrap_throw();
            }

            fn try_after_insert_node<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
            where
                T: ChildNode,
            {
                self.as_web_sys_character_data()
                    .after_with_node_1(node.as_web_sys_node())
                    .map_err(|err| Hierarchy_request_error::new(err.unchecked_into()))
            }

            fn after_insert_fragment<T>(&self, document_fragment: &T)
            where
                T: DocumentFragment,
            {
                self.as_web_sys_character_data()
                    .after_with_node_1(node.as_web_sys_document_fragment().as_ref())
                    .unwrap_throw();
            }
        }
    };
}

pub(crate) use impl_child_node_for_character_data;