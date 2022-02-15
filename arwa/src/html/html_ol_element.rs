#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OrderedListType {
    Decimal,
    LowerAlpha,
    UpperAlpha,
    LowerRoman,
    UpperRoman,
}

impl Default for OrderedListType {
    fn default() -> Self {
        OrderedListType::Decimal
    }
}

#[derive(Clone)]
pub struct HtmlOlElement {
    inner: web_sys::HtmlOlElement,
}

impl HtmlOlElement {
    delegate! {
        target self.inner {
            pub fn reversed(&self) -> bool;

            pub fn set_reversed(&self, reversed: bool);

            pub fn start(&self) -> i32;

            pub fn set_start(&self, start: i32);
        }
    }

    pub fn list_type(&self) -> OrderedListType {
        match &*self.inner.type_() {
            "a" => OrderedListType::LowerAlpha,
            "A" => OrderedListType::UpperAlpha,
            "i" => OrderedListType::LowerRoman,
            "I" => OrderedListType::UpperRoman,
            _ => OrderedListType::Decimal,
        }
    }

    pub fn set_list_type(&self, list_type: OrderedListType) {
        let list_type = match list_type {
            OrderedListType::Decimal => "1",
            OrderedListType::LowerAlpha => "a",
            OrderedListType::UpperAlpha => "A",
            OrderedListType::LowerRoman => "i",
            OrderedListType::UpperRoman => "I",
        };

        self.inner.set_type(list_type);
    }
}

impl From<web_sys::HtmlOListElement> for HtmlOlElement {
    fn from(inner: web_sys::HtmlOListElement) -> Self {
        HtmlOListElement { inner }
    }
}

impl AsRef<web_sys::HtmlOListElement> for HtmlOlElement {
    fn as_ref(&self) -> &web_sys::HtmlOListElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlOlElement);
impl_try_from_element!(HtmlOlElement, web_sys::HtmlOListElement);
impl_known_element!(HtmlOListElement, "OL");
