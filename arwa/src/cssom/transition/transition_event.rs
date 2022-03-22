use delegate::delegate;

mod transition_event_seal {
    pub trait Seal {}
}

pub trait TransitionEvent: transition_event_seal::Seal {
    fn property_name(&self) -> String;

    fn elapsed_time(&self) -> f32;
}

macro_rules! transition_event {
    ($event:ident, $name:literal) => {
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

transition_event!(TransitionStartEvent, "transitionstart");
transition_event!(TransitionEndEvent, "transitionend");
transition_event!(TransitionRunEvent, "transitionrun");
transition_event!(TransitionCancelEvent, "transitioncancel");
