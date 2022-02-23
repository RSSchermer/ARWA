use std::marker;

use wasm_bindgen::JsCast;

use crate::collection::{Collection, Sequence};
use crate::event::{impl_typed_event_traits, typed_event_iterator};
use crate::media::VideoTrack;

pub struct VideoTracks {
    inner: web_sys::VideoTrackList,
}

impl VideoTracks {
    pub(crate) fn new(inner: web_sys::VideoTrackList) -> Self {
        VideoTracks { inner }
    }

    pub fn lookup(&self, id: &str) -> Option<VideoTrack> {
        self.inner.get_track_by_id(id).map(|t| t.into())
    }

    pub fn selected_track(&self) -> Option<VideoTrack> {
        let selected_index = self.inner.selected_index();

        match selected_index {
            -1 => None,
            _ => self.get(selected_index as u32),
        }
    }

    pub fn on_change(&self) -> OnChangeVideoTrack<Self> {
        OnChangeVideoTrack::new(self.inner.as_ref())
    }

    pub fn on_add_track(&self) -> OnAddVideoTrack<Self> {
        OnAddVideoTrack::new(self.inner.as_ref())
    }

    pub fn on_remove_track(&self) -> OnRemoveVideoTrack<Self> {
        OnRemoveVideoTrack::new(self.inner.as_ref())
    }
}

impl Collection for VideoTracks {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for VideoTracks {
    type Item = VideoTrack;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get(index).map(|t| t.into())
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

#[derive(Clone)]
pub struct AddVideoTrackEvent<T> {
    inner: web_sys::TrackEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> AddVideoTrackEvent<T> {
    pub fn track(&self) -> Option<VideoTrack> {
        self.inner
            .track()
            .map(|t| VideoTrack::from(t.unchecked_into::<web_sys::VideoTrack>()))
    }
}

impl<T> AsRef<web_sys::TrackEvent> for AddVideoTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_typed_event_traits!(AddVideoTrackEvent, TrackEvent, "addtrack");

#[derive(Clone)]
pub struct RemoveVideoTrackEvent<T> {
    inner: web_sys::TrackEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> RemoveVideoTrackEvent<T> {
    pub fn track(&self) -> Option<VideoTrack> {
        self.inner
            .track()
            .map(|t| VideoTrack::from(t.unchecked_into::<web_sys::VideoTrack>()))
    }
}

impl<T> AsRef<web_sys::TrackEvent> for RemoveVideoTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_typed_event_traits!(RemoveVideoTrackEvent, TrackEvent, "removetrack");

#[derive(Clone)]
pub struct ChangeVideoTrackEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(ChangeVideoTrackEvent, Event, "change");

typed_event_iterator!(
    OnAddVideoTrack,
    OnAddVideoTrackWithOptions,
    AddVideoTrackEvent,
    "addtrack"
);
typed_event_iterator!(
    OnRemoveVideoTrack,
    OnRemoveVideoTrackWithOptions,
    RemoveVideoTrackEvent,
    "removetrack"
);
typed_event_iterator!(
    OnChangeVideoTrack,
    OnChangeVideoTrackWithOptions,
    ChangeVideoTrackEvent,
    "change"
);
