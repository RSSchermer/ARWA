use delegate::delegate;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::connection::{connection_event_target_seal, ConnectionEventTarget};
use crate::crypto::Crypto;
use crate::cssom::{CssReadOnlyStyleDeclaration, Screen};
use crate::dom::{DynamicDocument, DynamicElement, Element};
use crate::event::{impl_event_target_traits, impl_try_from_event_target, typed_event_iterator};
use crate::execution::{execution_event_target_seal, ExecutionEventTarget};
use crate::fetch::{
    cache_context_seal, fetch_context_seal, CacheContext, Fetch, FetchContext, Request,
};
use crate::history::History;
use crate::html::{slot_change_event_target_seal, CustomElementRegistry, SlotChangeEventTarget};
use crate::message::{message_event_target_seal, MessageEventTarget};
use crate::performance::Performance;
use crate::scroll::{scrollable_seal, ScrollByOptions, ScrollToOptions, Scrollable};
use crate::security::{security_context_seal, SecurityContext};
use crate::storage::Storage;
use crate::timer::{timer_context_seal, Duration, Interval, Timeout, TimerContext};
use crate::ui::{ui_event_target_seal, UiEventTarget};
use crate::window::{
    AfterPrintEvent, AppInstalledEvent, BeforePrintEvent, BeforeUnloadEvent, HashChangeEvent,
    LocationBar, MenuBar, PageHideEvent, PageShowEvent, PersonalBar, PopStateEvent,
    RequestAnimationFrame, ScrollBars, StatusBar, StorageEvent, ToolBar, WindowLocation,
    WindowNavigator,
};

pub fn window() -> Option<Window> {
    web_sys::window().map(|inner| inner.into())
}

#[derive(Clone)]
pub struct Window {
    inner: web_sys::Window,
}

impl Window {
    // Skip `frames` for now, implementation gets messy and the naming seems confusing, prefer
    // `document.query_selector_all("iframe")` instead.

    pub fn name(&self) -> String {
        // No indication in the spec that this can actually fail, unwrap for now.
        self.inner.name().unwrap_throw()
    }

    pub fn set_name(&self, name: &str) {
        // No indication in the spec that this can actually fail, unwrap for now.
        self.inner.set_name(name).unwrap_throw();
    }

    pub fn document(&self) -> DynamicDocument {
        // The spec gives no indication this can be null in a browser context, neither for top
        // level contexts, nor for iframed contexts (though as I understand it the iframe's
        // contentWindow can be null, but if it isn't then the contentWindow's document is never
        // null).
        self.inner.document().unwrap_throw().into()
    }

    // TODO: custom_elements

    pub fn crypto(&self) -> Crypto {
        // It's unclear to me if/when window.crypto could fail from the spec, unwrap for now.
        self.inner.crypto().unwrap_throw().into()
    }

    pub fn custom_elements(&self) -> CustomElementRegistry {
        self.inner.custom_elements().into()
    }

    pub fn history(&self) -> History {
        // No indication in the spec that this can actually fail, unwrap for now.
        self.inner.history().unwrap_throw().into()
    }

    // TODO: indexed_db

    pub fn navigator(&self) -> WindowNavigator {
        self.inner.navigator().into()
    }

    pub fn location(&self) -> WindowLocation {
        self.inner.location().into()
    }

    pub fn performance(&self) -> Performance {
        // There is no indication in the spec that `performance` could ever return `null` in modern
        // (WASM-capable) browsers, unwrap for now.
        self.inner.performance().unwrap_throw().into()
    }

    pub fn screen(&self) -> Screen {
        // There is no indication in the spec that `performance` could ever return `null` in modern
        // (WASM-capable) browsers, unwrap for now.
        self.inner.screen().unwrap_throw().into()
    }

    pub fn session_storage(&self) -> Option<Storage> {
        self.inner
            .session_storage()
            .ok()
            .flatten()
            .map(|s| s.into())
    }

    pub fn location_bar(&self) -> LocationBar {
        // No indication in the spec that this can actually fail, unwrap for now.
        LocationBar::new(self.inner.locationbar().unwrap_throw())
    }

    pub fn menu_bar(&self) -> MenuBar {
        // No indication in the spec that this can actually fail, unwrap for now.
        MenuBar::new(self.inner.menubar().unwrap_throw())
    }

    pub fn personal_bar(&self) -> PersonalBar {
        // No indication in the spec that this can actually fail, unwrap for now.
        PersonalBar::new(self.inner.personalbar().unwrap_throw())
    }

    pub fn scroll_bars(&self) -> ScrollBars {
        // No indication in the spec that this can actually fail, unwrap for now.
        ScrollBars::new(self.inner.scrollbars().unwrap_throw())
    }

    pub fn status_bar(&self) -> StatusBar {
        // No indication in the spec that this can actually fail, unwrap for now.
        StatusBar::new(self.inner.statusbar().unwrap_throw())
    }

    pub fn tool_bar(&self) -> ToolBar {
        // No indication in the spec that this can actually fail, unwrap for now.
        ToolBar::new(self.inner.toolbar().unwrap_throw())
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

    pub fn frame_element(&self) -> Option<DynamicElement> {
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

    pub fn screen_left(&self) -> f64 {
        self.inner
            .screen_x()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0)
    }

    pub fn screen_top(&self) -> f64 {
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
        self.inner.alert_with_message(message).unwrap_throw();
    }

    pub fn blur(&self) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.blur().unwrap_throw();
    }

    pub fn focus(&self) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.focus().unwrap_throw();
    }

    // TODO: `open` with typed window features, rather than a string?

    pub fn close(&self) {
        // Close fails when called on a window that was not opened by the current script. However,
        // this does not seem to cause an exception, it simply puts a warning message in the
        // console (tested on Firefox 71.0 and Chromium 79.0).
        self.inner.close().unwrap_throw();
    }

    pub fn stop(&self) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.stop().unwrap_throw();
    }

    pub fn print(&self) {
        // Spec indicates that print may fail, but won't throw an exception (will either silently
        // fail or might e.g. report to the user that no printers are available); unwrap for now.
        self.inner.print().unwrap_throw();
    }

    pub fn prompt(&self, message: &str) -> Option<String> {
        self.inner.prompt_with_message(message).ok().flatten()
    }

    pub fn confirm(&self, message: &str) -> bool {
        self.inner.confirm_with_message(message).unwrap_or(false)
    }

    pub fn computed_style_for<E>(&self, element: E) -> CssReadOnlyStyleDeclaration
    where
        E: Element,
    {
        // This should never fail
        self.inner
            .get_computed_style(element.as_web_sys_element())
            .unwrap_throw()
            .unwrap_throw()
            .into()
    }

    // TODO: pseudo selector parsing
    // pub fn computed_style_for_psuedo<E>(
    //     &self,
    //     element: E,
    //     pseudo: &str,
    // ) -> Result<CssReadOnlyStyleDeclaration, InvalidPseudoElement>
    // where
    //     E: Element,
    // {
    //     self.inner
    //         .get_computed_style_with_pseudo_elt(element.as_ref(), pseudo)
    //         .map(|ok| {
    //             // No indication in the spec that this can be null, unwrap for now.
    //             ok.unwrap_throw().into()
    //         })
    //         .map_err(|err| {
    //             let err: js_sys::TypeError = err.unchecked_into();
    //
    //             TypeError::new(err)
    //         })
    // }

    pub fn move_by(&self, x: i32, y: i32) {
        // Note: move_by can fail, but this fails silently and does not throw an error, unwrap for
        // now.
        self.inner.move_by(x, y).unwrap_throw();
    }

    pub fn move_to(&self, x: i32, y: i32) {
        // Note: move_to can fail, but this fails silently and does not throw an error, unwrap for
        // now.
        self.inner.move_to(x, y).unwrap_throw();
    }

    pub fn resize_by(&self, x: i32, y: i32) {
        // Note: resize_by can fail, but this fails silently and does not throw an error, unwrap for
        // now.
        self.inner.resize_by(x, y).unwrap_throw();
    }

    pub fn resize_to(&self, x: i32, y: i32) {
        // Note: resize_to can fail, but this fails silently and does not throw an error, unwrap for
        // now.
        self.inner.resize_to(x, y).unwrap_throw();
    }

    pub fn request_animation_frame(&self) -> RequestAnimationFrame {
        RequestAnimationFrame::new(self.inner.clone())
    }

    // Note: ignoring `getSelection` here, prefer accessing the selection through the document.

    pub fn on_before_print(&self) -> OnBeforePrint<Self> {
        OnBeforePrint::new(self.inner.as_ref())
    }

    pub fn on_after_print(&self) -> OnAfterPrint<Self> {
        OnAfterPrint::new(self.inner.as_ref())
    }

    pub fn on_app_installed(&self) -> OnAppInstalled<Self> {
        OnAppInstalled::new(self.inner.as_ref())
    }

    pub fn on_page_show(&self) -> OnPageShow<Self> {
        OnPageShow::new(self.inner.as_ref())
    }

    pub fn on_page_hide(&self) -> OnPageHide<Self> {
        OnPageHide::new(self.inner.as_ref())
    }

    pub fn on_before_unload(&self) -> OnBeforeUnload<Self> {
        OnBeforeUnload::new(self.inner.as_ref())
    }

    pub fn on_hash_change(&self) -> OnHashChange<Self> {
        OnHashChange::new(self.inner.as_ref())
    }

    pub fn on_pop_state(&self) -> OnPopState<Self> {
        OnPopState::new(self.inner.as_ref())
    }

    pub fn on_storage(&self) -> OnStorage<Self> {
        OnStorage::new(self.inner.as_ref())
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

impl execution_event_target_seal::Seal for Window {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl ExecutionEventTarget for Window {}

impl connection_event_target_seal::Seal for Window {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl ConnectionEventTarget for Window {}

impl message_event_target_seal::Seal for Window {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl MessageEventTarget for Window {}

impl scrollable_seal::Seal for Window {}

impl Scrollable for Window {
    fn scroll_left(&self) -> f64 {
        self.inner.scroll_x().unwrap_or(0.0)
    }

    fn scroll_top(&self) -> f64 {
        self.inner.scroll_y().unwrap_or(0.0)
    }

    fn scroll_to(&self, options: ScrollToOptions) {
        let mut opts = web_sys::ScrollToOptions::new();

        opts.left(options.left.into());
        opts.top(options.top.into());
        opts.behavior(options.behavior.into());

        self.inner.scroll_to_with_scroll_to_options(&opts);
    }

    fn scroll_by(&self, options: ScrollByOptions) {
        let mut opts = web_sys::ScrollToOptions::new();

        opts.left(options.x.into());
        opts.top(options.y.into());
        opts.behavior(options.behavior.into());

        self.inner.scroll_by_with_scroll_to_options(&opts);
    }
}

impl fetch_context_seal::Seal for Window {}

impl FetchContext for Window {
    fn fetch(&self, request: &Request) -> Fetch {
        Fetch::window_context(self.inner.clone(), Clone::clone(request.as_ref()))
    }
}

impl cache_context_seal::Seal for Window {}

impl CacheContext for Window {}

impl timer_context_seal::Seal for Window {}

impl TimerContext for Window {
    fn interval(&self, duration: Duration) -> Interval {
        Interval::window_context(self.inner.clone(), duration)
    }

    fn timeout(&self, duration: Duration) -> Timeout {
        Timeout::window_context(self.inner.clone(), duration)
    }
}

impl security_context_seal::Seal for Window {}

impl SecurityContext for Window {
    delegate! {
        target self.inner {
            fn is_secure_context(&self) -> bool;

            fn origin(&self) -> String;
        }
    }
}

impl ui_event_target_seal::Seal for Window {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl UiEventTarget for Window {}

impl slot_change_event_target_seal::Seal for Window {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl SlotChangeEventTarget for Window {}

impl_event_target_traits!(Window);
impl_try_from_event_target!(Window);

typed_event_iterator!(
    OnBeforePrint,
    OnBeforePrintWithOptions,
    BeforePrintEvent,
    "beforeprint"
);
typed_event_iterator!(
    OnAfterPrint,
    OnAfterPrintWithOptions,
    AfterPrintEvent,
    "afterprint"
);
typed_event_iterator!(
    OnAppInstalled,
    OnAppInstalledWithOptions,
    AppInstalledEvent,
    "appinstalled"
);
typed_event_iterator!(OnPageShow, OnPageShowWithOptions, PageShowEvent, "pageshow");
typed_event_iterator!(OnPageHide, OnPageHideWithOptions, PageHideEvent, "pagehide");
typed_event_iterator!(
    OnBeforeUnload,
    OnBeforeUnloadWithOptions,
    BeforeUnloadEvent,
    "beforeunload"
);
typed_event_iterator!(
    OnHashChange,
    OnHashChangeWithOptions,
    HashChangeEvent,
    "hashchange"
);
typed_event_iterator!(OnPopState, OnPopStateWithOptions, PopStateEvent, "popstate");
typed_event_iterator!(OnStorage, OnStorageWithOptions, StorageEvent, "storage");
