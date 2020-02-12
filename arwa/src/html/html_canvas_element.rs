use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::error::SecurityError;
use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{
    Element, GenericElement, GenericNode, GlobalEventHandlers, ImageQuality, InvalidCast, Node,
};

#[derive(Clone)]
pub struct HtmlCanvasElement {
    inner: web_sys::HtmlCanvasElement,
}

impl HtmlCanvasElement {
    delegate! {
        target self.inner {
            pub fn width(&self) -> u32;

            pub fn set_width(&self, width: u32);

            pub fn height(&self) -> u32;

            pub fn set_height(&self, height: u32);
        }
    }

    // TODO: it's unclear from the WHATWG spec if there can also be an encoding error, or if that
    // would only result in an empty string, needs to be experimented with.
    pub fn to_data_url(
        &self,
        mime_type: &str,
        quality: ImageQuality,
    ) -> Result<String, SecurityError> {
        let quality: f64 = quality.into();

        self.inner
            .to_data_url_with_type_and_encoder_options(mime_type, &quality.into())
            .map_err(|err| {
                let err: web_sys::DomException = err.unchecked_into();

                SecurityError::new(err)
            })
    }

    // TODO: figure our what to do about obtaining contexts. Leave that to external crates?

    // TODO: to_blob. The javascript version takes a callback that receives the blob, rather than
    // returning a blob. There is no indication that the blob can't outlive this callback though,
    // this needs to be experimented with.
}

impl_html_common_traits!(HtmlCanvasElement);
