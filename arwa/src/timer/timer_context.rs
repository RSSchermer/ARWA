use crate::timer::{Duration, Interval, Timeout};

pub(crate) mod timer_context_seal {
    pub trait Seal {}
}

pub trait TimerContext: timer_context_seal::Seal {
    fn interval(&self, duration: Duration) -> Interval;

    fn timeout(&self, duration: Duration) -> Timeout;
}
