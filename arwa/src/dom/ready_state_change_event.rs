use std::marker;

#[derive(Clone)]
pub struct ReadyStateChangeEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(ReadyStateChangeEvent, web_sys::Event, "readystatechange");
