use std::marker;

#[derive(Clone)]
pub struct AppInstalledEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(AppInstalledEvent, web_sys::Event, "appinstalled");
