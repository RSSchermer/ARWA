use std::marker;

use crate::event::DynamicEventTarget;
use crate::ui::impl_ui_event_traits;

pub struct FocusInEvent<T> {
    inner: web_sys::FocusEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> FocusInEvent<T> {
    /// The [EventTarget] that loses focus due to this focus transition, if any.
    ///
    /// Can be `None` for security reasons.
    pub fn blurred_target(&self) -> Option<DynamicEventTarget> {
        self.inner.related_target().map(|t| t.into())
    }
}

impl<T> AsRef<web_sys::FocusEvent> for FocusInEvent<T> {
    fn as_ref(&self) -> &web_sys::FocusEvent {
        &self.inner
    }
}

impl_ui_event_traits!(FocusInEvent, FocusEvent, "focusin");

pub struct FocusOutEvent<T> {
    inner: web_sys::FocusEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> FocusOutEvent<T> {
    /// The [EventTarget] that gains focus due to this focus transition, if any.
    ///
    /// Can be `None` for security reasons.
    pub fn focussed_target(&self) -> Option<DynamicEventTarget> {
        self.inner.related_target().map(|t| t.into())
    }
}

impl<T> AsRef<web_sys::FocusEvent> for FocusOutEvent<T> {
    fn as_ref(&self) -> &web_sys::FocusEvent {
        &self.inner
    }
}

impl_ui_event_traits!(FocusOutEvent, FocusEvent, "focusout");
