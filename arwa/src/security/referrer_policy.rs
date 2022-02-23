#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReferrerPolicy {
    Default,
    NoReferrer,
    NoReferrerWhenDowngrade,
    SameOrigin,
    Origin,
    StrictOrigin,
    OriginWhenCrossOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

impl ReferrerPolicy {
    pub(crate) fn from_web_sys(referrer_policy: web_sys::ReferrerPolicy) -> Self {
        match referrer_policy {
            web_sys::ReferrerPolicy::None => ReferrerPolicy::Default,
            web_sys::ReferrerPolicy::NoReferrer => ReferrerPolicy::NoReferrer,
            web_sys::ReferrerPolicy::NoReferrerWhenDowngrade => {
                ReferrerPolicy::NoReferrerWhenDowngrade
            }
            web_sys::ReferrerPolicy::Origin => ReferrerPolicy::Origin,
            web_sys::ReferrerPolicy::OriginWhenCrossOrigin => ReferrerPolicy::OriginWhenCrossOrigin,
            web_sys::ReferrerPolicy::UnsafeUrl => ReferrerPolicy::UnsafeUrl,
            web_sys::ReferrerPolicy::SameOrigin => ReferrerPolicy::SameOrigin,
            web_sys::ReferrerPolicy::StrictOrigin => ReferrerPolicy::StrictOrigin,
            web_sys::ReferrerPolicy::StrictOriginWhenCrossOrigin => {
                ReferrerPolicy::StrictOriginWhenCrossOrigin
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn to_web_sys(&self) -> web_sys::ReferrerPolicy {
        match self {
            ReferrerPolicy::Default => web_sys::ReferrerPolicy::None,
            ReferrerPolicy::NoReferrer => web_sys::ReferrerPolicy::NoReferrer,
            ReferrerPolicy::NoReferrerWhenDowngrade => {
                web_sys::ReferrerPolicy::NoReferrerWhenDowngrade
            }
            ReferrerPolicy::SameOrigin => web_sys::ReferrerPolicy::SameOrigin,
            ReferrerPolicy::Origin => web_sys::ReferrerPolicy::Origin,
            ReferrerPolicy::StrictOrigin => web_sys::ReferrerPolicy::StrictOrigin,
            ReferrerPolicy::OriginWhenCrossOrigin => web_sys::ReferrerPolicy::OriginWhenCrossOrigin,
            ReferrerPolicy::StrictOriginWhenCrossOrigin => {
                web_sys::ReferrerPolicy::StrictOriginWhenCrossOrigin
            }
            ReferrerPolicy::UnsafeUrl => web_sys::ReferrerPolicy::UnsafeUrl,
        }
    }

    pub(crate) fn from_str(policy: &str) -> Self {
        match policy {
            "no-referrer" => ReferrerPolicy::NoReferrer,
            "no-referrer-when-downgrade" => ReferrerPolicy::NoReferrerWhenDowngrade,
            "same-origin" => ReferrerPolicy::SameOrigin,
            "origin" => ReferrerPolicy::Origin,
            "strict-origin" => ReferrerPolicy::StrictOrigin,
            "origin-when-cross-origin" => ReferrerPolicy::OriginWhenCrossOrigin,
            "strict-origin-when-cross-origin" => ReferrerPolicy::StrictOriginWhenCrossOrigin,
            "unsafe-url" => ReferrerPolicy::UnsafeUrl,
            _ => ReferrerPolicy::Default,
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        match self {
            ReferrerPolicy::Default => "",
            ReferrerPolicy::NoReferrer => "no-referrer",
            ReferrerPolicy::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            ReferrerPolicy::SameOrigin => "same-origin",
            ReferrerPolicy::Origin => "origin",
            ReferrerPolicy::StrictOrigin => "strict-origin",
            ReferrerPolicy::OriginWhenCrossOrigin => "origin-when-cross-origin",
            ReferrerPolicy::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
            ReferrerPolicy::UnsafeUrl => "unsafe-url",
        }
    }
}

impl Default for ReferrerPolicy {
    fn default() -> Self {
        ReferrerPolicy::Default
    }
}
