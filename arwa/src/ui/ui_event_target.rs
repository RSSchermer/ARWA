pub(crate) mod ui_event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait UiEventTarget: ui_event_target_seal::Seal {
    // Note: the `focus` and `blur` events don't always seem to result in FocusEvents, they
    // sometimes result in generic events. It's also unclear to me whether they add functionality
    // over the `focusin` and `focusout` events that cannot be emulated by listening in the capture
    // phase. Ignoring `focus` and `blur` until a use-case comes up.

    fn on_focus_in(&self) -> OnFocusIn<Self> {
        OnFocusIn::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_focus_out(&self) -> OnFocusOut<Self> {
        OnFocusOut::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_click(&self) -> OnClick<Self> {
        OnClick::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_aux_click(&self) -> OnAuxClick<Self> {
        OnClick::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_context_menu(&self) -> OnContextMenu<Self> {
        OnContextMenu::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_pointer_cancel(&self) -> OnPointerCancel<Self> {
        OnPointerCancel::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_pointer_down(&self) -> OnPointerDown<Self> {
        OnPointerDown::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_pointer_move(&self) -> OnPointerMove<Self> {
        OnPointerMove::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_pointer_up(&self) -> OnPointerUp<Self> {
        OnPointerUp::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_pointer_out(&self) -> OnPointerOut<Self> {
        OnPointerOut::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_pointer_over(&self) -> OnPointerOver<Self> {
        OnPointerOver::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_pointer_enter(&self) -> OnPointerEnter<Self> {
        OnPointerEnter::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_pointer_leave(&self) -> OnPointerLeave<Self> {
        OnPointerLeave::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_got_pointer_capture(&self) -> OnGotPointerCapture<Self> {
        OnGotPointerCapture::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_lost_pointer_capture(&self) -> OnLostPointerCapture<Self> {
        OnLostPointerCapture::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_drag(&self) -> OnDrag<Self> {
        OnDrag::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_drag_end(&self) -> OnDragEnd<Self> {
        OnDragEnd::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_drag_enter(&self) -> OnDragEnter<Self> {
        OnDragEnter::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_drag_leave(&self) -> OnDragLeave<Self> {
        OnDragLeave::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_drag_over(&self) -> OnDragOver<Self> {
        OnDragOver::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_drag_start(&self) -> OnDragStart<Self> {
        OnDragStart::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_drop(&self) -> OnDrop<Self> {
        OnDrop::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_key_down(&self) -> OnKeyDown<Self> {
        OnKeyDown::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_key_up(&self) -> OnKeyUp<Self> {
        OnKeyUp::new(self.as_web_sys_event_target().clone().into())
    }

    fn on_wheel(&self) -> OnWheel<Self> {
        OnWheel::new(self.as_web_sys_event_target().clone().into())
    }
}

typed_event_stream!(OnInput, OnInputWithOptions, InputEvent, "input");
typed_event_stream!(
    OnBeforeInput,
    OnBeforeInputWithOptions,
    BeforeInputEvent,
    "beforeinput"
);
typed_event_stream!(OnFocusOut, OnFocusOutWithOptions, FocusOutEvent, "focusout");
typed_event_stream!(OnFocusIn, OnFocusInWithOptions, FocusInEvent, "focusin");
typed_event_stream!(OnFocusOut, OnFocusOutWithOptions, FocusOutEvent, "focusout");
typed_event_stream!(OnClick, OnClickWithOptions, ClickEvent, "click");
typed_event_stream!(OnAuxClick, OnAuxClickWithOptions, AuxClickEvent, "auxclick");
typed_event_stream!(
    OnContextMenu,
    OnContextMenuWithOptions,
    ContextMenuEvent,
    "contextmenu"
);
typed_event_stream!(
    OnPointerCancel,
    OnPointerCancelWithOptions,
    PointerCancelEvent,
    "pointercancel"
);
typed_event_stream!(
    OnPointerDown,
    OnPointerDownWithOptions,
    PointerDownEvent,
    "pointerdown"
);
typed_event_stream!(
    OnPointerEnter,
    OnPointerEnterWithOptions,
    PointerEnterEvent,
    "pointerenter"
);
typed_event_stream!(
    OnPointerLeave,
    OnPointerLeaveWithOptions,
    PointerLeaveEvent,
    "pointerleave"
);
typed_event_stream!(
    OnPointerMove,
    OnPointerMoveWithOptions,
    PointerMoveEvent,
    "pointermove"
);
typed_event_stream!(
    OnPointerOut,
    OnPointerOutWithOptions,
    PointerOutEvent,
    "pointerout"
);
typed_event_stream!(
    OnPointerOver,
    OnPointerOverWithOptions,
    PointerOverEvent,
    "pointerover"
);
typed_event_stream!(
    OnPointerUp,
    OnPointerUpWithOptions,
    PointerUpEvent,
    "pointerup"
);
typed_event_stream!(
    OnGotPointerCapture,
    OnGotPointerCaptureWithOptions,
    GotPointerCaptureEvent,
    "gotpointercapture"
);
typed_event_stream!(
    OnLostPointerCapture,
    OnLostPointerCaptureWithOptions,
    LostPointerCaptureEvent,
    "lostpointercapture"
);
typed_event_stream!(OnDrag, OnDragWithOptions, DragEvent, "drag");
typed_event_stream!(OnDragEnd, OnDragEndWithOptions, DragEndEvent, "dragend");
typed_event_stream!(
    OnDragEnter,
    OnDragEnterWithOptions,
    DragEnterEvent,
    "dragenter"
);
typed_event_stream!(
    OnDragLeave,
    OnDragLeaveWithOptions,
    DragLeaveEvent,
    "dragleave"
);
typed_event_stream!(OnDragOver, OnDragOverWithOptions, DragOverEvent, "dragover");
typed_event_stream!(
    OnDragStart,
    OnDragStartWithOptions,
    DragStartEvent,
    "dragstart"
);
typed_event_stream!(OnDrop, OnDropWithOptions, DropEvent, "drop");
typed_event_stream!(OnKeyDown, OnKeyDownWithOptions, KeyDownEvent, "keydown");
typed_event_stream!(OnKeyUp, OnKeyUpWithOptions, KeyUpEvent, "keyup");
typed_event_stream!(OnWheel, OnWheelWithOptions, WheelEvent, "wheel");

macro_rules! impl_ui_event_target_for_element {
    ($element:ident) => {
        impl $crate::ui::ui_event_target_seal::Seal for $element {
            fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
                self.as_web_sys_element().as_ref()
            }
        }

        impl $crate::ui::UiEventTarget for $element {}
    };
}

pub(crate) use impl_ui_event_target_for_element;
