use std::marker;

#[derive(Clone)]
pub struct PointerLockErrorEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(PointerLockErrorEvent, web_sys::Event, "pointerlockerror");
