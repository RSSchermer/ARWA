use wasm_bindgen::JsValue;

#[doc(hidden)]
pub use web_sys::console as web_sys_console;

#[doc(hidden)]
pub use js_sys::Array as JsArray;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LogLevel {
    Log,
    Info,
    Debug,
    Warning,
    Error,
}

pub struct Writer {
    log_level: LogLevel,
}

impl Writer {
    pub fn new(log_level: LogLevel) -> Self {
        Writer { log_level }
    }

    pub fn log_level(&self) -> LogLevel {
        self.log_level
    }

    pub fn set_log_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level;
    }

    pub fn group(&self, label: Option<&str>) {
        if let Some(label) = label {
            web_sys::console::group_1(&label.into());
        } else {
            web_sys::console::group_0();
        }
    }

    pub fn group_collapsed(&self, label: Option<&str>) {
        if let Some(label) = label {
            web_sys::console::group_collapsed_1(&label.into());
        } else {
            web_sys::console::group_collapsed_0();
        }
    }

    pub fn group_end(&self) {
        web_sys::console::group_end();
    }

    pub fn write_1(&self, value: &JsValue) {
        match self.log_level {
            LogLevel::Log => self.log_1(value),
            LogLevel::Info => self.info_1(value),
            LogLevel::Debug => self.debug_1(value),
            LogLevel::Warning => self.warn_1(value),
            LogLevel::Error => self.error_1(value),
        }
    }

    pub fn write_2(&self, v0: &JsValue, v1: &JsValue) {
        match self.log_level {
            LogLevel::Log => self.log_2(v0, v1),
            LogLevel::Info => self.info_2(v0, v1),
            LogLevel::Debug => self.debug_2(v0, v1),
            LogLevel::Warning => self.warn_2(v0, v1),
            LogLevel::Error => self.error_2(v0, v1),
        }
    }

    pub fn write_3(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue) {
        match self.log_level {
            LogLevel::Log => self.log_3(v0, v1, v2),
            LogLevel::Info => self.info_3(v0, v1, v2),
            LogLevel::Debug => self.debug_3(v0, v1, v2),
            LogLevel::Warning => self.warn_3(v0, v1, v2),
            LogLevel::Error => self.error_3(v0, v1, v2),
        }
    }

    pub fn write_4(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue, v3: &JsValue) {
        match self.log_level {
            LogLevel::Log => self.log_4(v0, v1, v2, v3),
            LogLevel::Info => self.info_4(v0, v1, v2, v3),
            LogLevel::Debug => self.debug_4(v0, v1, v2, v3),
            LogLevel::Warning => self.warn_4(v0, v1, v2, v3),
            LogLevel::Error => self.error_4(v0, v1, v2, v3),
        }
    }

    pub fn write_5(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue, v3: &JsValue, v4: &JsValue) {
        match self.log_level {
            LogLevel::Log => self.log_5(v0, v1, v2, v3, v4),
            LogLevel::Info => self.info_5(v0, v1, v2, v3, v4),
            LogLevel::Debug => self.debug_5(v0, v1, v2, v3, v4),
            LogLevel::Warning => self.warn_5(v0, v1, v2, v3, v4),
            LogLevel::Error => self.error_5(v0, v1, v2, v3, v4),
        }
    }

    pub fn write_6(
        &self,
        v0: &JsValue,
        v1: &JsValue,
        v2: &JsValue,
        v3: &JsValue,
        v4: &JsValue,
        v5: &JsValue,
    ) {
        match self.log_level {
            LogLevel::Log => self.log_6(v0, v1, v2, v3, v4, v5),
            LogLevel::Info => self.info_6(v0, v1, v2, v3, v4, v5),
            LogLevel::Debug => self.debug_6(v0, v1, v2, v3, v4, v5),
            LogLevel::Warning => self.warn_6(v0, v1, v2, v3, v4, v5),
            LogLevel::Error => self.error_6(v0, v1, v2, v3, v4, v5),
        }
    }

    pub fn write_7(
        &self,
        v0: &JsValue,
        v1: &JsValue,
        v2: &JsValue,
        v3: &JsValue,
        v4: &JsValue,
        v5: &JsValue,
        v6: &JsValue,
    ) {
        match self.log_level {
            LogLevel::Log => self.log_7(v0, v1, v2, v3, v4, v5, v6),
            LogLevel::Info => self.info_7(v0, v1, v2, v3, v4, v5, v6),
            LogLevel::Debug => self.debug_7(v0, v1, v2, v3, v4, v5, v6),
            LogLevel::Warning => self.warn_7(v0, v1, v2, v3, v4, v5, v6),
            LogLevel::Error => self.error_7(v0, v1, v2, v3, v4, v5, v6),
        }
    }

    pub fn write_all(&self, values: &JsArray) {
        match self.log_level {
            LogLevel::Log => self.log_all(values),
            LogLevel::Info => self.info_all(values),
            LogLevel::Debug => self.debug_all(values),
            LogLevel::Warning => self.warn_all(values),
            LogLevel::Error => self.error_all(values),
        }
    }

    pub fn log_1(&self, value: &JsValue) {
        web_sys::console::log_1(value);
    }

    pub fn log_2(&self, v0: &JsValue, v1: &JsValue) {
        web_sys::console::log_2(v0, v1);
    }

    pub fn log_3(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue) {
        web_sys::console::log_3(v0, v1, v2);
    }

    pub fn log_4(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue, v3: &JsValue) {
        web_sys::console::log_4(v0, v1, v2, v3);
    }

    pub fn log_5(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue, v3: &JsValue, v4: &JsValue) {
        web_sys::console::log_5(v0, v1, v2, v3, v4);
    }

    pub fn log_6(
        &self,
        v0: &JsValue,
        v1: &JsValue,
        v2: &JsValue,
        v3: &JsValue,
        v4: &JsValue,
        v5: &JsValue,
    ) {
        web_sys::console::log_6(v0, v1, v2, v3, v4, v5);
    }

    pub fn log_7(
        &self,
        v0: &JsValue,
        v1: &JsValue,
        v2: &JsValue,
        v3: &JsValue,
        v4: &JsValue,
        v5: &JsValue,
        v6: &JsValue,
    ) {
        web_sys::console::log_7(v0, v1, v2, v3, v4, v5, v6);
    }

    pub fn log_all(&self, values: &JsArray) {
        web_sys::console::log(values);
    }

    pub fn info_1(&self, value: &JsValue) {
        web_sys::console::info_1(value);
    }

    pub fn info_2(&self, v0: &JsValue, v1: &JsValue) {
        web_sys::console::info_2(v0, v1);
    }

    pub fn info_3(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue) {
        web_sys::console::info_3(v0, v1, v2);
    }

    pub fn info_4(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue, v3: &JsValue) {
        web_sys::console::info_4(v0, v1, v2, v3);
    }

    pub fn info_5(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue, v3: &JsValue, v4: &JsValue) {
        web_sys::console::info_5(v0, v1, v2, v3, v4);
    }

    pub fn info_6(
        &self,
        v0: &JsValue,
        v1: &JsValue,
        v2: &JsValue,
        v3: &JsValue,
        v4: &JsValue,
        v5: &JsValue,
    ) {
        web_sys::console::info_6(v0, v1, v2, v3, v4, v5);
    }

    pub fn info_7(
        &self,
        v0: &JsValue,
        v1: &JsValue,
        v2: &JsValue,
        v3: &JsValue,
        v4: &JsValue,
        v5: &JsValue,
        v6: &JsValue,
    ) {
        web_sys::console::info_7(v0, v1, v2, v3, v4, v5, v6);
    }

    pub fn info_all(&self, values: &JsArray) {
        web_sys::console::info(values);
    }

    pub fn warn_1(&self, value: &JsValue) {
        web_sys::console::warn_1(value);
    }

    pub fn warn_2(&self, v0: &JsValue, v1: &JsValue) {
        web_sys::console::warn_2(v0, v1);
    }

    pub fn warn_3(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue) {
        web_sys::console::warn_3(v0, v1, v2);
    }

    pub fn warn_4(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue, v3: &JsValue) {
        web_sys::console::warn_4(v0, v1, v2, v3);
    }

    pub fn warn_5(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue, v3: &JsValue, v4: &JsValue) {
        web_sys::console::warn_5(v0, v1, v2, v3, v4);
    }

    pub fn warn_6(
        &self,
        v0: &JsValue,
        v1: &JsValue,
        v2: &JsValue,
        v3: &JsValue,
        v4: &JsValue,
        v5: &JsValue,
    ) {
        web_sys::console::warn_6(v0, v1, v2, v3, v4, v5);
    }

    pub fn warn_7(
        &self,
        v0: &JsValue,
        v1: &JsValue,
        v2: &JsValue,
        v3: &JsValue,
        v4: &JsValue,
        v5: &JsValue,
        v6: &JsValue,
    ) {
        web_sys::console::warn_7(v0, v1, v2, v3, v4, v5, v6);
    }

    pub fn warn_all(&self, values: &JsArray) {
        web_sys::console::warn(values);
    }

    pub fn error_1(&self, value: &JsValue) {
        web_sys::console::error_1(value);
    }

    pub fn error_2(&self, v0: &JsValue, v1: &JsValue) {
        web_sys::console::error_2(v0, v1);
    }

    pub fn error_3(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue) {
        web_sys::console::error_3(v0, v1, v2);
    }

    pub fn error_4(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue, v3: &JsValue) {
        web_sys::console::error_4(v0, v1, v2, v3);
    }

    pub fn error_5(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue, v3: &JsValue, v4: &JsValue) {
        web_sys::console::error_5(v0, v1, v2, v3, v4);
    }

    pub fn error_6(
        &self,
        v0: &JsValue,
        v1: &JsValue,
        v2: &JsValue,
        v3: &JsValue,
        v4: &JsValue,
        v5: &JsValue,
    ) {
        web_sys::console::error_6(v0, v1, v2, v3, v4, v5);
    }

    pub fn error_7(
        &self,
        v0: &JsValue,
        v1: &JsValue,
        v2: &JsValue,
        v3: &JsValue,
        v4: &JsValue,
        v5: &JsValue,
        v6: &JsValue,
    ) {
        web_sys::console::error_7(v0, v1, v2, v3, v4, v5, v6);
    }

    pub fn error_all(&self, values: &JsArray) {
        web_sys::console::error(values);
    }

    pub fn debug_1(&self, value: &JsValue) {
        web_sys::console::debug_1(value);
    }

    pub fn debug_2(&self, v0: &JsValue, v1: &JsValue) {
        web_sys::console::debug_2(v0, v1);
    }

    pub fn debug_3(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue) {
        web_sys::console::debug_3(v0, v1, v2);
    }

    pub fn debug_4(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue, v3: &JsValue) {
        web_sys::console::debug_4(v0, v1, v2, v3);
    }

    pub fn debug_5(&self, v0: &JsValue, v1: &JsValue, v2: &JsValue, v3: &JsValue, v4: &JsValue) {
        web_sys::console::debug_5(v0, v1, v2, v3, v4);
    }

    pub fn debug_6(
        &self,
        v0: &JsValue,
        v1: &JsValue,
        v2: &JsValue,
        v3: &JsValue,
        v4: &JsValue,
        v5: &JsValue,
    ) {
        web_sys::console::debug_6(v0, v1, v2, v3, v4, v5);
    }

    pub fn debug_7(
        &self,
        v0: &JsValue,
        v1: &JsValue,
        v2: &JsValue,
        v3: &JsValue,
        v4: &JsValue,
        v5: &JsValue,
        v6: &JsValue,
    ) {
        web_sys::console::debug_7(v0, v1, v2, v3, v4, v5, v6);
    }

    pub fn debug_all(&self, values: &JsArray) {
        web_sys::console::debug(values);
    }
}

pub trait Write {
    fn write(&self, writer: &mut Writer);
}

impl Write for JsValue {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self);
    }
}

impl Write for &'_ str {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(&(*self).into());
    }
}

impl Write for String {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(&self.into());
    }
}

impl Write for bool {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(&(*self).into());
    }
}

impl Write for i8 {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(&(*self).into());
    }
}

impl Write for u8 {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(&(*self).into());
    }
}

impl Write for i16 {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(&(*self).into());
    }
}

impl Write for u16 {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(&(*self).into());
    }
}

impl Write for i32 {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(&(*self).into());
    }
}

impl Write for u32 {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(&(*self).into());
    }
}

impl<T> Write for &'_ T
where
    T: Write,
{
    fn write(&self, writer: &mut Writer) {
        (*self).write(writer);
    }
}

// TODO: more core/std types? Derive macro?

#[doc(hidden)]
#[macro_export]
macro_rules! log {
    ($t:expr) => {
        {
            let mut writer = $crate::console::Writer::new($crate::console::LogLevel::Log);

            $crate::console::Write::write(&$t, &mut writer);
        }
    };
    ($t:tt, $($i:tt)+) => {
        {
            let formatted = format_args!($t, $($i)*).to_string();

            $crate::console::log!(formatted);
        }
    };
}

pub use crate::log;

#[doc(hidden)]
#[macro_export]
macro_rules! info {
    ($t:expr) => {
        {
            let mut writer = $crate::console::Writer::new($crate::console::LogLevel::Info);

            $crate::console::Write::write(&$t, &mut writer);
        }
    };
    ($t:tt, $($i:tt)+) => {
        {
            let formatted = format_args!($t, $($i)*).to_string();

            $crate::console::info!(formatted);
        }
    };
}

pub use crate::info;

#[doc(hidden)]
#[macro_export]
macro_rules! debug {
    ($t:expr) => {
        {
            let mut writer = $crate::console::Writer::new($crate::console::LogLevel::Debug);

            $crate::console::Write::write(&$t, &mut writer);
        }
    };
    ($t:tt, $($i:tt)+) => {
        {
            let formatted = format_args!($t, $($i)*).to_string();

            $crate::console::debug!(formatted);
        }
    };
}

pub use crate::debug;

#[doc(hidden)]
#[macro_export]
macro_rules! warn {
    ($t:expr) => {
        {
            let mut writer = $crate::console::Writer::new($crate::console::LogLevel::Warning);

            $crate::console::Write::write(&$t, &mut writer);
        }
    };
    ($t:tt, $($i:tt)+) => {
        {
            let formatted = format_args!($t, $($i)*).to_string();

            $crate::console::warn!(formatted);
        }
    };
}

pub use crate::warn;

#[doc(hidden)]
#[macro_export]
macro_rules! error {
    ($t:expr) => {
        {
            let mut writer = $crate::console::Writer::new($crate::console::LogLevel::Error);

            $crate::console::Write::write(&$t, &mut writer);
        }
    };
    ($t:tt, $($i:tt)+) => {
        {
            let formatted = format_args!($t, $($i)*).to_string();

            $crate::console::error!(formatted);
        }
    };
}

pub use crate::error;
