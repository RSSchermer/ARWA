pub(crate) mod animation_event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait AnimationEventTarget: animation_event_target_seal::Seal {
    fn on_animation_cancel(&self) -> OnAnimationCancel<Self> {
        OnAnimationCancel::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_animation_end(&self) -> OnAnimationEnd<Self> {
        OnAnimationEnd::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_animation_iteration(&self) -> OnAnimationIteration<Self> {
        OnAnimationIteration::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_animation_start(&self) -> OnAnimationStart<Self> {
        OnAnimationStart::new(self.as_web_sys_event_target().clone().into())
    }
}

typed_event_stream!(
    OnAnimationCancel,
    OnAnimationCancelWithOptions,
    AnimationCancelEvent,
    "animationcancel"
);
typed_event_stream!(
    OnAnimationEnd,
    OnAnimationEndWithOptions,
    AnimationEndEvent,
    "animationend"
);
typed_event_stream!(
    OnAnimationIteration,
    OnAnimationIterationWithOptions,
    AnimationIterationEvent,
    "animationiteration"
);
typed_event_stream!(
    OnAnimationStart,
    OnAnimationStartWithOptions,
    AnimationStartEvent,
    "animationstart"
);

macro_rules! impl_animation_event_target_for_element {
    ($element:ident) => {
        impl crate::cssom::animation_event_target_seal::Seal for $element {
            fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
                self.as_web_sys_element().as_ref()
            }
        }

        impl crate::cssom::AnimationEventTarget for $element {}
    };
}

pub(crate) use impl_animation_event_target_for_element;
