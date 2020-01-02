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

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct ScrollIntoViewOptions {
    pub behavior: ScrollBehavior,
    pub block: ScrollAlignment,
    pub inline: ScrollAlignment,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScrollBehavior {
    Auto,
    Smooth,
}

impl Default for ScrollBehavior {
    fn default() -> Self {
        ScrollBehavior::Auto
    }
}

impl Into<web_sys::ScrollBehavior> for ScrollBehavior {
    fn into(self) -> web_sys::ScrollBehavior {
        match self {
            ScrollBehavior::Auto => web_sys::ScrollBehavior::Auto,
            ScrollBehavior::Smooth => web_sys::ScrollBehavior::Smooth,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScrollAlignment {
    Start,
    Center,
    End,
    Nearest,
}

impl Default for ScrollAlignment {
    fn default() -> Self {
        ScrollAlignment::Start
    }
}

impl Into<web_sys::ScrollLogicalPosition> for ScrollAlignment {
    fn into(self) -> web_sys::ScrollLogicalPosition {
        match self {
            ScrollAlignment::Start => web_sys::ScrollLogicalPosition::Start,
            ScrollAlignment::Center => web_sys::ScrollLogicalPosition::Center,
            ScrollAlignment::End => web_sys::ScrollLogicalPosition::End,
            ScrollAlignment::Nearest => web_sys::ScrollLogicalPosition::Nearest,
        }
    }
}
