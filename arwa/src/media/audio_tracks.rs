use std::marker;

use wasm_bindgen::JsCast;

use crate::collection::{Collection, Sequence};
use crate::event::{impl_typed_event_traits, typed_event_iterator};
use crate::media::AudioTrack;

pub struct AudioTracks {
    inner: web_sys::AudioTrackList,
}

impl AudioTracks {
    pub(crate) fn new(inner: web_sys::AudioTrackList) -> Self {
        AudioTracks { inner }
    }

    pub fn lookup(&self, id: &str) -> Option<AudioTrack> {
        self.inner.get_track_by_id(id).map(|t| t.into())
    }

    pub fn on_change(&self) -> OnChangeAudioTrack<Self> {
        OnChangeAudioTrack::new(self.inner.as_ref())
    }

    pub fn on_add_track(&self) -> OnAddAudioTrack<Self> {
        OnAddAudioTrack::new(self.inner.as_ref())
    }

    pub fn on_remove_track(&self) -> OnRemoveAudioTrack<Self> {
        OnRemoveAudioTrack::new(self.inner.as_ref())
    }
}

impl Collection for AudioTracks {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for AudioTracks {
    type Item = AudioTrack;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get(index).map(|t| t.into())
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

#[derive(Clone)]
pub struct AddAudioTrackEvent<T> {
    inner: web_sys::TrackEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> AddAudioTrackEvent<T> {
    pub fn track(&self) -> Option<AudioTrack> {
        self.inner
            .track()
            .map(|t| AudioTrack::from(t.unchecked_into::<web_sys::AudioTrack>()))
    }
}

impl<T> AsRef<web_sys::TrackEvent> for AddAudioTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_typed_event_traits!(AddAudioTrackEvent, TrackEvent, "addtrack");

#[derive(Clone)]
pub struct RemoveAudioTrackEvent<T> {
    inner: web_sys::TrackEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> RemoveAudioTrackEvent<T> {
    pub fn track(&self) -> Option<AudioTrack> {
        self.inner
            .track()
            .map(|t| AudioTrack::from(t.unchecked_into::<web_sys::AudioTrack>()))
    }
}

impl<T> AsRef<web_sys::TrackEvent> for RemoveAudioTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_typed_event_traits!(RemoveAudioTrackEvent, TrackEvent, "removetrack");

#[derive(Clone)]
pub struct ChangeAudioTrackEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(ChangeAudioTrackEvent, Event, "change");

typed_event_iterator!(
    OnAddAudioTrack,
    OnAddAudioTrackWithOptions,
    AddAudioTrackEvent,
    "addtrack"
);
typed_event_iterator!(
    OnRemoveAudioTrack,
    OnRemoveAudioTrackWithOptions,
    RemoveAudioTrackEvent,
    "removetrack"
);
typed_event_iterator!(
    OnChangeAudioTrack,
    OnChangeAudioTrackWithOptions,
    ChangeAudioTrackEvent,
    "change"
);
