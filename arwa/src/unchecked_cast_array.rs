macro_rules! unchecked_cast_array {
    ($tpe:ident, $inner_tpe:ident, $collection:ident) => {
        #[derive(Clone)]
        pub struct $collection {
            inner: js_sys::Array,
        }

        impl $collection {
            pub(crate) fn new(inner: js_sys::Array) -> Self {
                $collection { inner }
            }
        }

        impl $crate::collection::Collection for $collection {
            fn len(&self) -> u32 {
                self.inner.length()
            }
        }

        impl $crate::collection::Sequence for $collection {
            type Item = $tpe;

            fn get(&self, index: u32) -> Option<Self::Item> {
                use wasm_bindgen::JsCast;

                let value = self.inner.get(index);

                if value.is_undefined() {
                    None
                } else {
                    Some($tpe::from(value.unchecked_into::<$inner_tpe>()))
                }
            }

            fn to_host_array(&self) -> js_sys::Array {
                js_sys::Array::from(self.inner.as_ref())
            }
        }
    };
}

pub(crate) use unchecked_cast_array;
