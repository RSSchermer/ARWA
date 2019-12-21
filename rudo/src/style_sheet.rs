use crate::GenericNode;
use delegate::delegate;
use std::convert::TryFrom;

pub struct StyleSheet {
    inner: web_sys::StyleSheet,
}

impl StyleSheet {
    delegate! {
        target self.inner {
            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);

            pub fn title(&self) -> Option<String>;
        }
    }

    pub fn href(&self) -> Option<String> {
        self.inner.href().ok().flatten()
    }

    pub fn style_sheet_type(&self) -> String {
        self.inner.type_()
    }

    pub fn owner_node(&self) -> Option<GenericNode> {
        self.inner.owner_node().map(|n| GenericNode::from(n))
    }

    pub fn parent_style_sheet(&self) -> Option<StyleSheet> {
        self.inner.parent_style_sheet().map(|s| StyleSheet::from(s))
    }

    pub fn media(&self) -> StylesheetMedia {
        StylesheetMedia {
            inner: self.inner.media(),
        }
    }

    pub fn set_media(&self, media: &str) {
        self.inner.media().set_media_text(media);
    }
}

impl AsRef<web_sys::StyleSheet> for StyleSheet {
    fn as_ref(&self) -> &web_sys::StyleSheet {
        &self.inner
    }
}

impl From<web_sys::StyleSheet> for StyleSheet {
    fn from(inner: web_sys::StyleSheet) -> Self {
        StyleSheet { inner }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InvalidMedium(String);

pub struct StylesheetMedia {
    inner: web_sys::MediaList,
}

impl StylesheetMedia {
    pub fn get(&self, index: usize) -> Option<String> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get(index))
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
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

    pub fn push(&self, medium: &str) -> Result<(), InvalidMedium> {
        self.inner
            .append_medium(medium)
            .map_err(|_| InvalidMedium(medium.to_string()))
    }

    pub fn remove(&self, medium: &str) -> bool {
        self.inner.delete_medium(medium).is_ok()
    }

    pub fn iter(&self) -> StylesheetMediaIter {
        StylesheetMediaIter {
            stylesheet_media: self,
            current: 0,
        }
    }
}

impl IntoIterator for StylesheetMedia {
    type Item = String;
    type IntoIter = StylesheetMediaIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        StylesheetMediaIntoIter {
            stylesheet_media: self,
            current: 0,
        }
    }
}

pub struct StylesheetMediaIter<'a> {
    stylesheet_media: &'a StylesheetMedia,
    current: usize,
}

impl<'a> Iterator for StylesheetMediaIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.stylesheet_media.get(current)
    }
}

pub struct StylesheetMediaIntoIter {
    stylesheet_media: StylesheetMedia,
    current: usize,
}

impl Iterator for StylesheetMediaIntoIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.stylesheet_media.get(current)
    }
}
