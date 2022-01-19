use std::marker;

#[derive(Clone)]
pub struct PointerLockChangeEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(PointerLockChangeEvent, web_sys::Event, "pointerlockchange");
