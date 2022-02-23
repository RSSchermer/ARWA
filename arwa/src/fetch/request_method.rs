use std::fmt;

pub use arwa_parse::request_method::InvalidRequestMethod;

#[doc(hidden)]
pub struct StaticallyParsedRequestMethod {
    #[doc(hidden)]
    pub request_method: &'static str,
}

impl AsRef<str> for StaticallyParsedRequestMethod {
    fn as_ref(&self) -> &str {
        self.request_method
    }
}

enum RequestMethodInternal {
    Static(StaticallyParsedRequestMethod),
    Dynamic(arwa_parse::request_method::RequestMethod),
}

pub struct RequestMethod {
    internal: RequestMethodInternal,
}

impl RequestMethod {
    pub const GET: RequestMethod = RequestMethod {
        internal: RequestMethodInternal::Static(StaticallyParsedRequestMethod {
            request_method: "GET",
        }),
    };

    pub const HEAD: RequestMethod = RequestMethod {
        internal: RequestMethodInternal::Static(StaticallyParsedRequestMethod {
            request_method: "HEAD",
        }),
    };

    pub const OPTIONS: RequestMethod = RequestMethod {
        internal: RequestMethodInternal::Static(StaticallyParsedRequestMethod {
            request_method: "OPTIONS",
        }),
    };

    pub const POST: RequestMethod = RequestMethod {
        internal: RequestMethodInternal::Static(StaticallyParsedRequestMethod {
            request_method: "POST",
        }),
    };

    pub const PUT: RequestMethod = RequestMethod {
        internal: RequestMethodInternal::Static(StaticallyParsedRequestMethod {
            request_method: "PUT",
        }),
    };

    pub const PATCH: RequestMethod = RequestMethod {
        internal: RequestMethodInternal::Static(StaticallyParsedRequestMethod {
            request_method: "PATCH",
        }),
    };

    pub const CONNECT: RequestMethod = RequestMethod {
        internal: RequestMethodInternal::Static(StaticallyParsedRequestMethod {
            request_method: "CONNECT",
        }),
    };

    pub const TRACE: RequestMethod = RequestMethod {
        internal: RequestMethodInternal::Static(StaticallyParsedRequestMethod {
            request_method: "TRACE",
        }),
    };

    pub fn parse(token: &str) -> Result<Self, InvalidRequestMethod> {
        arwa_parse::request_method::RequestMethod::parse(token).map(|method| RequestMethod {
            internal: RequestMethodInternal::Dynamic(method),
        })
    }

    #[doc(hidden)]
    pub fn from_statically_parsed(token: StaticallyParsedRequestMethod) -> Self {
        RequestMethod {
            internal: RequestMethodInternal::Static(token),
        }
    }

    pub(crate) fn trusted(request_method: String) -> Self {
        RequestMethod {
            internal: RequestMethodInternal::Dynamic(
                arwa_parse::request_method::RequestMethod::trusted(request_method),
            ),
        }
    }
}

impl Default for RequestMethod {
    fn default() -> Self {
        RequestMethod::GET
    }
}

impl AsRef<str> for RequestMethod {
    fn as_ref(&self) -> &str {
        match &self.internal {
            RequestMethodInternal::Static(request_method) => request_method.as_ref(),
            RequestMethodInternal::Dynamic(request_method) => request_method.as_ref(),
        }
    }
}

impl PartialEq for RequestMethod {
    fn eq(&self, other: &Self) -> bool {
        let self_as_str: &str = self.as_ref();
        let other_as_str: &str = other.as_ref();

        self_as_str == other_as_str
    }
}

impl PartialEq<str> for RequestMethod {
    fn eq(&self, other: &str) -> bool {
        let self_as_str: &str = self.as_ref();

        self_as_str == other
    }
}

impl PartialEq<RequestMethod> for str {
    fn eq(&self, other: &RequestMethod) -> bool {
        let other_as_str: &str = other.as_ref();

        self == other_as_str
    }
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Display::fmt(as_str, f)
    }
}

impl fmt::Debug for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Debug::fmt(as_str, f)
    }
}
