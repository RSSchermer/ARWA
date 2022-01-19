use crate::geolocation::{CurrentPosition, WatchPosition};
use crate::timer::Duration;

pub struct Geolocation {
    inner: web_sys::Geolocation,
}

impl Geolocation {
    pub fn current_position(&self, options: PositionOptions) -> CurrentPosition {
        CurrentPosition::new(self.inner.clone(), options.into())
    }

    pub fn watch_position(&self, options: PositionOptions) -> WatchPosition {
        WatchPosition::new(self.inner.clone(), options.into())
    }
}

impl From<web_sys::Geolocation> for Geolocation {
    fn from(inner: web_sys::Geolocation) -> Self {
        Geolocation { inner }
    }
}

impl AsRef<web_sys::Geolocation> for Geolocation {
    fn as_ref(&self) -> &web_sys::Geolocation {
        &self.inner
    }
}

impl_common_wrapper_traits!(Geolocation);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PositionOptions {
    pub enable_high_accuracy: bool,
    pub maximum_age: Duration,
    pub timeout: Duration,
}

impl From<PositionOptions> for web_sys::PositionOptions {
    fn from(options: PositionOptions) -> web_sys::PositionOptions {
        let PositionOptions {
            enable_high_accuracy,
            maximum_age,
            timeout,
        } = options;

        let mut result = web_sys::PositionOptions::new();

        result.enable_high_accuracy(enable_high_accuracy);

        match maximum_age {
            Duration::Milliseconds(milliseconds) => result.maximum_age(milliseconds),
            Duration::Infinity => unimplemented!("web-sys does not yet support Infinity"),
        }

        match timeout {
            Duration::Milliseconds(milliseconds) => result.timeout(milliseconds),
            Duration::Infinity => unimplemented!("web-sys does not yet support Infinity"),
        }

        result
    }
}

#[derive(Clone)]
pub struct Position {
    inner: web_sys::Position,
}

impl Position {
    pub fn coordinates(&self) -> Coordinates {
        self.inner.coords().into()
    }

    pub fn time_stamp(&self) -> u64 {
        self.inner.timestamp() as u64
    }
}

impl From<web_sys::Position> for Position {
    fn from(inner: web_sys::Position) -> Self {
        Position { inner }
    }
}

impl AsRef<web_sys::Position> for Position {
    fn as_ref(&self) -> &web_sys::Position {
        &self.inner
    }
}

impl_common_wrapper_traits!(Position);

#[derive(Clone)]
pub struct Coordinates {
    inner: web_sys::Coordinates,
}

impl Coordinates {
    delegate! {
        target self.inner {
            pub fn latitude(&self) -> f64;

            pub fn longitude(&self) -> f64;

            pub fn altitude(&self) -> Option<f64>;

            pub fn accuracy(&self) -> f64;

            pub fn altitude_accuracy(&self) -> Option<f64>;

            pub fn heading(&self) -> Option<f64>;

            pub fn speed(&self) -> Option<f64>;
        }
    }
}

impl From<web_sys::Coordinates> for Coordinates {
    fn from(inner: web_sys::Coordinates) -> Self {
        Coordinates { inner }
    }
}

impl AsRef<web_sys::Coordinates> for Coordinates {
    fn as_ref(&self) -> &web_sys::Coordinates {
        &self.inner
    }
}

impl_common_wrapper_traits!(Coordinates);
