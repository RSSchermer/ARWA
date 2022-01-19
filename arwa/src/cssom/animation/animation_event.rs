mod animation_event_seal {
    pub trait Seal {}
}

pub trait AnimationEvent: animation_event_seal::Seal {
    fn animation_name(&self) -> String;

    fn elapsed_time(&self) -> f32;
}

macro_rules! animation_event {
    ($event:ident, $name:literal) => {
        #[derive(Clone)]
        pub struct $event<T> {
            inner: web_sys::AnimationEvent,
            _marker: marker::PhantomData<T>,
        }

        impl<T> animation_event_seal::Seal for $event<T> {}

        impl<T> AnimationEvent for $event<T> {
            delegate! {
                target self.inner {
                    fn animation_name(&self) -> String;

                    fn elapsed_time(&self) -> f32;
                }
            }
        }

        impl AsRef<web_sys::AnimationEvent> for $event {
            fn as_ref(&self) -> web_sys::AnimationEvent {
                &self.inner
            }
        }

        impl_event_traits!($event, web_sys::AnimationEvent, $name);
    };
}

animation_event!(AnimationStart, "animationstart");
animation_event!(AnimationEnd, "animationend");
animation_event!(AnimationIteration, "animationiteration");
animation_event!(AnimationCancel, "animationcancel");
