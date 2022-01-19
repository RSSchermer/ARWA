pub(crate) use impl_common_wrapper_traits::impl_common_wrapper_traits;
pub use transferable::Transferable;

mod impl_common_wrapper_traits;
mod unchecked_cast_array;

pub mod collection;
pub mod connection;
pub mod console;
pub mod cssom;
pub mod dom;
pub mod event;
pub mod execution;
pub mod fetch;
pub mod file;
pub mod geolocation;
pub mod html;
pub mod loading;
pub mod message;
pub mod navigator;
pub mod performance;
pub mod scroll;
pub mod security;
pub mod storage;
pub mod timer;
pub mod ui;
pub mod url;
pub mod window;
pub mod worker;

mod audio_track;
mod crypto;
mod history;
mod image_quality;
mod transferable;
//mod indexed_collection;
//pub use indexed_collection::IndexedCollection;

mod subtle_crypto;
mod text_track;
mod video_track;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InvalidCast<T>(pub T);
