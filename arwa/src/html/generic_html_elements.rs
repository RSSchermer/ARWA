use crate::dom::{shadow_host_seal, ShadowHost};

macro_rules! generic_html_element {
    ($tpe:ident, $tag_name:literal) => {
        #[derive(Clone)]
        pub struct $tpe {
            inner: web_sys::HtmlElement
        }

        $crate::html::impl_html_element_traits!($tpe);
        $crate::dom::impl_try_from_element_with_tag_check($tpe, web_sys::HtmlElement, $tag_name);
        $crate::html::impl_know_element($tpe, web_sys::HtmlElement, $tag_name);
    }
}

generic_html_element!(HtmlAddressElement, "ADDRESS");
generic_html_element!(HtmlArticleElement, "ARTICLE");
generic_html_element!(HtmlAsideElement, "ASIDE");
generic_html_element!(HtmlFooterElement, "FOOTER");
generic_html_element!(HtmlHeaderElement, "HEADER");
generic_html_element!(HtmlMainElement, "MAIN");
generic_html_element!(HtmlNavElement, "NAV");
generic_html_element!(HtmlSectionElement, "SECTION");
generic_html_element!(HtmlBlockquoteElement, "BLOCKQUOTE");
generic_html_element!(HtmlDdElement, "DD");
generic_html_element!(HtmlDtElement, "DT");
generic_html_element!(HtmlFigcaptionElement, "FIGCAPTION");
generic_html_element!(HtmlFigureElement, "FIGURE");
generic_html_element!(HtmlAbbrElement, "ABBR");
generic_html_element!(HtmlBElement, "B");
generic_html_element!(HtmlBdiElement, "BDI");
generic_html_element!(HtmlBdoElement, "BDO");
generic_html_element!(HtmlCodeElement, "CODE");
generic_html_element!(HtmlDfnElement, "DFN");
generic_html_element!(HtmlEmElement, "EM");
generic_html_element!(HtmlIElement, "I");
generic_html_element!(HtmlKbdElement, "KBD");
generic_html_element!(HtmlMarkElement, "MARK");
generic_html_element!(HtmlQElement, "Q");
generic_html_element!(HtmlRpElement, "RP");
generic_html_element!(HtmlRtElement, "RT");
generic_html_element!(HtmlRubyElement, "RUBY");
generic_html_element!(HtmlSElement, "S");
generic_html_element!(HtmlSampElement, "SAMP");
generic_html_element!(HtmlSmallElement, "SMALL");
generic_html_element!(HtmlStrongElement, "STRONG");
generic_html_element!(HtmlSubElement, "SUB");
generic_html_element!(HtmlSupElement, "SUP");
generic_html_element!(HtmlUElement, "U");
generic_html_element!(HtmlVarElement, "VAR");
generic_html_element!(HtmlWbrElement, "WBR");
generic_html_element!(HtmlNoscriptElement, "NOSCRIPT");
generic_html_element!(HtmlSummaryElement, "SUMMARY");

impl_shadow_host_for_element!(HtmlArticleElement);
impl_shadow_host_for_element!(HtmlAsideElement);
impl_shadow_host_for_element!(HtmlBlockquoteElement);
impl_shadow_host_for_element!(HtmlHeaderElement);
impl_shadow_host_for_element!(HtmlFooterElement);
impl_shadow_host_for_element!(HtmlMainElement);
impl_shadow_host_for_element!(HtmlNavElement);
impl_shadow_host_for_element!(HtmlSectionElement);
