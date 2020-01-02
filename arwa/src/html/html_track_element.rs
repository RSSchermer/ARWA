use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{
    Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node, TextTrack,
    TextTrackReadyState,
};

#[derive(Clone)]
pub struct HtmlTrackElement {
    inner: web_sys::HtmlTrackElement,
}

impl HtmlTrackElement {
    delegate! {
        target self.inner {
            pub fn src(&self) -> String;

            pub fn set_src(&self, src: &str);

            pub fn srclang(&self) -> String;

            pub fn set_srclang(&self, srclang: &str);

            pub fn label(&self) -> String;

            pub fn set_label(&self, label: &str);

            pub fn default(&self) -> bool;

            pub fn set_default(&self, default: bool);
        }
    }

    pub fn ready_state(&self) -> TextTrackReadyState {
        match self.inner.ready_state() {
            0 => TextTrackReadyState::None,
            1 => TextTrackReadyState::Loading,
            2 => TextTrackReadyState::Loaded,
            3 => TextTrackReadyState::Error,
            _ => unreachable!(),
        }
    }

    pub fn track(&self) -> Option<TextTrack> {
        self.inner.track().map(|t| t.into())
    }
}

impl_html_common_traits!(HtmlTrackElement);
