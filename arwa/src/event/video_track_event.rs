use wasm_bindgen::JsCast;

use crate::event::{Event, FromEvent};
use crate::VideoTrack;

pub struct VideoTrackEvent {
    inner: web_sys::TrackEvent,
}

impl VideoTrackEvent {
    pub fn track(&self) -> Option<VideoTrack> {
        self.inner.track().map(|track| {
            let track: web_sys::VideoTrack = track.unchecked_into();

            track.into()
        })
    }
}

impl_common_event_traits!(VideoTrackEvent, TrackEvent);
