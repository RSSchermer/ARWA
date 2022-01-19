use std::marker;

#[derive(Clone)]
pub struct ErrorEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(ErrorEvent, web_sys::Event, "error");
