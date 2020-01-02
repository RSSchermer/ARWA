#![feature(async_closure)]
use std::convert::TryInto;

use arwa::html::{GenericHtmlElement, HtmlElement};
use arwa::{document, navigator, Document, PositionOptions};
use futures::{future, StreamExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn start() {
    let document = document().unwrap();
    let navigator = navigator().unwrap();
    let geolocation = navigator.geolocation().unwrap();

    let position_container: GenericHtmlElement = document
        .query_id("position_container")
        .expect("No element with id `position_container`")
        .try_into()
        .expect("Element is not an html element");

    spawn_local(
        geolocation
            .watch_position(PositionOptions::new())
            .for_each(move |result| {
                match result {
                    Ok(position) => {
                        let coordinates = position.coordinates();

                        position_container.set_inner_text(&format!(
                            "Lat: {}, Long: {}",
                            coordinates.latitude(),
                            coordinates.longitude()
                        ));
                    }
                    Err(err) => web_sys::console::log_1(&format!("Error: {:?}", err).into()),
                };

                future::ready(())
            }),
    );
}
