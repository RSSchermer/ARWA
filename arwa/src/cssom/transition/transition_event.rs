use delegate::delegate;

mod transition_event_seal {
    pub trait Seal {}
}

pub trait TransitionEvent: transition_event_seal::Seal {
    fn property_name(&self) -> String;

    fn elapsed_time(&self) -> f32;
}

macro_rules! transition_event {
    ($(#[$($doc:tt)*])* $event:ident, $name:literal) => {
        $(#[$($doc)*])*
        #[derive(Clone)]
        pub struct $event<T> {
            inner: web_sys::TransitionEvent,
            _marker: std::marker::PhantomData<T>,
        }

        impl<T> transition_event_seal::Seal for $event<T> {}

        impl<T> TransitionEvent for $event<T> {
            delegate! {
                to self.inner {
                    fn property_name(&self) -> String;

                    fn elapsed_time(&self) -> f32;
                }
            }
        }

        impl<T> AsRef<web_sys::TransitionEvent> for $event<T> {
            fn as_ref(&self) -> &web_sys::TransitionEvent {
                &self.inner
            }
        }

        $crate::event::impl_typed_event_traits!($event, TransitionEvent, $name);
    };
}

transition_event!{
    /// Event emitted on [TransitionEventTarget] types when a transition begins.
    ///
    /// Differs from a [TransitionRunEvent] in that a [TransitionStartEvent] is emitted after
    /// the initial delay (if any), whereas a [TransitionRunEvent] is emitted before any such
    /// initial delay.
    TransitionStartEvent, "transitionstart"
}
transition_event!{
    /// Event emitted on [TransitionEventTarget] types when a transition completes.
    TransitionEndEvent, "transitionend"
}
transition_event!{
    /// Event emitted on [TransitionEventTarget] types when a transition is first created.
    ///
    /// Differs from a [TransitionStartEvent] in that a [TransitionStartEvent] is emitted after
    /// the initial delay (if any), whereas a [TransitionRunEvent] is emitted before any such
    /// initial delay.
    TransitionRunEvent, "transitionrun"
}
transition_event!{
    /// Event emitted on [TransitionEventTarget] types when a transition is cancelled.
    TransitionCancelEvent, "transitioncancel"
}
