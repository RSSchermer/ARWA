#![feature(const_type_id, get_mut_unchecked, iter_intersperse, ptr_to_from_bits)]

pub mod collection;
pub mod connection;
pub mod console;
pub mod crypto;
pub mod cssom;
pub mod dom;
pub mod event;
pub mod execution;
pub mod fetch;
pub mod file;
pub mod geolocation;
pub mod history;
pub mod html;
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
pub mod subtle_crypto;
pub mod timer;
pub mod ui;
pub mod url;
pub mod window;
pub mod worker;

pub use wasm_bindgen_futures::spawn_local;

mod exception_wrapper;
pub(crate) use self::exception_wrapper::*;

mod impl_common_wrapper_traits;
pub(crate) use self::impl_common_wrapper_traits::*;

mod invalid_cast;
pub use self::invalid_cast::*;

mod transferable;
pub use self::transferable::*;

mod finalization_registry;
mod unchecked_cast_array;
mod util;
mod weak_ref;
