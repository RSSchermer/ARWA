use crate::cssom::{inline_style_context_seal, CssStyleDeclaration, InlineStyleContext};
use crate::dom::{DynamicElement, TextDirectionality};
use crate::html::html_element_seal::Seal;
use crate::html::HtmlDocument;
use crate::lang::LanguageTag;
use crate::InvalidCast;
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ContentEditable {
    True,
    False,
    Inherit,
}

pub(crate) mod html_element_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_html_element(&self) -> &web_sys::HtmlElement;
    }
}

pub trait HtmlElement: html_element_seal::Seal {
    fn blur(&self) {
        self.as_web_sys_html_element().blur().unwrap();
    }

    fn click(&self) {
        self.as_web_sys_html_element().click();
    }

    fn focus(&self) {
        self.as_web_sys_html_element().focus().unwrap();
    }

    fn title(&self) -> String {
        self.as_web_sys_html_element().title()
    }

    fn set_title(&self, title: &str) {
        self.as_web_sys_html_element().set_title(title);
    }

    fn lang(&self) -> Option<LanguageTag> {
        LanguageTag::parse(self.as_web_sys_html_element().lang()).ok()
    }

    fn set_lang(&self, lang: Option<&LanguageTag>) {
        self.as_web_sys_html_element()
            .set_lang(lang.map(|l| l.as_ref()).unwrap_or(""));
    }

    fn dir(&self) -> TextDirectionality {
        match &*self.as_web_sys_html_element().dir().to_lowercase() {
            "ltr" => TextDirectionality::LeftToRight,
            "rtl" => TextDirectionality::RightToLeft,
            _ => TextDirectionality::Auto,
        }
    }

    fn set_dir(&self, dir: TextDirectionality) {
        let text_directionality = match dir {
            TextDirectionality::Auto => "auto",
            TextDirectionality::LeftToRight => "ltr",
            TextDirectionality::RightToLeft => "rtl",
        };

        self.as_web_sys_html_element().set_dir(text_directionality);
    }

    // TODO: decide what to do with `innerText`.

    fn hidden(&self) -> bool {
        self.as_web_sys_html_element().hidden()
    }

    fn set_hidden(&self, hidden: bool) {
        self.as_web_sys_html_element().set_hidden(hidden);
    }

    fn tab_index(&self) -> i32 {
        self.as_web_sys_html_element().tab_index()
    }

    fn set_tab_index(&self, tab_index: i32) {
        self.as_web_sys_html_element().set_tab_index(tab_index);
    }

    fn draggable(&self) -> bool {
        self.as_web_sys_html_element().draggable()
    }

    fn set_draggable(&self, draggable: bool) {
        self.as_web_sys_html_element().set_draggable(draggable);
    }

    fn content_editable(&self) -> ContentEditable {
        match &*self
            .as_web_sys_html_element()
            .content_editable()
            .to_lowercase()
        {
            "inherit" => ContentEditable::Inherit,
            "false" => ContentEditable::False,
            _ => ContentEditable::True,
        }
    }

    fn set_content_editable(&self, content_editable: ContentEditable) {
        let content_editable = match content_editable {
            ContentEditable::True => "true",
            ContentEditable::False => "false",
            ContentEditable::Inherit => "inherit",
        };

        self.as_web_sys_html_element()
            .set_content_editable(content_editable);
    }

    fn is_content_editable(&self) -> bool {
        self.as_web_sys_html_element().is_content_editable()
    }

    fn spellcheck(&self) -> bool {
        self.as_web_sys_html_element().spellcheck()
    }

    fn set_spellcheck(&self, spellcheck: bool) {
        self.as_web_sys_html_element().set_spellcheck(spellcheck);
    }

    fn offset_parent(&self) -> Option<DynamicElement> {
        self.as_ref().offset_parent().map(|e| e.into())
    }

    fn offset_top(&self) -> i32 {
        self.as_web_sys_html_element().offset_top()
    }

    fn offset_left(&self) -> i32 {
        self.as_web_sys_html_element().offset_left()
    }

    fn offset_width(&self) -> i32 {
        self.as_web_sys_html_element().offset_width()
    }

    fn offset_height(&self) -> i32 {
        self.as_web_sys_html_element().offset_height()
    }
}

#[derive(Clone, PartialEq)]
pub struct DynamicHtmlElement {
    inner: web_sys::HtmlElement,
}

impl html_element_seal::Seal for DynamicHtmlElement {
    fn as_web_sys_html_element(&self) -> &web_sys::HtmlElement {
        &self.inner
    }
}

impl HtmlElement for DynamicHtmlElement {}

impl inline_style_context_seal::Seal for DynamicHtmlElement {}

impl InlineStyleContext for DynamicHtmlElement {
    fn style(&self) -> CssStyleDeclaration {
        self.as_web_sys_html_element().style().into()
    }
}

impl From<web_sys::HtmlElement> for DynamicHtmlElement {
    fn from(inner: web_sys::HtmlElement) -> Self {
        DynamicHtmlElement { inner }
    }
}

impl From<DynamicHtmlElement> for web_sys::HtmlElement {
    fn from(value: DynamicHtmlElement) -> Self {
        value.inner
    }
}

impl AsRef<web_sys::HtmlElement> for DynamicHtmlElement {
    fn as_ref(&self) -> &web_sys::HtmlElement {
        &self.inner
    }
}

impl_element_traits!(DynamicHtmlElement, web_sys::HtmlElement);

macro_rules! impl_html_element_traits {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl $crate::html::html_element_seal::Seal for $tpe {
            fn as_web_sys_html_element(&self) -> &web_sys::HtmlElement {
                &self.inner
            }
        }

        impl $crate::html::HtmlElement for $tpe {}

        impl $crate::cssom::inline_style_context_seal::Seal for $tpe {}

        impl $crate::cssom::InlineStyleContext for $tpe {
            fn style(&self) -> CssStyleDeclaration {
                self.as_web_sys_html_element().style().into()
            }
        }

        impl $crate::html::slot_change_event_target_seal::Seal for $tpe {
            fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
                self.as_web_sys_html_element().as_ref()
            }
        }

        impl $crate::html::SlotChangeEventTarget for $tpe {}

        impl AsRef<web_sys::HtmlElement> for $tpe {
            fn as_ref(&self) -> &web_sys::HtmlElement {
                self.as_web_sys_html_element()
            }
        }

        impl_element_traits!($tpe, $web_sys_tpe);
    };
    ($tpe:ident) => {
        $crate::html::impl_html_element_traits!($tpe, $tpe);
    };
}

pub(crate) use impl_html_element_traits;
