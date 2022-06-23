use delegate::delegate;

mod animation_event_seal {
    pub trait Seal {}
}

/// Implemented for animation-related event types.
pub trait AnimationEvent: animation_event_seal::Seal {
    /// Returns the [CSS animation-name](https://developer.mozilla.org/en-US/docs/Web/CSS/animation-name)
    /// of the CSS animation that generated the animation that caused this event to be emitted.
    fn animation_name(&self) -> String;

    /// The amount of time in milliseconds for which this animation has been running at the time
    /// this event was emitted.
    ///
    /// Excludes the amount of time for which the animation was paused (if any).
    fn elapsed_time(&self) -> f32;
}

macro_rules! animation_event {
    ($(#[$($doc:tt)*])* $event:ident, $name:literal) => {
        $(#[$($doc)*])*
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

animation_event! {
    /// Event emitted on [AnimationEventTarget] types when an animation begins.
    AnimationStartEvent, "animationstart"
}
animation_event! {
    /// Event emitted on [AnimationEventTarget] types when an animation completes.
    AnimationEndEvent, "animationend"
}
animation_event! {
    /// Event emitted on [AnimationEventTarget] types when an animation iteration completes.
    AnimationIterationEvent, "animationiteration"
}
animation_event! {
    /// Event emitted on [AnimationEventTarget] types when an animation is cancelled.
    AnimationCancelEvent, "animationcancel"
}
