use std::marker;

use crate::event::impl_typed_event_traits;

#[derive(Clone)]
pub struct FullscreenErrorEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(FullscreenErrorEvent, Event, "fullscreenerror");
