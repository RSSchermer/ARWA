use crate::connection::OfflineEvent;
use crate::connection::OnlineEvent;
use crate::event::typed_event_iterator;

pub(crate) mod connection_event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

/// Implemented for types that emit events when their connection status changes.
pub trait ConnectionEventTarget: connection_event_target_seal::Seal + Sized {
    /// Returns an async iterator over [OfflineEvent]s emitted when the connection status changes to
    /// "offline".
    fn on_offline(&self) -> OnOffline<Self> {
        OnOffline::new(self.as_web_sys_event_target())
    }

    /// Returns an async iterator over [OnlineEvent]s emitted when the connection status changes to
    /// "online".
    fn on_online(&self) -> OnOnline<Self> {
        OnOnline::new(self.as_web_sys_event_target())
    }
}

typed_event_iterator!(OnOnline, OnOnlineWithOptions, OnlineEvent, "online");
typed_event_iterator!(OnOffline, OnOfflineWithOptions, OfflineEvent, "offline");
