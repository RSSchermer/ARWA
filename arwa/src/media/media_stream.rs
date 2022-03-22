use delegate::delegate;
use web_sys::MediaStreamTrack;

use crate::event::{impl_event_target_traits, impl_try_from_event_target, typed_event_iterator};
use crate::unchecked_cast_array::unchecked_cast_array;

#[derive(Clone)]
pub struct MediaStream {
    inner: web_sys::MediaStream,
}

impl MediaStream {
    delegate! {
        to self.inner {
            pub fn id(&self) -> String;

            pub fn active(&self) -> bool;
        }
    }

    pub fn lookup(&self, track_id: &str) -> Option<StreamTrack> {
        self.inner.get_track_by_id(track_id).map(|t| t.into())
    }

    pub fn insert(&self, track: &StreamTrack) {
        self.inner.add_track(track.as_ref());
    }

    pub fn remove(&self, track: &StreamTrack) {
        self.inner.remove_track(track.as_ref());
    }

    pub fn tracks(&self) -> MediaStreamTracks {
        MediaStreamTracks::new(self.inner.get_tracks())
    }

    pub fn audio_tracks(&self) -> MediaStreamAudioTracks {
        MediaStreamAudioTracks::new(self.inner.get_audio_tracks())
    }

    pub fn video_tracks(&self) -> MediaStreamVideoTracks {
        MediaStreamVideoTracks::new(self.inner.get_video_tracks())
    }

    pub fn on_add_track(&self) -> OnMediaStreamAddTrack<Self> {
        OnMediaStreamAddTrack::new(self.inner.as_ref())
    }

    pub fn on_remove_track(&self) -> OnMediaStreamRemoveTrack<Self> {
        OnMediaStreamRemoveTrack::new(self.inner.as_ref())
    }
}

impl From<web_sys::MediaStream> for MediaStream {
    fn from(inner: web_sys::MediaStream) -> Self {
        MediaStream { inner }
    }
}

impl AsRef<web_sys::MediaStream> for MediaStream {
    fn as_ref(&self) -> &web_sys::MediaStream {
        &self.inner
    }
}

impl_event_target_traits!(MediaStream);
impl_try_from_event_target!(MediaStream);

unchecked_cast_array!(StreamTrack, MediaStreamTrack, MediaStreamTracks);

unchecked_cast_array!(StreamTrack, MediaStreamTrack, MediaStreamAudioTracks);

unchecked_cast_array!(StreamTrack, MediaStreamTrack, MediaStreamVideoTracks);

macro_rules! media_stream_event {
    ($tpe:ident, $name:literal) => {
        #[derive(Clone)]
        pub struct $tpe<T> {
            inner: web_sys::MediaStreamTrackEvent,
            _marker: std::marker::PhantomData<T>,
        }

        impl<T> $tpe<T> {
            pub fn track(&self) -> StreamTrack {
                self.inner.track().into()
            }
        }

        impl<T> AsRef<web_sys::MediaStreamTrackEvent> for $tpe<T> {
            fn as_ref(&self) -> &web_sys::MediaStreamTrackEvent {
                &self.inner
            }
        }

        $crate::event::impl_typed_event_traits!($tpe, MediaStreamTrackEvent, $name);
    };
}

media_stream_event!(MediaStreamAddTrackEvent, "addtrack");
media_stream_event!(MediaStreamRemoveTrackEvent, "removetrack");

typed_event_iterator!(
    OnMediaStreamAddTrack,
    OnMediaStreamAddTrackWithOptions,
    MediaStreamAddTrackEvent,
    "addtrack"
);
typed_event_iterator!(
    OnMediaStreamRemoveTrack,
    OnMediaStreamRemoveTrackWithOptions,
    MediaStreamRemoveTrackEvent,
    "removetrack"
);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StreamTrackReadyState {
    Live,
    Ended,
}

impl StreamTrackReadyState {
    fn from_web_sys(ready_state: web_sys::MediaStreamTrackState) -> Self {
        match ready_state {
            web_sys::MediaStreamTrackState::Live => StreamTrackReadyState::Live,
            web_sys::MediaStreamTrackState::Ended => StreamTrackReadyState::Ended,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct StreamTrack {
    inner: web_sys::MediaStreamTrack,
}

impl StreamTrack {
    // TODO: constraints and capabilities

    delegate! {
        to self.inner {
            pub fn id(&self) -> String;

            pub fn label(&self) -> String;

            pub fn enabled(&self) -> bool;

            pub fn set_enabled(&self, value: bool);

            pub fn muted(&self) -> bool;

            pub fn stop(&self);
        }
    }

    pub fn ready_state(&self) -> StreamTrackReadyState {
        StreamTrackReadyState::from_web_sys(self.inner.ready_state())
    }

    pub fn on_track_ended(&self) -> OnStreamTrackEnded<Self> {
        OnStreamTrackEnded::new(self.inner.as_ref())
    }

    pub fn on_track_mute(&self) -> OnStreamTrackMute<Self> {
        OnStreamTrackMute::new(self.inner.as_ref())
    }

    pub fn on_track_unmute(&self) -> OnStreamTrackUnmute<Self> {
        OnStreamTrackUnmute::new(self.inner.as_ref())
    }
}

impl From<web_sys::MediaStreamTrack> for StreamTrack {
    fn from(inner: web_sys::MediaStreamTrack) -> Self {
        StreamTrack { inner }
    }
}

impl AsRef<web_sys::MediaStreamTrack> for StreamTrack {
    fn as_ref(&self) -> &web_sys::MediaStreamTrack {
        &self.inner
    }
}

impl_event_target_traits!(StreamTrack);
impl_try_from_event_target!(StreamTrack, MediaStreamTrack);

macro_rules! media_track_event {
    ($tpe:ident, $name:literal) => {
        #[derive(Clone)]
        pub struct $tpe<T> {
            inner: web_sys::Event,
            _marker: std::marker::PhantomData<T>,
        }

        $crate::event::impl_typed_event_traits!($tpe, Event, $name);
    };
}

media_track_event!(StreamTrackEndedEvent, "ended");
media_track_event!(StreamTrackMuteEvent, "mute");
media_track_event!(StreamTrackUnmuteEvent, "unmute");

typed_event_iterator!(
    OnStreamTrackEnded,
    OnStreamTrackEndedWithOptions,
    StreamTrackEndedEvent,
    "ended"
);
typed_event_iterator!(
    OnStreamTrackMute,
    OnStreamTrackMuteWithOptions,
    StreamTrackMuteEvent,
    "mute"
);
typed_event_iterator!(
    OnStreamTrackUnmute,
    OnStreamTrackUnmuteWithOptions,
    StreamTrackUnmuteEvent,
    "unmute"
);
