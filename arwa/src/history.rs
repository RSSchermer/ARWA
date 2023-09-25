use wasm_bindgen::{throw_val, JsCast, JsValue};

use crate::security::SecurityError;
use crate::{dom_exception_wrapper, impl_common_wrapper_traits, impl_js_cast};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScrollRestoration {
    Auto,
    Manual,
}

impl ScrollRestoration {
    fn from_web_sys(scroll_restoration: web_sys::ScrollRestoration) -> Self {
        match scroll_restoration {
            web_sys::ScrollRestoration::Auto => ScrollRestoration::Auto,
            web_sys::ScrollRestoration::Manual => ScrollRestoration::Manual,
            _ => unreachable!(),
        }
    }

    fn to_web_sys(&self) -> web_sys::ScrollRestoration {
        match self {
            ScrollRestoration::Auto => web_sys::ScrollRestoration::Auto,
            ScrollRestoration::Manual => web_sys::ScrollRestoration::Manual,
        }
    }
}

pub struct History {
    inner: web_sys::History,
}

impl History {
    // Note: preferring verb based method names here, as to me that tracks much better with `try_`
    // and every method here needs a fallible alternative.

    pub fn count_entries(&self) -> u32 {
        match self.inner.length() {
            Ok(length) => length,
            Err(err) => throw_val(err),
        }
    }

    pub fn try_count_entries(&self) -> Result<u32, SecurityError> {
        self.inner
            .length()
            .map_err(|e| SecurityError::new(e.unchecked_into()))
    }

    pub fn get_state(&self) -> Option<JsValue> {
        let state = match self.inner.state() {
            Ok(state) => state,
            Err(err) => throw_val(err),
        };

        if state.is_null() {
            None
        } else {
            Some(state)
        }
    }

    pub fn try_get_state(&self) -> Result<Option<JsValue>, SecurityError> {
        self.inner
            .state()
            .map(|state| if state.is_null() { None } else { Some(state) })
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    pub fn get_scroll_restoration(&self) -> ScrollRestoration {
        let scroll_restoration = match self.inner.scroll_restoration() {
            Ok(scroll_restoration) => scroll_restoration,
            Err(err) => throw_val(err),
        };

        ScrollRestoration::from_web_sys(scroll_restoration)
    }

    pub fn try_get_scroll_restoration(&self) -> Result<ScrollRestoration, SecurityError> {
        self.inner
            .scroll_restoration()
            .map(|r| ScrollRestoration::from_web_sys(r))
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    pub fn set_scroll_restoration(&self, scroll_restoration: ScrollRestoration) {
        if let Err(err) = self
            .inner
            .set_scroll_restoration(scroll_restoration.to_web_sys())
        {
            throw_val(err)
        }
    }

    pub fn try_set_scroll_restoration(
        &self,
        scroll_restoration: ScrollRestoration,
    ) -> Result<(), SecurityError> {
        self.inner
            .set_scroll_restoration(scroll_restoration.to_web_sys())
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    pub fn go(&self, delta: i32) {
        if let Err(err) = self.inner.go_with_delta(delta) {
            throw_val(err)
        }
    }

    pub fn try_go(&self, delta: i32) -> Result<(), SecurityError> {
        // Note: `go` does not do bounds checking, per the spec its a no-op if the target entry is
        // out of bounds.

        self.inner
            .go_with_delta(delta)
            .map_err(|e| SecurityError::new(e.unchecked_into()))
    }

    pub fn go_back(&self) {
        if let Err(err) = self.inner.back() {
            throw_val(err)
        }
    }

    pub fn try_go_back(&self) -> Result<(), SecurityError> {
        self.inner
            .back()
            .map_err(|e| SecurityError::new(e.unchecked_into()))
    }

    pub fn go_forward(&self) {
        if let Err(err) = self.inner.forward() {
            throw_val(err)
        }
    }

    pub fn try_go_forward(&self) -> Result<(), SecurityError> {
        self.inner
            .forward()
            .map_err(|e| SecurityError::new(e.unchecked_into()))
    }

    // TODO: same as with `postMessage`. There's arguably potential to leverage the type system to
    // statically avoid serialization errors at runtime. Should we?
    //
    // fn push_state(&self, state: &JsValue, url: Option<&Url>) {
    //     self.inner.push_state_with_url(state, "", url.map(|url| url.as_ref())).unwrap_throw()
    // }
    //
    // fn try_push_state(&self, state: &JsValue, url: Option<&Url>) -> Result<(), StateError> {
    //     self.inner.push_state_with_url(state, "", url.map(|url| url.as_ref())).map_err(|err| StateError::new(err.unchecked_into()))
    // }
    //
    // fn replace_state(&self, state: &JsValue, url: Option<&Url>) {
    //     self.inner.replace_state_with_url(state, "", url.map(|url| url.as_ref())).unwrap_throw()
    // }
    //
    // fn try_replace_state(&self, state: &JsValue, url: Option<&Url>) -> Result<(), StateError> {
    //     self.inner.replace_state_with_url(state, "", url.map(|url| url.as_ref())).map_err(|err| StateError::new(err.unchecked_into()))
    // }
}

impl From<web_sys::History> for History {
    fn from(inner: web_sys::History) -> Self {
        History { inner }
    }
}

impl AsRef<web_sys::History> for History {
    fn as_ref(&self) -> &web_sys::History {
        &self.inner
    }
}

impl_common_wrapper_traits!(History);
impl_js_cast!(History);

dom_exception_wrapper!(StateError);

dom_exception_wrapper!(StateSerializationError);
