use std::marker;

use crate::event::impl_typed_event_traits;

mod page_transition_event_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_page_transition_event(&self) -> &web_sys::PageTransitionEvent;
    }
}

pub trait PageTransitionEvent: page_transition_event_seal::Seal {
    fn persisted(&self) -> bool {
        self.as_web_sys_page_transition_event().persisted()
    }
}

#[derive(Clone)]
pub struct PageShowEvent<T> {
    inner: web_sys::PageTransitionEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> page_transition_event_seal::Seal for PageShowEvent<T> {
    fn as_web_sys_page_transition_event(&self) -> &web_sys::PageTransitionEvent {
        &self.inner
    }
}

impl<T> PageTransitionEvent for PageShowEvent<T> {}

impl_typed_event_traits!(PageShowEvent, PageTransitionEvent, "pageshow");

#[derive(Clone)]
pub struct PageHideEvent<T> {
    inner: web_sys::PageTransitionEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> page_transition_event_seal::Seal for PageHideEvent<T> {
    fn as_web_sys_page_transition_event(&self) -> &web_sys::PageTransitionEvent {
        &self.inner
    }
}

impl<T> PageTransitionEvent for PageHideEvent<T> {}

impl_typed_event_traits!(PageHideEvent, PageTransitionEvent, "pagehide");
