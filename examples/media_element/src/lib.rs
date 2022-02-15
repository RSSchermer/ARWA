#![feature(async_closure)]
use std::convert::TryInto;

use arwa::html::{HtmlAudioElement, HtmlButtonElement, MediaElement};
use arwa::{console, document, Document, GlobalEventHandlers};
use futures::{FutureExt, StreamExt, TryFutureExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn start() {
    let document = document().unwrap();

    // Obtain a reference to the `web_sys::HtmlButtonElement` we want to listen to.
    let audio: HtmlAudioElement = document
        .query_id("audio")
        .expect("No element with the id `audio`")
        .try_into()
        .expect("Element is not an audio element");

    let play_button: HtmlButtonElement = document
        .query_id("play")
        .expect("No element with the id `play`")
        .try_into()
        .expect("Element is not an button element");

    spawn_local(play_button.on_click().for_each(move |_| {
        console::log!("Click!");

        audio
            .play()
            .map_ok(|_| {
                console::log!("Started playing!");
            })
            .map_err(|err| {
                console::log!("{:?}", err);
            })
            .map(|_| ())
    }));
}
