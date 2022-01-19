use std::future::Future;
use std::pin::Pin;

use futures::task::{Context, Poll};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::event::{
    OnAddAudioTrack, OnAddVideoTrack, OnChange, OnRemoveAudioTrack, OnRemoveVideoTrack,
};
use crate::{AudioTrack, TextTrack, VideoTrack, CORS};

use crate::console::{Write, Writer};
use std::convert::TryFrom;
pub use web_sys::TextTrackKind;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MediaError {
    Aborted(String),
    NetworkError(String),
    DecodingError(String),
    NotSupported(String),
}

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

pub struct TextTrackDescriptor<'a, 'b> {
    kind: TextTrackKind,
    label: Option<&'a str>,
    language: Option<&'b str>,
}

impl<'a, 'b> TextTrackDescriptor<'a, 'b> {
    pub fn new(kind: TextTrackKind) -> Self {
        TextTrackDescriptor {
            kind,
            label: None,
            language: None,
        }
    }

    pub fn with_label(kind: TextTrackKind, label: &'a str) -> Self {
        TextTrackDescriptor {
            kind,
            label: Some(label),
            language: None,
        }
    }

    pub fn with_label_and_language(kind: TextTrackKind, label: &'a str, language: &'b str) -> Self {
        TextTrackDescriptor {
            kind,
            label: Some(label),
            language: Some(language),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CanPlayType {
    Probably,
    Maybe,
    No,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CannotPlayMedia {
    NotAllowed(String),
    NotSupported(String),
}

pub trait HtmlMediaElement: AsRef<web_sys::HtmlMediaElement> {
    fn src(&self) -> String {
        self.as_ref().src()
    }

    fn set_src(&self, src: &str) {
        self.as_ref().set_src(src)
    }

    fn current_src(&self) -> String {
        self.as_ref().current_src()
    }

    fn src_object(&self) -> Option<MediaStream> {
        self.as_ref().src_object().map(|s| s.into())
    }

    fn set_src_object(&self, src_object: Option<&MediaStream>) {
        self.as_ref().set_src_object(src_object.map(|s| s.as_ref()))
    }

    fn cross_origin(&self) -> CORS {
        if let Some(cross_origin) = self.as_ref().cross_origin() {
            match &*cross_origin {
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

        self.as_ref().set_cross_origin(Some(cross_origin));
    }

    fn network_state(&self) -> MediaNetworkState {
        match self.as_ref().network_state() {
            0 => MediaNetworkState::Empty,
            1 => MediaNetworkState::Idle,
            2 => MediaNetworkState::Loading,
            4 => MediaNetworkState::NoSource,
            _ => unreachable!(),
        }
    }

    fn preload(&self) -> MediaPreload {
        match &*self.as_ref().preload() {
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

        self.as_ref().set_preload(preload);
    }

    fn buffered(&self) -> MediaBuffered {
        MediaBuffered {
            inner: self.as_ref().buffered(),
        }
    }

    fn ready_state(&self) -> MediaReadyState {
        match self.as_ref().ready_state() {
            0 => MediaReadyState::HaveNothing,
            1 => MediaReadyState::HaveMetadata,
            2 => MediaReadyState::HaveCurrentData,
            3 => MediaReadyState::HaveFutureData,
            4 => MediaReadyState::HaveEnoughData,
            _ => unreachable!(),
        }
    }

    fn seeking(&self) -> bool {
        self.as_ref().seeking()
    }

    fn paused(&self) -> bool {
        self.as_ref().paused()
    }

    fn ended(&self) -> bool {
        self.as_ref().ended()
    }

    fn duration(&self) -> f64 {
        self.as_ref().duration()
    }

    fn current_time(&self) -> f64 {
        self.as_ref().current_time()
    }

    fn set_current_time(&self, current_time: f64) {
        self.as_ref().set_current_time(current_time);
    }

    fn default_playback_rate(&self) -> f64 {
        self.as_ref().default_playback_rate()
    }

    fn set_default_playback_rate(&self, default_playback_rate: f64) {
        self.as_ref()
            .set_default_playback_rate(default_playback_rate);
    }

    fn playback_rate(&self) -> f64 {
        self.as_ref().playback_rate()
    }

    fn set_playback_rate(&self, playback_rate: f64) {
        self.as_ref().set_playback_rate(playback_rate);
    }

    fn autoplay(&self) -> bool {
        self.as_ref().autoplay()
    }

    fn set_autoplay(&self, autoplay: bool) {
        self.as_ref().set_autoplay(autoplay);
    }

    fn loops(&self) -> bool {
        self.as_ref().loop_()
    }

    fn set_loops(&self, loops: bool) {
        self.as_ref().set_loop(loops);
    }

    fn controls(&self) -> bool {
        self.as_ref().controls()
    }

    fn set_controls(&self, controls: bool) {
        self.as_ref().set_controls(controls);
    }

    fn volume(&self) -> f64 {
        self.as_ref().volume()
    }

    fn set_volume(&self, volume: f64) {
        self.as_ref().set_volume(volume);
    }

    fn muted(&self) -> bool {
        self.as_ref().muted()
    }

    fn set_muted(&self, muted: bool) {
        self.as_ref().set_muted(muted);
    }

    fn default_muted(&self) -> bool {
        self.as_ref().default_muted()
    }

    fn set_default_muted(&self, default_muted: bool) {
        self.as_ref().set_default_muted(default_muted);
    }

    fn error(&self) -> Option<MediaError> {
        self.as_ref().error().map(|error| {
            let message = error.message();

            match error.code() {
                1 => MediaError::Aborted(message),
                2 => MediaError::NetworkError(message),
                3 => MediaError::DecodingError(message),
                4 => MediaError::NotSupported(message),
                _ => unreachable!(),
            }
        })
    }

    fn can_play_type(&self, media_type: &str) -> CanPlayType {
        match &*self.as_ref().can_play_type(media_type) {
            "probably" => CanPlayType::Probably,
            "maybe" => CanPlayType::Maybe,
            _ => CanPlayType::No,
        }
    }

    fn fast_seek(&self, time: f64) {
        // There's no indication in the spec that this can actually fail, unwrap for now.
        self.as_ref().fast_seek(time).unwrap();
    }

    fn load(&self) {
        self.as_ref().load();
    }

    fn pause(&self) {
        // There's no indication in the spec that this can actually fail, unwrap for now.
        self.as_ref().pause().unwrap();
    }

    fn play(&self) -> MediaPlay {
        // There's no indication in the spec that the function call can fail (the promise can
        // reject), unwrap for now.
        let promise = self.as_ref().play().unwrap();

        MediaPlay {
            inner: JsFuture::from(promise),
        }
    }

    fn audio_tracks(&self) -> MediaAudioTracks {
        MediaAudioTracks {
            inner: self.as_ref().audio_tracks(),
        }
    }

    fn video_tracks(&self) -> MediaVideoTracks {
        MediaVideoTracks {
            inner: self.as_ref().video_tracks(),
        }
    }

    // TODO: `text_tracks`. Why does web_sys return an Option here? Can we always unwrap and expect
    // a TextTrackList? Move `add_text_track` onto a MediaTextTracks object?

    fn add_text_track(&self, descriptor: TextTrackDescriptor) -> TextTrack {
        let TextTrackDescriptor {
            kind,
            label,
            language,
        } = descriptor;

        let text_track = match (label, language) {
            (Some(label), Some(language)) => self
                .as_ref()
                .add_text_track_with_label_and_language(kind, label, language),
            (Some(label), None) => self.as_ref().add_text_track_with_label(kind, label),
            _ => self.as_ref().add_text_track(kind),
        };

        text_track.into()
    }
}

// TODO: implement
pub struct MediaStream {
    inner: web_sys::MediaStream,
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

impl Write for MediaStream {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

pub struct MediaPlay {
    inner: JsFuture,
}

impl Future for MediaPlay {
    type Output = Result<(), CannotPlayMedia>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            self.map_unchecked_mut(|s| &mut s.inner)
                .poll(cx)
                .map_ok(|_| ())
                .map_err(|err| {
                    let err: web_sys::DomException = err.unchecked_into();
                    let message = err.message();

                    match &*err.name() {
                        "NotAllowedError" => CannotPlayMedia::NotAllowed(message),
                        "NotSupportedError" => CannotPlayMedia::NotSupported(message),
                        _ => unreachable!(),
                    }
                })
        }
    }
}

pub struct MediaBuffered {
    inner: web_sys::TimeRanges,
}

impl MediaBuffered {
    pub fn get(&self, index: usize) -> Option<MediaTimeRange> {
        u32::try_from(index).ok().and_then(|index| {
            self.inner.start(index).ok().map(|start| MediaTimeRange {
                start,
                end: self.inner.end(index).unwrap(),
            })
        })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<MediaTimeRange> {
        self.get(0)
    }

    pub fn last(&self) -> Option<MediaTimeRange> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> MediaBufferedIter {
        MediaBufferedIter {
            media_buffered: self,
            current: 0,
        }
    }
}

impl Write for MediaBuffered {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl IntoIterator for MediaBuffered {
    type Item = MediaTimeRange;
    type IntoIter = MediaBufferedIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        MediaBufferedIntoIter {
            media_buffered: self,
            current: 0,
        }
    }
}

pub struct MediaBufferedIter<'a> {
    media_buffered: &'a MediaBuffered,
    current: usize,
}

impl<'a> Iterator for MediaBufferedIter<'a> {
    type Item = MediaTimeRange;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.media_buffered.get(current)
    }
}

pub struct MediaBufferedIntoIter {
    media_buffered: MediaBuffered,
    current: usize,
}

impl Iterator for MediaBufferedIntoIter {
    type Item = MediaTimeRange;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.media_buffered.get(current)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MediaTimeRange {
    start: f64,
    end: f64,
}

impl MediaTimeRange {
    pub fn start(&self) -> f64 {
        self.start
    }

    pub fn end(&self) -> f64 {
        self.end
    }
}

pub struct MediaAudioTracks {
    inner: web_sys::AudioTrackList,
}

impl MediaAudioTracks {
    pub fn get(&self, index: usize) -> Option<AudioTrack> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get(index))
            .map(|t| t.into())
    }

    pub fn find_by_id(&self, id: &str) -> Option<AudioTrack> {
        self.inner.get_track_by_id(id).map(|t| t.into())
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<AudioTrack> {
        self.get(0)
    }

    pub fn last(&self) -> Option<AudioTrack> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> MediaAudioTracksIter {
        MediaAudioTracksIter {
            media_audio_tracks: self,
            current: 0,
        }
    }

    pub fn on_change(&self) -> OnChange {
        OnChange::new(self.inner.clone().into())
    }

    pub fn on_add_audio_track(&self) -> OnAddAudioTrack {
        OnAddAudioTrack::new(self.inner.clone().into())
    }

    pub fn on_remove_audio_track(&self) -> OnRemoveAudioTrack {
        OnRemoveAudioTrack::new(self.inner.clone().into())
    }
}

impl Write for MediaAudioTracks {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl IntoIterator for MediaAudioTracks {
    type Item = AudioTrack;
    type IntoIter = MediaAudioTracksIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        MediaAudioTracksIntoIter {
            media_audio_tracks: self,
            current: 0,
        }
    }
}

pub struct MediaAudioTracksIter<'a> {
    media_audio_tracks: &'a MediaAudioTracks,
    current: usize,
}

impl<'a> Iterator for MediaAudioTracksIter<'a> {
    type Item = AudioTrack;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.media_audio_tracks.get(current)
    }
}

pub struct MediaAudioTracksIntoIter {
    media_audio_tracks: MediaAudioTracks,
    current: usize,
}

impl Iterator for MediaAudioTracksIntoIter {
    type Item = AudioTrack;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.media_audio_tracks.get(current)
    }
}

pub struct MediaVideoTracks {
    inner: web_sys::VideoTrackList,
}

impl MediaVideoTracks {
    pub fn get(&self, index: usize) -> Option<VideoTrack> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get(index))
            .map(|t| t.into())
    }

    pub fn find_by_id(&self, id: &str) -> Option<VideoTrack> {
        self.inner.get_track_by_id(id).map(|t| t.into())
    }

    // TODO: separate `selected_index`?

    pub fn selected(&self) -> Option<VideoTrack> {
        let selected_index = self.inner.selected_index();

        match selected_index {
            -1 => None,
            _ => self.get(selected_index as usize),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<VideoTrack> {
        self.get(0)
    }

    pub fn last(&self) -> Option<VideoTrack> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> MediaVideoTracksIter {
        MediaVideoTracksIter {
            media_video_tracks: self,
            current: 0,
        }
    }

    pub fn on_change(&self) -> OnChange {
        OnChange::new(self.inner.clone().into())
    }

    pub fn on_add_video_track(&self) -> OnAddVideoTrack {
        OnAddVideoTrack::new(self.inner.clone().into())
    }

    pub fn on_remove_video_track(&self) -> OnRemoveVideoTrack {
        OnRemoveVideoTrack::new(self.inner.clone().into())
    }
}

impl Write for MediaVideoTracks {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl IntoIterator for MediaVideoTracks {
    type Item = VideoTrack;
    type IntoIter = MediaVideoTracksIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        MediaVideoTracksIntoIter {
            media_video_tracks: self,
            current: 0,
        }
    }
}

pub struct MediaVideoTracksIter<'a> {
    media_video_tracks: &'a MediaVideoTracks,
    current: usize,
}

impl<'a> Iterator for MediaVideoTracksIter<'a> {
    type Item = VideoTrack;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.media_video_tracks.get(current)
    }
}

pub struct MediaVideoTracksIntoIter {
    media_video_tracks: MediaVideoTracks,
    current: usize,
}

impl Iterator for MediaVideoTracksIntoIter {
    type Item = VideoTrack;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.media_video_tracks.get(current)
    }
}
