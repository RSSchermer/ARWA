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

macro_rules! typed_event_stream {
    ($stream:ident, $event:ident, $name:tt) => {
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
        }

        impl Stream for $stream {
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

typed_event_stream!(OnAddAudioTrack, AudioTrackEvent, "addtrack");
typed_event_stream!(OnAddVideoTrack, VideoTrackEvent, "addtrack");
typed_event_stream!(OnAfterPrint, GenericEvent, "afterprint");
typed_event_stream!(OnAnimationCancel, AnimationEvent, "animationcancel");
typed_event_stream!(OnAnimationEnd, AnimationEvent, "animationend");
typed_event_stream!(OnAnimationIteration, AnimationEvent, "animationiteration");
typed_event_stream!(OnAnimationStart, AnimationEvent, "animationstart");
typed_event_stream!(OnBeforePrint, GenericEvent, "beforeprint");
typed_event_stream!(OnBeforeUnload, GenericEvent, "beforeunload");
typed_event_stream!(OnBlur, FocusEvent, "blur");
typed_event_stream!(OnChange, GenericEvent, "change");
typed_event_stream!(OnClick, MouseButtonEvent, "click");
typed_event_stream!(OnContextMenu, MouseButtonEvent, "contextmenu");
typed_event_stream!(OnControllerChange, GenericEvent, "controllerchange");
typed_event_stream!(OnDoubleClick, MouseButtonEvent, "dblclick");
typed_event_stream!(OnDrag, DragEvent, "drag");
typed_event_stream!(OnDragEnd, DragEvent, "dragend");
typed_event_stream!(OnDragEnter, DragEvent, "dragenter");
typed_event_stream!(OnDragLeave, DragEvent, "dragleave");
typed_event_stream!(OnDragOver, DragEvent, "dragover");
typed_event_stream!(OnDragStart, DragEvent, "dragstart");
typed_event_stream!(OnDrop, DragEvent, "drop");
typed_event_stream!(OnDurationChanged, GenericEvent, "durationchanged");
typed_event_stream!(OnEmptied, GenericEvent, "emptied");
typed_event_stream!(OnEnded, GenericEvent, "ended");
typed_event_stream!(OnError, ErrorEvent, "error");
typed_event_stream!(OnFocus, FocusEvent, "focus");
typed_event_stream!(OnFullscreenChange, GenericEvent, "fullscreenchange");
typed_event_stream!(OnFullscreenError, GenericEvent, "fullscreenerror");
typed_event_stream!(OnGotPointerCapture, PointerEvent, "gotpointercapture");
typed_event_stream!(OnHashChange, HashChangeEvent, "hashchange");
typed_event_stream!(OnInput, InputEvent, "input");
typed_event_stream!(OnInvalid, GenericEvent, "invalid");
typed_event_stream!(OnKeyDown, KeyboardEvent, "keydown");
typed_event_stream!(OnKeyUp, KeyboardEvent, "keyup");
typed_event_stream!(OnLoad, GenericEvent, "load");
typed_event_stream!(OnLoadEnd, ProgressEvent, "loadend");
typed_event_stream!(OnLoadStart, ProgressEvent, "loadstart");
typed_event_stream!(OnLoadedData, GenericEvent, "loadeddata");
typed_event_stream!(OnLoadedMetadata, GenericEvent, "loadedmetadata");
typed_event_stream!(OnLostPointerCapture, PointerEvent, "lostpointercapture");
typed_event_stream!(OnMessage, MessageEvent, "message");
typed_event_stream!(OnMessageError, MessageEvent, "messageerror");
typed_event_stream!(OnMouseDown, MouseButtonEvent, "mousedown");
typed_event_stream!(OnMouseEnter, GenericMouseEvent, "mouseenter");
typed_event_stream!(OnMouseLeave, GenericMouseEvent, "mouseleave");
typed_event_stream!(OnMouseMove, GenericMouseEvent, "mousemove");
typed_event_stream!(OnMouseOut, GenericMouseEvent, "mouseout");
typed_event_stream!(OnMouseOver, GenericMouseEvent, "mouseover");
typed_event_stream!(OnMouseUp, MouseButtonEvent, "mouseup");
typed_event_stream!(OnOffline, GenericEvent, "offline");
typed_event_stream!(OnOnline, GenericEvent, "online");
typed_event_stream!(OnPageHide, PageTransitionEvent, "pagehide");
typed_event_stream!(OnPageShow, PageTransitionEvent, "pageshow");
typed_event_stream!(OnPause, GenericEvent, "pause");
typed_event_stream!(OnPlay, GenericEvent, "play");
typed_event_stream!(OnPlaying, GenericEvent, "playing");
typed_event_stream!(OnPointerCancel, PointerEvent, "pointercancel");
typed_event_stream!(OnPointerDown, PointerEvent, "pointerdown");
typed_event_stream!(OnPointerEnter, PointerEvent, "pointerenter");
typed_event_stream!(OnPointerLeave, PointerEvent, "pointerleave");
typed_event_stream!(OnPointerMove, PointerEvent, "pointermove");
typed_event_stream!(OnPointerOut, PointerEvent, "pointerout");
typed_event_stream!(OnPointerOver, PointerEvent, "pointerover");
typed_event_stream!(OnPointerUp, PointerEvent, "pointerup");
typed_event_stream!(OnPopState, PopStateEvent, "popstate");
typed_event_stream!(OnProgress, ProgressEvent, "progress");
typed_event_stream!(OnRateChange, GenericEvent, "ratechange");
typed_event_stream!(OnReadyStateChange, GenericEvent, "readystatechange");
typed_event_stream!(
    OnRejectionHandled,
    PromiseRejectionEvent,
    "rejectionhandled"
);
typed_event_stream!(OnRemoveAudioTrack, AudioTrackEvent, "removetrack");
typed_event_stream!(OnRemoveVideoTrack, VideoTrackEvent, "removetrack");
typed_event_stream!(OnReset, GenericEvent, "reset");
typed_event_stream!(OnResize, GenericUiEvent, "resize");
typed_event_stream!(OnScroll, GenericUiEvent, "scroll");
typed_event_stream!(OnSeeked, GenericEvent, "seeked");
typed_event_stream!(OnSeeking, GenericEvent, "seeking");
typed_event_stream!(OnSelect, GenericEvent, "select");
typed_event_stream!(OnSlotChange, GenericEvent, "slotchange");
typed_event_stream!(OnStalled, GenericEvent, "stalled");
typed_event_stream!(OnStateChange, GenericEvent, "statechange");
typed_event_stream!(OnStorage, StorageEvent, "storage");
typed_event_stream!(OnSubmit, FocusEvent, "submit");
typed_event_stream!(OnSuspend, GenericEvent, "suspend");
typed_event_stream!(OnTimeUpdate, GenericEvent, "timeupdate");
typed_event_stream!(OnToggle, GenericEvent, "toggle");
typed_event_stream!(OnTransitionCancel, TransitionEvent, "transitioncancel");
typed_event_stream!(OnTransitionEnd, TransitionEvent, "transitionend");
typed_event_stream!(OnTransitionRun, TransitionEvent, "transitionrun");
typed_event_stream!(OnTransitionStart, TransitionEvent, "transitionstart");
typed_event_stream!(
    OnUnhandledRejection,
    PromiseRejectionEvent,
    "unhandledrejection"
);
typed_event_stream!(OnUnload, GenericEvent, "unload");
typed_event_stream!(OnVisibilityChange, GenericEvent, "visibilitychange");
typed_event_stream!(OnVolumeChange, GenericEvent, "volumechange");
typed_event_stream!(OnWaiting, GenericEvent, "waiting");
typed_event_stream!(OnWheel, WheelEvent, "wheel");
