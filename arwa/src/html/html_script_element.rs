use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node, CORS};

#[derive(Clone)]
pub struct HtmlScriptElement {
    inner: web_sys::HtmlScriptElement,
}

impl HtmlScriptElement {
    delegate! {
        target self.inner {
            pub fn src(&self) -> String;

            pub fn set_src(&self, src: &str);

            pub fn no_module(&self) -> bool;

            pub fn set_no_module(&self, no_module: bool);

            pub fn defer(&self) -> bool;

            pub fn set_defer(&self, defer: bool);

            pub fn integrity(&self) -> String;

            pub fn set_integrity(&self, integrity: &str);
        }
    }

    pub fn asynchronous(&self) -> bool {
        self.inner.r#async()
    }

    pub fn set_asynchronous(&self, asynchronous: bool) {
        self.inner.set_async(asynchronous);
    }

    pub fn mime_type(&self) -> String {
        self.inner.type_()
    }

    pub fn set_mime_type(&self, mime_type: &str) {
        self.inner.set_type(mime_type);
    }

    pub fn text(&self) -> String {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.text().unwrap()
    }

    pub fn set_text(&self, text: &str) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.set_text(text).unwrap();
    }

    pub fn cross_origin(&self) -> CORS {
        if let Some(cross_origin) = self.inner.cross_origin() {
            match &*cross_origin {
                "use-credentials" => CORS::UseCredentials,
                _ => CORS::Anonymous,
            }
        } else {
            CORS::Anonymous
        }
    }

    pub fn set_cross_origin(&self, cross_origin: CORS) {
        let cross_origin = match cross_origin {
            CORS::Anonymous => "anonymous",
            CORS::UseCredentials => "use-credentials",
        };

        self.inner.set_cross_origin(Some(cross_origin));
    }

    // TODO: referrer_policy absent in web_sys.
}

impl_html_common_traits!(HtmlScriptElement);
