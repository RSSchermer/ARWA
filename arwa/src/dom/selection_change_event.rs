use std::marker;

use crate::event::impl_typed_event_traits;

// Note: only implemented for Document at the moment, though it is specced to also be available on
// textarea and input[type="text"], and it bubbles, so can be listened to on any element. However,
// only Firefox seems to support that at the moment. That behavior also seems confusing to me,
// because although you can listen for selectionchange on any element, selections inside that
// element will only

#[derive(Clone)]
pub struct SelectionChangeEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(SelectionChangeEvent, Event, "selectionchange");
