#![feature(async_closure)]

mod my_element;

use std::convert::TryInto;

use arwa::dom::{selector, ChildNode, ParentNode};
use arwa::html::{custom_element_name, HtmlButtonElement, HtmlDocument};
use arwa::spawn_local;
use arwa::ui::UiEventTarget;
use arwa::window::window;
use futures::{FutureExt, StreamExt};
use wasm_bindgen::prelude::*;

use crate::my_element::{MyElement, MyElementExt, MY_ELEMENT};

#[wasm_bindgen(start)]
pub fn start() {
    let window = window().unwrap();

    window
        .custom_elements()
        .try_register(&custom_element_name!("my-element"), MY_ELEMENT)
        .unwrap();

    let document: HtmlDocument = window.document().try_into().unwrap();

    let my_element: MyElement = document
        .query_selector_first(&selector!("my-element"))
        .unwrap()
        .try_into()
        .unwrap();

    let reconnect_button: HtmlButtonElement = document
        .query_selector_first(&selector!("#reconnect_button"))
        .expect("No element with id `reconnect_button`.")
        .try_into()
        .expect("Not a button element.");

    let my_element_clone = my_element.clone();
    let body = document.body().unwrap();

    spawn_local(
        reconnect_button
            .on_click()
            .take(3)
            .for_each(move |_| {
                my_element_clone.disconnect();
                body.prepend_child(&my_element_clone);

                futures::future::ready(())
            })
            .map(move |_| {
                my_element.disconnect();
                reconnect_button.set_disabled(true);
            }),
    );

    let change_message_button: HtmlButtonElement = document
        .query_selector_first(&selector!("#change_message_button"))
        .expect("No element with id `change_message_button`.")
        .try_into()
        .expect("Not a button element.");

    spawn_local(change_message_button.on_click().take(1).for_each(move |_| {
        if let Some(element) = document.query_selector_first(&selector!("my-element")) {
            let element: MyElement = element.try_into().unwrap();

            element.set_message("Goodbye!");
        }

        change_message_button.set_disabled(true);

        futures::future::ready(())
    }));
}
