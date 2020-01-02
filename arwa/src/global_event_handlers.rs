use crate::event::{
    OnAnimationCancel, OnAnimationEnd, OnAnimationIteration, OnAnimationStart, OnBlur, OnChange,
    OnClick, OnContextMenu, OnDoubleClick, OnDrag, OnDragEnd, OnDragEnter, OnDragLeave, OnDragOver,
    OnDragStart, OnDrop, OnDurationChanged, OnEmptied, OnEnded, OnError, OnFocus,
    OnGotPointerCapture, OnInput, OnInvalid, OnKeyDown, OnKeyUp, OnLoad, OnLoadEnd, OnLoadStart,
    OnLoadedData, OnLoadedMetadata, OnLostPointerCapture, OnMouseDown, OnMouseEnter, OnMouseLeave,
    OnMouseMove, OnMouseOut, OnMouseOver, OnMouseUp, OnPause, OnPlay, OnPlaying, OnPointerCancel,
    OnPointerDown, OnPointerEnter, OnPointerLeave, OnPointerMove, OnPointerOut, OnPointerOver,
    OnPointerUp, OnProgress, OnRateChange, OnReset, OnResize, OnScroll, OnSeeked, OnSeeking,
    OnSelect, OnStalled, OnSubmit, OnSuspend, OnTimeUpdate, OnToggle, OnTransitionCancel,
    OnTransitionEnd, OnTransitionRun, OnTransitionStart, OnVolumeChange, OnWaiting, OnWheel,
};

pub trait GlobalEventHandlers: AsRef<web_sys::EventTarget> {
    fn on_blur(&self) -> OnBlur {
        OnBlur::new(self.as_ref().clone().into())
    }

    fn on_focus(&self) -> OnFocus {
        OnFocus::new(self.as_ref().clone().into())
    }

    fn on_change(&self) -> OnChange {
        OnChange::new(self.as_ref().clone().into())
    }

    fn on_click(&self) -> OnClick {
        OnClick::new(self.as_ref().clone().into())
    }

    fn on_context_menu(&self) -> OnContextMenu {
        OnContextMenu::new(self.as_ref().clone().into())
    }

    fn on_double_click(&self) -> OnDoubleClick {
        OnDoubleClick::new(self.as_ref().clone().into())
    }

    fn on_drag(&self) -> OnDrag {
        OnDrag::new(self.as_ref().clone().into())
    }

    fn on_drag_end(&self) -> OnDragEnd {
        OnDragEnd::new(self.as_ref().clone().into())
    }

    fn on_drag_enter(&self) -> OnDragEnter {
        OnDragEnter::new(self.as_ref().clone().into())
    }

    fn on_drag_leave(&self) -> OnDragLeave {
        OnDragLeave::new(self.as_ref().clone().into())
    }

    fn on_drag_over(&self) -> OnDragOver {
        OnDragOver::new(self.as_ref().clone().into())
    }

    fn on_drag_start(&self) -> OnDragStart {
        OnDragStart::new(self.as_ref().clone().into())
    }

    fn on_drop(&self) -> OnDrop {
        OnDrop::new(self.as_ref().clone().into())
    }

    fn on_input(&self) -> OnInput {
        OnInput::new(self.as_ref().clone().into())
    }

    fn on_invalid(&self) -> OnInvalid {
        OnInvalid::new(self.as_ref().clone().into())
    }

    fn on_key_down(&self) -> OnKeyDown {
        OnKeyDown::new(self.as_ref().clone().into())
    }

    fn on_key_up(&self) -> OnKeyUp {
        OnKeyUp::new(self.as_ref().clone().into())
    }

    fn on_load(&self) -> OnLoad {
        OnLoad::new(self.as_ref().clone().into())
    }

    fn on_load_start(&self) -> OnLoadStart {
        OnLoadStart::new(self.as_ref().clone().into())
    }

    fn on_load_end(&self) -> OnLoadEnd {
        OnLoadEnd::new(self.as_ref().clone().into())
    }

    fn on_progress(&self) -> OnProgress {
        OnProgress::new(self.as_ref().clone().into())
    }

    fn on_mouse_down(&self) -> OnMouseDown {
        OnMouseDown::new(self.as_ref().clone().into())
    }

    fn on_mouse_enter(&self) -> OnMouseEnter {
        OnMouseEnter::new(self.as_ref().clone().into())
    }

    fn on_mouse_leave(&self) -> OnMouseLeave {
        OnMouseLeave::new(self.as_ref().clone().into())
    }

    fn on_mouse_move(&self) -> OnMouseMove {
        OnMouseMove::new(self.as_ref().clone().into())
    }

    fn on_mouse_out(&self) -> OnMouseOut {
        OnMouseOut::new(self.as_ref().clone().into())
    }

    fn on_mouse_over(&self) -> OnMouseOver {
        OnMouseOver::new(self.as_ref().clone().into())
    }

    fn on_mouse_up(&self) -> OnMouseUp {
        OnMouseUp::new(self.as_ref().clone().into())
    }

    fn on_wheel(&self) -> OnWheel {
        OnWheel::new(self.as_ref().clone().into())
    }

    fn on_reset(&self) -> OnReset {
        OnReset::new(self.as_ref().clone().into())
    }

    fn on_resize(&self) -> OnResize {
        OnResize::new(self.as_ref().clone().into())
    }

    fn on_scroll(&self) -> OnScroll {
        OnScroll::new(self.as_ref().clone().into())
    }

    fn on_select(&self) -> OnSelect {
        OnSelect::new(self.as_ref().clone().into())
    }

    fn on_submit(&self) -> OnSubmit {
        OnSubmit::new(self.as_ref().clone().into())
    }

    fn on_toggle(&self) -> OnToggle {
        OnToggle::new(self.as_ref().clone().into())
    }

    fn on_pointer_cancel(&self) -> OnPointerCancel {
        OnPointerCancel::new(self.as_ref().clone().into())
    }

    fn on_pointer_down(&self) -> OnPointerDown {
        OnPointerDown::new(self.as_ref().clone().into())
    }

    fn on_pointer_move(&self) -> OnPointerMove {
        OnPointerMove::new(self.as_ref().clone().into())
    }

    fn on_pointer_up(&self) -> OnPointerUp {
        OnPointerUp::new(self.as_ref().clone().into())
    }

    fn on_pointer_out(&self) -> OnPointerOut {
        OnPointerOut::new(self.as_ref().clone().into())
    }

    fn on_pointer_over(&self) -> OnPointerOver {
        OnPointerOver::new(self.as_ref().clone().into())
    }

    fn on_pointer_enter(&self) -> OnPointerEnter {
        OnPointerEnter::new(self.as_ref().clone().into())
    }

    fn on_pointer_leave(&self) -> OnPointerLeave {
        OnPointerLeave::new(self.as_ref().clone().into())
    }

    fn on_got_pointer_capture(&self) -> OnGotPointerCapture {
        OnGotPointerCapture::new(self.as_ref().clone().into())
    }

    fn on_lost_pointer_capture(&self) -> OnLostPointerCapture {
        OnLostPointerCapture::new(self.as_ref().clone().into())
    }

    fn on_animation_cancel(&self) -> OnAnimationCancel {
        OnAnimationCancel::new(self.as_ref().clone().into())
    }

    fn on_animation_end(&self) -> OnAnimationEnd {
        OnAnimationEnd::new(self.as_ref().clone().into())
    }

    fn on_animation_iteration(&self) -> OnAnimationIteration {
        OnAnimationIteration::new(self.as_ref().clone().into())
    }

    fn on_animation_start(&self) -> OnAnimationStart {
        OnAnimationStart::new(self.as_ref().clone().into())
    }

    fn on_transition_cancel(&self) -> OnTransitionCancel {
        OnTransitionCancel::new(self.as_ref().clone().into())
    }

    fn on_transition_end(&self) -> OnTransitionEnd {
        OnTransitionEnd::new(self.as_ref().clone().into())
    }

    fn on_transition_run(&self) -> OnTransitionRun {
        OnTransitionRun::new(self.as_ref().clone().into())
    }

    fn on_transition_start(&self) -> OnTransitionStart {
        OnTransitionStart::new(self.as_ref().clone().into())
    }

    fn on_error(&self) -> OnError {
        OnError::new(self.as_ref().clone().into())
    }

    fn on_duration_changed(&self) -> OnDurationChanged {
        OnDurationChanged::new(self.as_ref().clone().into())
    }

    fn on_emptied(&self) -> OnEmptied {
        OnEmptied::new(self.as_ref().clone().into())
    }

    fn on_ended(&self) -> OnEnded {
        OnEnded::new(self.as_ref().clone().into())
    }

    fn on_loaded_data(&self) -> OnLoadedData {
        OnLoadedData::new(self.as_ref().clone().into())
    }

    fn on_loaded_metadata(&self) -> OnLoadedMetadata {
        OnLoadedMetadata::new(self.as_ref().clone().into())
    }

    fn on_pause(&self) -> OnPause {
        OnPause::new(self.as_ref().clone().into())
    }

    fn on_play(&self) -> OnPlay {
        OnPlay::new(self.as_ref().clone().into())
    }

    fn on_playing(&self) -> OnPlaying {
        OnPlaying::new(self.as_ref().clone().into())
    }

    fn on_rate_change(&self) -> OnRateChange {
        OnRateChange::new(self.as_ref().clone().into())
    }

    fn on_seeked(&self) -> OnSeeked {
        OnSeeked::new(self.as_ref().clone().into())
    }

    fn on_seeking(&self) -> OnSeeking {
        OnSeeking::new(self.as_ref().clone().into())
    }

    fn on_stalled(&self) -> OnStalled {
        OnStalled::new(self.as_ref().clone().into())
    }

    fn on_suspend(&self) -> OnSuspend {
        OnSuspend::new(self.as_ref().clone().into())
    }

    fn on_time_update(&self) -> OnTimeUpdate {
        OnTimeUpdate::new(self.as_ref().clone().into())
    }

    fn on_volume_change(&self) -> OnVolumeChange {
        OnVolumeChange::new(self.as_ref().clone().into())
    }

    fn on_waiting(&self) -> OnWaiting {
        OnWaiting::new(self.as_ref().clone().into())
    }
}
