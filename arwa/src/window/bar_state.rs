pub trait BarState {
    fn visible(&self) -> bool;

    fn set_visible(&self, visible: bool);
}

macro_rules! impl_bar_state {
    ($tpe:ident) => {
        impl $tpe {
            pub(crate) fn new(inner: web_sys::BarProp) -> Self {
                $tpe { inner }
            }
        }

        impl BarState for $tpe {
            fn visible(&self) -> bool {
                // No indication in the spec this can actually fail, unwrap for now.
                self.inner.visible().unwrap()
            }

            fn set_visible(&self, visible: bool) {
                // No indication in the spec this can actually fail, unwrap for now.
                self.inner.set_visible(visible).unwrap();
            }
        }
    };
}

pub struct LocationBar {
    inner: web_sys::BarProp,
}

impl_bar_state!(LocationBar);

pub struct MenuBar {
    inner: web_sys::BarProp,
}

impl_bar_state!(MenuBar);

pub struct PersonalBar {
    inner: web_sys::BarProp,
}

impl_bar_state!(PersonalBar);

pub struct ScrollBars {
    inner: web_sys::BarProp,
}

impl_bar_state!(ScrollBars);

pub struct StatusBar {
    inner: web_sys::BarProp,
}

impl_bar_state!(StatusBar);

pub struct ToolBar {
    inner: web_sys::BarProp,
}

impl_bar_state!(ToolBar);
