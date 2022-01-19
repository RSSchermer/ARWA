pub(crate) mod execution_event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait ExecutionEventTarget {
    // Note: the `error` event is overloaded in the spec for both load errors and execution errors
    // with a different event type (generic Event, ErrorEvent respectively). Though I cant find any
    // conclusive information in the spec, it seems that the only type that triggers on execution
    // context objects (Window, Worker) is the ErrorEvent kind. For now, assume that all `error`
    // events on Window/Worker are ErrorEvent events. If a counter-example is found, we might have
    // to filter the stream for ErrorEvent.
    fn on_error(&self) -> OnError<Self> {
        OnError::new(self.as_web_sys_event_target())
    }

    fn on_rejection_handled(&self) -> OnRejectionHandled<Self> {
        OnRejectionHandled::new(self.as_web_sys_event_target())
    }

    fn on_unhandled_rejection(&self) -> OnUnhandledRejection<Self> {
        OnUnhandledRejection::new(self.as_web_sys_event_target())
    }
}

typed_event_stream!(OnError, OnErrorWithOptions, ErrorEvent, "error");
typed_event_stream!(
    OnRejectionHandled,
    OnRejectionHandledWithOptions,
    RejectionHandledEvent,
    "rejectionhandled"
);
typed_event_stream!(
    OnUnhandledRejection,
    OnUnhandledRejectionWithOptions,
    UnhandledRejectionEvent,
    "unhandledrejection"
);
