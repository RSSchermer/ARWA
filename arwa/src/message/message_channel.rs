use crate::message::MessagePort;

#[derive(Clone)]
pub struct MessageChannel {
    inner: web_sys::MessageChannel,
}

impl MessageChannel {
    // TODO: unclear how this would work with WASM threads. If this would only work on the main
    // thread, then perhaps returning a Result would be appropriate.
    pub fn new() -> Self {
        MessageChannel {
            inner: web_sys::MessageChannel::new().unwrap_throw(),
        }
    }

    pub fn port_one(&self) -> MessagePort {
        MessagePort {
            inner: self.inner.port1(),
        }
    }

    pub fn port_two(&self) -> MessagePort {
        MessagePort {
            inner: self.inner.port2(),
        }
    }
}

impl AsRef<web_sys::MessageChannel> for MessageChannel {
    fn as_ref(&self) -> &web_sys::MessageChannel {
        &self.inner
    }
}

impl From<web_sys::MessageChannel> for MessageChannel {
    fn from(inner: web_sys::MessageChannel) -> Self {
        MessageChannel { inner }
    }
}

impl_common_wrapper_traits!(MessagePort);
