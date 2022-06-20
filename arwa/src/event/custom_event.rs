use std::any::{Any, TypeId};
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ptr::DynMetadata;
use std::{marker, ptr};

use js_sys::{Object, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};

use crate::console::{Argument, ToArgument};
use crate::event::{event_seal, Event, EventTarget, EventType, EventTypeInternal, TypedEvent};
use crate::js_serialize::js_deserialize;

pub(crate) struct CustomEventData {
    pub(crate) address: *mut (),
    pub(crate) metadata: DynMetadata<dyn Any>,
}

impl CustomEventData {
    pub(crate) fn to_dyn_any_ptr(&self) -> *mut dyn Any {
        ptr::from_raw_parts_mut(self.address, self.metadata)
    }
}

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

        let serialized: Uint8Array = event.detail().unchecked_into();

        let mut custom_event_data = MaybeUninit::<CustomEventData>::uninit();
        let ptr = custom_event_data.as_mut_ptr() as *mut ();

        js_deserialize(&wasm_bindgen::memory(), ptr, &serialized);

        let custom_event_data = unsafe { custom_event_data.assume_init() };

        // Note that this pointer will be valid for the lifetime of this TypedCustomEvent, as it
        // holds a strong reference to the browser owned CustomEvent in `inner`. We only drop the
        // data this pointer points to via a FinalizationRegistry callback triggered after the
        // browser owned CustomEvent gets identified for garbage collection, which can only happen
        // after this TypedCustomEvent is dropped and its strong reference to the browser owned
        // CustomEvent is thus released.
        let data_ptr = custom_event_data.address as *const D;

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
