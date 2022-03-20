use crate::dom::impl_shadow_host_for_element;

macro_rules! generic_html_element {
    ($tpe:ident, $tag_name:literal, $tag_name_lower:literal) => {
        #[derive(Clone)]
        pub struct $tpe {
            inner: web_sys::HtmlElement,
        }

        $crate::html::impl_html_element_traits!($tpe);
        $crate::dom::impl_try_from_element_with_tag_check!($tpe, HtmlElement, $tag_name);
        $crate::html::impl_known_element!($tpe, HtmlElement, $tag_name);
        $crate::html::impl_extendable_element!($tpe, $tag_name_lower);
    };
}

generic_html_element!(HtmlAddressElement, "ADDRESS", "address");
generic_html_element!(HtmlArticleElement, "ARTICLE", "article");
generic_html_element!(HtmlAsideElement, "ASIDE", "aside");
generic_html_element!(HtmlFooterElement, "FOOTER", "footer");
generic_html_element!(HtmlHeaderElement, "HEADER", "header");
generic_html_element!(HtmlMainElement, "MAIN", "main");
generic_html_element!(HtmlNavElement, "NAV", "nav");
generic_html_element!(HtmlSectionElement, "SECTION", "section");
generic_html_element!(HtmlBlockquoteElement, "BLOCKQUOTE", "blockquote");
generic_html_element!(HtmlDdElement, "DD", "dd");
generic_html_element!(HtmlDtElement, "DT", "dt");
generic_html_element!(HtmlFigcaptionElement, "FIGCAPTION", "figcaption");
generic_html_element!(HtmlFigureElement, "FIGURE", "figure");
generic_html_element!(HtmlAbbrElement, "ABBR", "abbr");
generic_html_element!(HtmlBElement, "B", "b");
generic_html_element!(HtmlBdiElement, "BDI", "bdi");
generic_html_element!(HtmlBdoElement, "BDO", "bdo");
generic_html_element!(HtmlCodeElement, "CODE", "code");
generic_html_element!(HtmlDfnElement, "DFN", "dfn");
generic_html_element!(HtmlEmElement, "EM", "em");
generic_html_element!(HtmlIElement, "I", "i");
generic_html_element!(HtmlKbdElement, "KBD", "kbd");
generic_html_element!(HtmlMarkElement, "MARK", "mark");
generic_html_element!(HtmlQElement, "Q", "q");
generic_html_element!(HtmlRpElement, "RP", "rp");
generic_html_element!(HtmlRtElement, "RT", "rt");
generic_html_element!(HtmlRubyElement, "RUBY", "ruby");
generic_html_element!(HtmlSElement, "S", "s");
generic_html_element!(HtmlSampElement, "SAMP", "samp");
generic_html_element!(HtmlSmallElement, "SMALL", "small");
generic_html_element!(HtmlStrongElement, "STRONG", "strong");
generic_html_element!(HtmlSubElement, "SUB", "sub");
generic_html_element!(HtmlSupElement, "SUP", "sup");
generic_html_element!(HtmlUElement, "U", "u");
generic_html_element!(HtmlVarElement, "VAR", "var");
generic_html_element!(HtmlWbrElement, "WBR", "wbr");
generic_html_element!(HtmlNoscriptElement, "NOSCRIPT", "noscript");
generic_html_element!(HtmlSummaryElement, "SUMMARY", "summary");

impl_shadow_host_for_element!(HtmlArticleElement);
impl_shadow_host_for_element!(HtmlAsideElement);
impl_shadow_host_for_element!(HtmlBlockquoteElement);
impl_shadow_host_for_element!(HtmlHeaderElement);
impl_shadow_host_for_element!(HtmlFooterElement);
impl_shadow_host_for_element!(HtmlMainElement);
impl_shadow_host_for_element!(HtmlNavElement);
impl_shadow_host_for_element!(HtmlSectionElement);
