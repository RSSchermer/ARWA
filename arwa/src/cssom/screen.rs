use wasm_bindgen::UnwrapThrowExt;

use crate::impl_common_wrapper_traits;

pub struct Screen {
    inner: web_sys::Screen,
}

impl Screen {
    pub fn width(&self) -> u32 {
        self.inner.width().unwrap_throw() as u32
    }

    pub fn height(&self) -> u32 {
        self.inner.height().unwrap_throw() as u32
    }

    pub fn avail_width(&self) -> u32 {
        self.inner.avail_width().unwrap_throw() as u32
    }

    pub fn avail_height(&self) -> u32 {
        self.inner.avail_height().unwrap_throw() as u32
    }

    pub fn color_depth(&self) -> u32 {
        self.inner.color_depth().unwrap_throw() as u32
    }

    // Skip `pixel_depth`, seems to be an alias for `color_depth`.
}

impl From<web_sys::Screen> for Screen {
    fn from(inner: web_sys::Screen) -> Self {
        Screen { inner }
    }
}

impl AsRef<web_sys::Screen> for Screen {
    fn as_ref(&self) -> &web_sys::Screen {
        &self.inner
    }
}

impl_common_wrapper_traits!(Screen);
