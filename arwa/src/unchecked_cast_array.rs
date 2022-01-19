macro_rules! unchecked_cast_array {
    ($tpe:ident, $inner_tpe:ident, $collection:ident) => {
        #[derive(Clone)]
        pub struct $collection {
            inner: js_sys::Array,
        }

        impl $crate::collection::Collection for $collection {
            fn len(&self) -> u32 {
                self.inner.length()
            }
        }

        impl $crate::collection::Sequence for $collection {
            type Item = $tpe;

            fn get(&self, index: u32) -> Option<Self::Item> {
                let value = self.inner.get(index);

                if value.is_undefined() {
                    None
                } else {
                    Some($tpe::from(value.unchecked_into()))
                }
            }

            fn to_host_array(&self) -> js_sys::Array {
                js_sys::Array::from(self.inner.as_ref())
            }
        }
    };
}

pub(crate) use unchecked_cast_array;
