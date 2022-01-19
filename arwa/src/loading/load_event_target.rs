pub(crate) mod load_event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait LoadEventTarget: load_event_target_seal::Seal {
    fn on_load(&self) -> OnLoad<Self> {
        OnLoad::new(self.as_web_sys_event_target())
    }

    fn on_error(&self) -> OnError<Self> {
        OnError::new(self.as_web_sys_event_target())
    }
}

typed_event_stream!(OnLoad, OnLoadWithOptions, LoadEvent, "load");
typed_event_stream!(OnError, OnErrorWithOptions, ErrorEvent, "error");
