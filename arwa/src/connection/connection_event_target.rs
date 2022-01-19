pub(crate) mod connection_event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait ConnectionEventTarget: connection_event_target_seal::Seal {
    fn on_offline(&self) -> OnOffline<Self> {
        OnOffline::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_online(&self) -> OnOnline<Self> {
        OnOnline::new(self.as_web_sys_event_target().clone().into())
    }
}

typed_event_stream!(OnOnline, OnOnlineWithOptions, OnlineEvent, "online");
typed_event_stream!(OnOffline, OnOfflineWithOptions, OfflineEvent, "offline");
