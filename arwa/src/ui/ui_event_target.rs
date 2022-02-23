use crate::event::typed_event_iterator;
use crate::ui::{
    AuxClickEvent, BeforeInputEvent, ClickEvent, ContextMenuEvent, DragEndEvent, DragEnterEvent,
    DragEvent, DragLeaveEvent, DragOverEvent, DragStartEvent, DropEvent, FocusInEvent,
    FocusOutEvent, GotPointerCaptureEvent, InputEvent, KeyDownEvent, KeyUpEvent,
    LostPointerCaptureEvent, PointerCancelEvent, PointerDownEvent, PointerEnterEvent,
    PointerLeaveEvent, PointerMoveEvent, PointerOutEvent, PointerOverEvent, PointerUpEvent,
    WheelEvent,
};

pub(crate) mod ui_event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait UiEventTarget: ui_event_target_seal::Seal + Sized {
    fn on_input(&self) -> OnInput<Self> {
        OnInput::new(self.as_web_sys_event_target())
    }

    fn on_before_input(&self) -> OnBeforeInput<Self> {
        OnBeforeInput::new(self.as_web_sys_event_target())
    }
    // Note: the `focus` and `blur` events don't always seem to result in FocusEvents, they
    // sometimes result in generic events. It's also unclear to me whether they add functionality
    // over the `focusin` and `focusout` events that cannot be emulated by listening in the capture
    // phase. Ignoring `focus` and `blur` until a use-case comes up.

    fn on_focus_in(&self) -> OnFocusIn<Self> {
        OnFocusIn::new(self.as_web_sys_event_target())
    }

    fn on_focus_out(&self) -> OnFocusOut<Self> {
        OnFocusOut::new(self.as_web_sys_event_target())
    }

    fn on_click(&self) -> OnClick<Self> {
        OnClick::new(self.as_web_sys_event_target())
    }

    fn on_aux_click(&self) -> OnAuxClick<Self> {
        OnAuxClick::new(self.as_web_sys_event_target())
    }

    fn on_context_menu(&self) -> OnContextMenu<Self> {
        OnContextMenu::new(self.as_web_sys_event_target())
    }

    fn on_pointer_cancel(&self) -> OnPointerCancel<Self> {
        OnPointerCancel::new(self.as_web_sys_event_target())
    }

    fn on_pointer_down(&self) -> OnPointerDown<Self> {
        OnPointerDown::new(self.as_web_sys_event_target())
    }

    fn on_pointer_move(&self) -> OnPointerMove<Self> {
        OnPointerMove::new(self.as_web_sys_event_target())
    }

    fn on_pointer_up(&self) -> OnPointerUp<Self> {
        OnPointerUp::new(self.as_web_sys_event_target())
    }

    fn on_pointer_out(&self) -> OnPointerOut<Self> {
        OnPointerOut::new(self.as_web_sys_event_target())
    }

    fn on_pointer_over(&self) -> OnPointerOver<Self> {
        OnPointerOver::new(self.as_web_sys_event_target())
    }

    fn on_pointer_enter(&self) -> OnPointerEnter<Self> {
        OnPointerEnter::new(self.as_web_sys_event_target())
    }

    fn on_pointer_leave(&self) -> OnPointerLeave<Self> {
        OnPointerLeave::new(self.as_web_sys_event_target())
    }

    fn on_got_pointer_capture(&self) -> OnGotPointerCapture<Self> {
        OnGotPointerCapture::new(self.as_web_sys_event_target())
    }

    fn on_lost_pointer_capture(&self) -> OnLostPointerCapture<Self> {
        OnLostPointerCapture::new(self.as_web_sys_event_target())
    }

    fn on_drag(&self) -> OnDrag<Self> {
        OnDrag::new(self.as_web_sys_event_target())
    }

    fn on_drag_end(&self) -> OnDragEnd<Self> {
        OnDragEnd::new(self.as_web_sys_event_target())
    }

    fn on_drag_enter(&self) -> OnDragEnter<Self> {
        OnDragEnter::new(self.as_web_sys_event_target())
    }

    fn on_drag_leave(&self) -> OnDragLeave<Self> {
        OnDragLeave::new(self.as_web_sys_event_target())
    }

    fn on_drag_over(&self) -> OnDragOver<Self> {
        OnDragOver::new(self.as_web_sys_event_target())
    }

    fn on_drag_start(&self) -> OnDragStart<Self> {
        OnDragStart::new(self.as_web_sys_event_target())
    }

    fn on_drop(&self) -> OnDrop<Self> {
        OnDrop::new(self.as_web_sys_event_target())
    }

    fn on_key_down(&self) -> OnKeyDown<Self> {
        OnKeyDown::new(self.as_web_sys_event_target())
    }

    fn on_key_up(&self) -> OnKeyUp<Self> {
        OnKeyUp::new(self.as_web_sys_event_target())
    }

    fn on_wheel(&self) -> OnWheel<Self> {
        OnWheel::new(self.as_web_sys_event_target())
    }
}

typed_event_iterator!(OnInput, OnInputWithOptions, InputEvent, "input");
typed_event_iterator!(
    OnBeforeInput,
    OnBeforeInputWithOptions,
    BeforeInputEvent,
    "beforeinput"
);
typed_event_iterator!(OnFocusIn, OnFocusInWithOptions, FocusInEvent, "focusin");
typed_event_iterator!(OnFocusOut, OnFocusOutWithOptions, FocusOutEvent, "focusout");
typed_event_iterator!(OnClick, OnClickWithOptions, ClickEvent, "click");
typed_event_iterator!(OnAuxClick, OnAuxClickWithOptions, AuxClickEvent, "auxclick");
typed_event_iterator!(
    OnContextMenu,
    OnContextMenuWithOptions,
    ContextMenuEvent,
    "contextmenu"
);
typed_event_iterator!(
    OnPointerCancel,
    OnPointerCancelWithOptions,
    PointerCancelEvent,
    "pointercancel"
);
typed_event_iterator!(
    OnPointerDown,
    OnPointerDownWithOptions,
    PointerDownEvent,
    "pointerdown"
);
typed_event_iterator!(
    OnPointerEnter,
    OnPointerEnterWithOptions,
    PointerEnterEvent,
    "pointerenter"
);
typed_event_iterator!(
    OnPointerLeave,
    OnPointerLeaveWithOptions,
    PointerLeaveEvent,
    "pointerleave"
);
typed_event_iterator!(
    OnPointerMove,
    OnPointerMoveWithOptions,
    PointerMoveEvent,
    "pointermove"
);
typed_event_iterator!(
    OnPointerOut,
    OnPointerOutWithOptions,
    PointerOutEvent,
    "pointerout"
);
typed_event_iterator!(
    OnPointerOver,
    OnPointerOverWithOptions,
    PointerOverEvent,
    "pointerover"
);
typed_event_iterator!(
    OnPointerUp,
    OnPointerUpWithOptions,
    PointerUpEvent,
    "pointerup"
);
typed_event_iterator!(
    OnGotPointerCapture,
    OnGotPointerCaptureWithOptions,
    GotPointerCaptureEvent,
    "gotpointercapture"
);
typed_event_iterator!(
    OnLostPointerCapture,
    OnLostPointerCaptureWithOptions,
    LostPointerCaptureEvent,
    "lostpointercapture"
);
typed_event_iterator!(OnDrag, OnDragWithOptions, DragEvent, "drag");
typed_event_iterator!(OnDragEnd, OnDragEndWithOptions, DragEndEvent, "dragend");
typed_event_iterator!(
    OnDragEnter,
    OnDragEnterWithOptions,
    DragEnterEvent,
    "dragenter"
);
typed_event_iterator!(
    OnDragLeave,
    OnDragLeaveWithOptions,
    DragLeaveEvent,
    "dragleave"
);
typed_event_iterator!(OnDragOver, OnDragOverWithOptions, DragOverEvent, "dragover");
typed_event_iterator!(
    OnDragStart,
    OnDragStartWithOptions,
    DragStartEvent,
    "dragstart"
);
typed_event_iterator!(OnDrop, OnDropWithOptions, DropEvent, "drop");
typed_event_iterator!(OnKeyDown, OnKeyDownWithOptions, KeyDownEvent, "keydown");
typed_event_iterator!(OnKeyUp, OnKeyUpWithOptions, KeyUpEvent, "keyup");
typed_event_iterator!(OnWheel, OnWheelWithOptions, WheelEvent, "wheel");

macro_rules! impl_ui_event_target_for_element {
    ($element:ident) => {
        impl $crate::ui::ui_event_target_seal::Seal for $element {
            fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
                use crate::dom::element_seal::Seal;

                self.as_web_sys_element().as_ref()
            }
        }

        impl $crate::ui::UiEventTarget for $element {}
    };
}

pub(crate) use impl_ui_event_target_for_element;
