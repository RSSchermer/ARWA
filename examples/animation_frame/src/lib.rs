#![feature(async_closure, fn_traits, unboxed_closures)]
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

use arwa::html::{GenericHtmlElement, HtmlButtonElement, HtmlElement};
use arwa::{
    document, window, AnimationFrameCancelled, AnimationFrameHandle, Document, GlobalEventHandlers,
    Window,
};
use futures::future;
use futures::{FutureExt, StreamExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

// We want to loop a call to the same function on each animation frame, while tracking some state.
// Self referential functions with state can be a bit hairy in Rust. We'll use the unstable nightly
// `fn_traits` and `unboxed_closures` features to implement a custom `FnOnce` function type that
// handles this cleanly. Here the state is just a frame count; in a real use case this could be
// your game loop for example, where instead of a frame count you would track your game state.
struct FrameLoop {
    count: usize,
    display_container: GenericHtmlElement,
    frame_provider: Window,
    cancellation_handle: Rc<RefCell<Option<AnimationFrameHandle>>>,
}

impl FnOnce<(Result<f64, AnimationFrameCancelled>,)> for FrameLoop {
    type Output = ();

    extern "rust-call" fn call_once(
        mut self,
        args: (Result<f64, AnimationFrameCancelled>,),
    ) -> Self::Output {
        // Only loop if the frame has not been cancelled.
        if let Ok(time) = args.0 {
            // Update the count and our displayed text.
            self.count += 1;

            self.display_container.set_inner_text(&format!(
                "Frames counted: {}; Elapsed time (ms): {}",
                self.count, time
            ));

            // Create a request the next frame.
            let next = self.frame_provider.request_animation_frame();

            // Update our cancellation handle to be the handle for the new frame request.
            self.cancellation_handle.replace(Some(next.handle()));

            // Dispatch the request with another call to this function chained onto it.
            spawn_local(next.map(self));
        } else {
            self.display_container
                .set_inner_text("Loop cancelled! Refresh the page to start it back up.");
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let window = window().unwrap();
    let document = document().unwrap();

    // Obtain a reference to our "display container" element.
    let display_container: GenericHtmlElement = document
        .query_id("display_container")
        .expect("No element with id `display_container`.")
        .try_into()
        .expect("Element is not an html element.");

    // Initialize our frame loop.
    let frame_loop = FrameLoop {
        count: 0,
        display_container,
        frame_provider: window.clone(),
        cancellation_handle: Rc::new(RefCell::new(None)),
    };

    // Acquire a reference to the current request's cancellation handle so that we may cancel the
    // loop if we wish (see below).
    let cancellation_handle = frame_loop.cancellation_handle.clone();

    // Create our initial animation frame request.
    let request = window.request_animation_frame();

    // Dispatch our request with the initial call to our frame loop chained onto it.
    spawn_local(request.map(frame_loop));

    // Obtain a reference to the cancellation button.
    let button: HtmlButtonElement = document
        .query_id("cancel_button")
        .expect("No element with id `cancel_button`.")
        .try_into()
        .expect("Element is not a button element.");

    // Respond to click events on the button by cancelling the loop.
    spawn_local(button.on_click().for_each(move |_| {
        if let Some(handle) = &*cancellation_handle.borrow() {
            handle.cancel();
        }

        future::ready(())
    }));
}
