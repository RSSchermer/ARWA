use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{
    DynamicElement, DynamicNode, Element, GlobalEventHandlers, InvalidCast, Node, StyleSheet,
};

#[derive(Clone)]
pub struct HtmlLinkElement {
    inner: web_sys::HtmlLinkElement,
}

impl HtmlLinkElement {
    delegate! {
        target self.inner {
            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);

            pub fn href(&self) -> String;

            pub fn set_href(&self, href: &str);

            pub fn hreflang(&self) -> String;

            pub fn set_hreflang(&self, hreflang: &str);

            pub fn media(&self) -> String;

            pub fn set_media(&self, media: &str);
        }
    }

    pub fn as_content(&self) -> String {
        self.inner.as_()
    }

    pub fn set_as_content(&self, as_content: &str) {
        self.inner.set_as(as_content);
    }

    pub fn mime_type(&self) -> String {
        self.inner.type_()
    }

    pub fn set_mime_type(&self, mime_type: &str) {
        self.inner.set_type(mime_type);
    }

    pub fn sheet(&self) -> Option<StyleSheet> {
        self.inner.sheet().map(|s| s.into())
    }

    pub fn rel(&self) -> LinkRel {
        LinkRel {
            link: &self.inner,
            rel_list: self.inner.rel_list(),
        }
    }

    pub fn set_rel(&self, rel: &str) {
        self.inner.set_rel(rel);
    }

    pub fn sizes(&self) -> LinkSizes {
        LinkSizes {
            size_list: self.inner.sizes(),
        }
    }

    pub fn set_sizes(&self, sizes: &str) {
        self.inner.sizes().set_value(sizes);
    }
}

impl_html_common_traits!(HtmlLinkElement);

pub struct LinkRel<'a> {
    link: &'a web_sys::HtmlLinkElement,
    rel_list: web_sys::DomTokenList,
}

impl<'a> LinkRel<'a> {
    pub fn get(&self, index: usize) -> Option<String> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.rel_list.item(index))
    }

    pub fn len(&self) -> usize {
        self.rel_list.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, rel: &str) -> bool {
        self.rel_list.contains(rel)
    }

    // TODO: make insert and remove add a bool by adding a `contains` check?

    pub fn insert(&self, rel: &str) {
        self.rel_list.toggle_with_force(rel, true).unwrap();
    }

    pub fn remove(&self, rel: &str) {
        self.rel_list.remove_1(rel).unwrap();
    }

    pub fn toggle(&self, rel: &str) -> bool {
        self.rel_list.toggle(rel).unwrap()
    }

    pub fn replace(&self, old: &str, new: &str) -> bool {
        // It seems the error case covers old browser returning void instead of a bool, but I don't
        // believe there's any overlap between browsers that support WASM and browsers that still
        // return void, so this should never cause an error.
        self.rel_list.replace(old, new).unwrap()
    }

    pub fn iter(&self) -> LinkRelIter {
        LinkRelIter {
            link_rel: self,
            current: 0,
        }
    }
}

impl<'a> Write for LinkRel<'a> {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.rel_list.as_ref());
    }
}

impl<'a> ToString for LinkRel<'a> {
    fn to_string(&self) -> String {
        self.link.rel()
    }
}

impl<'a> IntoIterator for LinkRel<'a> {
    type Item = String;
    type IntoIter = LinkRelIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LinkRelIntoIter {
            link_rel: self,
            current: 0,
        }
    }
}

pub struct LinkRelIter<'a> {
    link_rel: &'a LinkRel<'a>,
    current: usize,
}

impl<'a> Iterator for LinkRelIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.link_rel.get(current)
    }
}

pub struct LinkRelIntoIter<'a> {
    link_rel: LinkRel<'a>,
    current: usize,
}

impl<'a> Iterator for LinkRelIntoIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.link_rel.get(current)
    }
}

pub struct LinkSizes {
    size_list: web_sys::DomTokenList,
}

impl LinkSizes {
    pub fn get(&self, index: usize) -> Option<String> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.size_list.item(index))
    }

    pub fn len(&self) -> usize {
        self.size_list.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, class: &str) -> bool {
        self.size_list.contains(class)
    }

    // TODO: make insert and remove add a bool by adding a `contains` check?

    pub fn insert(&self, class: &str) {
        self.size_list.toggle_with_force(class, true).unwrap();
    }

    pub fn remove(&self, class: &str) {
        self.size_list.remove_1(class).unwrap();
    }

    pub fn toggle(&self, class: &str) -> bool {
        self.size_list.toggle(class).unwrap()
    }

    pub fn replace(&self, old: &str, new: &str) -> bool {
        // It seems the error case covers old browser returning void instead of a bool, but I don't
        // believe there's any overlap between browsers that support WASM and browsers that still
        // return void, so this should never cause an error.
        self.size_list.replace(old, new).unwrap()
    }

    pub fn iter(&self) -> LinkSizesIter {
        LinkSizesIter {
            link_sizes: self,
            current: 0,
        }
    }
}

impl Write for LinkSizes {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.size_list.as_ref());
    }
}

impl ToString for LinkSizes {
    fn to_string(&self) -> String {
        self.size_list.value()
    }
}

impl IntoIterator for LinkSizes {
    type Item = String;
    type IntoIter = LinkSizesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        LinkSizesIntoIter {
            link_sizes: self,
            current: 0,
        }
    }
}

pub struct LinkSizesIter<'a> {
    link_sizes: &'a LinkSizes,
    current: usize,
}

impl<'a> Iterator for LinkSizesIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.link_sizes.get(current)
    }
}

pub struct LinkSizesIntoIter {
    link_sizes: LinkSizes,
    current: usize,
}

impl Iterator for LinkSizesIntoIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.link_sizes.get(current)
    }
}
