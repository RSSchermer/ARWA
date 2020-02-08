use std::cell::{Cell, RefCell};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use delegate::delegate;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::console::{Write, Writer};
use crate::error::TypeError;
use crate::{
    Crypto, CssStyleDeclaration, Element, GenericDocument, GenericElement, GlobalEventHandlers,
    History, Location, Navigator, Performance, Screen, ScrollByOptions, ScrollToOptions, Storage,
};
use futures::task::Waker;

pub fn window() -> Option<Window> {
    web_sys::window().map(|inner| inner.into())
}

pub fn document() -> Option<GenericDocument> {
    window().and_then(|w| w.document())
}

pub fn history() -> Option<History> {
    window().map(|w| w.history())
}

pub fn location() -> Option<Location> {
    window().map(|w| w.location())
}

pub fn navigator() -> Option<Navigator> {
    window().map(|w| w.navigator())
}

pub fn performance() -> Option<Performance> {
    window().map(|w| w.performance())
}

pub fn screen() -> Option<Screen> {
    window().map(|w| w.screen())
}

pub fn session_storage() -> Option<Storage> {
    window().and_then(|w| w.session_storage())
}

#[derive(Clone)]
pub struct Window {
    inner: web_sys::Window,
}

impl Window {
    delegate! {
        target self.inner {
            pub fn is_secure_context(&self) -> bool;

            pub fn origin(&self) -> String;
        }
    }

    // Skip `frames` for now, implementation gets messy and the naming seems confusing, prefer
    // `document.query_selector_all("iframe")` instead.

    pub fn name(&self) -> String {
        // No indication in the spec that this can actually fail, unwrap for now.
        self.inner.name().unwrap()
    }

    pub fn set_name(&self, name: &str) {
        // No indication in the spec that this can actually fail, unwrap for now.
        self.inner.set_name(name).unwrap();
    }

    pub fn status(&self) -> String {
        self.inner.status().unwrap()
    }

    pub fn set_status(&self, status: &str) {
        // MDN indicates that set_status won't work in a modern browser if the user has not
        // specifically allowed setting the status in the config options. However, it seems that in
        // this case setting the status simply does not update the status bar without throwing an
        // exception (a subsequent call to `status` will return the new status; tested on Firefox
        // 71.0 and Chromium 79.0), so we can just unwrap.
        self.inner.set_status(status).unwrap();
    }

    pub fn document(&self) -> Option<GenericDocument> {
        self.inner.document().map(|inner| inner.into())
    }

    // TODO: custom_elements

    pub fn crypto(&self) -> Crypto {
        // It's unclear to me if/when window.crypto could fail from the spec, unwrap for now.
        self.inner.crypto().unwrap().into()
    }

    pub fn history(&self) -> History {
        // No indication in the spec that this can actually fail, unwrap for now.
        self.inner.history().unwrap().into()
    }

    // TODO: indexed_db

    pub fn navigator(&self) -> Navigator {
        self.inner.navigator().into()
    }

    pub fn location(&self) -> Location {
        self.inner.location().into()
    }

    pub fn performance(&self) -> Performance {
        // There is no indication in the spec that `performance` could ever return `null` in modern
        // (WASM-capable) browsers, unwrap for now.
        self.inner.performance().unwrap().into()
    }

    pub fn screen(&self) -> Screen {
        // There is no indication in the spec that `performance` could ever return `null` in modern
        // (WASM-capable) browsers, unwrap for now.
        self.inner.screen().unwrap().into()
    }

    pub fn session_storage(&self) -> Option<Storage> {
        self.inner
            .session_storage()
            .ok()
            .flatten()
            .map(|s| s.into())
    }

    pub fn location_bar(&self) -> LocationBar {
        LocationBar {
            // No indication in the spec that this can actually fail, unwrap for now.
            inner: self.inner.locationbar().unwrap(),
        }
    }

    pub fn menu_bar(&self) -> MenuBar {
        MenuBar {
            // No indication in the spec that this can actually fail, unwrap for now.
            inner: self.inner.menubar().unwrap(),
        }
    }

    pub fn personal_bar(&self) -> PersonalBar {
        PersonalBar {
            // No indication in the spec that this can actually fail, unwrap for now.
            inner: self.inner.personalbar().unwrap(),
        }
    }

    pub fn scroll_bars(&self) -> ScrollBars {
        ScrollBars {
            // No indication in the spec that this can actually fail, unwrap for now.
            inner: self.inner.scrollbars().unwrap(),
        }
    }

    pub fn status_bar(&self) -> StatusBar {
        StatusBar {
            // No indication in the spec that this can actually fail, unwrap for now.
            inner: self.inner.statusbar().unwrap(),
        }
    }

    pub fn tool_bar(&self) -> ToolBar {
        ToolBar {
            // No indication in the spec that this can actually fail, unwrap for now.
            inner: self.inner.toolbar().unwrap(),
        }
    }

    pub fn opener(&self) -> Option<Window> {
        self.inner.opener().ok().map(|window| {
            let window: web_sys::Window = window.unchecked_into();

            window.into()
        })
    }

    pub fn parent(&self) -> Option<Window> {
        self.inner.parent().ok().flatten().map(|w| w.into())
    }

    pub fn top(&self) -> Option<Window> {
        self.inner.top().ok().flatten().map(|w| w.into())
    }

    pub fn frame_element(&self) -> Option<GenericElement> {
        self.inner.frame_element().ok().flatten().map(|e| e.into())
    }

    pub fn inner_width(&self) -> u32 {
        self.inner
            .inner_width()
            .ok()
            .and_then(|value| value.as_f64())
            .map(|width| width as u32)
            .unwrap_or(0)
    }

    pub fn inner_height(&self) -> u32 {
        self.inner
            .inner_height()
            .ok()
            .and_then(|value| value.as_f64())
            .map(|height| height as u32)
            .unwrap_or(0)
    }

    pub fn outer_width(&self) -> u32 {
        self.inner
            .outer_width()
            .ok()
            .and_then(|value| value.as_f64())
            .map(|width| width as u32)
            .unwrap_or(0)
    }

    pub fn outer_height(&self) -> u32 {
        self.inner
            .outer_height()
            .ok()
            .and_then(|value| value.as_f64())
            .map(|height| height as u32)
            .unwrap_or(0)
    }

    pub fn scroll_x(&self) -> f64 {
        self.inner.scroll_x().unwrap_or(0.0)
    }

    pub fn scroll_y(&self) -> f64 {
        self.inner.scroll_y().unwrap_or(0.0)
    }

    pub fn screen_x(&self) -> f64 {
        self.inner
            .screen_x()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0)
    }

    pub fn screen_y(&self) -> f64 {
        self.inner
            .screen_y()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0)
    }

    // Note: the API allows omitting the message argument, in which case an empty string will be
    // used; hence, to emulate calling alert without a message argument, simply call this function
    // with an empty string `""`.
    pub fn alert(&self, message: &str) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.alert_with_message(message).unwrap();
    }

    pub fn blur(&self) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.blur().unwrap();
    }

    pub fn focus(&self) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.focus().unwrap();
    }

    // TODO: `open` with typed window features, rather than a string?

    pub fn close(&self) {
        // Close fails when called on a window that was not opened by the current script. However,
        // this does not seem to cause an exception, it simply puts a warning message in the
        // console (tested on Firefox 71.0 and Chromium 79.0).
        self.inner.close().unwrap();
    }

    pub fn stop(&self) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.stop().unwrap();
    }

    pub fn print(&self) {
        // Spec indicates that print may fail, but won't throw an exception (will either silently
        // fail or might e.g. report to the user that no printers are available); unwrap for now.
        self.inner.print().unwrap();
    }

    pub fn prompt(&self, message: &str) -> Option<String> {
        self.inner.prompt_with_message(message).ok().flatten()
    }

    pub fn confirm(&self, message: &str) -> bool {
        self.inner.confirm_with_message(message).unwrap_or(false)
    }

    pub fn computed_style_for<E>(
        &self,
        element: E,
        pseudo: &str,
    ) -> Result<CssStyleDeclaration, TypeError>
    where
        E: Element,
    {
        self.inner
            .get_computed_style_with_pseudo_elt(element.as_ref(), pseudo)
            .map(|ok| {
                // No indication in the spec that this can be null, unwrap for now.
                ok.unwrap().into()
            })
            .map_err(|err| {
                let err: js_sys::TypeError = err.unchecked_into();

                TypeError::new(err)
            })
    }

    pub fn move_by(&self, x: i32, y: i32) {
        // Note: move_by can fail, but this fails silently and does not throw an error, unwrap for
        // now.
        self.inner.move_by(x, y).unwrap();
    }

    pub fn move_to(&self, x: i32, y: i32) {
        // Note: move_to can fail, but this fails silently and does not throw an error, unwrap for
        // now.
        self.inner.move_to(x, y).unwrap();
    }

    pub fn resize_by(&self, x: i32, y: i32) {
        // Note: resize_by can fail, but this fails silently and does not throw an error, unwrap for
        // now.
        self.inner.resize_by(x, y).unwrap();
    }

    pub fn resize_to(&self, x: i32, y: i32) {
        // Note: resize_to can fail, but this fails silently and does not throw an error, unwrap for
        // now.
        self.inner.resize_to(x, y).unwrap();
    }

    pub fn scroll_to(&self, options: ScrollToOptions) {
        let mut opts = web_sys::ScrollToOptions::new();

        opts.left(options.left.into());
        opts.top(options.top.into());
        opts.behavior(options.behavior.into());

        self.inner.scroll_to_with_scroll_to_options(&opts);
    }

    pub fn scroll_by(&self, options: ScrollByOptions) {
        let mut opts = web_sys::ScrollToOptions::new();

        opts.left(options.x.into());
        opts.top(options.y.into());
        opts.behavior(options.behavior.into());

        self.inner.scroll_by_with_scroll_to_options(&opts);
    }

    pub fn request_animation_frame(&self) -> RequestAnimationFrame {
        RequestAnimationFrame {
            provider: self.inner.clone(),
            state: Rc::new(AnimationFrameState {
                cancelled: Cell::new(false),
                time: Cell::new(None),
                handle: Cell::new(None),
                waker: RefCell::new(None),
            }),
            callback: None,
        }
    }
}

impl From<web_sys::Window> for Window {
    fn from(inner: web_sys::Window) -> Self {
        Window { inner }
    }
}

impl AsRef<web_sys::Window> for Window {
    fn as_ref(&self) -> &web_sys::Window {
        &self.inner
    }
}

impl AsRef<web_sys::EventTarget> for Window {
    fn as_ref(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl Write for Window {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

impl GlobalEventHandlers for Window {}

pub trait BarState {
    fn visible(&self) -> bool;

    fn set_visible(&self, visible: bool);
}

macro_rules! impl_bar_state {
    ($tpe:ident) => {
        impl BarState for $tpe {
            fn visible(&self) -> bool {
                // No indication in the spec this can actually fail, unwrap for now.
                self.inner.visible().unwrap()
            }

            fn set_visible(&self, visible: bool) {
                // No indication in the spec this can actually fail, unwrap for now.
                self.inner.set_visible(visible).unwrap();
            }
        }
    };
}

pub struct LocationBar {
    inner: web_sys::BarProp,
}

impl_bar_state!(LocationBar);

pub struct MenuBar {
    inner: web_sys::BarProp,
}

impl_bar_state!(MenuBar);

pub struct PersonalBar {
    inner: web_sys::BarProp,
}

impl_bar_state!(PersonalBar);

pub struct ScrollBars {
    inner: web_sys::BarProp,
}

impl_bar_state!(ScrollBars);

pub struct StatusBar {
    inner: web_sys::BarProp,
}

impl_bar_state!(StatusBar);

pub struct ToolBar {
    inner: web_sys::BarProp,
}

impl_bar_state!(ToolBar);

pub struct RequestAnimationFrame {
    provider: web_sys::Window,
    state: Rc<AnimationFrameState>,
    callback: Option<Closure<dyn FnMut(JsValue)>>,
}

impl RequestAnimationFrame {
    pub fn handle(&self) -> AnimationFrameHandle {
        AnimationFrameHandle {
            provider: self.provider.clone(),
            state: self.state.clone(),
        }
    }
}

impl Future for RequestAnimationFrame {
    type Output = Result<f64, AnimationFrameCancelled>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let request = self.get_mut();
        let state = &request.state;

        if state.cancelled.get() {
            return Poll::Ready(Err(AnimationFrameCancelled {}));
        }

        if let Some(time) = state.time.get() {
            return Poll::Ready(Ok(time));
        }

        if state.handle.get().is_none() {
            // Initialize
            let waker = cx.waker().clone();
            let state_clone = state.clone();

            let callback = Closure::wrap(Box::new(move |t: JsValue| {
                let t = t.as_f64().unwrap();

                state_clone.time.set(Some(t));
                waker.wake_by_ref();
            }) as Box<dyn FnMut(JsValue)>);

            let handle = request
                .provider
                .request_animation_frame(callback.as_ref().unchecked_ref())
                .unwrap();

            state.handle.set(Some(handle));
            *state.waker.borrow_mut() = Some(cx.waker().clone());

            // Hold on to callback to prevent it from being dropped prematurely.
            request.callback = Some(callback);
        }

        Poll::Pending
    }
}

struct AnimationFrameState {
    cancelled: Cell<bool>,
    time: Cell<Option<f64>>,
    handle: Cell<Option<i32>>,
    waker: RefCell<Option<Waker>>,
}

#[derive(Clone)]
pub struct AnimationFrameHandle {
    provider: web_sys::Window,
    state: Rc<AnimationFrameState>,
}

impl AnimationFrameHandle {
    pub fn cancel(&self) {
        let state = &self.state;

        if state.time.get().is_none() && !state.cancelled.replace(true) {
            if let Some(handle) = state.handle.replace(None) {
                self.provider.cancel_animation_frame(handle).unwrap();
            }

            if let Some(waker) = state.waker.borrow_mut().take() {
                waker.wake_by_ref();
            }
        }
    }
}

pub struct AnimationFrameCancelled {}
