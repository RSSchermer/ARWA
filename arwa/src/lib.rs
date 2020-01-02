pub mod error;
pub mod event;
pub mod html;

mod attribute;
pub use self::attribute::*;

mod audio_track;
pub use self::audio_track::*;

mod cors;
pub use self::cors::*;

mod crypto;
pub use self::crypto::*;

mod css_style_declaration;
pub use self::css_style_declaration::*;

mod document;
pub use self::document::*;

mod document_fragment;
pub use self::document_fragment::*;

mod document_type;
pub use self::document_type::*;

mod element;
pub use self::element::*;

mod geolocation;
pub use self::geolocation::*;

mod global_event_handlers;
pub use self::global_event_handlers::*;

mod history;
pub use self::history::*;

mod image_quality;
pub use self::image_quality::*;

mod location;
pub use self::location::*;

//mod indexed_collection;
//pub use indexed_collection::IndexedCollection;

mod navigator;
pub use self::navigator::*;

mod node;
pub use self::node::*;

mod performance;
pub use self::performance::*;

mod pointer_id;
pub use self::pointer_id::*;

mod query_selector_all;
pub use self::query_selector_all::*;

mod referrer_policy;
pub use self::referrer_policy::*;

mod screen;
pub use self::screen::*;

mod scroll;
pub use self::scroll::*;

mod selection_direction;
pub use self::selection_direction::*;

mod service_worker;
pub use self::service_worker::*;

mod storage;
pub use self::storage::*;

mod style_sheet;
pub use self::style_sheet::*;

mod subtle_crypto;
pub use self::subtle_crypto::*;

mod text_directionality;
pub use self::text_directionality::*;

mod text_track;
pub use self::text_track::*;

mod text_wrap;
pub use self::text_wrap::*;

mod video_track;
pub use self::video_track::*;

mod window;
pub use self::window::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InvalidCast<T>(pub T);
