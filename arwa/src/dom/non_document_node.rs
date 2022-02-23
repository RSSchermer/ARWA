use wasm_bindgen::UnwrapThrowExt;

use crate::dom::DynamicDocument;

pub(crate) mod owned_node_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_node(&self) -> &web_sys::Node;
    }
}

pub trait OwnedNode: owned_node_seal::Seal {
    fn owner_document(&self) -> DynamicDocument {
        self.as_web_sys_node()
            .owner_document()
            .unwrap_throw()
            .into()
    }
}

macro_rules! impl_owned_node {
    ($tpe:ident) => {
        impl $crate::dom::owned_node_seal::Seal for $tpe {
            fn as_web_sys_node(&self) -> &web_sys::Node {
                self.as_ref()
            }
        }

        impl $crate::dom::OwnedNode for $tpe {}
    };
}

pub(crate) use impl_owned_node;
