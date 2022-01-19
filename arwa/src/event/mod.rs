mod event;
pub(crate) use self::event::impl_typed_event_traits;
pub use self::event::{DynamicEvent, Event, EventPhase, GenericEvent, TypedEvent};

mod event_target;
pub(crate) use self::event_target::impl_event_target_traits;
pub use self::event_target::{DynamicEventTarget, EventTarget};

mod on_event;
pub use self::on_event::{EventStreamOptions, OnEvent, OnEventWithOptions};
