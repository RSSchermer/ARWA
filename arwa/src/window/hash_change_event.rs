use std::marker;
use url::Url;

#[derive(Clone)]
pub struct HashChangeEvent<T> {
    inner: web_sys::HashChangeEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> HashChangeEvent<T> {
    pub fn old_url(&self) -> Url {
        Url::parse(self.inner.old_url().as_ref()).unwrap()
    }

    pub fn new_url(&self) -> Url {
        Url::parse(self.inner.new_url().as_ref()).unwrap()
    }
}

impl_event_traits!(HashChangeEvent, web_sys::HashChangeEvent, "hashchange");
