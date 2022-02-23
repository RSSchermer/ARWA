use crate::event::typed_event_iterator;
use crate::message::{MessageErrorEvent, MessageEvent};

pub(crate) mod message_event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait MessageEventTarget: message_event_target_seal::Seal + Sized {
    fn on_message(&self) -> OnMessage<Self> {
        OnMessage::new(self.as_web_sys_event_target())
    }

    fn on_message_error(&self) -> OnMessageError<Self> {
        OnMessageError::new(self.as_web_sys_event_target())
    }
}

typed_event_iterator!(OnMessage, OnMessageWithOptions, MessageEvent, "message");
typed_event_iterator!(
    OnMessageError,
    OnMessageErrorWithOptions,
    MessageErrorEvent,
    "messageerror"
);
