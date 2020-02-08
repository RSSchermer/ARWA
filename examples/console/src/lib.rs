use arwa::{console, document};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console::log!(1);
    console::log!("Formatted {}", 1);
    console::info!(1);
    console::info!("Formatted {}", 1);
    console::debug!(1);
    console::debug!("Formatted {}", 1);
    console::warn!(1);
    console::warn!("Formatted {}", 1);
    console::error!(1);
    console::error!("Formatted {}", 1);

    console::log!(document().unwrap());
}
