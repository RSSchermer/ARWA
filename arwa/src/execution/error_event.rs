#[derive(Clone)]
pub struct ErrorEvent {
    inner: web_sys::ErrorEvent,
}

impl ErrorEvent {
    delegate! {
        to self.inner {
            pub fn message(&self) -> String;

            pub fn filename(&self) -> String;

            pub fn lineno(&self) -> u32;

            pub fn colno(&self) -> u32;
        }
    }
}

impl_common_event_traits!(ErrorEvent);
