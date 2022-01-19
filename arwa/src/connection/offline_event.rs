use std::marker;

#[derive(Clone)]
pub struct OfflineEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(OfflineEvent, web_sys::Event, "offline");
