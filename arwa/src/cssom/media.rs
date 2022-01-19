#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InvalidMedium(String);

pub struct Media {
    inner: web_sys::MediaList,
}

impl Media {
    pub fn get(&self, index: u32) -> Option<String> {
        self.inner.get(index)
    }

    pub fn len(&self) -> u32 {
        self.inner.length()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<String> {
        self.get(0)
    }

    pub fn last(&self) -> Option<String> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn insert(&self, medium: &str) -> Result<(), InvalidMedium> {
        // This data structure acts like a set, so follow std::collections::HashSet naming
        self.inner
            .append_medium(medium)
            .map_err(|_| InvalidMedium(medium.to_string()))
    }

    pub fn remove(&self, medium: &str) -> bool {
        self.inner.delete_medium(medium).is_ok()
    }

    pub fn snapshot(&self) -> MediaSnapshot {
        StyleSheetMediaSnapshot::new(Array::from(self.inner.as_ref()))
    }
}

unchecked_cast_array_wrapper!(String, js_sys::JsString, MediaSnapshot, MediaSnapshotIter);
