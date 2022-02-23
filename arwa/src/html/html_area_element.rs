use std::fmt::Write;
use std::str::FromStr;

use delegate::delegate;
use wasm_bindgen::UnwrapThrowExt;

use crate::dom::impl_try_from_element;
use crate::html::AreaRelationshipTypes;
use crate::html::{impl_html_element_traits, impl_known_element};
use crate::security::ReferrerPolicy;
use crate::url::{absolute_url_or_relative_url_seal::Seal, AbsoluteOrRelativeUrl, Url};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AreaShapeType {
    Default,
    Rectangle,
    Circle,
    Polygon,
}

impl Default for AreaShapeType {
    fn default() -> Self {
        AreaShapeType::Default
    }
}

pub enum AreaShape<'a> {
    Default,
    Circle {
        center_coords: (f64, f64),
        radius: f64,
    },
    Rectangle {
        top_left_coords: (f64, f64),
        bottom_right_coords: (f64, f64),
    },
    Polygon(&'a [(f64, f64)]),
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

            pub fn target(&self) -> String;

            pub fn set_target(&self, target: &str);

            pub fn download(&self) -> String;

            pub fn set_download(&self, download: &str);
        }
    }

    pub fn href(&self) -> Option<Url> {
        Url::parse(self.inner.href().as_ref()).ok()
    }

    pub fn set_href<T>(&self, href: T)
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.inner.set_href(href.as_str());
    }

    pub fn ping(&self) -> Vec<Url> {
        let ping = self.inner.ping();

        let iter = ping.split_ascii_whitespace();
        let mut result = Vec::new();

        for candidate in iter {
            if let Ok(parsed) = Url::parse(candidate) {
                result.push(parsed)
            }
        }

        result
    }

    pub fn set_ping<I>(&self, mut ping: I)
    where
        I: Iterator,
        I::Item: AbsoluteOrRelativeUrl,
    {
        let mut serialized = String::new();

        if let Some(url) = ping.next() {
            serialized.push_str(url.as_str());
        }

        for url in ping {
            serialized.push(' ');
            serialized.push_str(url.as_str());
        }

        self.inner.set_ping(&serialized);
    }

    pub fn shape_type(&self) -> AreaShapeType {
        match self.inner.shape().as_ref() {
            "circle" => AreaShapeType::Circle,
            "circ" => AreaShapeType::Circle,
            "rect" => AreaShapeType::Rectangle,
            "rectangle" => AreaShapeType::Rectangle,
            "poly" => AreaShapeType::Polygon,
            "polygon" => AreaShapeType::Polygon,
            "default" => AreaShapeType::Default,
            // Note: missing value and invalid value default is "rect" per
            // https://html.spec.whatwg.org/multipage/image-maps.html#the-area-element
            _ => AreaShapeType::Rectangle,
        }
    }

    pub fn coordinates(&self) -> Vec<f64> {
        const SEPARATORS: &'static [char] = &[
            '\u{0009}', '\u{000A}', '\u{000C}', '\u{000D}', '\u{0020}', '\u{002C}', '\u{003B}',
        ];

        self.inner
            .coords()
            .trim_matches(SEPARATORS)
            .split(SEPARATORS)
            .filter(|s| !s.is_empty())
            .map(|c| f64::from_str(c).unwrap_or(0.0))
            .collect()
    }

    pub fn set_shape(&self, shape: AreaShape) {
        let (shape, coords) = match shape {
            AreaShape::Default => ("default", None),
            AreaShape::Circle {
                center_coords,
                radius,
            } => {
                let coords = format!("{}, {}, {}", center_coords.0, center_coords.1, radius);

                ("circle", Some(coords))
            }
            AreaShape::Rectangle {
                top_left_coords,
                bottom_right_coords,
            } => {
                let coords = format!(
                    "{}, {}, {}, {}",
                    top_left_coords.0,
                    top_left_coords.1,
                    bottom_right_coords.0,
                    bottom_right_coords.1
                );

                ("rect", Some(coords))
            }
            AreaShape::Polygon(path) => {
                let mut coords = String::new();

                let mut iter = path.iter();

                if let Some((x, y)) = iter.next() {
                    write!(&mut coords, "{},{}", x, y).unwrap_throw();
                }

                for (x, y) in iter {
                    write!(&mut coords, ",{},{}", x, y).unwrap_throw();
                }

                ("poly", Some(coords))
            }
        };

        self.inner.set_shape(shape);
        self.inner.set_coords(&coords.unwrap_or(String::new()))
    }

    pub fn referrer_policy(&self) -> ReferrerPolicy {
        ReferrerPolicy::from_str(&self.inner.referrer_policy())
    }

    pub fn set_referrer_policy(&self, referrer_policy: ReferrerPolicy) {
        self.inner.set_referrer_policy(referrer_policy.as_str())
    }

    pub fn rel(&self) -> AreaRelationshipTypes {
        AreaRelationshipTypes::new(self.inner.clone())
    }
}

impl From<web_sys::HtmlAreaElement> for HtmlAreaElement {
    fn from(inner: web_sys::HtmlAreaElement) -> Self {
        HtmlAreaElement { inner }
    }
}

impl AsRef<web_sys::HtmlAreaElement> for HtmlAreaElement {
    fn as_ref(&self) -> &web_sys::HtmlAreaElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlAreaElement);
impl_try_from_element!(HtmlAreaElement);
impl_known_element!(HtmlAreaElement, "AREA");
