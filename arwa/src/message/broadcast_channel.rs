use delegate::delegate;

use crate::event::impl_event_target_traits;
use crate::message::{
    message_event_target_seal, message_sender_seal, MessageEventTarget, MessageSender,
};

#[derive(Clone)]
pub struct BroadcastChannel {
    inner: web_sys::BroadcastChannel,
}

impl BroadcastChannel {
    // TODO: not yet supported on Safari, but seems to be in the current technology preview (dec
    // 2021). Omit constructor for now. For now, if you want to construct one, use
    // From<web_sys::BroadcastChannel>.

    delegate! {
        target self.inner {
            pub fn name(&self) -> String;
        }
    }
}

impl message_sender_seal::Seal for BroadcastChannel {}
impl MessageSender for BroadcastChannel {}

impl message_event_target_seal::Seal for BroadcastChannel {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl MessageEventTarget for BroadcastChannel {}

impl AsRef<web_sys::BroadcastChannel> for BroadcastChannel {
    fn as_ref(&self) -> &web_sys::BroadcastChannel {
        &self.inner
    }
}

impl From<web_sys::BroadcastChannel> for BroadcastChannel {
    fn from(inner: web_sys::BroadcastChannel) -> Self {
        BroadcastChannel { inner }
    }
}

impl_event_target_traits!(BroadcastChannel);
