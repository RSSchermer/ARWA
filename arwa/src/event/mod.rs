macro_rules! impl_common_event_traits {
    ($crate_tpe:ident, $web_sys_tpe:ident) => {
        impl FromEvent for $crate_tpe {
            fn from_event(event: web_sys::Event) -> Self {
                $crate_tpe {
                    inner: event.unchecked_into(),
                }
            }
        }

        impl AsRef<web_sys::$web_sys_tpe> for $crate_tpe {
            fn as_ref(&self) -> &web_sys::$web_sys_tpe {
                &self.inner
            }
        }

        impl AsRef<web_sys::Event> for $crate_tpe {
            fn as_ref(&self) -> &web_sys::Event {
                self.inner.as_ref()
            }
        }

        impl $crate::console::Write for $crate_tpe {
            fn write(&self, writer: &mut $crate::console::Writer) {
                writer.write_1(self.inner.as_ref())
            }
        }

        impl Event for $crate_tpe {}
    };
    ($tpe:ident) => {
        impl_common_event_traits!($tpe, $tpe);
    };
}

macro_rules! impl_common_ui_event_traits {
    ($crate_tpe:ident, $web_sys_tpe:ident) => {
        impl_common_event_traits!($crate_tpe, $web_sys_tpe);

        impl AsRef<web_sys::UiEvent> for $crate_tpe {
            fn as_ref(&self) -> &web_sys::UiEvent {
                self.inner.as_ref()
            }
        }

        impl UiEvent for $crate_tpe {}
    };
    ($tpe:ident) => {
        impl_common_ui_event_traits!($tpe, $tpe);
    };
}

macro_rules! impl_common_mouse_event_traits {
    ($crate_tpe:ident, $web_sys_tpe:ident) => {
        impl_common_ui_event_traits!($crate_tpe, $web_sys_tpe);

        impl AsRef<web_sys::MouseEvent> for $crate_tpe {
            fn as_ref(&self) -> &web_sys::MouseEvent {
                self.inner.as_ref()
            }
        }

        impl MouseEvent for $crate_tpe {}
    };
    ($tpe:ident) => {
        impl_common_mouse_event_traits!($tpe, $tpe);
    };
}

mod animation_event;
pub use self::animation_event::*;

mod audio_track_event;
pub use self::audio_track_event::*;

mod drag_event;
pub use self::drag_event::*;

mod error_event;
pub use self::error_event::*;

mod event;
pub use self::event::*;

mod focus_event;
pub use self::focus_event::*;

mod hash_change_event;
pub use self::hash_change_event::*;

mod input_event;
pub use self::input_event::*;

mod keyboard_event;
pub use self::keyboard_event::*;

mod message_event;
pub use self::message_event::*;

mod mouse_button_event;
pub use self::mouse_button_event::*;

mod mouse_event;
pub use self::mouse_event::*;

mod on_event;
use self::on_event::*;

mod page_transition_event;
pub use self::page_transition_event::*;

mod pointer_event;
pub use self::pointer_event::*;

mod pop_state_event;
pub use self::pop_state_event::*;

mod progress_event;
pub use self::progress_event::*;

mod promise_rejection_event;
pub use self::promise_rejection_event::*;

mod storage_event;
pub use self::storage_event::*;

mod transition_event;
pub use self::transition_event::*;

mod ui_event;
pub use self::ui_event::*;

mod video_track_event;
pub use self::video_track_event::*;

mod wheel_event;
pub use self::wheel_event::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Phase {
    Bubble,
    Capture
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct EventStreamOptions {
    pub phase: Phase,
    pub passive: bool
}

impl Default for EventStreamOptions {
    fn default() -> Self {
        EventStreamOptions {
            phase: Phase::Bubble,
            passive: true
        }
    }
}

macro_rules! typed_event_stream {
    ($stream:ident, $stream_with_options:ident, $event:ident, $name:tt) => {
        #[must_use = "streams do nothing unless polled or spawned"]
        pub struct $stream {
            inner: OnEvent<$event>,
        }

        impl $stream {
            pub(crate) fn new(target: web_sys::EventTarget) -> Self {
                $stream {
                    inner: OnEvent::new(target, $name),
                }
            }

            pub fn with_options(self, options: EventStreamOptions) -> $stream_with_options {
                $stream_with_options {
                    inner: OnEventWithOptions::new(self.inner, options)
                }
            }
        }

        impl Stream for $stream {
            type Item = $event;

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                unsafe { self.map_unchecked_mut(|s| &mut s.inner).poll_next(cx) }
            }
        }

        pub struct $stream_with_options {
            inner: OnEventWithOptions<$event>,
        }

        impl Stream for $stream_with_options {
            type Item = $event;

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                unsafe { self.map_unchecked_mut(|s| &mut s.inner).poll_next(cx) }
            }
        }
    };
}

use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;

typed_event_stream!(OnAddAudioTrack, OnAddAudioTrackWithOptions, AudioTrackEvent, "addtrack");
typed_event_stream!(OnAddVideoTrack, OnAddVideoTrackWithOptions, VideoTrackEvent, "addtrack");
typed_event_stream!(OnAfterPrint, OnAfterPrintWithOptions, GenericEvent, "afterprint");
typed_event_stream!(OnAnimationCancel, OnAnimationCancelWithOptions, AnimationEvent, "animationcancel");
typed_event_stream!(OnAnimationEnd, OnAnimationEndWithOptions, AnimationEvent, "animationend");
typed_event_stream!(OnAnimationIteration, OnAnimationIterationWithOptions, AnimationEvent, "animationiteration");
typed_event_stream!(OnAnimationStart, OnAnimationStartWithOptions, AnimationEvent, "animationstart");
typed_event_stream!(OnBeforePrint, OnBeforePrintWithOptions, GenericEvent, "beforeprint");
typed_event_stream!(OnBeforeUnload, OnBeforeUnloadWithOptions, GenericEvent, "beforeunload");
typed_event_stream!(OnBlur, OnBlurWithOptions, FocusEvent, "blur");
typed_event_stream!(OnChange, OnChangeWithOptions, GenericEvent, "change");
typed_event_stream!(OnClick, OnClickWithOptions, MouseButtonEvent, "click");
typed_event_stream!(OnContextMenu, OnContextMenuWithOptions, MouseButtonEvent, "contextmenu");
typed_event_stream!(OnControllerChange, OnControllerChangeWithOptions, GenericEvent, "controllerchange");
typed_event_stream!(OnDoubleClick, OnDoubleClickWithOptions, MouseButtonEvent, "dblclick");
typed_event_stream!(OnDrag, OnDragWithOptions, DragEvent, "drag");
typed_event_stream!(OnDragEnd, OnDragEndWithOptions, DragEvent, "dragend");
typed_event_stream!(OnDragEnter, OnDragEnterWithOptions, DragEvent, "dragenter");
typed_event_stream!(OnDragLeave, OnDragLeaveWithOptions, DragEvent, "dragleave");
typed_event_stream!(OnDragOver, OnDragOverWithOptions, DragEvent, "dragover");
typed_event_stream!(OnDragStart, OnDragStartWithOptions, DragEvent, "dragstart");
typed_event_stream!(OnDrop, OnDropWithOptions, DragEvent, "drop");
typed_event_stream!(OnDurationChanged, OnDurationChangedWithOptions, GenericEvent, "durationchanged");
typed_event_stream!(OnEmptied, OnEmptiedWithOptions, GenericEvent, "emptied");
typed_event_stream!(OnEnded, OnEndedWithOptions, GenericEvent, "ended");
typed_event_stream!(OnError, OnErrorWithOptions, ErrorEvent, "error");
typed_event_stream!(OnFocus, OnFocusWithOptions, FocusEvent, "focus");
typed_event_stream!(OnFullscreenChange, OnFullscreenChangeWithOptions, GenericEvent, "fullscreenchange");
typed_event_stream!(OnFullscreenError, OnFullscreenErrorWithOptions, GenericEvent, "fullscreenerror");
typed_event_stream!(OnGotPointerCapture, OnGotPointerCaptureWithOptions, PointerEvent, "gotpointercapture");
typed_event_stream!(OnHashChange, OnHashChangeWithOptions, HashChangeEvent, "hashchange");
typed_event_stream!(OnInput, OnInputWithOptions, InputEvent, "input");
typed_event_stream!(OnInvalid, OnInvalidWithOptions, GenericEvent, "invalid");
typed_event_stream!(OnKeyDown, OnKeyDownWithOptions, KeyboardEvent, "keydown");
typed_event_stream!(OnKeyUp, OnKeyUpWithOptions, KeyboardEvent, "keyup");
typed_event_stream!(OnLoad, OnLoadWithOptions, GenericEvent, "load");
typed_event_stream!(OnLoadEnd, OnLoadEndWithOptions, ProgressEvent, "loadend");
typed_event_stream!(OnLoadStart, OnLoadStartWithOptions, ProgressEvent, "loadstart");
typed_event_stream!(OnLoadedData, OnLoadedDataWithOptions, GenericEvent, "loadeddata");
typed_event_stream!(OnLoadedMetadata, OnLoadedMetadataWithOptions, GenericEvent, "loadedmetadata");
typed_event_stream!(OnLostPointerCapture, OnLostPointerCaptureWithOptions, PointerEvent, "lostpointercapture");
typed_event_stream!(OnMessage, OnMessageWithOptions, MessageEvent, "message");
typed_event_stream!(OnMessageError, OnMessageErrorWithOptions, MessageEvent, "messageerror");
typed_event_stream!(OnMouseDown, OnMouseDownWithOptions, MouseButtonEvent, "mousedown");
typed_event_stream!(OnMouseEnter, OnMouseEnterWithOptions, GenericMouseEvent, "mouseenter");
typed_event_stream!(OnMouseLeave, OnMouseLeaveWithOptions, GenericMouseEvent, "mouseleave");
typed_event_stream!(OnMouseMove, OnMouseMoveWithOptions, GenericMouseEvent, "mousemove");
typed_event_stream!(OnMouseOut, OnMouseOutWithOptions, GenericMouseEvent, "mouseout");
typed_event_stream!(OnMouseOver, OnMouseOverWithOptions, GenericMouseEvent, "mouseover");
typed_event_stream!(OnMouseUp, OnMouseUpWithOptions, MouseButtonEvent, "mouseup");
typed_event_stream!(OnOffline, OnOfflineWithOptions, GenericEvent, "offline");
typed_event_stream!(OnOnline, OnOnlineWithOptions, GenericEvent, "online");
typed_event_stream!(OnPageHide, OnPageHideWithOptions, PageTransitionEvent, "pagehide");
typed_event_stream!(OnPageShow, OnPageShowWithOptions, PageTransitionEvent, "pageshow");
typed_event_stream!(OnPause, OnPauseWithOptions, GenericEvent, "pause");
typed_event_stream!(OnPlay, OnPlayWithOptions, GenericEvent, "play");
typed_event_stream!(OnPlaying, OnPlayingWithOptions, GenericEvent, "playing");
typed_event_stream!(OnPointerCancel, OnPointerCancelWithOptions, PointerEvent, "pointercancel");
typed_event_stream!(OnPointerDown, OnPointerDownWithOptions, PointerEvent, "pointerdown");
typed_event_stream!(OnPointerEnter, OnPointerEnterWithOptions, PointerEvent, "pointerenter");
typed_event_stream!(OnPointerLeave, OnPointerLeaveWithOptions, PointerEvent, "pointerleave");
typed_event_stream!(OnPointerLockChange, OnPointerLockChangeWithOptions, GenericEvent, "pointerlockchange");
typed_event_stream!(OnPointerLockError, OnPointerLockErrorWithOptions, GenericEvent, "pointerlockerror");
typed_event_stream!(OnPointerMove, OnPointerMoveWithOptions, PointerEvent, "pointermove");
typed_event_stream!(OnPointerOut, OnPointerOutWithOptions, PointerEvent, "pointerout");
typed_event_stream!(OnPointerOver, OnPointerOverWithOptions, PointerEvent, "pointerover");
typed_event_stream!(OnPointerUp, OnPointerUpWithOptions, PointerEvent, "pointerup");
typed_event_stream!(OnPopState, OnPopStateWithOptions, PopStateEvent, "popstate");
typed_event_stream!(OnProgress, OnProgressWithOptions, ProgressEvent, "progress");
typed_event_stream!(OnRateChange, OnRateChangeWithOptions, GenericEvent, "ratechange");
typed_event_stream!(OnReadyStateChange, OnReadyStateChangeWithOptions, GenericEvent, "readystatechange");
typed_event_stream!(
    OnRejectionHandled,
    OnRejectionHandledWithOptions,
    PromiseRejectionEvent,
    "rejectionhandled"
);
typed_event_stream!(OnRemoveAudioTrack, OnRemoveAudioTrackWithOptions, AudioTrackEvent, "removetrack");
typed_event_stream!(OnRemoveVideoTrack, OnRemoveVideoTrackWithOptions, VideoTrackEvent, "removetrack");
typed_event_stream!(OnReset, OnResetWithOptions, GenericEvent, "reset");
typed_event_stream!(OnResize, OnResizeWithOptions, GenericUiEvent, "resize");
typed_event_stream!(OnScroll, OnScrollWithOptions, GenericUiEvent, "scroll");
typed_event_stream!(OnSeeked, OnSeekedWithOptions, GenericEvent, "seeked");
typed_event_stream!(OnSeeking, OnSeekingWithOptions, GenericEvent, "seeking");
typed_event_stream!(OnSelect, OnSelectWithOptions, GenericEvent, "select");
typed_event_stream!(OnSlotChange, OnSlotChangeWithOptions, GenericEvent, "slotchange");
typed_event_stream!(OnStalled, OnStalledWithOptions, GenericEvent, "stalled");
typed_event_stream!(OnStateChange, OnStateChangeWithOptions, GenericEvent, "statechange");
typed_event_stream!(OnStorage, OnStorageWithOptions, StorageEvent, "storage");
typed_event_stream!(OnSubmit, OnSubmitWithOptions, FocusEvent, "submit");
typed_event_stream!(OnSuspend, OnSuspendWithOptions, GenericEvent, "suspend");
typed_event_stream!(OnTimeUpdate, OnTimeUpdateWithOptions, GenericEvent, "timeupdate");
typed_event_stream!(OnToggle, OnToggleWithOptions, GenericEvent, "toggle");
typed_event_stream!(OnTransitionCancel, OnTransitionCancelWithOptions, TransitionEvent, "transitioncancel");
typed_event_stream!(OnTransitionEnd, OnTransitionEndWithOptions, TransitionEvent, "transitionend");
typed_event_stream!(OnTransitionRun, OnTransitionRunWithOptions, TransitionEvent, "transitionrun");
typed_event_stream!(OnTransitionStart, OnTransitionStartWithOptions, TransitionEvent, "transitionstart");
typed_event_stream!(
    OnUnhandledRejection,
    OnUnhandledRejectionWithOptions,
    PromiseRejectionEvent,
    "unhandledrejection"
);
typed_event_stream!(OnUnload, OnUnloadWithOptions, GenericEvent, "unload");
typed_event_stream!(OnVisibilityChange, OnVisibilityChangeWithOptions, GenericEvent, "visibilitychange");
typed_event_stream!(OnVolumeChange, OnVolumeChangeWithOptions, GenericEvent, "volumechange");
typed_event_stream!(OnWaiting, OnWaitingWithOptions, GenericEvent, "waiting");
typed_event_stream!(OnWheel, OnWheelWithOptions, WheelEvent, "wheel");
