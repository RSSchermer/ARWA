macro_rules! heading_element {
    ($element:ident, $tag_name:literal) => {
        #[derive(Clone)]
        pub struct $element {
            inner: web_sys::HtmlHeadingElement,
        }

        impl AsRef<web_sys::HtmlHeadingElement> for $element {
            fn as_ref(&self) -> &web_sys::HtmlHeadingElement {
                &self.inner
            }
        }

        $crate::html::impl_html_element_traits!($element);
        $crate::dom::impl_try_from_element_with_tag_check!($element, HtmlHeadingElement, $tag_name);
        $crate::html::impl_known_element!($element, HtmlHeadingElement, $tag_name);
        $crate::dom::impl_shadow_host_for_element!($element);
    };
}

heading_element!(H1Element, "H1");
heading_element!(H2Element, "H2");
heading_element!(H3Element, "H3");
heading_element!(H4Element, "H4");
heading_element!(H5Element, "H5");
heading_element!(H6Element, "H6");
