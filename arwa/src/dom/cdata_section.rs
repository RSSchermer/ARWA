use crate::dom::impl_text_data_traits;
use crate::dom_exception_wrapper;

/// Represents an XML CDATA section that can be used within XML to include unescaped text.
///
/// Inside a CDATA section the symbols `<` and `&` do not need to be escaped.
///
/// ```xml
/// <foo>Here is a CDATA section: <![CDATA[ < > & ]]> with all kinds of unescaped text.</foo>
/// ```
///
/// Note that inside HTML documents CDATA sections
#[derive(Clone)]
pub struct CDataSection {
    inner: web_sys::CdataSection,
}

impl From<web_sys::CdataSection> for CDataSection {
    fn from(inner: web_sys::CdataSection) -> Self {
        CDataSection { inner }
    }
}

impl_text_data_traits!(CDataSection, CdataSection);

dom_exception_wrapper!(CDataError);
