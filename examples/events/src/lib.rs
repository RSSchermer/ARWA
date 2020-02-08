#![feature(async_closure)]
use std::convert::TryInto;

use arwa::html::HtmlButtonElement;
use arwa::{console, document, Document, GlobalEventHandlers};
use futures::StreamExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn start() {
    let document = document().unwrap();

    // Obtain a reference to the HtmlButtonElement we want to listen to.
    let button: HtmlButtonElement = document
        .query_id("button")
        .expect("No element with id `button`.")
        .try_into()
        .expect("Element is not a button element.");

    spawn_local(button.on_click().for_each(async move |_| {
        console::log!("Click!");
    }));
}
