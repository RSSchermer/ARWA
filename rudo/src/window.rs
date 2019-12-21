
pub struct Window {
    inner: web_sys::Window
}

impl Window {
    delegate! {
        target self.inner {

        }
    }
}