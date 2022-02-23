use crate::dom::element::DynamicElement;

pub(crate) mod element_sibling_seal {
    pub trait Seal {}
}

pub trait ElementSibling: element_sibling_seal::Seal {
    fn previous_element_sibling(&self) -> Option<DynamicElement>;

    fn next_element_sibling(&self) -> Option<DynamicElement>;
}

macro_rules! impl_element_sibling_for_element {
    ($tpe:ident) => {
        impl $crate::dom::element_sibling_seal::Seal for $tpe {}

        impl $crate::dom::ElementSibling for $tpe {
            fn previous_element_sibling(&self) -> Option<$crate::dom::DynamicElement> {
                use crate::dom::element_seal::Seal;

                self.as_web_sys_element()
                    .previous_element_sibling()
                    .map(|e| e.into())
            }

            fn next_element_sibling(&self) -> Option<$crate::dom::DynamicElement> {
                use crate::dom::element_seal::Seal;

                self.as_web_sys_element()
                    .next_element_sibling()
                    .map(|e| e.into())
            }
        }
    };
}

pub(crate) use impl_element_sibling_for_element;

macro_rules! impl_element_sibling_for_character_data {
    ($tpe:ident) => {
        impl $crate::dom::element_sibling_seal::Seal for $tpe {}

        impl $crate::dom::ElementSibling for $tpe {
            fn previous_element_sibling(&self) -> Option<$crate::dom::DynamicElement> {
                use crate::dom::character_data_seal::Seal;

                self.as_web_sys_character_data()
                    .previous_element_sibling()
                    .map(|e| e.into())
            }

            fn next_element_sibling(&self) -> Option<$crate::dom::DynamicElement> {
                use crate::dom::character_data_seal::Seal;

                self.as_web_sys_character_data()
                    .next_element_sibling()
                    .map(|e| e.into())
            }
        }
    };
}

pub(crate) use impl_element_sibling_for_character_data;
