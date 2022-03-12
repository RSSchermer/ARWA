#![feature(async_closure)]

use arwa::dom::{selector, ParentNode, DynamicElement};
use arwa::window::window;
use arwa::{console, spawn_local};
use futures::StreamExt;
use wasm_bindgen::prelude::*;
use arwa::event::{EventTarget, TypedCustomEvent, EventOptions};
use arwa::ui::UiEventTarget;

struct MyEvent {
    message: String
}

impl Drop for MyEvent {
    fn drop(&mut self) {
        console::log!("Dropping event data...")
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let window = window().unwrap();
    let document = window.document();

    let outer = document
        .query_selector_first(&selector!("#outer"))
        .expect("No element with id `outer`.");

    let inner = document
        .query_selector_first(&selector!("#inner"))
        .expect("No element with id `inner`.");

    spawn_local(
        inner
            .on_typed_event::<TypedCustomEvent<MyEvent, DynamicElement>>()
            .for_each(async move |event| {
                console::log!("Message from inner: %s", event.message);
            }),
    );

    spawn_local(
        outer
            .on_typed_event::<TypedCustomEvent<MyEvent, DynamicElement>>()
            .for_each(async move |event| {
                console::log!("Message from outer: %s", event.message);
            }),
    );

    let dispatch_button = document
        .query_selector_first(&selector!("#dispatch_button"))
        .expect("No element with id `dispatch_button`.");

    spawn_local(dispatch_button.on_click().for_each(move |_| {
        inner.dispatch_typed_event(MyEvent {
            message: "Hello!".to_string()
        }, EventOptions {
            bubbles: true,
            ..Default::default()
        });

        futures::future::ready(())
    }));
}
