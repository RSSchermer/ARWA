#![feature(async_closure)]

use futures::{FutureExt, StreamExt, TryFutureExt};
use rudo::html::{HtmlAudioElement, HtmlButtonElement, HtmlMediaElement};
use rudo::GlobalEventHandlers;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn start() {
    // Obtain a reference to the `web_sys::HtmlButtonElement` we want to listen to.
    let audio: web_sys::HtmlAudioElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("audio")
        .unwrap()
        .dyn_into()
        .unwrap();

    let audio = HtmlAudioElement::from(audio);

    let play_button: web_sys::HtmlButtonElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("play")
        .unwrap()
        .dyn_into()
        .unwrap();

    let play_button = HtmlButtonElement::from(play_button);

    spawn_local(
        audio
            .play()
            .map_ok(|_| {
                web_sys::console::log_1(&"Started playing!".into());
            })
            .map_err(|err| {
                web_sys::console::log_1(&format!("{:?}", err).into());
            })
            .map(|_| ()),
    );

    spawn_local(play_button.on_click().for_each(move |_| {
        web_sys::console::log_1(&"Click!".into());

        audio
            .play()
            .map_ok(|_| {
                web_sys::console::log_1(&"Started playing!".into());
            })
            .map_err(|err| {
                web_sys::console::log_1(&format!("{:?}", err).into());
            })
            .map(|_| ())
    }));
}
