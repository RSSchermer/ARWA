#![feature(async_closure)]

use arwa::dom::{selector, ChildNode, ParentNode};
use arwa::ui::UiEventTarget;
use arwa::window::window;
use arwa::{console, spawn_local};
use futures::StreamExt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = window().document();

    // Obtain a reference to the HtmlButtonElement we want to listen to.
    let button = document
        .query_selector_first(&selector!("#button"))
        .ok_or(JsError::new("No element with id `button`."))?;

    let mut click_events = button.on_click();

    spawn_local(async move {
        while let Some(_) = click_events.next().await {
            console::log!("Click!");
        }

        console::log!("Event stream cleaned up")
    });

    let remove_event_target_button = document
        .query_selector_first(&selector!("#remove_event_target_button"))
        .ok_or(JsError::new(
            "No element with id `remove_event_target_button`.",
        ))?;

    let mut remove_clicks = remove_event_target_button.on_click();

    spawn_local(async move {
        remove_clicks.next().await;

        button.disconnect();
    });

    Ok(())
}
