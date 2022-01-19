use std::marker;

#[derive(Clone)]
pub struct VisibilityChangeEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(VisibilityChangeEvent, web_sys::Event, "visibilitychange");
