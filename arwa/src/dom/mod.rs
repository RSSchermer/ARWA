//! The Document Object Model.
//!
//! The Document Object Model (DOM)is a collection of interfaces that permit access to and
//! manipulation of the structure of and information state of documents. Documents consist of
//! [Node]s, where certain node types can act as [ParentNode]s that may have an ordered collection
//! of associated [ChildNode]s, thus creating a hierarchical tree structure. A [Document] node acts
//! as a top level container of a DOM document. It may have a single [Element] child node that acts
//! as the "root" node of the document tree. The DOM API allows you to traverse and change the
//! structure of a document and its tree.
//!
//! Document standards such as [html] and [svg] build on top of the DOM, defining their own sets
//! of node types and associated behaviors.
//!
//! # The Window Associated Document
//!
//! The window associated document is the document the browser engine will attempt to present to the
//! user. A reference to the document associated with a browser [Window] can be obtained by calling
//! the [Window::document] method:
//!
//! ```
//! use arwa::window::window;
//! use arwa::dom::DynamicDocument;
//!
//! let window = window();
//! let document: DynamicDocument = window.document();
//! ```
//!
//! This returns a [DynamicDocument] value that represents a document reference of which the
//! specific document type is not statically known. You may use [TryFrom] / [TryInto] conversion
//! to convert a [DynamicDocument] into its specific document type:
//!
//! ```
//! # use arwa::window::window;
//! # use arwa::dom::DynamicDocument;
//! # let window = window();
//! # let document: DynamicDocument = window.document();
//! use arwa::html::HtmlDocument;
//!
//! let html_document: HtmlDocument = document.try_into().expect("not an HTML document");
//! ```
//!
//! Note that the window associated document can only change when the window is navigated to a new
//! URL.
//!
//! See also [Window::document].
//!
//! # Invalid DOM hierarchies
//!
//! Certain manipulations of the DOM tree can result in invalid DOM hierarchies. See
//! [HierarchyRequestError] for details on how an operation might produce an invalid hierarchy.

mod attribute;
pub use self::attribute::*;

mod cdata_section;
pub use self::cdata_section::*;

mod character_data;
pub use self::character_data::*;

mod child_node;
pub use self::child_node::*;

mod comment;
pub use self::comment::*;

mod document;
pub use self::document::*;

mod document_fragment;
pub use self::document_fragment::*;

mod document_type;
pub use self::document_type::*;

mod element;
pub use self::element::*;

mod fullscreen_change_event;
pub use self::fullscreen_change_event::*;

mod fullscreen_error_event;
pub use self::fullscreen_error_event::*;

mod hierarchy_request_error;
pub use self::hierarchy_request_error::*;

mod name;
pub use self::name::*;

mod node;
pub use self::node::*;

mod non_colon_name;
pub use self::non_colon_name::*;

mod owned_node;
pub use self::owned_node::*;

mod element_sibling;
pub use self::element_sibling::*;

mod parent_node;
pub use self::parent_node::*;

mod pointer_lock_change_event;
pub use self::pointer_lock_change_event::*;

mod pointer_lock_error_event;
pub use self::pointer_lock_error_event::*;

mod processing_instruction;
pub use self::processing_instruction::*;

mod qualified_name;
pub use self::qualified_name::*;

mod range;
pub use self::range::*;

mod ready_state_change_event;
pub use self::ready_state_change_event::*;

mod selection;
pub use self::selection::*;

mod selection_change_event;
pub use self::selection_change_event::*;

mod shadow_root;
pub use self::shadow_root::*;

mod text;
pub use self::text::*;

mod text_directionality;
pub use self::text_directionality::*;

mod token;
pub use self::token::*;

mod selector;
pub use selector::*;

mod visibility_change_event;
pub use self::visibility_change_event::*;

pub use arwa_macro::name;

pub use arwa_macro::non_colon_name;

pub use arwa_macro::qualified_name;

pub use arwa_macro::selector;

pub use arwa_macro::token;
