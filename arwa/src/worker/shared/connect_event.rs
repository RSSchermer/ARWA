use crate::message::{messaging_event_seal, MessagePort, MessagingEvent};
use std::marker;

#[derive(Clone)]
pub struct ConnectEvent<T> {
    inner: web_sys::MessageEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> ConnectEvent<T> {
    pub fn port(&self) -> MessagePort {
        let port: web_sys::MessagePort = self.inner.ports().get(0).unchecked_into();

        port.into()
    }
}

impl<T> messaging_event_seal::Seal for ConnectEvent<T> {
    fn as_web_sys_message_event(&self) -> &web_sys::MessageEvent {
        &self.inner
    }
}

impl<T> MessagingEvent for ConnectEvent<T> {}

impl<T> AsRef<web_sys::MessageEvent> for ConnectEvent<T> {
    fn as_ref(&self) -> &web_sys::MessageEvent {
        &self.inner
    }
}

impl_event_traits!(ConnectEvent, web_sys::MessageEvent);
