pub struct History {
    inner: web_sys::History,
}

impl History {
    // TODO: implement after figuring out how to handle SecurityErrors. It is very unclear to me
    // under what exact circumstances they can occur. Will a browsing context not always have at
    // least 1 active document?

    // TODO: Pick a good name for the `length` function. Neither `length` nor `len` feel like to
    // most appropriate name for this function (`length` seems to translate to `len` in the rust
    // ecosystem and `len` feels like it should ways return a plain `usize` to match the general
    // expectation, not a Result). Possible options: `size`, `depth`, `try_len`.
}

impl From<web_sys::History> for History {
    fn from(inner: web_sys::History) -> Self {
        History { inner }
    }
}

impl AsRef<web_sys::History> for History {
    fn as_ref(&self) -> &web_sys::History {
        &self.inner
    }
}
