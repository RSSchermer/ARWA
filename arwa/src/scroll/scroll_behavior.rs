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

impl From<ScrollBehavior> for web_sys::ScrollBehavior {
    fn from(scroll_behaviour: ScrollBehavior) -> web_sys::ScrollBehavior {
        match scroll_behaviour {
            ScrollBehavior::Auto => web_sys::ScrollBehavior::Auto,
            ScrollBehavior::Smooth => web_sys::ScrollBehavior::Smooth,
        }
    }
}

macro_rules! impl_scrollable_for_element {
    ($element:ident) => {
        impl $crate::scroll::scrollable_seal::Seal for $element {}

        impl Scrollabe for $element {
            // Note: web_sys currently declares scroll_left and scroll_top as integers, but they
            // are specced as doubles. Convert them for now for BC; resolve this in web_sys at some
            // point.
            fn scroll_left(&self) -> f64 {
                self.as_ref().scroll_left() as f64
            }

            fn scroll_top(&self) -> f64 {
                self.as_ref().scroll_top() as f64
            }

            fn scroll_to(&self, options: $crate::scroll::ScrollToOptions) {
                let mut opts = web_sys::ScrollToOptions::new();

                opts.left(options.left.into());
                opts.top(options.top.into());
                opts.behavior(options.behavior.into());

                self.as_ref().scroll_to_with_scroll_to_options(&opts);
            }

            fn scroll_by(&self, options: $crate::scroll::ScrollByOptions) {
                let mut opts = web_sys::ScrollToOptions::new();

                opts.left(options.x.into());
                opts.top(options.y.into());
                opts.behavior(options.behavior.into());

                self.as_ref().scroll_by_with_scroll_to_options(&opts);
            }
        }
    };
}

pub(crate) use impl_scrollable_for_element;
