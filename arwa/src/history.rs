use crate::console::{Write, Writer};
use crate::security::SecurityError;
use url::Url;
use wasm_bindgen::JsValue;

pub enum ScrollRestoration {
    Auto,
    Manual,
}

pub struct History {
    inner: web_sys::History,
}

impl History {
    // Note: preferring verb based method names here, as to me that tracks much better with `try_`
    // and every method here needs a fallible alternative.

    fn count_entries(&self) -> u32 {
        self.inner
            .length()
            .map_err(|e| SecurityError::new(e.unchecked_into()))
    }

    fn try_count_entries(&self) -> Result<u32, SecurityError> {
        self.inner
            .length()
            .map_err(|e| SecurityError::new(e.unchecked_into()))
    }

    fn get_state(&self) -> Option<JsValue> {
        let state = self.inner.state().unwrap_throw();

        if state.is_null() {
            None
        } else {
            Some(state)
        }
    }

    fn try_get_state(&self) -> Result<Option<JsValue>, SecurityError> {
        self.inner
            .state()
            .map(|state| if state.is_null() { None } else { Some(state) })
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    fn get_scroll_restoration(&self) -> ScrollRestoration {
        match self.inner.scroll_restoration().unwrap_throw() {
            web_sys::ScrollRestoration::Auto => ScrollRestoration::Auto,
            web_sys::ScrollRestoration::Manual => ScrollRestoration::Manual,
        }
    }

    fn try_get_scroll_restoration(&self) -> Result<ScrollRestoration, SecurityError> {
        self.inner
            .scroll_restoration()
            .map(|r| match r {
                web_sys::ScrollRestoration::Auto => ScrollRestoration::Auto,
                web_sys::ScrollRestoration::Manual => ScrollRestoration::Manual,
            })
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    fn set_scroll_restoration(&self, scroll_restoration: ScrollRestoration) {
        let raw = match scroll_restoration {
            ScrollRestoration::Auto => web_sys::ScrollRestoration::Auto,
            ScrollRestoration::Manual => web_sys::ScrollRestoration::Manual,
        };

        self.inner.set_scroll_restoration(raw).unwrap_throw()
    }

    fn try_set_scroll_restoration(
        &self,
        scroll_restoration: ScrollRestoration,
    ) -> Result<(), SecurityError> {
        let raw = match scroll_restoration {
            ScrollRestoration::Auto => web_sys::ScrollRestoration::Auto,
            ScrollRestoration::Manual => web_sys::ScrollRestoration::Manual,
        };

        self.inner
            .set_scroll_restoration(raw)
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    fn go(&self, delta: i32) {
        self.inner.go_with_delta(delta).unwrap_throw()
    }

    fn try_go(&self) -> Result<(), SecurityError> {
        self.inner
            .go_with_delta(delta)
            .map_err(|e| SecurityError::new(e.unchecked_into()))
    }

    fn go_back(&self) {
        self.inner.back().unwrap_throw()
    }

    fn try_go_back(&self) -> Result<(), SecurityError> {
        self.inner
            .back()
            .map_err(|e| SecurityError::new(e.unchecked_into()))
    }

    fn go_forward(&self) {
        self.inner.forward().unwrap_throw()
    }

    fn try_go_forward(&self) -> Result<(), SecurityError> {
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

#[derive(Clone)]
pub enum StateError {
    SerializationError(StateSerializationError),
    SecurityError(SecurityError),
}

impl StateError {
    fn new(error: web_sys::DomException) -> Self {
        if error.code() == 18 {
            StateError::SecurityError(SecurityError::new(error))
        } else {
            StateError::SerializationError(StateSerializationError::new(error))
        }
    }
}

#[derive(Clone)]
pub struct StateSerializationError {
    inner: web_sys::DomException,
}

impl StateSerializationError {
    fn new(inner: web_sys::DomException) -> Self {
        StateSerializationError { inner }
    }
}
