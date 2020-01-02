use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{
    Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node, ReferrerPolicy,
};

#[derive(Clone)]
pub struct HtmlAnchorElement {
    inner: web_sys::HtmlAnchorElement,
}

impl HtmlAnchorElement {
    delegate! {
        target self.inner {
            pub fn target(&self) -> String;

            pub fn set_target(&self, target: &str);

            pub fn download(&self) -> String;

            pub fn set_download(&self, download: &str);

            pub fn ping(&self) -> String;

            pub fn set_ping(&self, ping: &str);

            pub fn href(&self) -> String;

            pub fn set_href(&self, href: &str);

            pub fn origin(&self) -> String;

            pub fn protocol(&self) -> String;

            pub fn set_protocol(&self, protocol: &str);

            pub fn username(&self) -> String;

            pub fn set_username(&self, username: &str);

            pub fn password(&self) -> String;

            pub fn set_password(&self, password: &str);

            pub fn host(&self) -> String;

            pub fn set_host(&self, host: &str);

            pub fn hostname(&self) -> String;

            pub fn set_hostname(&self, hostname: &str);

            pub fn port(&self) -> String;

            pub fn set_port(&self, port: &str);

            pub fn pathname(&self) -> String;

            pub fn set_pathname(&self, pathname: &str);

            pub fn search(&self) -> String;

            pub fn set_search(&self, search: &str);

            pub fn hash(&self) -> String;

            pub fn set_hash(&self, hash: &str);
        }
    }

    pub fn text(&self) -> String {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.text().unwrap()
    }

    pub fn set_text(&self, text: &str) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.set_text(text).unwrap();
    }

    pub fn mime_type(&self) -> String {
        self.inner.type_()
    }

    pub fn set_mime_type(&self, mime_type: &str) {
        self.inner.set_type(mime_type);
    }

    pub fn referrer_policy(&self) -> ReferrerPolicy {
        ReferrerPolicy::from_str(&self.inner.referrer_policy())
    }

    pub fn set_referrer_policy(&self, referrer_policy: ReferrerPolicy) {
        self.inner.set_referrer_policy(referrer_policy.as_ref())
    }

    pub fn rel(&self) -> AnchorRel {
        AnchorRel {
            anchor: &self.inner,
            rel_list: self.inner.rel_list(),
        }
    }

    pub fn set_rel(&self, rel: &str) {
        self.inner.set_rel(rel);
    }
}

impl_html_common_traits!(HtmlAnchorElement);

pub struct AnchorRel<'a> {
    anchor: &'a web_sys::HtmlAnchorElement,
    rel_list: web_sys::DomTokenList,
}

impl<'a> AnchorRel<'a> {
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

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
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

    pub fn iter(&self) -> AnchorRelIter {
        AnchorRelIter {
            anchor_rel: self,
            current: 0,
        }
    }
}

impl<'a> ToString for AnchorRel<'a> {
    fn to_string(&self) -> String {
        self.anchor.rel()
    }
}

impl<'a> IntoIterator for AnchorRel<'a> {
    type Item = String;
    type IntoIter = AnchorRelIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AnchorRelIntoIter {
            anchor_rel: self,
            current: 0,
        }
    }
}

pub struct AnchorRelIter<'a> {
    anchor_rel: &'a AnchorRel<'a>,
    current: usize,
}

impl<'a> Iterator for AnchorRelIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.anchor_rel.get(current)
    }
}

pub struct AnchorRelIntoIter<'a> {
    anchor_rel: AnchorRel<'a>,
    current: usize,
}

impl<'a> Iterator for AnchorRelIntoIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.anchor_rel.get(current)
    }
}
