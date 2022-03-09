use arwa::dom::{selector, Element, ParentNode};
use arwa::window::window;
use arwa::{console, spawn_local};
use futures::{future, StreamExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    let window = window().unwrap();
    let document = window.document();
    let navigator = window.navigator();
    let geolocation = navigator.geolocation().unwrap();

    let position_container = document
        .query_selector_first(&selector!("#position_container"))
        .expect("No element with id `position_container`");

    spawn_local(
        geolocation
            .watch_position(Default::default())
            .for_each(move |result| {
                match result {
                    Ok(position) => {
                        let coordinates = position.coordinates();

                        position_container.deserialize_inner(&format!(
                            "Lat: {}, Long: {}",
                            coordinates.latitude(),
                            coordinates.longitude()
                        ));
                    }
                    Err(err) => console::error!(err),
                };

                future::ready(())
            }),
    );
}
