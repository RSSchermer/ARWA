use std::marker;

#[derive(Clone)]
pub struct FullscreenErrorEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(FullscreenErrorEvent, web_sys::Event, "fullscreenerror");
