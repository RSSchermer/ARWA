#![feature(async_closure)]

use arwa::dom::{selector, ChildNode, ParentNode};
use arwa::ui::UiEventTarget;
use arwa::window::window;
use arwa::{console, spawn_local};
use futures::{FutureExt, StreamExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = window().document();

    // Obtain a reference to the HtmlButtonElement we want to listen to.
    let button = document
        .query_selector_first(&selector!("#button"))
        .ok_or(JsError::new("No element with id `button`."))?;

    spawn_local(
        button
            .on_click()
            .for_each(async move |_| {
                console::log!("Click!");
            })
            .map(|_| console::log!("Event target cleaned up")),
    );

    let remove_event_target_button = document
        .query_selector_first(&selector!("#remove_event_target_button"))
        .ok_or(JsError::new(
            "No element with id `remove_event_target_button`.",
        ))?;

    spawn_local(
        remove_event_target_button
            .on_click()
            .take(1)
            .for_each(move |_| {
                button.disconnect();

                futures::future::ready(())
            }),
    );

    Ok(())
}
