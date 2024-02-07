#![feature(
    box_into_inner,
    const_type_id,
    get_mut_unchecked,
    iter_intersperse,
    ptr_metadata
)]

pub use wasm_bindgen_futures::spawn_local;

pub(crate) use self::exception_wrapper::*;
pub(crate) use self::impl_common_wrapper_traits::*;
pub(crate) use self::impl_js_cast::*;
pub use self::invalid_cast::*;
pub use self::transferable::*;

pub mod collection;
pub mod compression;
pub mod connection;
pub mod console;
pub mod crypto;
pub mod cssom;
pub mod dom;
pub mod event;
pub mod execution;
pub mod fetch;
pub mod file;
pub mod file_system;
pub mod geolocation;
pub mod history;
pub mod html;
pub mod image_bitmap;
pub mod indexed_db;
pub mod lang;
pub mod loading;
pub mod media;
pub mod media_type;
pub mod message;
pub mod navigator;
pub mod performance;
pub mod scroll;
pub mod security;
pub mod storage;
pub mod storage_manager;
pub mod stream;
pub mod subtle_crypto;
pub mod timer;
pub mod ui;
pub mod url;
pub mod window;
pub mod worker;

mod exception_wrapper;
mod finalization_registry;
mod impl_common_wrapper_traits;
mod impl_js_cast;
mod invalid_cast;
mod js_serialize;
mod transferable;
mod unchecked_cast_array;
mod util;
mod weak_ref;
