use js_sys::Object;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type FinalizationRegistry;

    #[wasm_bindgen(constructor)]
    pub fn new(callback: &Closure<dyn FnMut(JsValue)>) -> FinalizationRegistry;

    #[wasm_bindgen(method)]
    pub fn register(this: &FinalizationRegistry, target: &JsValue, held_value: &JsValue);

    #[wasm_bindgen(method, js_name = register)]
    pub fn register_with_unregister_token(
        this: &FinalizationRegistry,
        target: &JsValue,
        held_value: &JsValue,
        unregister_token: &JsValue,
    );

    #[wasm_bindgen(method)]
    pub fn unregister(this: &FinalizationRegistry, unregister_token: &JsValue);
}
