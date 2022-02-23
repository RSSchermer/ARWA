use crate::scroll::ScrollBehavior;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct ScrollIntoViewOptions {
    pub behavior: ScrollBehavior,
    pub block: ScrollAlignment,
    pub inline: ScrollAlignment,
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

pub(crate) mod scroll_into_view_seal {
    pub trait Seal {}
}

pub trait ScrollIntoView: scroll_into_view_seal::Seal {
    fn scroll_into_view(&self, options: ScrollIntoViewOptions);
}

macro_rules! impl_scroll_into_view_for_element {
    ($element:ident) => {
        impl $crate::scroll::scroll_into_view_seal::Seal for $element {}

        impl $crate::scroll::ScrollIntoView for $element {
            fn scroll_into_view(&self, options: $crate::scroll::ScrollIntoViewOptions) {
                use crate::dom::element_seal::Seal;

                let mut opts = web_sys::ScrollIntoViewOptions::new();

                opts.behavior(options.behavior.into());
                opts.block(options.block.into());
                opts.inline(options.inline.into());

                self.as_web_sys_element()
                    .scroll_into_view_with_scroll_into_view_options(&opts);
            }
        }
    };
}

pub(crate) use impl_scroll_into_view_for_element;
