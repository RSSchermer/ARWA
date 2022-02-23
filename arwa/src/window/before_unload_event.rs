use std::marker;

use crate::event::impl_typed_event_traits;

#[derive(Clone)]
pub struct BeforeUnloadEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(BeforeUnloadEvent, Event, "beforeunload");
