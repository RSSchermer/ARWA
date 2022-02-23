use std::marker;

use crate::event::impl_typed_event_traits;

#[derive(Clone)]
pub struct BeforePrintEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(BeforePrintEvent, Event, "beforeprint");

#[derive(Clone)]
pub struct AfterPrintEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(AfterPrintEvent, Event, "afterprint");
