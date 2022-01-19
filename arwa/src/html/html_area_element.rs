use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{
    DynamicElement, DynamicNode, Element, GlobalEventHandlers, InvalidCast, Node, ReferrerPolicy,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AreaShape {
    Default,
    Rectangle,
    Circle,
    Polygon,
}

impl Default for AreaShape {
    fn default() -> Self {
        AreaShape::Default
    }
}

#[derive(Clone)]
pub struct HtmlAreaElement {
    inner: web_sys::HtmlAreaElement,
}

impl HtmlAreaElement {
    // TODO: way to ensure setting coords conforms to shape always (e.g. acquire shape enum, which
    // contains shape object for one specific shape type, set coords through shape object as
    // specific coord type (RectangleCoords, CircleCoords, PolygonCoords, no coords if shape is
    // "default")?)

    delegate! {
        target self.inner {
            pub fn alt(&self) -> String;

            pub fn set_alt(&self, alt: &str);

            pub fn coords(&self) -> String;

            pub fn set_coords(&self, coords: &str);

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

    pub fn shape(&self) -> AreaShape {
        match &*self.inner.shape() {
            "circle" => AreaShape::Circle,
            "circ" => AreaShape::Circle,
            "rect" => AreaShape::Rectangle,
            "rectangle" => AreaShape::Rectangle,
            "poly" => AreaShape::Polygon,
            "polygon" => AreaShape::Polygon,
            "default" => AreaShape::Default,
            // Note missing value and invalid value default is "rect" per
            // https://html.spec.whatwg.org/multipage/image-maps.html#the-area-element
            _ => AreaShape::Rectangle,
        }
    }

    pub fn set_shape(&self, shape: AreaShape) {
        let shape = match shape {
            AreaShape::Circle => "circle",
            AreaShape::Rectangle => "rect",
            AreaShape::Polygon => "poly",
            AreaShape::Default => "default",
        };

        self.inner.set_shape(shape);
    }

    pub fn referrer_policy(&self) -> ReferrerPolicy {
        ReferrerPolicy::from_str(&self.inner.referrer_policy())
    }

    pub fn set_referrer_policy(&self, referrer_policy: ReferrerPolicy) {
        self.inner.set_referrer_policy(referrer_policy.as_ref())
    }

    pub fn rel(&self) -> AreaRel {
        AreaRel {
            area: &self.inner,
            rel_list: self.inner.rel_list(),
        }
    }

    pub fn set_rel(&self, rel: &str) {
        self.inner.set_rel(rel);
    }
}

impl_html_common_traits!(HtmlAreaElement);

pub struct AreaRel<'a> {
    area: &'a web_sys::HtmlAreaElement,
    rel_list: web_sys::DomTokenList,
}

impl<'a> AreaRel<'a> {
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

    pub fn iter(&self) -> AreaRelIter {
        AreaRelIter {
            area_rel: self,
            current: 0,
        }
    }
}

impl<'a> Write for AreaRel<'a> {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.rel_list.as_ref());
    }
}

impl<'a> ToString for AreaRel<'a> {
    fn to_string(&self) -> String {
        self.area.rel()
    }
}

impl<'a> IntoIterator for AreaRel<'a> {
    type Item = String;
    type IntoIter = AreaRelIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AreaRelIntoIter {
            area_rel: self,
            current: 0,
        }
    }
}

pub struct AreaRelIter<'a> {
    area_rel: &'a AreaRel<'a>,
    current: usize,
}

impl<'a> Iterator for AreaRelIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.area_rel.get(current)
    }
}

pub struct AreaRelIntoIter<'a> {
    area_rel: AreaRel<'a>,
    current: usize,
}

impl<'a> Iterator for AreaRelIntoIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.area_rel.get(current)
    }
}
