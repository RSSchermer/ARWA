macro_rules! heading_element {
    ($element:ident, $tag_name:literal, $tag_name_lower:literal) => {
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
        $crate::html::impl_extendable_element!($element, $tag_name_lower);
        $crate::dom::impl_shadow_host_for_element!($element);
    };
}

heading_element!(HtmlH1Element, "H1", "h1");
heading_element!(HtmlH2Element, "H2", "h2");
heading_element!(HtmlH3Element, "H3", "h3");
heading_element!(HtmlH4Element, "H4", "h4");
heading_element!(HtmlH5Element, "H5", "h5");
heading_element!(HtmlH6Element, "H6", "h6");
