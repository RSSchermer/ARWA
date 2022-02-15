use std::cell::RefCell;

use crate::collection::{Collection, Sequence};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnchorRelationshipType {
    Alternate,
    Author,
    Bookmark,
    External,
    Help,
    License,
    Next,
    NoFollow,
    NoOpener,
    NoReferrer,
    Opener,
    Previous,
    Search,
    Tag,
}

impl AnchorRelationshipType {
    fn from_token(token: &str) -> Option<Self> {
        match token {
            "alternate" => Some(AnchorRelationshipType::Alternate),
            "author" => Some(AnchorRelationshipType::Author),
            "bookmark" => Some(AnchorRelationshipType::Bookmark),
            "external" => Some(AnchorRelationshipType::External),
            "help" => Some(AnchorRelationshipType::Help),
            "license" => Some(AnchorRelationshipType::License),
            "next" => Some(AnchorRelationshipType::Next),
            "nofollow" => Some(AnchorRelationshipType::NoFollow),
            "noopener" => Some(AnchorRelationshipType::NoOpener),
            "noreferrer" => Some(AnchorRelationshipType::NoReferrer),
            "opener" => Some(AnchorRelationshipType::Opener),
            "prev" => Some(AnchorRelationshipType::Previous),
            "search" => Some(AnchorRelationshipType::Search),
            "tag" => Some(AnchorRelationshipType::Tag),
            _ => None
        }
    }

    fn to_token(&self) -> &'static str {
        match self {
            AnchorRelationshipType::Alternate => "alternate",
            AnchorRelationshipType::Author => "author",
            AnchorRelationshipType::Bookmark => "bookmark",
            AnchorRelationshipType::External => "external",
            AnchorRelationshipType::Help => "help",
            AnchorRelationshipType::License => "license",
            AnchorRelationshipType::Next => "next",
            AnchorRelationshipType::NoFollow => "nofollow",
            AnchorRelationshipType::NoOpener => "noopener",
            AnchorRelationshipType::NoReferrer => "noreferrer",
            AnchorRelationshipType::Opener => "opener",
            AnchorRelationshipType::Previous => "prev",
            AnchorRelationshipType::Search => "search",
            AnchorRelationshipType::Tag => "tag",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AreaRelationshipType {
    Alternate,
    Author,
    Bookmark,
    External,
    Help,
    License,
    Next,
    NoFollow,
    NoOpener,
    NoReferrer,
    Opener,
    Previous,
    Search,
    Tag,
}

impl AreaRelationshipType {
    fn from_token(token: &str) -> Option<Self> {
        match token {
            "alternate" => Some(AreaRelationshipType::Alternate),
            "author" => Some(AreaRelationshipType::Author),
            "bookmark" => Some(AreaRelationshipType::Bookmark),
            "external" => Some(AreaRelationshipType::External),
            "help" => Some(AreaRelationshipType::Help),
            "license" => Some(AreaRelationshipType::License),
            "next" => Some(AreaRelationshipType::Next),
            "nofollow" => Some(AreaRelationshipType::NoFollow),
            "noopener" => Some(AreaRelationshipType::NoOpener),
            "noreferrer" => Some(AreaRelationshipType::NoReferrer),
            "opener" => Some(AreaRelationshipType::Opener),
            "prev" => Some(AreaRelationshipType::Previous),
            "search" => Some(AreaRelationshipType::Search),
            "tag" => Some(AreaRelationshipType::Tag),
            _ => None
        }
    }

    fn to_token(&self) -> &'static str {
        match self {
            AreaRelationshipType::Alternate => "alternate",
            AreaRelationshipType::Author => "author",
            AreaRelationshipType::Bookmark => "bookmark",
            AreaRelationshipType::External => "external",
            AreaRelationshipType::Help => "help",
            AreaRelationshipType::License => "license",
            AreaRelationshipType::Next => "next",
            AreaRelationshipType::NoFollow => "nofollow",
            AreaRelationshipType::NoOpener => "noopener",
            AreaRelationshipType::NoReferrer => "noreferrer",
            AreaRelationshipType::Opener => "opener",
            AreaRelationshipType::Previous => "prev",
            AreaRelationshipType::Search => "search",
            AreaRelationshipType::Tag => "tag",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LinkRelationshipType {
    Alternate,
    Canonical,
    Author,
    DnsPrefetch,
    Help,
    Icon,
    Manifest,
    ModulePreload,
    License,
    Next,
    Pingback,
    Preconnect,
    Prefetch,
    Preload,
    Prerender,
    Previous,
    Search,
    Stylesheet,
}

impl LinkRelationshipType {
    fn from_token(token: &str) -> Option<Self> {
        match token {
            "alternate" => Some(LinkRelationshipType::Alternate),
            "canonical" => Some(LinkRelationshipType::Canonical),
            "author" => Some(LinkRelationshipType::Author),
            "dns-prefetch" => Some(LinkRelationshipType::DnsPrefetch),
            "help" => Some(LinkRelationshipType::Help),
            "icon" => Some(LinkRelationshipType::Icon),
            "manifest" => Some(LinkRelationshipType::Manifest),
            "modulepreload" => Some(LinkRelationshipType::ModulePreload),
            "license" => Some(LinkRelationshipType::License),
            "next" => Some(LinkRelationshipType::Next),
            "pingback" => Some(LinkRelationshipType::Pingback),
            "preconnect" => Some(LinkRelationshipType::Preconnect),
            "prefetch" => Some(LinkRelationshipType::Prefetch),
            "preload" => Some(LinkRelationshipType::Preload),
            "prerender" => Some(LinkRelationshipType::Prerender),
            "prev" => Some(LinkRelationshipType::Previous),
            "search" => Some(LinkRelationshipType::Search),
            "stylesheet" => Some(LinkRelationshipType::Stylesheet),
            _ => None
        }
    }

    fn to_token(&self) -> &'static str {
        match self {
            LinkRelationshipType::Alternate => "alternate",
            LinkRelationshipType::Canonical => "canonical",
            LinkRelationshipType::Author => "author",
            LinkRelationshipType::DnsPrefetch => "dns-prefetch",
            LinkRelationshipType::Help => "help",
            LinkRelationshipType::Icon => "icon",
            LinkRelationshipType::Manifest => "manifest",
            LinkRelationshipType::ModulePreload => "modulepreload",
            LinkRelationshipType::License => "license",
            LinkRelationshipType::Next => "next",
            LinkRelationshipType::Pingback => "pingback",
            LinkRelationshipType::Preconnect => "preconnect",
            LinkRelationshipType::Prefetch => "prefetch",
            LinkRelationshipType::Preload => "preload",
            LinkRelationshipType::Prerender => "prerender",
            LinkRelationshipType::Previous => "prev",
            LinkRelationshipType::Search => "search",
            LinkRelationshipType::Stylesheet => "stylesheet",
        }
    }
}

macro_rules! link_types {
    ($tpe:ident, $cache:ident, $web_sys_tpe:ident, $target:ident) => {
        struct $cache {
            raw: String,
            parsed: Vec<$target>,
        }

        impl $cache {
            fn refresh(&mut self, mut rel_string: String) {
                rel_string.make_ascii_lowercase();

                if self.raw != rel_string {
                    let mut parsed_new = Vec::new();

                    for token in rel_string.split_ascii_whitespace() {
                        if let Some(rel) = $target::from_token(token) {
                            if !parsed_new.iter().any(|r| r == rel) {
                                parsed_new.push(rel);
                            }
                        }
                    }

                    self.parsed = parsed_new;
                    self.raw = rel_string;
                }
            }

            fn contains(&self, rel: $target) -> bool {
                self.parsed.iter().any(|r| r == rel)
            }

            fn serialize(&self) -> String {
                self.parsed.iter().map(|rel| rel.to_token()).join(" ")
            }
        }

        pub struct $tpe {
            link: $web_sys_tpe,
            cached: RefCell<$cache>,
        }

        impl $tpe {
            pub(crate) fn new(link: $web_sys_tpe) -> Self {
                $tpe {
                    link,
                    cached: RefCell::new($cache {
                        raw: String::new(),
                        parsed: Vec::new()
                    })
                }
            }

            fn refresh(&self) -> RefMut<$cache> {
                let mut cached = self.cached.borrow_mut();

                cached.refresh(self.link.rel());

                cached
            }

            pub fn contains(&self, rel: $target) -> bool {
                self.refresh().contains(rel)
            }

            pub fn insert(&self, rel: $target) -> bool {
                let cached = self.refresh();

                if !cached.contains(rel) {
                    cached.parsed.push(rel);

                    let new_rel = cached.serialize();

                    self.link.set_rel(&new_rel);

                    cached.raw = new_rel;

                    true
                } else {
                    false
                }
            }

            pub fn remove(&self, rel: $target) -> bool {
                let cached = self.refresh();

                if cached.contains(rel) {
                    cached.parsed.retain(|r| r != rel);

                    let new_rel = cached.serialize();

                    self.link.set_rel(&new_rel);

                    cached.raw = new_rel;

                    true
                } else {
                    false
                }
            }

            pub fn toggle(&self, rel: $target) -> bool {
                let cached = self.refresh();

                let output = if cached.contains(rel) {
                    cached.parsed.retain(|r| r != rel);

                    false
                } else {
                    cached.parsed.push(rel);

                    true
                };

                let new_rel = cached.serialize();

                self.link.set_rel(&new_rel);

                cached.raw = new_rel;

                output
            }

            pub fn replace(&self, old: $target, new: $target) -> bool {
                let cached = self.refresh();

                let mut did_replace = false;

                for rel in cached.parsed.iter_mut() {
                    if rel == old {
                        *rel = new;

                        did_replace = true;

                        break;
                    }
                }

                if did_replace {
                    let new_rel = cached.serialize();

                    self.link.set_rel(&new_rel);

                    cached.raw = new_rel;

                    true
                } else {
                    false
                }
            }

            pub fn serialize(&self) -> String {
                self.refresh().serialize()
            }

            pub fn deserialize(&self, serialized: &str) {
                let serialized = serialized.to_string();

                let mut cached = self.cached.borrow_mut();

                cached.refresh(serialized);

                self.link.set_rel(cached.serialized());
            }
        }

        impl Collection for $tpe {
            fn len(&self) -> u32 {
                self.refresh().parsed.len() as u32
            }
        }

        impl Sequence for $tpe {
            type Item = $target;

            fn get(&self, index: u32) -> Option<Self::Item> {
                self.refresh().parsed.get(index as usize).copied()
            }

            fn to_host_array(&self) -> js_sys::Array {
                js_sys::JsString::from(self.refresh().serialize()).split(" ")
            }
        }
    }
}

link_types!(AnchorRelationshipTypes, AnchorRelationshipTypesCache, web_sys::HtmlAnchorElement, AnchorRelationshipType);
link_types!(AreaRelationshipTypes, AreaRelationshipTypesCache, web_sys::HtmlAreaElement, AreaRelationshipType);
link_types!(LinkRelationshipTypes, LinkRelationshipTypesCache, web_sys::HtmlLinkElement, LinkRelationshipType);
