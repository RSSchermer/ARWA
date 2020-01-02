use std::convert::TryFrom;

use crate::event::on_event::FromEvent;
use wasm_bindgen::JsCast;

pub trait Event: AsRef<web_sys::Event> {
    fn bubbles(&self) -> bool {
        self.as_ref().bubbles()
    }

    fn cancelable(&self) -> bool {
        self.as_ref().cancelable()
    }

    fn composed(&self) -> bool {
        self.as_ref().composed()
    }

    fn is_trusted(&self) -> bool {
        self.as_ref().is_trusted()
    }

    fn default_prevented(&self) -> bool {
        self.as_ref().default_prevented()
    }

    fn event_phase(&self) -> EventPhase {
        match self.as_ref().event_phase() {
            web_sys::Event::NONE => EventPhase::None,
            web_sys::Event::CAPTURING_PHASE => EventPhase::CapturingPhase,
            web_sys::Event::AT_TARGET => EventPhase::AtTarget,
            web_sys::Event::BUBBLING_PHASE => EventPhase::BubblingPhase,
            _ => unreachable!(),
        }
    }

    fn event_type(&self) -> String {
        self.as_ref().type_()
    }

    fn target(&self) -> Option<GenericEventTarget> {
        self.as_ref().target().map(|t| t.into())
    }

    fn current_target(&self) -> Option<GenericEventTarget> {
        self.as_ref().current_target().map(|t| t.into())
    }

    fn composed_path(&self) -> ComposedPath {
        ComposedPath {
            inner: self.as_ref().composed_path(),
        }
    }

    fn prevent_default(&self) {
        self.as_ref().prevent_default()
    }

    fn stop_propagation(&self) {
        self.as_ref().stop_propagation()
    }

    fn stop_immediate_propagation(&self) {
        self.as_ref().stop_immediate_propagation()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EventPhase {
    None,
    CapturingPhase,
    AtTarget,
    BubblingPhase,
}

pub struct GenericEventTarget {
    inner: web_sys::EventTarget,
}

impl From<web_sys::EventTarget> for GenericEventTarget {
    fn from(inner: web_sys::EventTarget) -> Self {
        GenericEventTarget { inner }
    }
}

impl AsRef<web_sys::EventTarget> for GenericEventTarget {
    fn as_ref(&self) -> &web_sys::EventTarget {
        &self.inner
    }
}

pub struct ComposedPath {
    inner: js_sys::Array,
}

impl ComposedPath {
    pub fn get(&self, index: usize) -> Option<GenericEventTarget> {
        u32::try_from(index).ok().map(|index| GenericEventTarget {
            inner: self.inner.get(index).unchecked_into(),
        })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<GenericEventTarget> {
        self.get(0)
    }

    pub fn last(&self) -> Option<GenericEventTarget> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }
}

impl IntoIterator for ComposedPath {
    type Item = GenericEventTarget;
    type IntoIter = ComposedPathIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        ComposedPathIntoIter {
            composed_path: self,
            current: 0,
        }
    }
}

pub struct ComposedPathIter<'a> {
    composed_path: &'a ComposedPath,
    current: usize,
}

impl<'a> Iterator for ComposedPathIter<'a> {
    type Item = GenericEventTarget;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.composed_path.get(current)
    }
}

pub struct ComposedPathIntoIter {
    composed_path: ComposedPath,
    current: usize,
}

impl Iterator for ComposedPathIntoIter {
    type Item = GenericEventTarget;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.composed_path.get(current)
    }
}

pub struct GenericEvent {
    inner: web_sys::Event,
}

impl From<web_sys::Event> for GenericEvent {
    fn from(inner: web_sys::Event) -> Self {
        GenericEvent { inner }
    }
}

impl FromEvent for GenericEvent {
    fn from_event(inner: web_sys::Event) -> Self {
        GenericEvent { inner }
    }
}

impl AsRef<web_sys::Event> for GenericEvent {
    fn as_ref(&self) -> &web_sys::Event {
        &self.inner
    }
}

impl Event for GenericEvent {}
