use std::marker;

#[derive(Clone)]
pub struct FullscreenChangeEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(FullscreenChangeEvent, web_sys::Event, "fullscreenchange");
