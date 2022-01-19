use std::marker;

#[derive(Clone)]
pub struct PopStateEvent<T> {
    inner: web_sys::PopStateEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> PopStateEvent<T> {
    delegate! {
        to self.inner {
            pub fn state(&self) -> JsValue;
        }
    }
}

impl_event_traits!(PopStateEvent, web_sys::PopStateEvent, "popstate");
