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

        impl Into<wasm_bindgen::JsValue> for $tpe {
            fn into(self) -> wasm_bindgen::JsValue {
                self.inner.into()
            }
        }

        impl $crate::console::ToArgument for $tpe {
            fn to_argument(&self) -> $crate::console::Argument {
                let as_js_value: &wasm_bindgen::JsValue = self.as_ref();

                $crate::console::ToArgument::to_argument(as_js_value)
            }
        }
    };
}

pub(crate) use impl_common_wrapper_traits;
