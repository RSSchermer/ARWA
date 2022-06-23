use crate::dom_exception_wrapper;

dom_exception_wrapper! {
    /// Error that is returned when an operation would result in an invalid DOM tree hierarchy.
    ///
    /// DOM modifying operations that would result in one of the following DOM state will result in
    /// this error:
    ///
    /// - The operation would result in a cycle, when an inserted node is an ancestor of the node
    ///   it is to be inserted into.
    /// - The operation results in a [Text] node becoming a child of a document.
    /// - The operation would result in a [DocumentType] node becoming a child of a node that is
    ///   not a [Document] node.
    /// - The operation would result in a [Document] node having more than one [Element] child node.
    HierarchyRequestError
}
