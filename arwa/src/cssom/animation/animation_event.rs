use delegate::delegate;

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
            _marker: std::marker::PhantomData<T>,
        }

        impl<T> animation_event_seal::Seal for $event<T> {}

        impl<T> AnimationEvent for $event<T> {
            delegate! {
                to self.inner {
                    fn animation_name(&self) -> String;

                    fn elapsed_time(&self) -> f32;
                }
            }
        }

        impl<T> AsRef<web_sys::AnimationEvent> for $event<T> {
            fn as_ref(&self) -> &web_sys::AnimationEvent {
                &self.inner
            }
        }

        $crate::event::impl_typed_event_traits!($event, AnimationEvent, $name);
    };
}

animation_event!(AnimationStartEvent, "animationstart");
animation_event!(AnimationEndEvent, "animationend");
animation_event!(AnimationIterationEvent, "animationiteration");
animation_event!(AnimationCancelEvent, "animationcancel");
