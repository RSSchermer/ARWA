use arwa::dom::{selector, Element, ParentNode};
use arwa::window::window;
use arwa::{console, spawn_local};
use futures::{future, StreamExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = window();
    let document = window.document();
    let geolocation = window
        .navigator()
        .geolocation()
        .ok_or(JsError::new("Geolocation unavailable"))?;

    let position_container = document
        .query_selector_first(&selector!("#position_container"))
        .ok_or(JsError::new("No element with id `position_container`"))?;

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

    Ok(())
}
