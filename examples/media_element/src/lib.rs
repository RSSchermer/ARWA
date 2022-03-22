#![feature(async_closure)]
use arwa::dom::{selector, ParentNode};
use arwa::html::{HtmlAudioElement, MediaElement};
use arwa::ui::UiEventTarget;
use arwa::window::window;
use arwa::{console, spawn_local};
use futures::StreamExt;
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
    let mut play_clicks = play_button.on_click();

    spawn_local(async move {
        while let Some(_) = play_clicks.next().await {
            match audio_clone.play().await {
                Ok(_) => console::log!("Started playing!"),
                Err(err) => console::error!(err),
            }
        }
    });

    let pause_button = document
        .query_selector_first(&selector!("#pause"))
        .ok_or(JsError::new("No element with the id `pause`"))?;
    let mut pause_clicks = pause_button.on_click();

    spawn_local(async move {
        while let Some(_) = pause_clicks.next().await {
            if !audio.paused() {
                audio.pause();
                console::log!("Paused...");
            }
        }
    });

    Ok(())
}
