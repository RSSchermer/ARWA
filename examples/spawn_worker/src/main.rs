#![feature(async_closure)]

use arwa::dom::{selector, ParentNode};
use arwa::message::MessageEventTarget;
use arwa::ui::UiEventTarget;
use arwa::window::window;
use arwa::worker::dedicated::DedicatedWorker;
use arwa::{console, spawn_local};
use futures::StreamExt;
use wasm_bindgen::JsValue;

fn main() {
    for i in 0..10u32 {
        DedicatedWorker::spawn(move |_| {
            console::log!("Hello from worker %i!", i);
        });
    }

    let worker = DedicatedWorker::spawn(|cx| {
        spawn_local(async move {
            while let Some(_) = cx.on_message().next().await {
                console::log!("Worker was notified!");
            }
        });
    });

    let button = window()
        .document()
        .query_selector(&selector!("#notify_worker"))
        .unwrap();

    spawn_local(async move {
        while let Some(_) = button.on_click().next().await {
            // ARWA does not currently implement "post message" functionality for workers, as there
            // are still some unresolved questions as to how it is best implemented. For now, cast
            // to a web_sys::Worker and call post_message on that.

            let as_web_sys: &web_sys::Worker = worker.as_ref();

            as_web_sys.post_message(&JsValue::null()).unwrap();
        }
    });
}
