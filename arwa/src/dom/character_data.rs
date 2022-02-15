mod character_data_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_character_data(&self) -> &web_sys::CharacterData;
    }
}

pub trait CharacterData: character_data_seal::Seal {
    fn len(&self) -> u32 {
        self.as_web_sys_character_data().length()
    }

    fn data(&self) -> String {
        self.as_web_sys_character_data()
            .node_value()
            .unwrap_or(String::new())
    }

    fn set_data(&self, value: &str) {
        self.as_web_sys_character_data().set_node_value(Some(value));
    }
}

macro_rules! impl_character_data_traits {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl $crate::dom::character_data_seal::Seal for $tpe {
            fn as_web_sys_character_data(&self) -> &web_sys::CharacterData {
                self.inner.as_ref()
            }
        }

        impl $crate::dom::CharacterData for $tpe {}

        impl AsRef<web_sys::CharacterData> for $tpe {
            fn as_ref(&self) -> &web_sys::CharacterData {
                self.as_web_sys_character_data()
            }
        }

        $crate::dom::impl_node_traits!($tpe);
        $crate::dom::impl_try_from_node!($tpe, $web_sys_tpe);
    };
}

pub(crate) use impl_character_data_traits;
