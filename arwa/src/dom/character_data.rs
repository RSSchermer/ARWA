pub(crate) mod character_data_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_character_data(&self) -> &web_sys::CharacterData;
    }
}

/// Implemented for nodes that contain characters.
pub trait CharacterData: character_data_seal::Seal {
    /// The size of the character data string associated with the node.
    fn len(&self) -> u32 {
        self.as_web_sys_character_data().length()
    }

    /// Returns the characters associated with the node as a string.
    fn data(&self) -> String {
        self.as_web_sys_character_data()
            .node_value()
            .unwrap_or(String::new())
    }

    /// Sets the characters associated with the node from the given string.
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
                use $crate::dom::character_data_seal::Seal;

                self.as_web_sys_character_data()
            }
        }

        impl $crate::dom::range_bound_container_seal::Seal for $tpe {
            fn as_web_sys_node(&self) -> &web_sys::Node {
                use $crate::dom::character_data_seal::Seal;

                self.as_web_sys_character_data().as_ref()
            }
        }

        impl $crate::dom::RangeBoundContainer for $tpe {}

        $crate::dom::impl_node_traits!($tpe);
        $crate::dom::impl_child_node!($tpe);
        $crate::dom::impl_owned_node!($tpe);
        $crate::dom::impl_element_sibling_for_character_data!($tpe);
        $crate::dom::impl_try_from_node!($tpe, $web_sys_tpe);
        $crate::dom::impl_try_from_child_node!($tpe, $web_sys_tpe);
    };
    ($tpe:ident) => {
        $crate::dom::impl_character_data_traits!($tpe, $tpe);
    };
}

pub(crate) use impl_character_data_traits;
