use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::InvalidCast;

// TODO: the spec also allows just and end-mark (from navigation to named mark) by specifying
// `undefined` for the start mark, but web_sys's API currently does not allow this
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PerformanceMarkRange<'a, 'b> {
    FromNavigation,
    From(&'a str),
    Between(&'a str, &'b str),
}

impl Default for PerformanceMarkRange<'_, '_> {
    fn default() -> Self {
        PerformanceMarkRange::FromNavigation
    }
}

pub struct Performance {
    inner: web_sys::Performance,
}

impl Performance {
    delegate! {
        target self.inner {
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
        self.inner.mark(mark_name).unwrap();
    }

    pub fn measure(&self, measure_name: &str, range: PerformanceMarkRange) {
        // TODO: unwrap or return Error? The only error that should occur is an invalid mark name
        // (note: negative ranges are explicitly permitted), which *should* always indicate
        // programmer error, hence panic?

        match range {
            PerformanceMarkRange::FromNavigation => {
                self.inner.measure(measure_name).unwrap();
            }
            PerformanceMarkRange::From(start_mark) => {
                self.inner
                    .measure_with_start_mark(measure_name, start_mark)
                    .unwrap();
            }
            PerformanceMarkRange::Between(start_mark, end_mark) => {
                self.inner
                    .measure_with_start_mark_and_end_mark(measure_name, start_mark, end_mark)
                    .unwrap();
            }
        }
    }

    pub fn clear_marks_named(&self, mark_name: &str) {
        self.inner.clear_marks_with_mark_name(mark_name);
    }

    pub fn clear_measures_named(&self, measure_name: &str) {
        self.inner.clear_measures_with_measure_name(measure_name);
    }

    pub fn entries(&self) -> PerformanceEntries {
        PerformanceEntries {
            inner: self.inner.get_entries(),
        }
    }

    pub fn marks(&self) -> PerformanceMarks {
        PerformanceMarks {
            inner: self.inner.get_entries_by_type("mark"),
        }
    }

    pub fn measures(&self) -> PerformanceMeasures {
        PerformanceMeasures {
            inner: self.inner.get_entries_by_type("measure"),
        }
    }

    pub fn resource_timings(&self) -> PerformanceResourceTimings {
        PerformanceResourceTimings {
            inner: self.inner.get_entries_by_type("resource"),
        }
    }

    pub fn entries_named(&self, name: &str) -> PerformanceEntries {
        PerformanceEntries {
            inner: self.inner.get_entries_by_name(name),
        }
    }

    pub fn marks_named(&self, name: &str) -> PerformanceMarks {
        PerformanceMarks {
            inner: self.inner.get_entries_by_name_with_entry_type(name, "mark"),
        }
    }

    pub fn measures_named(&self, name: &str) -> PerformanceMeasures {
        PerformanceMeasures {
            inner: self
                .inner
                .get_entries_by_name_with_entry_type(name, "measure"),
        }
    }

    pub fn resource_timings_named(&self, name: &str) -> PerformanceResourceTimings {
        PerformanceResourceTimings {
            inner: self
                .inner
                .get_entries_by_name_with_entry_type(name, "resource"),
        }
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

pub struct PerformanceEntry {
    inner: web_sys::PerformanceEntry,
}

impl From<web_sys::PerformanceEntry> for PerformanceEntry {
    fn from(inner: web_sys::PerformanceEntry) -> Self {
        PerformanceEntry { inner }
    }
}

impl From<PerformanceEntry> for web_sys::PerformanceEntry {
    fn from(value: PerformanceEntry) -> Self {
        value.inner
    }
}

impl AsRef<web_sys::PerformanceEntry> for PerformanceEntry {
    fn as_ref(&self) -> &web_sys::PerformanceEntry {
        &self.inner
    }
}

pub struct PerformanceEntries {
    inner: js_sys::Array,
}

impl PerformanceEntries {
    pub fn get(&self, index: usize) -> Option<PerformanceEntry> {
        u32::try_from(index).ok().and_then(|index| {
            let e = self.inner.get(index);

            if e.is_undefined() {
                None
            } else {
                let e: web_sys::PerformanceEntry = e.unchecked_into();

                Some(e.into())
            }
        })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<PerformanceEntry> {
        self.get(0)
    }

    pub fn last(&self) -> Option<PerformanceEntry> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> PerformanceEntriesIter {
        PerformanceEntriesIter {
            performance_entries: self,
            current: 0,
        }
    }
}

impl IntoIterator for PerformanceEntries {
    type Item = PerformanceEntry;
    type IntoIter = PerformanceEntriesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        PerformanceEntriesIntoIter {
            performance_entries: self,
            current: 0,
        }
    }
}

pub struct PerformanceEntriesIter<'a> {
    performance_entries: &'a PerformanceEntries,
    current: usize,
}

impl<'a> Iterator for PerformanceEntriesIter<'a> {
    type Item = PerformanceEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.performance_entries.get(current)
    }
}

pub struct PerformanceEntriesIntoIter {
    performance_entries: PerformanceEntries,
    current: usize,
}

impl Iterator for PerformanceEntriesIntoIter {
    type Item = PerformanceEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.performance_entries.get(current)
    }
}

pub struct PerformanceMark {
    inner: web_sys::PerformanceMark,
}

impl PerformanceMark {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;
        }
    }

    pub fn time(&self) -> f64 {
        self.inner.start_time()
    }
}

impl From<web_sys::PerformanceMark> for PerformanceMark {
    fn from(inner: web_sys::PerformanceMark) -> Self {
        PerformanceMark { inner }
    }
}

impl TryFrom<PerformanceEntry> for PerformanceMark {
    type Error = InvalidCast<PerformanceEntry>;

    fn try_from(value: PerformanceEntry) -> Result<Self, Self::Error> {
        let e: web_sys::PerformanceEntry = value.into();

        e.dyn_into::<web_sys::PerformanceMark>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl AsRef<web_sys::PerformanceMark> for PerformanceMark {
    fn as_ref(&self) -> &web_sys::PerformanceMark {
        &self.inner
    }
}

impl AsRef<web_sys::PerformanceEntry> for PerformanceMark {
    fn as_ref(&self) -> &web_sys::PerformanceEntry {
        self.inner.as_ref()
    }
}

pub struct PerformanceMarks {
    inner: js_sys::Array,
}

impl PerformanceMarks {
    pub fn get(&self, index: usize) -> Option<PerformanceMark> {
        u32::try_from(index).ok().and_then(|index| {
            let e = self.inner.get(index);

            if e.is_undefined() {
                None
            } else {
                let e: web_sys::PerformanceMark = e.unchecked_into();

                Some(e.into())
            }
        })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<PerformanceMark> {
        self.get(0)
    }

    pub fn last(&self) -> Option<PerformanceMark> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> PerformanceMarksIter {
        PerformanceMarksIter {
            performance_marks: self,
            current: 0,
        }
    }
}

impl IntoIterator for PerformanceMarks {
    type Item = PerformanceMark;
    type IntoIter = PerformanceMarksIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        PerformanceMarksIntoIter {
            performance_marks: self,
            current: 0,
        }
    }
}

pub struct PerformanceMarksIter<'a> {
    performance_marks: &'a PerformanceMarks,
    current: usize,
}

impl<'a> Iterator for PerformanceMarksIter<'a> {
    type Item = PerformanceMark;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.performance_marks.get(current)
    }
}

pub struct PerformanceMarksIntoIter {
    performance_marks: PerformanceMarks,
    current: usize,
}

impl Iterator for PerformanceMarksIntoIter {
    type Item = PerformanceMark;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.performance_marks.get(current)
    }
}

pub struct PerformanceMeasure {
    inner: web_sys::PerformanceMeasure,
}

impl PerformanceMeasure {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn start_time(&self) -> f64;

            pub fn duration(&self) -> f64;
        }
    }
}

impl From<web_sys::PerformanceMeasure> for PerformanceMeasure {
    fn from(inner: web_sys::PerformanceMeasure) -> Self {
        PerformanceMeasure { inner }
    }
}

impl TryFrom<PerformanceEntry> for PerformanceMeasure {
    type Error = InvalidCast<PerformanceEntry>;

    fn try_from(value: PerformanceEntry) -> Result<Self, Self::Error> {
        let e: web_sys::PerformanceEntry = value.into();

        e.dyn_into::<web_sys::PerformanceMeasure>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl AsRef<web_sys::PerformanceMeasure> for PerformanceMeasure {
    fn as_ref(&self) -> &web_sys::PerformanceMeasure {
        &self.inner
    }
}

impl AsRef<web_sys::PerformanceEntry> for PerformanceMeasure {
    fn as_ref(&self) -> &web_sys::PerformanceEntry {
        self.inner.as_ref()
    }
}

pub struct PerformanceMeasures {
    inner: js_sys::Array,
}

impl PerformanceMeasures {
    pub fn get(&self, index: usize) -> Option<PerformanceMeasure> {
        u32::try_from(index).ok().and_then(|index| {
            let e = self.inner.get(index);

            if e.is_undefined() {
                None
            } else {
                let e: web_sys::PerformanceMeasure = e.unchecked_into();

                Some(e.into())
            }
        })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<PerformanceMeasure> {
        self.get(0)
    }

    pub fn last(&self) -> Option<PerformanceMeasure> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> PerformanceMeasuresIter {
        PerformanceMeasuresIter {
            performance_measures: self,
            current: 0,
        }
    }
}

impl IntoIterator for PerformanceMeasures {
    type Item = PerformanceMeasure;
    type IntoIter = PerformanceMeasuresIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        PerformanceMeasuresIntoIter {
            performance_measures: self,
            current: 0,
        }
    }
}

pub struct PerformanceMeasuresIter<'a> {
    performance_measures: &'a PerformanceMeasures,
    current: usize,
}

impl<'a> Iterator for PerformanceMeasuresIter<'a> {
    type Item = PerformanceMeasure;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.performance_measures.get(current)
    }
}

pub struct PerformanceMeasuresIntoIter {
    performance_measures: PerformanceMeasures,
    current: usize,
}

impl Iterator for PerformanceMeasuresIntoIter {
    type Item = PerformanceMeasure;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.performance_measures.get(current)
    }
}

pub struct PerformanceResourceTiming {
    inner: web_sys::PerformanceResourceTiming,
}

impl PerformanceResourceTiming {
    delegate! {
        target self.inner {
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

            // TODO: do these next 3 methods always return whole numbers? Perhaps u64 makes more
            // sense?

            pub fn transfer_size(&self) -> f64;

            pub fn encoded_body_size(&self) -> f64;

            pub fn decoded_body_size(&self) -> f64;
        }
    }

    // TODO: MDN seems to indicate that this may return null/undefined/SecurityError if not on a
    // secure connection (although the Server Timings spec gives no indication of this); needs
    // investigating.
    pub fn server_timings(&self) -> PerformanceServerTimings {
        PerformanceServerTimings {
            inner: self.inner.server_timing(),
        }
    }
}

impl From<web_sys::PerformanceResourceTiming> for PerformanceResourceTiming {
    fn from(inner: web_sys::PerformanceResourceTiming) -> Self {
        PerformanceResourceTiming { inner }
    }
}

impl TryFrom<PerformanceEntry> for PerformanceResourceTiming {
    type Error = InvalidCast<PerformanceEntry>;

    fn try_from(value: PerformanceEntry) -> Result<Self, Self::Error> {
        let e: web_sys::PerformanceEntry = value.into();

        e.dyn_into::<web_sys::PerformanceResourceTiming>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl AsRef<web_sys::PerformanceResourceTiming> for PerformanceResourceTiming {
    fn as_ref(&self) -> &web_sys::PerformanceResourceTiming {
        &self.inner
    }
}

impl AsRef<web_sys::PerformanceEntry> for PerformanceResourceTiming {
    fn as_ref(&self) -> &web_sys::PerformanceEntry {
        self.inner.as_ref()
    }
}

pub struct PerformanceResourceTimings {
    inner: js_sys::Array,
}

impl PerformanceResourceTimings {
    pub fn get(&self, index: usize) -> Option<PerformanceResourceTiming> {
        u32::try_from(index).ok().and_then(|index| {
            let e = self.inner.get(index);

            if e.is_undefined() {
                None
            } else {
                let e: web_sys::PerformanceResourceTiming = e.unchecked_into();

                Some(e.into())
            }
        })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<PerformanceResourceTiming> {
        self.get(0)
    }

    pub fn last(&self) -> Option<PerformanceResourceTiming> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> PerformanceResourceTimingsIter {
        PerformanceResourceTimingsIter {
            performance_resource_timings: self,
            current: 0,
        }
    }
}

impl IntoIterator for PerformanceResourceTimings {
    type Item = PerformanceResourceTiming;
    type IntoIter = PerformanceResourceTimingsIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        PerformanceResourceTimingsIntoIter {
            performance_resource_timings: self,
            current: 0,
        }
    }
}

pub struct PerformanceResourceTimingsIter<'a> {
    performance_resource_timings: &'a PerformanceResourceTimings,
    current: usize,
}

impl<'a> Iterator for PerformanceResourceTimingsIter<'a> {
    type Item = PerformanceResourceTiming;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.performance_resource_timings.get(current)
    }
}

pub struct PerformanceResourceTimingsIntoIter {
    performance_resource_timings: PerformanceResourceTimings,
    current: usize,
}

impl Iterator for PerformanceResourceTimingsIntoIter {
    type Item = PerformanceResourceTiming;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.performance_resource_timings.get(current)
    }
}

pub struct PerformanceServerTiming {
    inner: web_sys::PerformanceServerTiming,
}

impl PerformanceServerTiming {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn description(&self) -> String;

            pub fn duration(&self) -> f64;
        }
    }
}

impl From<web_sys::PerformanceServerTiming> for PerformanceServerTiming {
    fn from(inner: web_sys::PerformanceServerTiming) -> Self {
        PerformanceServerTiming { inner }
    }
}

impl AsRef<web_sys::PerformanceServerTiming> for PerformanceServerTiming {
    fn as_ref(&self) -> &web_sys::PerformanceServerTiming {
        &self.inner
    }
}

pub struct PerformanceServerTimings {
    inner: js_sys::Array,
}

impl PerformanceServerTimings {
    pub fn get(&self, index: usize) -> Option<PerformanceServerTiming> {
        u32::try_from(index).ok().and_then(|index| {
            let e = self.inner.get(index);

            if e.is_undefined() {
                None
            } else {
                let e: web_sys::PerformanceServerTiming = e.unchecked_into();

                Some(e.into())
            }
        })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<PerformanceServerTiming> {
        self.get(0)
    }

    pub fn last(&self) -> Option<PerformanceServerTiming> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> PerformanceServerTimingsIter {
        PerformanceServerTimingsIter {
            performance_server_timings: self,
            current: 0,
        }
    }
}

impl IntoIterator for PerformanceServerTimings {
    type Item = PerformanceServerTiming;
    type IntoIter = PerformanceServerTimingsIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        PerformanceServerTimingsIntoIter {
            performance_server_timings: self,
            current: 0,
        }
    }
}

pub struct PerformanceServerTimingsIter<'a> {
    performance_server_timings: &'a PerformanceServerTimings,
    current: usize,
}

impl<'a> Iterator for PerformanceServerTimingsIter<'a> {
    type Item = PerformanceServerTiming;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.performance_server_timings.get(current)
    }
}

pub struct PerformanceServerTimingsIntoIter {
    performance_server_timings: PerformanceServerTimings,
    current: usize,
}

impl Iterator for PerformanceServerTimingsIntoIter {
    type Item = PerformanceServerTiming;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.performance_server_timings.get(current)
    }
}
