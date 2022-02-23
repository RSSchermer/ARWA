use std::marker;

use wasm_bindgen::JsCast;

use crate::collection::{Collection, Sequence};
use crate::event::{impl_typed_event_traits, typed_event_iterator};
use crate::media::TextTrack;

pub struct TextTracks {
    inner: web_sys::TextTrackList,
}

impl TextTracks {
    pub(crate) fn new(inner: web_sys::TextTrackList) -> Self {
        TextTracks { inner }
    }

    pub fn lookup(&self, id: &str) -> Option<TextTrack> {
        self.inner.get_track_by_id(id).map(|t| t.into())
    }

    pub fn on_change(&self) -> OnChangeTextTrack<Self> {
        OnChangeTextTrack::new(self.inner.as_ref())
    }

    pub fn on_add_track(&self) -> OnAddTextTrack<Self> {
        OnAddTextTrack::new(self.inner.as_ref())
    }

    pub fn on_remove_track(&self) -> OnRemoveTextTrack<Self> {
        OnRemoveTextTrack::new(self.inner.as_ref())
    }
}

impl Collection for TextTracks {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for TextTracks {
    type Item = TextTrack;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get(index).map(|t| t.into())
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

#[derive(Clone)]
pub struct AddTextTrackEvent<T> {
    inner: web_sys::TrackEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> AddTextTrackEvent<T> {
    pub fn track(&self) -> Option<TextTrack> {
        self.inner
            .track()
            .map(|t| TextTrack::from(t.unchecked_into::<web_sys::TextTrack>()))
    }
}

impl<T> AsRef<web_sys::TrackEvent> for AddTextTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_typed_event_traits!(AddTextTrackEvent, TrackEvent, "addtrack");

#[derive(Clone)]
pub struct RemoveTextTrackEvent<T> {
    inner: web_sys::TrackEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> RemoveTextTrackEvent<T> {
    pub fn track(&self) -> Option<TextTrack> {
        self.inner
            .track()
            .map(|t| TextTrack::from(t.unchecked_into::<web_sys::TextTrack>()))
    }
}

impl<T> AsRef<web_sys::TrackEvent> for RemoveTextTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_typed_event_traits!(RemoveTextTrackEvent, TrackEvent, "removetrack");

#[derive(Clone)]
pub struct ChangeTextTrackEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(ChangeTextTrackEvent, Event, "change");

typed_event_iterator!(
    OnAddTextTrack,
    OnAddTextTrackWithOptions,
    AddTextTrackEvent,
    "addtrack"
);
typed_event_iterator!(
    OnRemoveTextTrack,
    OnRemoveTextTrackWithOptions,
    RemoveTextTrackEvent,
    "removetrack"
);
typed_event_iterator!(
    OnChangeTextTrack,
    OnChangeTextTrackWithOptions,
    ChangeTextTrackEvent,
    "change"
);
