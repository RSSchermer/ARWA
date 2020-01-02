use wasm_bindgen::JsCast;

use crate::event::{Event, FromEvent};
use crate::AudioTrack;

pub struct AudioTrackEvent {
    inner: web_sys::TrackEvent,
}

impl AudioTrackEvent {
    pub fn track(&self) -> Option<AudioTrack> {
        self.inner.track().map(|track| {
            let track: web_sys::AudioTrack = track.unchecked_into();

            track.into()
        })
    }
}

impl_common_event_traits!(AudioTrackEvent, TrackEvent);
