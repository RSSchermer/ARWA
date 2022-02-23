use crate::event::impl_event_target_traits;
use crate::message::{
    message_event_target_seal, message_sender_seal, MessageEventTarget, MessageSender,
};
use crate::{transferable_seal, Transferable};

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

impl_event_target_traits!(MessagePort);
