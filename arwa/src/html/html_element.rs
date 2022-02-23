use wasm_bindgen::UnwrapThrowExt;

use crate::dom::{DynamicElement, TextDirectionality};
use crate::lang::LanguageTag;

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
        self.as_web_sys_html_element().blur().unwrap_throw();
    }

    fn click(&self) {
        self.as_web_sys_html_element().click();
    }

    fn focus(&self) {
        self.as_web_sys_html_element().focus().unwrap_throw();
    }

    fn title(&self) -> String {
        self.as_web_sys_html_element().title()
    }

    fn set_title(&self, title: &str) {
        self.as_web_sys_html_element().set_title(title);
    }

    fn lang(&self) -> Option<LanguageTag> {
        LanguageTag::parse(self.as_web_sys_html_element().lang().as_ref()).ok()
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
        self.as_web_sys_html_element()
            .offset_parent()
            .map(|e| e.into())
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

macro_rules! impl_html_element_traits {
    ($tpe:ident) => {
        impl $crate::html::html_element_seal::Seal for $tpe {
            fn as_web_sys_html_element(&self) -> &web_sys::HtmlElement {
                &self.inner
            }
        }

        impl $crate::html::HtmlElement for $tpe {}

        impl $crate::cssom::styled_inline_seal::Seal for $tpe {}

        impl $crate::cssom::StyledInline for $tpe {
            fn style(&self) -> $crate::cssom::CssStyleDeclaration {
                use crate::html::html_element_seal::Seal;

                self.as_web_sys_html_element().style().into()
            }
        }

        impl $crate::html::slot_change_event_target_seal::Seal for $tpe {
            fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
                use crate::html::html_element_seal::Seal;

                self.as_web_sys_html_element().as_ref()
            }
        }

        impl $crate::html::SlotChangeEventTarget for $tpe {}

        impl AsRef<web_sys::HtmlElement> for $tpe {
            fn as_ref(&self) -> &web_sys::HtmlElement {
                use crate::html::html_element_seal::Seal;

                self.as_web_sys_html_element()
            }
        }

        $crate::dom::impl_element_traits!($tpe);
    };
}

pub(crate) use impl_html_element_traits;
