use crate::dom::impl_text_data_traits;
use crate::dom_exception_wrapper;

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

dom_exception_wrapper!(CDATAError);
