#![feature(async_closure)]
use arwa::dom::{selector, ParentNode};
use arwa::html::{HtmlAudioElement, MediaElement};
use arwa::ui::UiEventTarget;
use arwa::window::window;
use arwa::{console, spawn_local};
use futures::{FutureExt, StreamExt, TryFutureExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = window().document();

    // Obtain a reference to the `web_sys::HtmlButtonElement` we want to listen to.
    let audio: HtmlAudioElement = document
        .query_selector_first(&selector!("#audio"))
        .ok_or(JsError::new("No element with the id `audio`"))?
        .try_into()?;

    let play_button = document
        .query_selector_first(&selector!("#play"))
        .ok_or(JsError::new("No element with the id `play`"))?;

    let audio_clone = audio.clone();

    spawn_local(play_button.on_click().for_each(move |_| {
        audio_clone
            .play()
            .map_ok(|_| {
                console::log!("Started playing!");
            })
            .map_err(|err| {
                console::error!(err);
            })
            .map(|_| ())
    }));

    let pause_button = document
        .query_selector_first(&selector!("#pause"))
        .ok_or(JsError::new("No element with the id `pause`"))?;

    spawn_local(pause_button.on_click().for_each(move |_| {
        audio.pause();
        console::log!("Paused...");

        futures::future::ready(())
    }));

    Ok(())
}
