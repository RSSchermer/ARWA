#![feature(iter_intersperse)]

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

mod impl_common_wrapper_traits;
pub(crate) use impl_common_wrapper_traits::impl_common_wrapper_traits;

mod transferable;
pub use transferable::Transferable;

mod unchecked_cast_array;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InvalidCast<T>(pub T);
