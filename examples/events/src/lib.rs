#![feature(async_closure)]

use futures::StreamExt;
use rudo::HtmlButtonElement;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn start() {
    // Obtain a reference to the `web_sys::HtmlButtonElement` we want to listen to.
    let button: web_sys::HtmlButtonElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("button")
        .unwrap()
        .dyn_into()
        .unwrap();

    let button = HtmlButtonElement::from(button);

    spawn_local(button.on_click().for_each(async move |_| {
        web_sys::console::log_1(&"click!".into());
    }));
}
