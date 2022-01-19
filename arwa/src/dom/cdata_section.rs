#[derive(Clone)]
pub struct CDATASection {
    inner: web_sys::CdataSection,
}

impl From<web_sys::CdataSection> for CDATASection {
    fn from(inner: web_sys::CdataSection) -> Self {
        CDATASection { inner }
    }
}

impl_text_data_traits!(CDATASection, web_sys::CdataSection);

#[derive(Clone)]
pub struct CDATAError {
    inner: web_sys::DomException,
}

impl CDATAError {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        CDATAError { inner }
    }
}
