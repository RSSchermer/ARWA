use std::marker;

#[derive(Clone)]
pub struct BeforePrintEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(BeforePrintEvent, web_sys::Event, "beforeprint");

#[derive(Clone)]
pub struct AfterPrintEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(AfterPrintEvent, web_sys::Event, "afterprint");
