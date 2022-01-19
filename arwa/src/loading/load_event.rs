use std::marker;

#[derive(Clone)]
pub struct LoadEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(LoadEvent, web_sys::Event, "load");
