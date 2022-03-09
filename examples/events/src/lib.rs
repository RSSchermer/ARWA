#![feature(async_closure)]
use std::convert::TryInto;

use arwa::dom::{selector, ParentNode};
use arwa::html::HtmlButtonElement;
use arwa::ui::UiEventTarget;
use arwa::window::window;
use arwa::{console, spawn_local};
use futures::StreamExt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    let window = window().unwrap();
    let document = window.document();

    // Obtain a reference to the HtmlButtonElement we want to listen to.
    let button: HtmlButtonElement = document
        .query_selector_first(&selector!("#button"))
        .expect("No element with id `button`.")
        .try_into()
        .expect("Element is not a button element.");

    spawn_local(button.on_click().for_each(async move |_| {
        console::log!("Click!");
    }));
}
