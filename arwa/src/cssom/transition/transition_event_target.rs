use crate::cssom::{
    TransitionCancelEvent, TransitionEndEvent, TransitionRunEvent, TransitionStartEvent,
};
use crate::event::typed_event_iterator;

pub(crate) mod transition_event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait TransitionEventTarget: transition_event_target_seal::Seal + Sized {
    fn on_transition_cancel(&self) -> OnTransitionCancel<Self> {
        OnTransitionCancel::new(self.as_web_sys_event_target())
    }

    fn on_transition_end(&self) -> OnTransitionEnd<Self> {
        OnTransitionEnd::new(self.as_web_sys_event_target())
    }

    fn on_transition_iteration(&self) -> OnTransitionRun<Self> {
        OnTransitionRun::new(self.as_web_sys_event_target())
    }

    fn on_transition_start(&self) -> OnTransitionStart<Self> {
        OnTransitionStart::new(self.as_web_sys_event_target())
    }
}

typed_event_iterator!(
    OnTransitionCancel,
    OnTransitionCancelWithOptions,
    TransitionCancelEvent,
    "transitioncancel"
);
typed_event_iterator!(
    OnTransitionEnd,
    OnTransitionEndWithOptions,
    TransitionEndEvent,
    "transitionend"
);
typed_event_iterator!(
    OnTransitionRun,
    OnTransitionRunWithOptions,
    TransitionRunEvent,
    "transitionrun"
);
typed_event_iterator!(
    OnTransitionStart,
    OnTransitionStartWithOptions,
    TransitionStartEvent,
    "transitionstart"
);

macro_rules! impl_transition_event_target_for_element {
    ($element:ident) => {
        impl crate::cssom::transition_event_target_seal::Seal for $element {
            fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
                use crate::dom::element_seal::Seal;

                self.as_web_sys_element().as_ref()
            }
        }

        impl crate::cssom::TransitionEventTarget for $element {}
    };
}

pub(crate) use impl_transition_event_target_for_element;
