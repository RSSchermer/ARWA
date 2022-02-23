use std::marker;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::ServiceWorker;

use crate::event::impl_typed_event_traits;
use crate::message::MessagePort;
use crate::window::Window;

pub enum MessageEventSource {
    Window(Window),
    MessagePort(MessagePort),
    ServiceWorker(ServiceWorker),
    Unknown,
}

pub(crate) mod messaging_event_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_message_event(&self) -> &web_sys::MessageEvent;
    }
}

pub trait MessagingEvent: messaging_event_seal::Seal {
    fn origin(&self) -> String {
        self.as_web_sys_message_event().origin()
    }

    fn last_event_id(&self) -> String {
        self.as_web_sys_message_event().last_event_id()
    }

    fn source(&self) -> MessageEventSource {
        if let Some(object) = self.as_web_sys_message_event().source() {
            if object.is_instance_of::<web_sys::Window>() {
                return MessageEventSource::Window(Window::from(
                    object.unchecked_into::<web_sys::Window>(),
                ));
            } else if object.is_instance_of::<web_sys::MessagePort>() {
                return MessageEventSource::MessagePort(MessagePort::from(
                    object.unchecked_into::<web_sys::MessagePort>(),
                ));
            } else if object.is_instance_of::<web_sys::ServiceWorker>() {
                return MessageEventSource::ServiceWorker(ServiceWorker::from(
                    object.unchecked_into::<web_sys::ServiceWorker>(),
                ));
            }

            unreachable!()
        } else {
            MessageEventSource::Unknown
        }
    }
}

#[derive(Clone)]
pub struct MessageEvent<T> {
    inner: web_sys::MessageEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> messaging_event_seal::Seal for MessageEvent<T> {
    fn as_web_sys_message_event(&self) -> &web_sys::MessageEvent {
        &self.inner
    }
}

impl<T> MessagingEvent for MessageEvent<T> {}

impl<T> MessageEvent<T> {
    pub fn data(&self) -> JsValue {
        self.inner.data()
    }
}

impl<T> AsRef<web_sys::MessageEvent> for MessageEvent<T> {
    fn as_ref(&self) -> &web_sys::MessageEvent {
        &self.inner
    }
}

impl_typed_event_traits!(MessageEvent, MessageEvent, "message");

#[derive(Clone)]
pub struct MessageErrorEvent<T> {
    inner: web_sys::MessageEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> messaging_event_seal::Seal for MessageErrorEvent<T> {
    fn as_web_sys_message_event(&self) -> &web_sys::MessageEvent {
        &self.inner
    }
}

impl<T> MessagingEvent for MessageErrorEvent<T> {}

impl<T> AsRef<web_sys::MessageEvent> for MessageErrorEvent<T> {
    fn as_ref(&self) -> &web_sys::MessageEvent {
        &self.inner
    }
}

impl_typed_event_traits!(MessageErrorEvent, MessageEvent, "messageerror");
