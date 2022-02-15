use crate::collection::{Collection, Sequence};
use crate::html::media::VideoTrack;
use crate::media::VideoTrack;
use std::marker;

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

    pub fn on_change(&self) -> OnChangeVideoTrack {
        OnChangeVideoTrack::new(self.inner.clone().into())
    }

    pub fn on_add_track(&self) -> OnAddVideoTrack {
        OnAddVideoTrack::new(self.inner.clone().into())
    }

    pub fn on_remove_track(&self) -> OnRemoveVideoTrack {
        OnRemoveVideoTrack::new(self.inner.clone().into())
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
    pub fn track(&self) -> VideoTrack {
        VideoTrack::from(self.inner.track().unchecked_into())
    }
}

impl<T> AsRef<web_sys::TrackEvent> for AddVideoTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_event_traits!(AddVideoTrackEvent, web_sys::TrackEvent, "addtrack");

#[derive(Clone)]
pub struct RemoveVideoTrackEvent<T> {
    inner: web_sys::TrackEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> RemoveVideoTrackEvent<T> {
    pub fn track(&self) -> VideoTrack {
        VideoTrack::from(self.inner.track().unchecked_into())
    }
}

impl<T> AsRef<web_sys::TrackEvent> for RemoveVideoTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_event_traits!(RemoveVideoTrackEvent, web_sys::TrackEvent, "removetrack");

#[derive(Clone)]
pub struct ChangeVideoTrackEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl<T> AsRef<web_sys::TrackEvent> for ChangeVideoTrackEvent<T> {
    fn as_ref(&self) -> &web_sys::TrackEvent {
        &self.inner
    }
}

impl_event_traits!(ChangeVideoTrackEvent, web_sys::TrackEvent, "change");

typed_event_stream!(
    OnAddVideoTrack,
    OnAddVideoTrackWithOptions,
    AddVideoTrackEvent,
    "addtrack"
);
typed_event_stream!(
    OnRemoveVideoTrack,
    OnRemoveVideoTrackWithOptions,
    RemoveVideoTrackEvent,
    "removetrack"
);
typed_event_stream!(
    OnChangeVideoTrack,
    OnChangeVideoTrackWithOptions,
    ChangeVideoTrackEvent,
    "change"
);
