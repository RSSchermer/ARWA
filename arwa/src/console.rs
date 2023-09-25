#[doc(hidden)]
pub use js_sys::Array as JsArray;
use wasm_bindgen::JsValue;
#[doc(hidden)]
pub use web_sys::console as web_sys_console;

pub use crate::{assert, debug, error, group, group_collapsed, info, log, time_log, trace, warn};

enum ArgumentInternal<'a> {
    Owned(JsValue),
    Borrowed(&'a JsValue),
}

pub struct Argument<'a> {
    internal: ArgumentInternal<'a>,
}

impl<'a> Argument<'a> {
    fn owned(js_value: JsValue) -> Self {
        Argument {
            internal: ArgumentInternal::Owned(js_value),
        }
    }

    fn borrowed(js_value: &'a JsValue) -> Self {
        Argument {
            internal: ArgumentInternal::Borrowed(js_value),
        }
    }
}

impl Argument<'_> {
    #[doc(hidden)]
    pub fn as_js_value(&self) -> &JsValue {
        match &self.internal {
            ArgumentInternal::Owned(js_value) => js_value,
            ArgumentInternal::Borrowed(js_value) => js_value,
        }
    }
}

pub trait ToArgument {
    fn to_argument(&self) -> Argument;
}

impl ToArgument for JsValue {
    fn to_argument(&self) -> Argument {
        Argument::borrowed(self)
    }
}

impl ToArgument for str {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from_str(self))
    }
}

impl ToArgument for String {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(self))
    }
}

impl ToArgument for i8 {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for i16 {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for i32 {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for i64 {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for i128 {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for isize {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for u8 {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for u16 {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for u32 {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for u64 {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for u128 {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for usize {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for f32 {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for f64 {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl ToArgument for bool {
    fn to_argument(&self) -> Argument {
        Argument::owned(JsValue::from(*self))
    }
}

impl<'a, T> ToArgument for &'a T
where
    T: ToArgument + ?Sized,
{
    fn to_argument(&self) -> Argument {
        ToArgument::to_argument(*self)
    }
}

impl<'a, T> ToArgument for &'a mut T
where
    T: ToArgument + ?Sized,
{
    fn to_argument(&self) -> Argument {
        ToArgument::to_argument(*self)
    }
}

pub fn clear() {
    web_sys_console::clear();
}

pub fn group_end() {
    web_sys_console::group_end();
}

pub fn count(label: &str) {
    web_sys_console::count_with_label(label);
}

pub fn count_reset(label: &str) {
    web_sys_console::count_reset_with_label(label);
}

pub fn dir<T>(value: T)
where
    T: ToArgument,
{
    web_sys_console::dir_1(value.to_argument().as_js_value());
}

pub fn dir_xml<T>(value: T)
where
    T: ToArgument,
{
    web_sys_console::dirxml_1(value.to_argument().as_js_value());
}

pub fn time(label: &str) {
    web_sys_console::time_with_label(label);
}

pub fn time_end(label: &str) {
    web_sys_console::time_end_with_label(label);
}

#[doc(hidden)]
#[macro_export]
macro_rules! log {
    ($v:expr $(,)?) => {
        $crate::console::web_sys_console::log_1($crate::console::ToArgument::to_argument(&$v).as_js_value());
    };
    ($v0:expr, $v1:expr $(,)?) => {
        $crate::console::web_sys_console::log_2(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr $(,)?) => {
        $crate::console::web_sys_console::log_3(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr $(,)?) => {
        $crate::console::web_sys_console::log_4(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr $(,)?) => {
        $crate::console::web_sys_console::log_5(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr $(,)?) => {
        $crate::console::web_sys_console::log_6(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr $(,)?) => {
        $crate::console::web_sys_console::log_7(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v6).as_js_value(),
        );
    };
    ($($v:expr),* $(,)?) => {
        let array = $crate::console::JsArray::new();

        $(array.push(&$crate::console::ToArgument::to_argument(&$v).as_js_value());)*

        $crate::console::web_sys_console::log(&array);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! info {
    ($v:expr $(,)?) => {
        $crate::console::web_sys_console::info_1($crate::console::ToArgument::to_argument(&$v).as_js_value());
    };
    ($v0:expr, $v1:expr $(,)?) => {
        $crate::console::web_sys_console::info_2(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr $(,)?) => {
        $crate::console::web_sys_console::info_3(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr $(,)?) => {
        $crate::console::web_sys_console::info_4(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr $(,)?) => {
        $crate::console::web_sys_console::info_5(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr $(,)?) => {
        $crate::console::web_sys_console::info_6(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr $(,)?) => {
        $crate::console::web_sys_console::info_7(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v6).as_js_value(),
        );
    };
    ($($v:expr),* $(,)?) => {
        let array = $crate::console::JsArray::new();

        $(array.push(&$crate::console::ToArgument::to_argument(&$v).as_js_value());)*

        $crate::console::web_sys_console::info(&array);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! warn {
    ($v:expr $(,)?) => {
        $crate::console::web_sys_console::warn_1($crate::console::ToArgument::to_argument(&$v).as_js_value());
    };
    ($v0:expr, $v1:expr $(,)?) => {
        $crate::console::web_sys_console::warn_2(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr $(,)?) => {
        $crate::console::web_sys_console::warn_3(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr $(,)?) => {
        $crate::console::web_sys_console::warn_4(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr $(,)?) => {
        $crate::console::web_sys_console::warn_5(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr $(,)?) => {
        $crate::console::web_sys_console::warn_6(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr $(,)?) => {
        $crate::console::web_sys_console::warn_7(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v6).as_js_value(),
        );
    };
    ($($v:expr),* $(,)?) => {
        let array = $crate::console::JsArray::new();

        $(array.push(&$crate::console::ToArgument::to_argument(&$v).as_js_value());)*

        $crate::console::web_sys_console::warn(&array);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! error {
    ($v:expr $(,)?) => {
        $crate::console::web_sys_console::error_1($crate::console::ToArgument::to_argument(&$v).as_js_value());
    };
    ($v0:expr, $v1:expr $(,)?) => {
        $crate::console::web_sys_console::error_2(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr $(,)?) => {
        $crate::console::web_sys_console::error_3(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr $(,)?) => {
        $crate::console::web_sys_console::error_4(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr $(,)?) => {
        $crate::console::web_sys_console::error_5(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr $(,)?) => {
        $crate::console::web_sys_console::error_6(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr $(,)?) => {
        $crate::console::web_sys_console::error_7(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v6).as_js_value(),
        );
    };
    ($($v:expr),* $(,)?) => {
        let array = $crate::console::JsArray::new();

        $(array.push(&$crate::console::ToArgument::to_argument(&$v).as_js_value());)*

        $crate::console::web_sys_console::error(&array);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! debug {
    ($v:expr $(,)?) => {
        $crate::console::web_sys_console::debug_1($crate::console::ToArgument::to_argument(&$v).as_js_value());
    };
    ($v0:expr, $v1:expr $(,)?) => {
        $crate::console::web_sys_console::debug_2(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr $(,)?) => {
        $crate::console::web_sys_console::debug_3(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr $(,)?) => {
        $crate::console::web_sys_console::debug_4(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr $(,)?) => {
        $crate::console::web_sys_console::debug_5(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr $(,)?) => {
        $crate::console::web_sys_console::debug_6(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr $(,)?) => {
        $crate::console::web_sys_console::debug_7(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v6).as_js_value(),
        );
    };
    ($($v:expr),* $(,)?) => {
        let array = $crate::console::JsArray::new();

        $(array.push(&$crate::console::ToArgument::to_argument(&$v).as_js_value());)*

        $crate::console::web_sys_console::debug(&array);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! group {
    () => {
        $crate::console::web_sys_console::group_0();
    };
    ($v:expr $(,)?) => {
        $crate::console::web_sys_console::group_1($crate::console::ToArgument::to_argument(&$v).as_js_value());
    };
    ($v0:expr, $v1:expr $(,)?) => {
        $crate::console::web_sys_console::group_2(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr $(,)?) => {
        $crate::console::web_sys_console::group_3(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr $(,)?) => {
        $crate::console::web_sys_console::group_4(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr $(,)?) => {
        $crate::console::web_sys_console::group_5(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr $(,)?) => {
        $crate::console::web_sys_console::group_6(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr $(,)?) => {
        $crate::console::web_sys_console::group_7(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v6).as_js_value(),
        );
    };
    ($($v:expr),* $(,)?) => {
        let array = $crate::console::JsArray::new();

        $(array.push(&$crate::console::ToArgument::to_argument(&$v).as_js_value());)*

        $crate::console::web_sys_console::group(&array);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! group_collapsed {
    () => {
        $crate::console::web_sys_console::group_collapsed_0();
    };
    ($v:expr $(,)?) => {
        $crate::console::web_sys_console::group_collapsed_1($crate::console::ToArgument::to_argument(&$v).as_js_value());
    };
    ($v0:expr, $v1:expr $(,)?) => {
        $crate::console::web_sys_console::group_collapsed_2(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr $(,)?) => {
        $crate::console::web_sys_console::group_collapsed_3(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr $(,)?) => {
        $crate::console::web_sys_console::group_collapsed_4(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr $(,)?) => {
        $crate::console::web_sys_console::group_collapsed_5(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr $(,)?) => {
        $crate::console::web_sys_console::group_collapsed_6(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr $(,)?) => {
        $crate::console::web_sys_console::group_collapsed_7(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v6).as_js_value(),
        );
    };
    ($($v:expr),* $(,)?) => {
        let array = $crate::console::JsArray::new();

        $(array.push(&$crate::console::ToArgument::to_argument(&$v).as_js_value());)*

        $crate::console::web_sys_console::group_collapsed(&array);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! trace {
    () => {
        $crate::console::web_sys_console::trace_0();
    };
    ($v:expr $(,)?) => {
        $crate::console::web_sys_console::trace_1($crate::console::ToArgument::to_argument(&$v).as_js_value());
    };
    ($v0:expr, $v1:expr $(,)?) => {
        $crate::console::web_sys_console::trace_2(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr $(,)?) => {
        $crate::console::web_sys_console::trace_3(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr $(,)?) => {
        $crate::console::web_sys_console::trace_4(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr $(,)?) => {
        $crate::console::web_sys_console::trace_5(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr $(,)?) => {
        $crate::console::web_sys_console::trace_6(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
        );
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr $(,)?) => {
        $crate::console::web_sys_console::trace_7(
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v6).as_js_value(),
        );
    };
    ($($v:expr),* $(,)?) => {
        let array = $crate::console::JsArray::new();

        $(array.push(&$crate::console::ToArgument::to_argument(&$v).as_js_value());)*

        $crate::console::web_sys_console::trace(&array);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert {
    ($condition:expr) => {
        $crate::console::web_sys_console::assert_with_condition_and_data_0($condition);
    };
    ($condition:expr, $v:expr $(,)?) => {
        $crate::console::web_sys_console::assert_with_condition_and_data_1(
            $condition,
            $crate::console::ToArgument::to_argument(&$v).as_js_value()
        );
    };
    ($condition:expr, $v0:expr, $v1:expr $(,)?) => {
        $crate::console::web_sys_console::assert_with_condition_and_data_2(
            $condition,
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
        );
    };
    ($condition:expr, $v0:expr, $v1:expr, $v2:expr $(,)?) => {
        $crate::console::web_sys_console::assert_with_condition_and_data_3(
            $condition,
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
        );
    };
    ($condition:expr, $v0:expr, $v1:expr, $v2:expr, $v3:expr $(,)?) => {
        $crate::console::web_sys_console::assert_with_condition_and_data_4(
            $condition,
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
        );
    };
    ($condition:expr, $v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr $(,)?) => {
        $crate::console::web_sys_console::assert_with_condition_and_data_5(
            $condition,
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
        );
    };
    ($condition:expr, $v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr $(,)?) => {
        $crate::console::web_sys_console::assert_with_condition_and_data_6(
            $condition,
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
        );
    };
    ($condition:expr, $v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr $(,)?) => {
        $crate::console::web_sys_console::assert_with_condition_and_data_7(
            $condition,
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v6).as_js_value(),
        );
    };
    ($condition:expr, $($v:expr),* $(,)?) => {
        let array = $crate::console::JsArray::new();

        $(array.push(&$crate::console::ToArgument::to_argument(&$v).as_js_value());)*

        $crate::console::web_sys_console::assert_with_condition_and_data($condition, &array);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! time_log {
    ($label:literal) => {
        $crate::console::web_sys_console::time_log_with_label_and_data_0($label);
    };
    ($label:literal, $v:expr $(,)?) => {
        $crate::console::web_sys_console::time_log_with_label_and_data_1(
            $label,
            $crate::console::ToArgument::to_argument(&$v).as_js_value()
        );
    };
    ($label:literal, $v0:expr, $v1:expr $(,)?) => {
        $crate::console::web_sys_console::time_log_with_label_and_data_2(
            $label,
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
        );
    };
    ($label:literal, $v0:expr, $v1:expr, $v2:expr $(,)?) => {
        $crate::console::web_sys_console::time_log_with_label_and_data_3(
            $label,
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
        );
    };
    ($label:literal, $v0:expr, $v1:expr, $v2:expr, $v3:expr $(,)?) => {
        $crate::console::web_sys_console::time_log_with_label_and_data_4(
            $label,
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
        );
    };
    ($label:literal, $v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr $(,)?) => {
        $crate::console::web_sys_console::time_log_with_label_and_data_5(
            $label,
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
        );
    };
    ($label:literal, $v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr $(,)?) => {
        $crate::console::web_sys_console::time_log_with_label_and_data_6(
            $label,
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
        );
    };
    ($label:literal, $v0:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr, $v6:expr $(,)?) => {
        $crate::console::web_sys_console::time_log_with_label_and_data_7(
            $label,
            $crate::console::ToArgument::to_argument(&$v0).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v1).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v2).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v3).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v4).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v5).as_js_value(),
            $crate::console::ToArgument::to_argument(&$v6).as_js_value(),
        );
    };
    ($label:literal, $($v:expr),* $(,)?) => {
        let array = $crate::console::JsArray::new();

        $(array.push(&$crate::console::ToArgument::to_argument(&$v).as_js_value());)*

        $crate::console::web_sys_console::time_log_with_label_and_data($condition, &array);
    };
}
