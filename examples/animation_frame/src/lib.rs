#![feature(async_closure, fn_traits, unboxed_closures)]
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

use arwa::dom::{selector, DynamicElement, Element, ParentNode};
use arwa::html::HtmlButtonElement;
use arwa::spawn_local;
use arwa::ui::UiEventTarget;
use arwa::window::{window, Window};
use futures::future::{AbortHandle, Abortable, Aborted};
use futures::{FutureExt, StreamExt};
use wasm_bindgen::prelude::*;

// We want to loop a call to the same function on each animation frame, while tracking some state.
// Self referential functions with state can be a bit hairy in Rust. We'll use the unstable nightly
// `fn_traits` and `unboxed_closures` features to implement a custom `FnOnce` function type that
// handles this cleanly. Here the state is just a frame count; in a real use case this could be
// your game loop for example, where instead of a frame count you would track your game state.
struct FrameLoop {
    count: usize,
    display_container: DynamicElement,
    frame_provider: Window,
    abort_handle: Rc<RefCell<AbortHandle>>,
}

impl FnOnce<(Result<f64, Aborted>,)> for FrameLoop {
    type Output = ();

    extern "rust-call" fn call_once(mut self, args: (Result<f64, Aborted>,)) -> Self::Output {
        // Only loop if the frame has not been cancelled.
        if let Ok(time) = args.0 {
            // Update the count and our displayed text.
            self.count += 1;

            self.display_container.deserialize_inner(&format!(
                "Frames counted: {}; Elapsed time (ms): {}",
                self.count, time
            ));

            let (abort_handle, abort_registration) = AbortHandle::new_pair();

            // Create a request the next frame.
            let next = self.frame_provider.request_animation_frame();
            let next_abortable = Abortable::new(next, abort_registration);

            // Update our cancellation handle to be the handle for the new frame request.
            self.abort_handle.replace(abort_handle);

            // Dispatch the request with another call to this function chained onto it.
            spawn_local(next_abortable.map(self));
        } else {
            self.display_container
                .deserialize_inner("Loop cancelled! Refresh the page to start it back up.");
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let window = window().unwrap();
    let document = window.document();

    // Obtain a reference to our "display container" element.
    let display_container = document
        .query_selector_first(&selector!("#display_container"))
        .expect("No element with id `display_container`.");

    // Set up an abort handle with which we may cancel the loop.
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let abort_handle = Rc::new(RefCell::new(abort_handle));

    // Initialize our frame loop.
    let frame_loop = FrameLoop {
        count: 0,
        display_container,
        frame_provider: window.clone(),
        abort_handle: abort_handle.clone(),
    };

    // Create our initial animation frame request.
    let request = window.request_animation_frame();
    let abortable_request = Abortable::new(request, abort_registration);

    // Dispatch our request with the initial call to our frame loop chained onto it.
    spawn_local(abortable_request.map(frame_loop));

    // Obtain a reference to the cancellation button.
    let button: HtmlButtonElement = document
        .query_selector_first(&selector!("#cancel_button"))
        .expect("No element with id `cancel_button`.")
        .try_into()
        .expect("Element is not a button element.");

    // Respond to click events on the button by cancelling the loop.
    spawn_local(button.on_click().take(1).for_each(move |_| {
        abort_handle.borrow().abort();

        futures::future::ready(())
    }));
}
