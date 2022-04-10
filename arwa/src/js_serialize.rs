use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js_support.js")]
extern "C" {
    #[wasm_bindgen(js_name = __arwa_js_serialize)]
    pub(crate) fn js_serialize(wasm_memory: &JsValue, pointer: *mut (), size: u32) -> Uint8Array;

    #[wasm_bindgen(js_name = __arwa_js_deserialize)]
    pub(crate) fn js_deserialize(
        wasm_memory: &JsValue,
        pointer: *mut (),
        serialized_data: &Uint8Array,
    );
}
