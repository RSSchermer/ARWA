macro_rules! impl_js_cast {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl wasm_bindgen::JsCast for $tpe {
            fn instanceof(val: &wasm_bindgen::JsValue) -> bool {
                <web_sys::$web_sys_tpe as wasm_bindgen::JsCast>::instanceof(val)
            }

            fn unchecked_from_js(val: wasm_bindgen::JsValue) -> Self {
                $tpe {
                    inner: <web_sys::$web_sys_tpe as wasm_bindgen::JsCast>::unchecked_from_js(val),
                }
            }

            fn unchecked_from_js_ref(val: &wasm_bindgen::JsValue) -> &Self {
                // Note: we essentially know this must be safe, because the implementation of
                // `uncheck_from_js` ensures the type is a single field struct that wraps a JsValue
                // type.
                unsafe { std::mem::transmute(val) }
            }
        }

        impl TryFrom<wasm_bindgen::JsValue> for $tpe {
            type Error = $crate::InvalidCast<wasm_bindgen::JsValue, $tpe>;

            fn try_from(value: wasm_bindgen::JsValue) -> Result<$tpe, Self::Error> {
                use wasm_bindgen::JsCast;

                value
                    .dyn_into::<$tpe>()
                    .map_err(|e| $crate::InvalidCast::new(e.into()))
            }
        }
    };
    ($tpe:ident) => {
        $crate::impl_js_cast!($tpe, $tpe);
    };
}

pub(crate) use impl_js_cast;
