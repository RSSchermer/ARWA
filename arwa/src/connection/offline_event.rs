use std::marker;

use crate::event::impl_typed_event_traits;

/// Event fired on [ConnectionEventTarget] types when the connection that changes to "offline".
#[derive(Clone)]
pub struct OfflineEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(OfflineEvent, Event, "offline");
