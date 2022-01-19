use crate::message::message_event_target::message_event_target_seal;
use crate::message::{
    message_sender_seal, MessageEventTarget, MessageSender, OnMessage, OnMessageError,
};
use crate::transferable::transferable_seal;
use crate::Transferable;
use wasm_bindgen::JsValue;

pub struct MessagePort {
    pub(crate) inner: web_sys::MessagePort,
}

impl transferable_seal::Seal for MessagePort {}
impl Transferable for MessagePort {}

impl message_sender_seal::Seal for MessagePort {}
impl MessageSender for MessagePort {}

impl message_event_target_seal::Seal for MessagePort {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl MessageEventTarget for MessagePort {}

impl AsRef<web_sys::MessagePort> for MessagePort {
    fn as_ref(&self) -> &web_sys::MessagePort {
        &self.inner
    }
}

impl From<web_sys::MessagePort> for MessagePort {
    fn from(inner: web_sys::MessagePort) -> Self {
        MessagePort { inner }
    }
}

impl_common_event_target_traits!(MessagePort);
impl_common_wrapper_traits!(MessagePort);
