use std::future::Future;
use std::ops::Range;
use std::pin::Pin;
use std::task::{Context, Poll};

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;
use web_sys::TextTrackKind;

use crate::collection::{Collection, Sequence};
use crate::event::typed_event_iterator;
use crate::lang::LanguageTag;
use crate::media::{AudioTracks, MediaStream, TextTrack, TextTracks, VideoTracks};
use crate::security::CORS;
use crate::url::{AbsoluteOrRelativeUrl, Url};
use crate::{dom_exception_wrapper, normalize_exception_message};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MediaErrorKind {
    Aborted,
    Network,
    Decode,
    SrcNotSupported,
}

#[derive(Clone)]
pub struct MediaError {
    inner: web_sys::MediaError,
}

impl MediaError {
    pub fn kind(&self) -> MediaErrorKind {
        match self.inner.code() {
            1 => MediaErrorKind::Aborted,
            2 => MediaErrorKind::Network,
            3 => MediaErrorKind::Decode,
            4 => MediaErrorKind::SrcNotSupported,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for MediaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut message = self.inner.message();

        normalize_exception_message(&mut message);

        fmt::Display::fmt(&message, f)
    }
}

impl fmt::Debug for MediaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl Error for MediaError {}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MediaNetworkState {
    Empty,
    Idle,
    Loading,
    NoSource,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MediaReadyState {
    HaveNothing,
    HaveMetadata,
    HaveCurrentData,
    HaveFutureData,
    HaveEnoughData,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MediaPreload {
    None,
    Metadata,
    Auto,
}

impl Default for MediaPreload {
    fn default() -> Self {
        MediaPreload::Auto
    }
}

pub struct TextTrackDescriptor<'a> {
    pub kind: TextTrackKind,
    pub label: &'a str,
    pub language: Option<&'a LanguageTag>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CanPlayMediaType {
    Probably,
    Maybe,
    No,
}

dom_exception_wrapper!(MediaPlayError);

pub(crate) mod media_element_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_html_media_element(&self) -> &web_sys::HtmlMediaElement;
    }
}

pub trait MediaElement: media_element_seal::Seal + Sized {
    fn src(&self) -> Option<Url> {
        Url::parse(self.as_web_sys_html_media_element().src().as_ref()).ok()
    }

    fn set_src<T>(&self, src: T)
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.as_web_sys_html_media_element().set_src(src.as_str());
    }

    fn current_src(&self) -> Option<Url> {
        Url::parse(self.as_web_sys_html_media_element().current_src().as_ref()).ok()
    }

    fn src_object(&self) -> Option<MediaStream> {
        self.as_web_sys_html_media_element()
            .src_object()
            .map(|s| s.into())
    }

    fn set_src_object(&self, src_object: Option<&MediaStream>) {
        self.as_web_sys_html_media_element()
            .set_src_object(src_object.map(|s| s.as_ref()))
    }

    fn cross_origin(&self) -> CORS {
        if let Some(cross_origin) = self.as_web_sys_html_media_element().cross_origin() {
            match cross_origin.as_ref() {
                "use-credentials" => CORS::UseCredentials,
                _ => CORS::Anonymous,
            }
        } else {
            CORS::Anonymous
        }
    }

    fn set_cross_origin(&self, cross_origin: CORS) {
        let cross_origin = match cross_origin {
            CORS::Anonymous => "anonymous",
            CORS::UseCredentials => "use-credentials",
        };

        self.as_web_sys_html_media_element()
            .set_cross_origin(Some(cross_origin));
    }

    fn network_state(&self) -> MediaNetworkState {
        match self.as_web_sys_html_media_element().network_state() {
            0 => MediaNetworkState::Empty,
            1 => MediaNetworkState::Idle,
            2 => MediaNetworkState::Loading,
            4 => MediaNetworkState::NoSource,
            _ => unreachable!(),
        }
    }

    fn preload(&self) -> MediaPreload {
        match self.as_web_sys_html_media_element().preload().as_ref() {
            "none" => MediaPreload::None,
            "metadata" => MediaPreload::Metadata,
            _ => MediaPreload::Auto,
        }
    }

    fn set_preload(&self, preload: MediaPreload) {
        let preload = match preload {
            MediaPreload::Auto => "auto",
            MediaPreload::None => "none",
            MediaPreload::Metadata => "metadata",
        };

        self.as_web_sys_html_media_element().set_preload(preload);
    }

    fn buffered(&self) -> TimeRanges {
        TimeRanges {
            inner: self.as_web_sys_html_media_element().buffered(),
        }
    }

    fn seekable(&self) -> TimeRanges {
        TimeRanges {
            inner: self.as_web_sys_html_media_element().seekable(),
        }
    }

    fn ready_state(&self) -> MediaReadyState {
        match self.as_web_sys_html_media_element().ready_state() {
            0 => MediaReadyState::HaveNothing,
            1 => MediaReadyState::HaveMetadata,
            2 => MediaReadyState::HaveCurrentData,
            3 => MediaReadyState::HaveFutureData,
            4 => MediaReadyState::HaveEnoughData,
            _ => unreachable!(),
        }
    }

    fn seeking(&self) -> bool {
        self.as_web_sys_html_media_element().seeking()
    }

    fn paused(&self) -> bool {
        self.as_web_sys_html_media_element().paused()
    }

    fn ended(&self) -> bool {
        self.as_web_sys_html_media_element().ended()
    }

    fn duration(&self) -> f64 {
        self.as_web_sys_html_media_element().duration()
    }

    fn current_time(&self) -> f64 {
        self.as_web_sys_html_media_element().current_time()
    }

    fn set_current_time(&self, current_time: f64) {
        self.as_web_sys_html_media_element()
            .set_current_time(current_time);
    }

    fn default_playback_rate(&self) -> f64 {
        self.as_web_sys_html_media_element().default_playback_rate()
    }

    fn set_default_playback_rate(&self, default_playback_rate: f64) {
        self.as_web_sys_html_media_element()
            .set_default_playback_rate(default_playback_rate);
    }

    fn playback_rate(&self) -> f64 {
        self.as_web_sys_html_media_element().playback_rate()
    }

    fn set_playback_rate(&self, playback_rate: f64) {
        self.as_web_sys_html_media_element()
            .set_playback_rate(playback_rate);
    }

    fn autoplay(&self) -> bool {
        self.as_web_sys_html_media_element().autoplay()
    }

    fn set_autoplay(&self, autoplay: bool) {
        self.as_web_sys_html_media_element().set_autoplay(autoplay);
    }

    fn loops(&self) -> bool {
        self.as_web_sys_html_media_element().loop_()
    }

    fn set_loops(&self, loops: bool) {
        self.as_web_sys_html_media_element().set_loop(loops);
    }

    fn controls(&self) -> bool {
        self.as_web_sys_html_media_element().controls()
    }

    fn set_controls(&self, controls: bool) {
        self.as_web_sys_html_media_element().set_controls(controls);
    }

    fn volume(&self) -> f64 {
        self.as_web_sys_html_media_element().volume()
    }

    fn set_volume(&self, volume: f64) {
        self.as_web_sys_html_media_element().set_volume(volume);
    }

    fn muted(&self) -> bool {
        self.as_web_sys_html_media_element().muted()
    }

    fn set_muted(&self, muted: bool) {
        self.as_web_sys_html_media_element().set_muted(muted);
    }

    fn default_muted(&self) -> bool {
        self.as_web_sys_html_media_element().default_muted()
    }

    fn set_default_muted(&self, default_muted: bool) {
        self.as_web_sys_html_media_element()
            .set_default_muted(default_muted);
    }

    fn error(&self) -> Option<MediaError> {
        self.as_web_sys_html_media_element()
            .error()
            .map(|inner| MediaError { inner })
    }

    fn can_play_type(&self, media_type: &str) -> CanPlayMediaType {
        match self
            .as_web_sys_html_media_element()
            .can_play_type(media_type)
            .as_ref()
        {
            "probably" => CanPlayMediaType::Probably,
            "maybe" => CanPlayMediaType::Maybe,
            _ => CanPlayMediaType::No,
        }
    }

    fn fast_seek(&self, time: f64) {
        // There's no indication in the spec that this can actually fail, unwrap for now.
        self.as_web_sys_html_media_element()
            .fast_seek(time)
            .unwrap_throw();
    }

    fn load(&self) {
        self.as_web_sys_html_media_element().load();
    }

    fn pause(&self) {
        // There's no indication in the spec that this can actually fail, unwrap for now.
        self.as_web_sys_html_media_element().pause().unwrap_throw();
    }

    fn play(&self) -> MediaPlay {
        MediaPlay {
            element: Some(self.as_web_sys_html_media_element().clone()),
            inner: None,
        }
    }

    fn audio_tracks(&self) -> AudioTracks {
        AudioTracks::new(self.as_web_sys_html_media_element().audio_tracks())
    }

    fn video_tracks(&self) -> VideoTracks {
        VideoTracks::new(self.as_web_sys_html_media_element().video_tracks())
    }

    fn text_tracks(&self) -> TextTracks {
        // For some reason web_sys returns an Option here. `textTracks` is not specced to be
        // nullable so we just unwrap.
        TextTracks::new(
            self.as_web_sys_html_media_element()
                .text_tracks()
                .unwrap_throw(),
        )
    }

    fn add_text_track(&self, descriptor: TextTrackDescriptor) -> TextTrack {
        let TextTrackDescriptor {
            kind,
            label,
            language,
        } = descriptor;

        self.as_web_sys_html_media_element()
            .add_text_track_with_label_and_language(
                kind,
                label,
                language.map(|l| l.as_ref()).unwrap_or(""),
            )
            .into()
    }

    fn on_abort(&self) -> OnAbort<Self> {
        OnAbort::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_can_play(&self) -> OnCanPlay<Self> {
        OnCanPlay::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_can_play_through(&self) -> OnCanPlayThrough<Self> {
        OnCanPlayThrough::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_duration_change(&self) -> OnDurationChange<Self> {
        OnDurationChange::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_emptied(&self) -> OnEmptied<Self> {
        OnEmptied::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_ended(&self) -> OnEnded<Self> {
        OnEnded::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_error(&self) -> OnError<Self> {
        OnError::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_loaded_metadata(&self) -> OnLoadedMetadata<Self> {
        OnLoadedMetadata::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_load_start(&self) -> OnLoadStart<Self> {
        OnLoadStart::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_pause(&self) -> OnPause<Self> {
        OnPause::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_play(&self) -> OnPlay<Self> {
        OnPlay::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_playing(&self) -> OnPlaying<Self> {
        OnPlaying::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_progress(&self) -> OnProgress<Self> {
        OnProgress::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_rate_change(&self) -> OnRateChange<Self> {
        OnRateChange::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_seeked(&self) -> OnSeeked<Self> {
        OnSeeked::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_seeking(&self) -> OnSeeking<Self> {
        OnSeeking::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_stalled(&self) -> OnStalled<Self> {
        OnStalled::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_suspend(&self) -> OnSuspend<Self> {
        OnSuspend::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_time_update(&self) -> OnTimeUpdate<Self> {
        OnTimeUpdate::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_volume_change(&self) -> OnVolumeChange<Self> {
        OnVolumeChange::new(self.as_web_sys_html_media_element().as_ref())
    }

    fn on_waiting(&self) -> OnWaiting<Self> {
        OnWaiting::new(self.as_web_sys_html_media_element().as_ref())
    }
}

macro_rules! impl_html_media_element_traits {
    ($tpe:ident) => {
        impl $crate::html::media_element_seal::Seal for $tpe {
            fn as_web_sys_html_media_element(&self) -> &web_sys::HtmlMediaElement {
                &self.inner
            }
        }

        impl $crate::html::MediaElement for $tpe {}

        $crate::html::impl_html_element_traits!($tpe);
        $crate::dom::impl_try_from_element!($tpe);
    };
}

pub(crate) use impl_html_media_element_traits;
use std::error::Error;
use std::fmt;

#[must_use = "futures do nothing unless polled or spawned"]
pub struct MediaPlay {
    element: Option<web_sys::HtmlMediaElement>,
    inner: Option<JsFuture>,
}

impl Future for MediaPlay {
    type Output = Result<(), MediaPlayError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(element) = self.element.take() {
            // No indication this is fallible in the spec (though the promise can reject).
            self.inner = Some(element.play().unwrap_throw().into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner
            .poll(cx)
            .map_ok(|_| ())
            .map_err(|err| MediaPlayError::new(err.unchecked_into()))
    }
}

pub struct TimeRanges {
    inner: web_sys::TimeRanges,
}

impl Collection for TimeRanges {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for TimeRanges {
    type Item = Range<f64>;

    fn get(&self, index: u32) -> Option<Self::Item> {
        if let Some(start) = self.inner.start(index).ok() {
            let end = self.inner.end(index).unwrap_throw();

            Some(start..end)
        } else {
            None
        }
    }

    fn to_host_array(&self) -> js_sys::Array {
        todo!()
    }
}

macro_rules! media_event {
    ($tpe:ident, $name:literal) => {
        #[derive(Clone)]
        pub struct $tpe<T> {
            inner: web_sys::Event,
            _marker: std::marker::PhantomData<T>,
        }

        $crate::event::impl_typed_event_traits!($tpe, Event, $name);
    };
}

media_event!(AbortEvent, "abort");
media_event!(CanPlayEvent, "canplay");
media_event!(CanPlayThroughEvent, "canplaythrough");
media_event!(DurationChangeEvent, "durationchange");
media_event!(EmptiedEvent, "emptied");
media_event!(EndedEvent, "ended");
media_event!(ErrorEvent, "error");
media_event!(LoadedDataEvent, "loadeddata");
media_event!(LoadedMetadataEvent, "loadedmetadata");
media_event!(LoadStartEvent, "loadstart");
media_event!(PauseEvent, "pause");
media_event!(PlayEvent, "play");
media_event!(PlayingEvent, "playing");
media_event!(ProgressEvent, "progress");
media_event!(RateChangeEvent, "ratechange");
media_event!(SeekedEvent, "seeked");
media_event!(SeekingEvent, "seeking");
media_event!(StalledEvent, "stalled");
media_event!(SuspendEvent, "suspend");
media_event!(TimeUpdateEvent, "timeupdate");
media_event!(VolumeChangeEvent, "volumechange");
media_event!(WaitingEvent, "waiting");

typed_event_iterator!(OnAbort, OnAbortWithOptions, AbortEvent, "abort");
typed_event_iterator!(OnCanPlay, OnCanPlayWithOptions, CanPlayEvent, "canplay");
typed_event_iterator!(
    OnCanPlayThrough,
    OnCanPlayThroughWithOptions,
    CanPlayThroughEvent,
    "canplaythrough"
);
typed_event_iterator!(
    OnDurationChange,
    OnDurationChangeWithOptions,
    DurationChangeEvent,
    "durationchange"
);
typed_event_iterator!(OnEmptied, OnEmptiedWithOptions, EmptiedEvent, "emptied");
typed_event_iterator!(OnEnded, OnEndedWithOptions, EndedEvent, "ended");
typed_event_iterator!(OnError, OnErrorWithOptions, ErrorEvent, "error");
typed_event_iterator!(
    OnLoadedMetadata,
    OnLoadedMetadataWithOptions,
    LoadedMetadataEvent,
    "loadedmetadata"
);
typed_event_iterator!(
    OnLoadStart,
    OnLoadStartWithOptions,
    LoadStartEvent,
    "loadstart"
);
typed_event_iterator!(OnPause, OnPauseWithOptions, PauseEvent, "pause");
typed_event_iterator!(OnPlay, OnPlayWithOptions, PlayEvent, "play");
typed_event_iterator!(OnPlaying, OnPlayingWithOptions, PlayingEvent, "playing");
typed_event_iterator!(OnProgress, OnProgressWithOptions, ProgressEvent, "progress");
typed_event_iterator!(
    OnRateChange,
    OnRateChangeWithOptions,
    RateChangeEvent,
    "ratechange"
);
typed_event_iterator!(OnSeeked, OnSeekedWithOptions, SeekedEvent, "seeked");
typed_event_iterator!(OnSeeking, OnSeekingWithOptions, SeekingEvent, "seeking");
typed_event_iterator!(OnStalled, OnStalledWithOptions, StalledEvent, "stalled");
typed_event_iterator!(OnSuspend, OnSuspendWithOptions, SuspendEvent, "suspend");
typed_event_iterator!(
    OnTimeUpdate,
    OnTimeUpdateWithOptions,
    TimeUpdateEvent,
    "timeupdate"
);
typed_event_iterator!(
    OnVolumeChange,
    OnVolumeChangeWithOptions,
    VolumeChangeEvent,
    "volumechange"
);
typed_event_iterator!(OnWaiting, OnWaitingWithOptions, WaitingEvent, "waiting");
