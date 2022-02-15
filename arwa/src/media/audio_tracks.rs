use crate::collection::{Collection, Sequence};
use crate::html::media::AudioTrack;
use crate::media::AudioTrack;
use std::marker;

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

    pub fn on_change(&self) -> OnChangeAudioTrack {
        OnChangeAudioTrack::new(self.inner.clone().into())
    }

    pub fn on_add_track(&self) -> OnAddAudioTrack {
        OnAddAudioTrack::new(self.inner.clone().into())
    }

    pub fn on_remove_track(&self) -> OnRemoveAudioTrack {
        OnRemoveAudioTrack::new(self.inner.clone().into())
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
    pub fn track(&self) -> AudioTrack {
        AudioTrack::from(self.inner.track().unchecked_into())
    }
}

impl<T> AsRef<web_sys::TrackEvent> for AddAudioTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_event_traits!(AddAudioTrackEvent, web_sys::TrackEvent, "addtrack");

#[derive(Clone)]
pub struct RemoveAudioTrackEvent<T> {
    inner: web_sys::TrackEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> RemoveAudioTrackEvent<T> {
    pub fn track(&self) -> AudioTrack {
        AudioTrack::from(self.inner.track().unchecked_into())
    }
}

impl<T> AsRef<web_sys::TrackEvent> for RemoveAudioTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_event_traits!(RemoveAudioTrackEvent, web_sys::TrackEvent, "removetrack");

#[derive(Clone)]
pub struct ChangeAudioTrackEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl<T> AsRef<web_sys::TrackEvent> for ChangeAudioTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_event_traits!(ChangeAudioTrackEvent, web_sys::TrackEvent, "change");

typed_event_stream!(
    OnAddAudioTrack,
    OnAddAudioTrackWithOptions,
    AddAudioTrackEvent,
    "addtrack"
);
typed_event_stream!(
    OnRemoveAudioTrack,
    OnRemoveAudioTrackWithOptions,
    RemoveAudioTrackEvent,
    "removetrack"
);
typed_event_stream!(
    OnChangeAudioTrack,
    OnChangeAudioTrackWithOptions,
    ChangeAudioTrackEvent,
    "change"
);
