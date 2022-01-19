pub(crate) mod transition_event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait TransitionEventTarget: transition_event_target_seal::Seal {
    fn on_transition_cancel(&self) -> OnTransitionCancel<Self> {
        OnTransitionCancel::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_transition_end(&self) -> OnTransitionEnd<Self> {
        OnTransitionEnd::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_transition_iteration(&self) -> OnTransitionRun<Self> {
        OnTransitionRun::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_transition_start(&self) -> OnTransitiontart<Self> {
        OnTransitionStart::new(self.as_web_sys_event_target().clone().into())
    }
}

typed_event_stream!(OnTransitionCancel, OnTransitionCancelWithOptions, TransitionCancelEvent, "transitioncancel");
typed_event_stream!(OnTransitionEnd, OnTransitionEndWithOptions, TransitionEndEvent, "transitionend");
typed_event_stream!(OnTransitionRun, OnTransitionRunWithOptions, TransitionRunEvent, "transitionrun");
typed_event_stream!(OnTransitionStart, OnTransitionStartWithOptions, TransitionStartEvent, "transitionstart");