use std::error::Error;
use std::fmt;
use wasm_bindgen::{JsError, JsValue};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Status {
    code: u16,
}

impl Status {
    pub const OK: Status = Status { code: 200 };

    pub const CREATED: Status = Status { code: 201 };

    pub const ACCEPTED: Status = Status { code: 202 };

    pub const NON_AUTHORITATIVE_INFORMATION: Status = Status { code: 203 };

    pub const NO_CONTENT: Status = Status { code: 204 };

    pub const RESET_CONTENT: Status = Status { code: 205 };

    pub const PARTIAL_CONTENT: Status = Status { code: 206 };

    pub const MULTI_STATUS: Status = Status { code: 207 };

    pub const ALREADY_REPORTED: Status = Status { code: 208 };

    pub const IM_USED: Status = Status { code: 226 };

    pub const MULTIPLE_CHOICE: Status = Status { code: 300 };

    pub const MOVED_PERMANENTLY: Status = Status { code: 301 };

    pub const FOUND: Status = Status { code: 302 };

    pub const SEE_OTHER: Status = Status { code: 303 };

    pub const NOT_MODIFIED: Status = Status { code: 304 };

    pub const TEMPORARY_REDIRECT: Status = Status { code: 307 };

    pub const PERMANENT_REDIRECT: Status = Status { code: 308 };

    pub const BAD_REQUEST: Status = Status { code: 400 };

    pub const UNAUTHORIZED: Status = Status { code: 401 };

    pub const PAYMENT_REQUIRED: Status = Status { code: 402 };

    pub const FORBIDDEN: Status = Status { code: 403 };

    pub const NOT_FOUND: Status = Status { code: 404 };

    pub const METHOD_NOT_ALLOWED: Status = Status { code: 405 };

    pub const NOT_ACCEPTABLE: Status = Status { code: 406 };

    pub const PROXY_AUTHENTICATION_REQUIRED: Status = Status { code: 407 };

    pub const REQUEST_TIMEOUT: Status = Status { code: 408 };

    pub const CONFLICT: Status = Status { code: 409 };

    pub const GONE: Status = Status { code: 410 };

    pub const LENGTH_REQUIRED: Status = Status { code: 411 };

    pub const PRECONDITION_FAILED: Status = Status { code: 412 };

    pub const PAYLOAD_TOO_LARGE: Status = Status { code: 413 };

    pub const URI_TOO_LONG: Status = Status { code: 414 };

    pub const UNSUPPORTED_MEDIA_TYPE: Status = Status { code: 415 };

    pub const RANGE_NOT_SATISFIABLE: Status = Status { code: 416 };

    pub const EXPECTATION_FAILED: Status = Status { code: 417 };

    pub const IM_A_TEAPOT: Status = Status { code: 418 };

    pub const MISDIRECTED_REQUEST: Status = Status { code: 421 };

    pub const UNPROCESSABLE_ENTITY: Status = Status { code: 422 };

    pub const LOCKED: Status = Status { code: 423 };

    pub const FAILED_DEPENDENCY: Status = Status { code: 424 };

    pub const TOO_EARLY: Status = Status { code: 425 };

    pub const UPGRADE_REQUIRED: Status = Status { code: 426 };

    pub const PRECONDITION_REQUIRED: Status = Status { code: 428 };

    pub const TOO_MANY_REQUESTS: Status = Status { code: 429 };

    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: Status = Status { code: 431 };

    pub const UNAVAILABLE_FOR_LEGAL_REASON: Status = Status { code: 451 };

    pub const INTERNAL_SERVER_ERROR: Status = Status { code: 500 };

    pub const NOT_IMPLEMENTED: Status = Status { code: 501 };

    pub const BAD_GATEWAY: Status = Status { code: 502 };

    pub const SERVICE_UNAVAILABLE: Status = Status { code: 503 };

    pub const GATEWAY_TIMEOUT: Status = Status { code: 504 };

    pub const HTTP_VERSION_NOT_SUPPORTED: Status = Status { code: 505 };

    pub const VARIANT_ALSO_NEGOTIATES: Status = Status { code: 506 };

    pub const INSUFFICIENT_STORAGE: Status = Status { code: 507 };

    pub const LOOP_DETECTED: Status = Status { code: 508 };

    pub const NOT_EXTENDED: Status = Status { code: 510 };

    pub const NETWORK_AUTHENTICATION_REQUIRED: Status = Status { code: 511 };
}

impl TryFrom<u16> for Status {
    type Error = StatusRangeError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        if code > 200 && code < 600 {
            Ok(Status { code })
        } else {
            Err(StatusRangeError { code })
        }
    }
}

impl From<Status> for u16 {
    fn from(status: Status) -> Self {
        status.code
    }
}

/// Error returned when initializing a [Status] with a code outside of the valid status code range
/// of `200..600`.
#[derive(Clone, Copy, PartialEq)]
pub struct StatusRangeError {
    code: u16,
}

impl StatusRangeError {
    /// The status code that triggered the error.
    pub fn code(&self) -> u16 {
        self.code
    }
}

impl fmt::Display for StatusRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}` is not a valid status code; must be in the range `200..600`",
            self.code
        )
    }
}

impl fmt::Debug for StatusRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for StatusRangeError {}

impl From<StatusRangeError> for JsValue {
    fn from(err: StatusRangeError) -> Self {
        JsError::from(err).into()
    }
}
