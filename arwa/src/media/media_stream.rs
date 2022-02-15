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
        MediaStreamTracks {
            inner: self.inner.get_tracks(),
        }
    }

    pub fn audio_tracks(&self) -> MediaStreamAudioTracks {
        MediaStreamAudioTracks {
            inner: self.inner.get_audio_tracks(),
        }
    }

    pub fn video_tracks(&self) -> MediaStreamVideoTracks {
        MediaStreamVideoTracks {
            inner: self.inner.get_video_tracks(),
        }
    }

    pub fn on_add_track(&self) -> OnMediaStreamAddTrack {
        OnMediaStreamAddTrack::new(self.as_web_sys_event_target().clone().into())
    }

    pub fn on_remove_track(&self) -> OnMediaStreamRemoveTrack {
        OnMediaStreamRemoveTrack::new(self.as_web_sys_event_target().clone().into())
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
impl_try_from_event_targets!(MediaStream, web_sys::MediaStream);

unchecked_cast_array!(
    MediaStreamTrack,
    web_sys::MediaStreamTrack,
    MediaStreamTracks
);

unchecked_cast_array!(
    MediaStreamTrack,
    web_sys::MediaStreamTrack,
    MediaStreamAudioTracks
);

unchecked_cast_array!(
    MediaStreamTrack,
    web_sys::MediaStreamTrack,
    MediaStreamVideoTracks
);

macro_rules! media_stream_event {
    ($tpe:ident, $name:literal) => {
        #[derive(Clone)]
        pub struct $tpe<T> {
            inner: web_sys::MediaStreamTrackEvent,
            _marker: std::marker::PhantomData<T>,
        }

        impl<T> $tpe<T> {
            pub fn track(&self) -> MediaStreamTrack {
                self.inner.track().into()
            }
        }

        impl<T> AsRef<web_sys::MediaStreamTrackEvent> for $tpe<T> {
            fn as_ref(&self) -> &web_sys::TrackEvent {
                &self.inner
            }
        }

        impl_event_traits!($tpe, web_sys::Event, $name);
    };
}

media_stream_event!(MediaStreamAddTrackEvent, "addtrack");
media_stream_event!(MediaStreamRemoveTrackEvent, "removetrack");

typed_event_stream!(
    OnMediaStreamAddTrack,
    OnMediaStreamAddTrackWithOptions,
    MediaStreamAddTrackEvent,
    "addtrack"
);
typed_event_stream!(
    OnMediaStreamRemoveTrack,
    OnMediaStreamRemoveTrackWithOptions,
    MediaStreamRemoveTrackEvent,
    "removetrack"
);

pub enum StreamTrackReadyState {
    Live,
    Ended,
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
        match self.inner.ready_state().as_ref() {
            "live" => StreamTrackReadyState::Live,
            "ended" => StreamTrackReadyState::Ended,
            _ => unreachable!(),
        }
    }

    pub fn on_track_ended(&self) -> OnStreamTrackEnded {
        OnStreamTrackEnded::new(self.as_web_sys_event_target().clone().into())
    }

    pub fn on_track_mute(&self) -> OnStreamTrackMute {
        OnStreamTrackMute::new(self.as_web_sys_event_target().clone().into())
    }

    pub fn on_track_unmute(&self) -> OnStreamTrackUnmute {
        OnStreamTrackUnmute::new(self.as_web_sys_event_target().clone().into())
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
impl_try_from_event_targets!(StreamTrack, web_sys::MediaStreamTrack);

media_event!(StreamTrackEndedEvent, "ended");
media_event!(StreamTrackMuteEvent, "mute");
media_event!(StreamTrackUnmuteEvent, "unmute");

typed_event_stream!(
    OnStreamTrackEnded,
    OnStreamTrackEndedWithOptions,
    StreamTrackEndedEvent,
    "ended"
);
typed_event_stream!(
    OnStreamTrackMute,
    OnStreamTrackMuteWithOptions,
    StreamTrackMuteEvent,
    "mute"
);
typed_event_stream!(
    OnStreamTrackUnmute,
    OnStreamTrackUnmuteWithOptions,
    StreamTrackUnmuteEvent,
    "unmute"
);
