#![feature(async_closure)]

use arwa::dom::{selector, DynamicElement, ParentNode};
use arwa::event::{EventOptions, EventTarget, TypedCustomEvent};
use arwa::ui::UiEventTarget;
use arwa::window::window;
use arwa::{console, spawn_local};
use futures::StreamExt;
use wasm_bindgen::{JsError, JsValue};

struct MyEvent {
    message: String,
}

impl Drop for MyEvent {
    fn drop(&mut self) {
        console::log!("Dropping event data...")
    }
}

fn main() -> Result<(), JsValue> {
    let document = window().document();

    let outer = document
        .query_selector_first(&selector!("#outer"))
        .ok_or(JsError::new("No element with id `outer`."))?;

    let inner = document
        .query_selector_first(&selector!("#inner"))
        .ok_or(JsError::new("No element with id `inner`."))?;

    let mut inner_events = inner.on_typed_event::<TypedCustomEvent<MyEvent, DynamicElement>>();

    spawn_local(async move {
        while let Some(event) = inner_events.next().await {
            console::log!("Message from inner: %s", event.message);
        }
    });

    let mut outer_events = outer.on_typed_event::<TypedCustomEvent<MyEvent, DynamicElement>>();

    spawn_local(async move {
        while let Some(event) = outer_events.next().await {
            console::log!("Message from outer: %s", event.message);
        }
    });

    let dispatch_button = document
        .query_selector_first(&selector!("#dispatch_button"))
        .ok_or(JsError::new("No element with id `dispatch_button`."))?;

    let mut dispatch_clicks = dispatch_button.on_click();

    spawn_local(async move {
        while let Some(_) = dispatch_clicks.next().await {
            inner.dispatch_typed_event(
                MyEvent {
                    message: "Hello!".to_string(),
                },
                EventOptions {
                    bubbles: true,
                    ..Default::default()
                },
            );
        }
    });

    Ok(())
}
