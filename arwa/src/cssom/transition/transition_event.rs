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
            _marker: marker::PhantomData<T>
        }

        impl<T> transition_event_seal::Seal for $event<T> {}

        impl<T> TransitionEvent for $event<T> {
            delegate! {
                target self.inner {
                    fn property_name(&self) -> String;

                    fn elapsed_time(&self) -> f32;
                }
            }
        }

        impl AsRef<web_sys::TransitionEvent> for $event {
            fn as_ref(&self) -> web_sys::TransitionEvent {
                &self.inner
            }
        }

        impl_event_traits!($event, web_sys::TransitionEvent, $name);
    }
}

transition_event!(TransitionStart, "transitionstart");
transition_event!(TransitionEnd, "transitionend");
transition_event!(TransitionRun, "transitionrun");
transition_event!(TransitionCancel, "transitioncancel");
