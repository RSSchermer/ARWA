use wasm_bindgen::UnwrapThrowExt;

pub(crate) mod text_data_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_text(&self) -> &web_sys::Text;
    }
}

pub trait TextData: text_data_seal::Seal {
    fn whole_text(&self) -> String {
        self.as_web_sys_text().whole_text().unwrap_or(String::new())
    }

    fn split_off(&self, at: u32) -> Text {
        let inner = self.as_web_sys_text().split_text(at).unwrap_throw();

        Text { inner }
    }
}

#[derive(Clone)]
pub struct Text {
    inner: web_sys::Text,
}

impl From<web_sys::Text> for Text {
    fn from(inner: web_sys::Text) -> Self {
        Text { inner }
    }
}

impl_text_data_traits!(Text);

macro_rules! impl_text_data_traits {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl $crate::dom::text_data_seal::Seal for $tpe {
            fn as_web_sys_text(&self) -> &web_sys::Text {
                &self.inner
            }
        }

        impl $crate::dom::TextData for $tpe {}

        impl AsRef<web_sys::Text> for $tpe {
            fn as_ref(&self) -> &web_sys::Text {
                use $crate::dom::text_data_seal::Seal;

                self.as_web_sys_text()
            }
        }

        $crate::dom::impl_character_data_traits!($tpe, $web_sys_tpe);
    };
    ($tpe:ident) => {
        $crate::dom::impl_text_data_traits!($tpe, $tpe);
    };
}

pub(crate) use impl_text_data_traits;
