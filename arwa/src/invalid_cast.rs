use std::any::type_name;
use std::error::Error;
use std::{fmt, marker};

pub struct InvalidCast<F, T> {
    from: F,
    _to: marker::PhantomData<T>,
}

impl<F, T> InvalidCast<F, T> {
    pub(crate) fn new(from: F) -> Self {
        InvalidCast {
            from,
            _to: marker::PhantomData,
        }
    }

    pub fn into_inner(self) -> F {
        self.from
    }
}

impl<F, T> fmt::Display for InvalidCast<F, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to cast from {} to {}",
            type_name::<F>(),
            type_name::<T>()
        )
    }
}

impl<F, T> fmt::Debug for InvalidCast<F, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<F, T> Error for InvalidCast<F, T> {}
