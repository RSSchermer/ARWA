use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::event::{
    OnAfterPrint, OnBeforePrint, OnBeforeUnload, OnHashChange, OnMessage, OnMessageError,
    OnOffline, OnOnline, OnPageHide, OnPageShow, OnPopState, OnRejectionHandled, OnStorage,
    OnUnhandledRejection, OnUnload,
};
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlBodyElement {
    inner: web_sys::HtmlBodyElement,
}

impl HtmlBodyElement {
    pub fn on_after_print(&self) -> OnAfterPrint {
        OnAfterPrint::new(self.inner.clone().into())
    }

    pub fn on_before_print(&self) -> OnBeforePrint {
        OnBeforePrint::new(self.inner.clone().into())
    }

    pub fn on_before_unload(&self) -> OnBeforeUnload {
        OnBeforeUnload::new(self.inner.clone().into())
    }

    pub fn on_unload(&self) -> OnUnload {
        OnUnload::new(self.inner.clone().into())
    }

    pub fn on_hash_change(&self) -> OnHashChange {
        OnHashChange::new(self.inner.clone().into())
    }

    pub fn on_message(&self) -> OnMessage {
        OnMessage::new(self.inner.clone().into())
    }

    pub fn on_message_error(&self) -> OnMessageError {
        OnMessageError::new(self.inner.clone().into())
    }

    pub fn on_offline(&self) -> OnOffline {
        OnOffline::new(self.inner.clone().into())
    }

    pub fn on_online(&self) -> OnOnline {
        OnOnline::new(self.inner.clone().into())
    }

    pub fn on_page_hide(&self) -> OnPageHide {
        OnPageHide::new(self.inner.clone().into())
    }

    pub fn on_page_show(&self) -> OnPageShow {
        OnPageShow::new(self.inner.clone().into())
    }

    pub fn on_pop_state(&self) -> OnPopState {
        OnPopState::new(self.inner.clone().into())
    }

    pub fn on_rejection_handled(&self) -> OnRejectionHandled {
        OnRejectionHandled::new(self.inner.clone().into())
    }

    pub fn on_unhandled_rejection(&self) -> OnUnhandledRejection {
        OnUnhandledRejection::new(self.inner.clone().into())
    }

    pub fn on_storage(&self) -> OnStorage {
        OnStorage::new(self.inner.clone().into())
    }
}

impl_html_common_traits!(HtmlBodyElement);
