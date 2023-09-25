#![feature(async_closure)]

mod my_element;

use std::ops::Deref;

use arwa::dom::{selector, ChildNode, ParentNode};
use arwa::html::{custom_element_name, HtmlButtonElement, HtmlDocument};
use arwa::spawn_local;
use arwa::ui::UiEventTarget;
use arwa::window::window;
use futures::StreamExt;
use wasm_bindgen::{JsError, JsValue};

use crate::my_element::{MyElement, MyElementExt};

fn main() -> Result<(), JsValue> {
    let window = window();
    let registry = window.custom_elements();

    my_element::register(&custom_element_name!("my-element"), &registry);

    let document: HtmlDocument = window.document().try_into()?;

    let my_element: MyElement = document
        .query_selector(&selector!("my-element"))
        .ok_or(JsError::new("No element of type `my-element`"))?
        .try_into()?;

    let reconnect_button: HtmlButtonElement = document
        .query_selector(&selector!("#reconnect_button"))
        .ok_or(JsError::new("No element with id `reconnect_button`."))?
        .try_into()?;

    let reconnect_clicks = reconnect_button.on_click();
    let my_element_clone = my_element.clone();
    let body = document
        .body()
        .ok_or(JsError::new("Document has no body element"))?;

    spawn_local(async move {
        // Let's take just the first 3 clicks and then disconnect the custom element, so we can
        // watch it get garbage collected (probably, eventually... there are essentially no
        // guarantees for if and when a browser will perform a garbage collection pass).
        let mut reconnect_clicks = reconnect_clicks.take(3);

        while let Some(_) = reconnect_clicks.next().await {
            my_element_clone.disconnect();
            body.prepend_child(my_element_clone.deref());
        }

        my_element.disconnect();
        reconnect_button.set_disabled(true);
    });

    let change_message_button: HtmlButtonElement = document
        .query_selector(&selector!("#change_message_button"))
        .ok_or(JsError::new("No element with id `change_message_button`."))?
        .try_into()?;

    let mut change_message_clicks = change_message_button.on_click();

    spawn_local(async move {
        // Wait for the first click, then try changing the message and then disable the button.
        change_message_clicks.next().await;

        if let Some(element) = document.query_selector(&selector!("my-element")) {
            let element: MyElement = element.try_into().unwrap();

            element.set_message("Goodbye!");
        }

        change_message_button.set_disabled(true);
    });

    Ok(())
}
