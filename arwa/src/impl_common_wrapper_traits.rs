macro_rules! impl_common_wrapper_traits {
    ($tpe:ident) => {
        impl AsRef<js_sys::Object> for $tpe {
            fn as_ref(&self) -> &js_sys::Object {
                self.inner.as_ref()
            }
        }

        impl AsRef<wasm_bindgen::JsValue> for $tpe {
            fn as_ref(&self) -> &wasm_bindgen::JsValue {
                self.inner.as_ref()
            }
        }
    };
}

pub(crate) use impl_common_wrapper_traits;
