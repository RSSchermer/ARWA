use delegate::delegate;
use wasm_bindgen::UnwrapThrowExt;

use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element};
use crate::media_type::MediaType;
use crate::security::CORS;
use crate::url::Url;

#[derive(Clone)]
pub struct HtmlScriptElement {
    inner: web_sys::HtmlScriptElement,
}

impl HtmlScriptElement {
    delegate! {
        target self.inner {
            pub fn no_module(&self) -> bool;

            pub fn set_no_module(&self, no_module: bool);

            pub fn defer(&self) -> bool;

            pub fn set_defer(&self, defer: bool);

            pub fn integrity(&self) -> String;

            pub fn set_integrity(&self, integrity: &str);
        }
    }

    pub fn src(&self) -> Option<Url> {
        Url::parse(self.inner.src().as_ref()).ok()
    }

    pub fn set_src(&self, src: &Url) {
        self.inner.set_src(src.as_ref());
    }

    pub fn asynchronous(&self) -> bool {
        self.inner.r#async()
    }

    pub fn set_asynchronous(&self, asynchronous: bool) {
        self.inner.set_async(asynchronous);
    }

    pub fn media_type(&self) -> Option<MediaType> {
        MediaType::parse(self.inner.type_().as_ref()).ok()
    }

    pub fn set_media_type(&self, media_type: Option<&MediaType>) {
        self.inner
            .set_type(media_type.map(|m| m.as_ref()).unwrap_or(""));
    }

    pub fn text(&self) -> String {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.text().unwrap_throw()
    }

    pub fn set_text(&self, text: &str) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.set_text(text).unwrap_throw();
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

impl From<web_sys::HtmlScriptElement> for HtmlScriptElement {
    fn from(inner: web_sys::HtmlScriptElement) -> Self {
        HtmlScriptElement { inner }
    }
}

impl AsRef<web_sys::HtmlScriptElement> for HtmlScriptElement {
    fn as_ref(&self) -> &web_sys::HtmlScriptElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlScriptElement);
impl_try_from_element!(HtmlScriptElement);
impl_known_element!(HtmlScriptElement, "SCRIPT");
