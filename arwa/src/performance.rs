use delegate::delegate;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{
    PerformanceEntry, PerformanceMark, PerformanceMeasure, PerformanceResourceTiming,
    PerformanceServerTiming,
};

use crate::unchecked_cast_array::unchecked_cast_array;
use crate::{impl_common_wrapper_traits, impl_js_cast, InvalidCast};

// TODO: the spec also allows just and end-mark (from navigation to named mark) by specifying
// `undefined` for the start mark, but web_sys's API currently does not allow this
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MarkRange<'a, 'b> {
    FromNavigation,
    From(&'a str),
    Between(&'a str, &'b str),
}

impl Default for MarkRange<'_, '_> {
    fn default() -> Self {
        MarkRange::FromNavigation
    }
}

#[derive(Clone)]
pub struct Performance {
    inner: web_sys::Performance,
}

impl Performance {
    delegate! {
        to self.inner {
            pub fn now(&self) -> f64;

            pub fn set_resource_timing_buffer_size(&self, max_size: u32);

            pub fn clear_marks(&self);

            pub fn clear_measures(&self);

            pub fn clear_resource_timings(&self);
        }
    }

    pub fn mark(&self, mark_name: &str) {
        // No clear indication in the spec that this can fail (there's a TypeError when invoking a
        // timestamp constructor with a negative timestamp, but I don't believe we could ever end up
        // creating a negative timestamp in this manner, barring a browser bug), unwrap for now.
        self.inner.mark(mark_name).unwrap_throw();
    }

    pub fn measure(&self, measure_name: &str, range: MarkRange) {
        // TODO: unwrap or return Error? The only error that should occur is an invalid mark name
        // (note: negative ranges are explicitly permitted), which *should* always indicate
        // programmer error, hence panic?

        match range {
            MarkRange::FromNavigation => {
                self.inner.measure(measure_name).unwrap_throw();
            }
            MarkRange::From(start_mark) => {
                self.inner
                    .measure_with_start_mark(measure_name, start_mark)
                    .unwrap_throw();
            }
            MarkRange::Between(start_mark, end_mark) => {
                self.inner
                    .measure_with_start_mark_and_end_mark(measure_name, start_mark, end_mark)
                    .unwrap_throw();
            }
        }
    }

    pub fn clear_marks_named(&self, mark_name: &str) {
        self.inner.clear_marks_with_mark_name(mark_name);
    }

    pub fn clear_measures_named(&self, measure_name: &str) {
        self.inner.clear_measures_with_measure_name(measure_name);
    }

    pub fn entries(&self) -> Entries {
        Entries::new(self.inner.get_entries())
    }

    pub fn marks(&self) -> Marks {
        Marks::new(self.inner.get_entries_by_type("mark"))
    }

    pub fn measures(&self) -> Measures {
        Measures::new(self.inner.get_entries_by_type("measure"))
    }

    pub fn resource_timings(&self) -> ResourceTimings {
        ResourceTimings::new(self.inner.get_entries_by_type("resource"))
    }

    pub fn entries_named(&self, name: &str) -> Entries {
        Entries::new(self.inner.get_entries_by_name(name))
    }

    pub fn marks_named(&self, name: &str) -> Marks {
        Marks::new(self.inner.get_entries_by_name_with_entry_type(name, "mark"))
    }

    pub fn measures_named(&self, name: &str) -> Measures {
        Measures::new(
            self.inner
                .get_entries_by_name_with_entry_type(name, "measure"),
        )
    }

    pub fn resource_timings_named(&self, name: &str) -> ResourceTimings {
        ResourceTimings::new(
            self.inner
                .get_entries_by_name_with_entry_type(name, "resource"),
        )
    }
}

impl From<web_sys::Performance> for Performance {
    fn from(inner: web_sys::Performance) -> Self {
        Performance { inner }
    }
}

impl AsRef<web_sys::Performance> for Performance {
    fn as_ref(&self) -> &web_sys::Performance {
        &self.inner
    }
}

impl_common_wrapper_traits!(Performance);
impl_js_cast!(Performance);

#[derive(Clone)]
pub struct Entry {
    inner: web_sys::PerformanceEntry,
}

impl From<web_sys::PerformanceEntry> for Entry {
    fn from(inner: web_sys::PerformanceEntry) -> Self {
        Entry { inner }
    }
}

impl From<Entry> for web_sys::PerformanceEntry {
    fn from(value: Entry) -> Self {
        value.inner
    }
}

impl AsRef<web_sys::PerformanceEntry> for Entry {
    fn as_ref(&self) -> &web_sys::PerformanceEntry {
        &self.inner
    }
}

impl_common_wrapper_traits!(Entry);
impl_js_cast!(Entry, PerformanceEntry);

unchecked_cast_array!(Entry, PerformanceEntry, Entries);

#[derive(Clone)]
pub struct Mark {
    inner: web_sys::PerformanceMark,
}

impl Mark {
    delegate! {
        to self.inner {
            pub fn name(&self) -> String;
        }
    }

    pub fn time(&self) -> f64 {
        self.inner.start_time()
    }
}

impl From<web_sys::PerformanceMark> for Mark {
    fn from(inner: web_sys::PerformanceMark) -> Self {
        Mark { inner }
    }
}

impl TryFrom<Entry> for Mark {
    type Error = InvalidCast<Entry, Mark>;

    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        let e: web_sys::PerformanceEntry = value.into();

        e.dyn_into::<web_sys::PerformanceMark>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast::new(e.into()))
    }
}

impl AsRef<web_sys::PerformanceMark> for Mark {
    fn as_ref(&self) -> &web_sys::PerformanceMark {
        &self.inner
    }
}

impl AsRef<web_sys::PerformanceEntry> for Mark {
    fn as_ref(&self) -> &web_sys::PerformanceEntry {
        self.inner.as_ref()
    }
}

impl_common_wrapper_traits!(Mark);
impl_js_cast!(Mark, PerformanceMark);

unchecked_cast_array!(Mark, PerformanceMark, Marks);

#[derive(Clone)]
pub struct Measure {
    inner: web_sys::PerformanceMeasure,
}

impl Measure {
    delegate! {
        to self.inner {
            pub fn name(&self) -> String;

            pub fn start_time(&self) -> f64;

            pub fn duration(&self) -> f64;
        }
    }
}

impl From<web_sys::PerformanceMeasure> for Measure {
    fn from(inner: web_sys::PerformanceMeasure) -> Self {
        Measure { inner }
    }
}

impl TryFrom<Entry> for Measure {
    type Error = InvalidCast<Entry, Measure>;

    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        let e: web_sys::PerformanceEntry = value.into();

        e.dyn_into::<web_sys::PerformanceMeasure>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast::new(e.into()))
    }
}

impl AsRef<web_sys::PerformanceMeasure> for Measure {
    fn as_ref(&self) -> &web_sys::PerformanceMeasure {
        &self.inner
    }
}

impl AsRef<web_sys::PerformanceEntry> for Measure {
    fn as_ref(&self) -> &web_sys::PerformanceEntry {
        self.inner.as_ref()
    }
}

impl_common_wrapper_traits!(Measure);
impl_js_cast!(Measure, PerformanceMeasure);

unchecked_cast_array!(Measure, PerformanceMeasure, Measures);

#[derive(Clone)]
pub struct ResourceTiming {
    inner: web_sys::PerformanceResourceTiming,
}

impl ResourceTiming {
    delegate! {
        to self.inner {
            pub fn name(&self) -> String;

            pub fn start_time(&self) -> f64;

            pub fn duration(&self) -> f64;

            pub fn initiator_type(&self) -> String;

            pub fn next_hop_protocol(&self) -> String;

            pub fn worker_start(&self) -> f64;

            pub fn redirect_start(&self) -> f64;

            pub fn redirect_end(&self) -> f64;

            pub fn fetch_start(&self) -> f64;

            pub fn domain_lookup_start(&self) -> f64;

            pub fn domain_lookup_end(&self) -> f64;

            pub fn connect_start(&self) -> f64;

            pub fn connect_end(&self) -> f64;

            pub fn secure_connection_start(&self) -> f64;

            pub fn request_start(&self) -> f64;

            pub fn response_start(&self) -> f64;

            pub fn response_end(&self) -> f64;
        }
    }

    pub fn transfer_size(&self) -> u64 {
        self.inner.transfer_size() as u64
    }

    pub fn encoded_body_size(&self) -> u64 {
        self.inner.encoded_body_size() as u64
    }

    pub fn decoded_body_size(&self) -> u64 {
        self.inner.decoded_body_size() as u64
    }

    // TODO: MDN seems to indicate that this may return null/undefined/SecurityError if not on a
    // secure connection (although the Server Timings spec gives no indication of this); needs
    // investigating.
    pub fn server_timings(&self) -> ServerTimings {
        ServerTimings::new(self.inner.server_timing())
    }
}

impl From<web_sys::PerformanceResourceTiming> for ResourceTiming {
    fn from(inner: web_sys::PerformanceResourceTiming) -> Self {
        ResourceTiming { inner }
    }
}

impl TryFrom<Entry> for ResourceTiming {
    type Error = InvalidCast<Entry, ResourceTiming>;

    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        let e: web_sys::PerformanceEntry = value.into();

        e.dyn_into::<web_sys::PerformanceResourceTiming>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast::new(e.into()))
    }
}

impl AsRef<web_sys::PerformanceResourceTiming> for ResourceTiming {
    fn as_ref(&self) -> &web_sys::PerformanceResourceTiming {
        &self.inner
    }
}

impl AsRef<web_sys::PerformanceEntry> for ResourceTiming {
    fn as_ref(&self) -> &web_sys::PerformanceEntry {
        self.inner.as_ref()
    }
}

impl_common_wrapper_traits!(ResourceTiming);
impl_js_cast!(ResourceTiming, PerformanceResourceTiming);

unchecked_cast_array!(ResourceTiming, PerformanceResourceTiming, ResourceTimings);

#[derive(Clone)]
pub struct ServerTiming {
    inner: web_sys::PerformanceServerTiming,
}

impl ServerTiming {
    delegate! {
        to self.inner {
            pub fn name(&self) -> String;

            pub fn description(&self) -> String;

            pub fn duration(&self) -> f64;
        }
    }
}

impl From<web_sys::PerformanceServerTiming> for ServerTiming {
    fn from(inner: web_sys::PerformanceServerTiming) -> Self {
        ServerTiming { inner }
    }
}

impl AsRef<web_sys::PerformanceServerTiming> for ServerTiming {
    fn as_ref(&self) -> &web_sys::PerformanceServerTiming {
        &self.inner
    }
}

impl_common_wrapper_traits!(ServerTiming);
impl_js_cast!(ServerTiming, PerformanceServerTiming);

unchecked_cast_array!(ServerTiming, PerformanceServerTiming, ServerTimings);
