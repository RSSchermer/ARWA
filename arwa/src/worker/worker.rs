pub(crate) mod worker_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait Worker: worker_seal::Seal {
    fn on_error(&self) -> OnError<Self> {
        OnError::new(self.as_web_sys_event_target())
    }
}

typed_event_stream!(OnError, OnErrorWithOptions, ErrorEvent, "error");
