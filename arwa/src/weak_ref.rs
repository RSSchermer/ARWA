use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type WeakRef;

    #[wasm_bindgen(constructor)]
    pub fn new(referenced: &JsValue) -> WeakRef;

    #[wasm_bindgen(method)]
    pub fn deref(this: &WeakRef) -> JsValue;
}
