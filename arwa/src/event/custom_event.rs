use std::any::TypeId;
use std::marker;
use std::mem;
use std::ops::Deref;

use js_sys::{Object, Uint8Array};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use crate::console::{Argument, ToArgument};
use crate::event::{event_seal, Event, EventTarget, EventType, EventTypeInternal, TypedEvent};

#[derive(Clone)]
pub struct CustomEvent<T> {
    inner: web_sys::CustomEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> CustomEvent<T> {
    pub fn detail(&self) -> JsValue {
        self.inner.detail()
    }
}

impl<T> event_seal::Seal for CustomEvent<T>
where
    T: EventTarget,
{
    fn from_web_sys_event_unchecked(event: web_sys::Event) -> Self {
        CustomEvent {
            inner: event.unchecked_into(),
            _marker: marker::PhantomData,
        }
    }

    fn as_web_sys_event(&self) -> &web_sys::Event {
        self.inner.as_ref()
    }
}

impl<T> Event for CustomEvent<T>
where
    T: EventTarget,
{
    type CurrentTarget = T;
}

impl<T> AsRef<web_sys::Event> for CustomEvent<T> {
    fn as_ref(&self) -> &web_sys::Event {
        self.inner.as_ref()
    }
}

impl<T> AsRef<Object> for CustomEvent<T> {
    fn as_ref(&self) -> &Object {
        self.inner.as_ref()
    }
}

impl<T> AsRef<JsValue> for CustomEvent<T> {
    fn as_ref(&self) -> &JsValue {
        self.inner.as_ref()
    }
}

impl<T> ToArgument for CustomEvent<T> {
    fn to_argument(&self) -> Argument {
        let as_js_value: &JsValue = self.as_ref();

        ToArgument::to_argument(as_js_value)
    }
}

#[derive(Clone)]
pub struct TypedCustomEvent<D, T> {
    inner: web_sys::CustomEvent,
    data_ptr: *const D,
    _marker: marker::PhantomData<T>,
}

impl<D, T> event_seal::Seal for TypedCustomEvent<D, T>
where
    T: EventTarget,
    D: 'static,
{
    fn from_web_sys_event_unchecked(event: web_sys::Event) -> Self {
        let event: web_sys::CustomEvent = event.unchecked_into();

        let pointer_data: Uint8Array = event.detail().unchecked_into();

        // Copy pointer data to WASM linear memory that we can operate on. The pointer data is a
        // `*mut dyn Any`, but here we know the concrete type `D`, so we only need to address
        // pointer and ignore the vtable pointer.
        let mut scratch = [0u8; 16];
        let size_of_usize = mem::size_of::<usize>();

        pointer_data.copy_to(&mut scratch[..size_of_usize * 2]);

        let address_bytes = &scratch[..size_of_usize];
        let address_usize = usize::from_ne_bytes(address_bytes.try_into().unwrap_throw());

        // Note that this pointer will be valid for the lifetime of this TypedCustomEvent, as it
        // holds a strong reference to the browser owned CustomEvent in `inner`. We only drop the
        // data this pointer points to via a FinalizationRegistry callback triggered after the
        // browser owned CustomEvent gets identified for garbage collection, which can only happen
        // after this TypedCustomEvent is dropped and its strong reference to the browser owned
        // CustomEvent is thus released.
        let data_ptr = <*const D>::from_bits(address_usize);

        TypedCustomEvent {
            inner: event,
            data_ptr,
            _marker: marker::PhantomData,
        }
    }

    fn as_web_sys_event(&self) -> &web_sys::Event {
        self.inner.as_ref()
    }
}

impl<D, T> Event for TypedCustomEvent<D, T>
where
    T: EventTarget,
    D: 'static,
{
    type CurrentTarget = T;
}

impl<D, T> TypedEvent for TypedCustomEvent<D, T>
where
    T: EventTarget,
    D: 'static,
{
    const EVENT_TYPE: EventType = EventType {
        internal: EventTypeInternal::TypeId(TypeId::of::<D>()),
    };
}

impl<D, T> Deref for TypedCustomEvent<D, T>
where
    D: 'static,
{
    type Target = D;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data_ptr }
    }
}

impl<D, T> AsRef<web_sys::Event> for TypedCustomEvent<D, T> {
    fn as_ref(&self) -> &web_sys::Event {
        self.inner.as_ref()
    }
}

impl<D, T> AsRef<Object> for TypedCustomEvent<D, T> {
    fn as_ref(&self) -> &Object {
        self.inner.as_ref()
    }
}

impl<D, T> AsRef<JsValue> for TypedCustomEvent<D, T> {
    fn as_ref(&self) -> &JsValue {
        self.inner.as_ref()
    }
}
