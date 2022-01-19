use std::marker;

#[derive(Clone)]
pub struct BeforeUnloadEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(BeforeUnloadEvent, web_sys::Event, "beforeunload");
