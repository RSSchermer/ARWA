macro_rules! dom_exception_wrapper {
    ($(#[$($doc:tt)*])* $wrapper_type:ident) => {
        $(#[$($doc)*])*
        #[derive(Clone)]
        pub struct $wrapper_type {
            inner: web_sys::DomException,
        }

        impl $wrapper_type {
            pub(crate) fn new(inner: web_sys::DomException) -> Self {
                $wrapper_type { inner }
            }
        }

        impl From<$wrapper_type> for wasm_bindgen::JsValue {
            fn from(value: $wrapper_type) -> wasm_bindgen::JsValue {
                value.into()
            }
        }

        impl std::fmt::Display for $wrapper_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut message = self.inner.message();

                $crate::exception_wrapper::normalize_exception_message(&mut message);

                std::fmt::Display::fmt(&message, f)
            }
        }

        impl std::fmt::Debug for $wrapper_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(self, f)
            }
        }

        impl std::error::Error for $wrapper_type {}

        impl AsRef<web_sys::DomException> for $wrapper_type {
            fn as_ref(&self) -> &web_sys::DomException {
                &self.inner
            }
        }

        impl $crate::console::ToArgument for $wrapper_type {
            fn to_argument(&self) -> $crate::console::Argument {
                let js_value: &wasm_bindgen::JsValue = self.inner.as_ref();

                $crate::console::ToArgument::to_argument(js_value)
            }
        }
    };
}

pub(crate) use dom_exception_wrapper;

macro_rules! type_error_wrapper {
    ($wrapper_type:ident) => {
        #[derive(Clone)]
        pub struct $wrapper_type {
            inner: js_sys::TypeError,
        }

        impl $wrapper_type {
            pub(crate) fn new(inner: js_sys::TypeError) -> Self {
                $wrapper_type { inner }
            }
        }

        impl From<$wrapper_type> for wasm_bindgen::JsValue {
            fn from(value: $wrapper_type) -> wasm_bindgen::JsValue {
                value.into()
            }
        }

        impl std::fmt::Display for $wrapper_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut message: String = self.inner.message().into();

                $crate::exception_wrapper::normalize_exception_message(&mut message);

                std::fmt::Display::fmt(&message, f)
            }
        }

        impl std::fmt::Debug for $wrapper_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(self, f)
            }
        }

        impl std::error::Error for $wrapper_type {}

        impl AsRef<js_sys::TypeError> for $wrapper_type {
            fn as_ref(&self) -> &js_sys::TypeError {
                &self.inner
            }
        }

        impl $crate::console::ToArgument for $wrapper_type {
            fn to_argument(&self) -> $crate::console::Argument {
                let js_value: &wasm_bindgen::JsValue = self.inner.as_ref();

                $crate::console::ToArgument::to_argument(js_value)
            }
        }
    };
}

pub(crate) use type_error_wrapper;

pub(crate) fn normalize_exception_message(message: &mut String) {
    // Modify the message to match Rust's conventions for error messages.

    // Decapitalize the first character, but only if the second character is ascii
    // lower case (or a space), otherwise assume the first word is an acronym
    let second_char_lower_case = if let Some(second_char) = message.chars().nth(1) {
        second_char.is_ascii_lowercase() || second_char.is_ascii_whitespace()
    } else {
        true
    };

    if let Some(first_char) = message.get_mut(0..1) {
        if second_char_lower_case {
            first_char.make_ascii_lowercase()
        }
    }

    // Remove punctuation at the end.
    if let Some(c) = message.chars().last() {
        if c == '.' || c == '!' {
            message.truncate(message.len() - 1);
        }
    }
}
