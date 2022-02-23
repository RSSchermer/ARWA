use std::marker;

use js_sys::JsString;
use wasm_bindgen::UnwrapThrowExt;

use crate::collection::{Collection, Sequence};
use crate::dom::Element;
use crate::file::File;
use crate::html::{input_files_source_seal, InputFilesSource};
use crate::ui::{impl_mouse_event_traits, impl_ui_event_traits};
use crate::unchecked_cast_array::unchecked_cast_array;

// I've experimented some with the behaviour of the DataTransfer and DataTransferItemList objects
// at various stages of a drag events life. During the `dragstart` event's macrotask and any
// microtasks queued during said macrotask, the transfer objects seem to be in read/write mode, but
// only for the `string` item kind, not for the `file` item kind. I cannot find any reference to
// this behaviour in the spec, I assume this is due to security concerns. Files can be attached to
// transfer objects, but it seems only by the browser (e.g. by dragging them from a file explorer).
// Because of this, the DataTransferItemList interface seems not the add any additional
// functionality to the base DataTransfer interface in the case of drag events. It also seems like a
// weird mix of concerns in that is acts both as a string-string key-value store, and a file list,
// mixing both in a single collection. Files can be resolved directly from this collection through
// `DataTransferItem.getAsFile()`, but strings only through callbacks
// (`DataTransferItem.getAsString(callback)`).
//
// In short: the whole DataTransfer interface seems a bit of a mess to me. I think the cleanest
// solution for now is to just take the good parts and expose them on the DragEvent interface
// directly.

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DropEffect {
    Copy,
    Move,
    Link,
    None,
}

pub enum DropEffectAllowed {
    None,
    All,
    Copy,
    CopyLink,
    CopyMove,
    Link,
    LinkMove,
    Move,
    Uninitialized,
}

mod drag_drop_event_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_drag_event(&self) -> &web_sys::DragEvent;
    }
}

pub trait DragDropEvent: drag_drop_event_seal::Seal {
    fn effect_allowed(&self) -> DropEffectAllowed {
        match self
            .as_web_sys_drag_event()
            .data_transfer()
            .unwrap_throw()
            .effect_allowed()
            .as_str()
        {
            "none" => DropEffectAllowed::None,
            "all" => DropEffectAllowed::All,
            "copy" => DropEffectAllowed::Copy,
            "copyLink" => DropEffectAllowed::CopyLink,
            "copyMove" => DropEffectAllowed::CopyMove,
            "link" => DropEffectAllowed::Link,
            "linkMove" => DropEffectAllowed::LinkMove,
            "move" => DropEffectAllowed::Move,
            _ => DropEffectAllowed::Uninitialized,
        }
    }

    fn drop_effect(&self) -> DropEffect {
        match self
            .as_web_sys_drag_event()
            .data_transfer()
            .unwrap_throw()
            .drop_effect()
            .as_str()
        {
            "copy" => DropEffect::Copy,
            "move" => DropEffect::Move,
            "link" => DropEffect::Link,
            _ => DropEffect::None,
        }
    }

    fn set_drop_effect(&self, drop_effect: DropEffect) {
        let val = match drop_effect {
            DropEffect::Copy => "copy",
            DropEffect::Move => "move",
            DropEffect::Link => "link",
            DropEffect::None => "none",
        };

        self.as_web_sys_drag_event()
            .data_transfer()
            .unwrap_throw()
            .set_drop_effect(val);
    }

    fn types(&self) -> DragEventTypes {
        DragEventTypes::new(
            self.as_web_sys_drag_event()
                .data_transfer()
                .unwrap_throw()
                .types(),
        )
    }
}

unchecked_cast_array!(String, JsString, DragEventTypes);

mod drag_drop_read_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_drag_event(&self) -> &web_sys::DragEvent;
    }
}

pub trait DragDropRead: drag_drop_read_seal::Seal {
    fn get_data(&self, format_type: &str) -> Option<String> {
        self.as_web_sys_drag_event()
            .data_transfer()
            .unwrap_throw()
            .get_data(format_type)
            .ok()
    }

    fn files(&self) -> DragEventFiles {
        DragEventFiles {
            inner: self
                .as_web_sys_drag_event()
                .data_transfer()
                .unwrap_throw()
                .files()
                .unwrap_throw(),
        }
    }
}

#[derive(Clone)]
pub struct DragEventFiles {
    inner: web_sys::FileList,
}

impl Collection for DragEventFiles {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for DragEventFiles {
    type Item = File;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get(index).map(|f| File::from(f))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

impl input_files_source_seal::Seal for DragEventFiles {
    fn as_web_sys_file_list(&self) -> &web_sys::FileList {
        &self.inner
    }
}

impl InputFilesSource for DragEventFiles {}

#[derive(Clone)]
pub struct DragStartEvent<T> {
    inner: web_sys::DragEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> DragStartEvent<T> {
    pub fn set_data(&self, format_type: &str, data: &str) {
        self.inner
            .data_transfer()
            .unwrap_throw()
            .set_data(format_type, data)
            .unwrap_throw();
    }

    pub fn set_effect_allowed(&self, effect_allowed: DropEffectAllowed) {
        let value = match effect_allowed {
            DropEffectAllowed::None => "none",
            DropEffectAllowed::All => "all",
            DropEffectAllowed::Copy => "copy",
            DropEffectAllowed::CopyLink => "copyLink",
            DropEffectAllowed::CopyMove => "copyMove",
            DropEffectAllowed::Link => "link",
            DropEffectAllowed::LinkMove => "linkMove",
            DropEffectAllowed::Move => "move",
            DropEffectAllowed::Uninitialized => "uninitialized",
        };

        self.inner
            .data_transfer()
            .unwrap_throw()
            .set_effect_allowed(value);
    }

    pub fn set_drag_image<I>(&self, element: I, offset_x: i32, offset_y: i32)
    where
        I: Element,
    {
        self.inner.data_transfer().unwrap_throw().set_drag_image(
            element.as_web_sys_element(),
            offset_x,
            offset_y,
        );
    }
}

impl<T> drag_drop_event_seal::Seal for DragStartEvent<T> {
    fn as_web_sys_drag_event(&self) -> &web_sys::DragEvent {
        &self.inner
    }
}

impl<T> DragDropEvent for DragStartEvent<T> {}

impl<T> drag_drop_read_seal::Seal for DragStartEvent<T> {
    fn as_web_sys_drag_event(&self) -> &web_sys::DragEvent {
        &self.inner
    }
}

impl<T> DragDropRead for DragStartEvent<T> {}

impl<T> AsRef<web_sys::DragEvent> for DragStartEvent<T> {
    fn as_ref(&self) -> &web_sys::DragEvent {
        &self.inner
    }
}

impl_ui_event_traits!(DragStartEvent, DragEvent, "dragstart");
impl_mouse_event_traits!(DragStartEvent);

#[derive(Clone)]
pub struct DropEvent<T> {
    inner: web_sys::DragEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> drag_drop_event_seal::Seal for DropEvent<T> {
    fn as_web_sys_drag_event(&self) -> &web_sys::DragEvent {
        &self.inner
    }
}

impl<T> DragDropEvent for DropEvent<T> {}

impl<T> drag_drop_read_seal::Seal for DropEvent<T> {
    fn as_web_sys_drag_event(&self) -> &web_sys::DragEvent {
        &self.inner
    }
}

impl<T> DragDropRead for DropEvent<T> {}

impl<T> AsRef<web_sys::DragEvent> for DropEvent<T> {
    fn as_ref(&self) -> &web_sys::DragEvent {
        &self.inner
    }
}

impl_ui_event_traits!(DropEvent, DragEvent, "drop");
impl_mouse_event_traits!(DropEvent);

macro_rules! protected_mode_drag_event {
    ($event:ident, $name:literal) => {
        #[derive(Clone)]
        pub struct $event<T> {
            inner: web_sys::DragEvent,
            _marker: marker::PhantomData<T>,
        }

        impl<T> drag_drop_event_seal::Seal for $event<T> {
            fn as_web_sys_drag_event(&self) -> &web_sys::DragEvent {
                &self.inner
            }
        }

        impl<T> DragDropEvent for $event<T> {}

        impl<T> AsRef<web_sys::DragEvent> for $event<T> {
            fn as_ref(&self) -> &web_sys::DragEvent {
                &self.inner
            }
        }

        $crate::ui::impl_ui_event_traits!($event, DragEvent, $name);
        $crate::ui::impl_mouse_event_traits!($event);
    };
}

protected_mode_drag_event!(DragEvent, "drag");
protected_mode_drag_event!(DragEnterEvent, "dragenter");
protected_mode_drag_event!(DragLeaveEvent, "dragleave");
protected_mode_drag_event!(DragOverEvent, "dragover");
protected_mode_drag_event!(DragEndEvent, "dragend");
