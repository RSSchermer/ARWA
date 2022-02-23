use std::marker;

use crate::event::impl_typed_event_traits;

#[derive(Clone)]
pub struct OnlineEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(OnlineEvent, Event, "online");
