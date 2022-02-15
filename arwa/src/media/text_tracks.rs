use crate::collection::{Collection, Sequence};
use crate::html::media::TextTrack;
use crate::media::TextTrack;
use std::marker;

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

    pub fn on_change(&self) -> OnChangeTextTrack {
        OnChangeTextTrack::new(self.inner.clone().into())
    }

    pub fn on_add_track(&self) -> OnAddTextTrack {
        OnAddTextTrack::new(self.inner.clone().into())
    }

    pub fn on_remove_track(&self) -> OnRemoveTextTrack {
        OnRemoveTextTrack::new(self.inner.clone().into())
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
    pub fn track(&self) -> TextTrack {
        TextTrack::from(self.inner.track().unchecked_into())
    }
}

impl<T> AsRef<web_sys::TrackEvent> for AddTextTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_event_traits!(AddTextTrackEvent, web_sys::TrackEvent, "addtrack");

#[derive(Clone)]
pub struct RemoveTextTrackEvent<T> {
    inner: web_sys::TrackEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> RemoveTextTrackEvent<T> {
    pub fn track(&self) -> TextTrack {
        TextTrack::from(self.inner.track().unchecked_into())
    }
}

impl<T> AsRef<web_sys::TrackEvent> for RemoveTextTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_event_traits!(RemoveTextTrackEvent, web_sys::TrackEvent, "removetrack");

#[derive(Clone)]
pub struct ChangeTextTrackEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl<T> AsRef<web_sys::TrackEvent> for ChangeTextTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_event_traits!(ChangeTextTrackEvent, web_sys::TrackEvent, "change");

typed_event_stream!(
    OnAddTextTrack,
    OnAddTextTrackWithOptions,
    AddTextTrackEvent,
    "addtrack"
);
typed_event_stream!(
    OnRemoveTextTrack,
    OnRemoveTextTrackWithOptions,
    RemoveTextTrackEvent,
    "removetrack"
);
typed_event_stream!(
    OnChangeTextTrack,
    OnChangeTextTrackWithOptions,
    ChangeTextTrackEvent,
    "change"
);
