use std::marker;

#[derive(Clone)]
pub struct OnlineEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(OnlineEvent, web_sys::Event, "online");
