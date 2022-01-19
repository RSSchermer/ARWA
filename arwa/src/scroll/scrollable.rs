use crate::scroll::ScrollBehavior;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct ScrollToOptions {
    pub top: u32,
    pub left: u32,
    pub behavior: ScrollBehavior,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct ScrollByOptions {
    pub x: u32,
    pub y: u32,
    pub behavior: ScrollBehavior,
}

pub(crate) mod scrollable_seal {
    pub trait Seal {}
}

pub trait Scrollable: scrollable_seal::Seal {
    fn scroll_left(&self) -> f64;

    fn scroll_top(&self) -> f64;

    fn scroll_to(&self, options: ScrollToOptions);

    fn scroll_by(&self, options: ScrollByOptions);
}
