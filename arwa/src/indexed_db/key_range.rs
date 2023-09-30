use std::convert::TryFrom;
use std::ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

use wasm_bindgen::{JsCast, JsValue};

use crate::{dom_exception_wrapper, impl_common_wrapper_traits};

dom_exception_wrapper!(CreateKeyRangeError);

pub struct KeyRange {
    inner: web_sys::IdbKeyRange,
}

impl_common_wrapper_traits!(KeyRange);

impl From<web_sys::IdbKeyRange> for KeyRange {
    fn from(inner: web_sys::IdbKeyRange) -> Self {
        KeyRange { inner }
    }
}

impl<T> TryFrom<Range<T>> for KeyRange
where
    T: AsRef<JsValue>,
{
    type Error = CreateKeyRangeError;

    fn try_from(value: Range<T>) -> Result<Self, Self::Error> {
        web_sys::IdbKeyRange::bound_with_lower_open_and_upper_open(
            value.start.as_ref(),
            value.end.as_ref(),
            false,
            true,
        )
        .map(|inner| KeyRange { inner })
        .map_err(|err| CreateKeyRangeError::new(err.unchecked_into()))
    }
}

impl<T> TryFrom<RangeInclusive<T>> for KeyRange
where
    T: AsRef<JsValue>,
{
    type Error = CreateKeyRangeError;

    fn try_from(value: RangeInclusive<T>) -> Result<Self, Self::Error> {
        web_sys::IdbKeyRange::bound(value.start().as_ref(), value.end().as_ref())
            .map(|inner| KeyRange { inner })
            .map_err(|err| CreateKeyRangeError::new(err.unchecked_into()))
    }
}

impl<T> TryFrom<RangeFrom<T>> for KeyRange
where
    T: AsRef<JsValue>,
{
    type Error = CreateKeyRangeError;

    fn try_from(value: RangeFrom<T>) -> Result<Self, Self::Error> {
        web_sys::IdbKeyRange::lower_bound(value.start.as_ref())
            .map(|inner| KeyRange { inner })
            .map_err(|err| CreateKeyRangeError::new(err.unchecked_into()))
    }
}

impl<T> TryFrom<RangeTo<T>> for KeyRange
where
    T: AsRef<JsValue>,
{
    type Error = CreateKeyRangeError;

    fn try_from(value: RangeTo<T>) -> Result<Self, Self::Error> {
        web_sys::IdbKeyRange::upper_bound_with_open(value.end.as_ref(), true)
            .map(|inner| KeyRange { inner })
            .map_err(|err| CreateKeyRangeError::new(err.unchecked_into()))
    }
}

impl<T> TryFrom<RangeToInclusive<T>> for KeyRange
where
    T: AsRef<JsValue>,
{
    type Error = CreateKeyRangeError;

    fn try_from(value: RangeToInclusive<T>) -> Result<Self, Self::Error> {
        web_sys::IdbKeyRange::upper_bound(value.end.as_ref())
            .map(|inner| KeyRange { inner })
            .map_err(|err| CreateKeyRangeError::new(err.unchecked_into()))
    }
}
