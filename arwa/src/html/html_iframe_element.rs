use std::cell::{RefCell, RefMut};
use std::str::FromStr;

use delegate::delegate;

use crate::collection::{Collection, Sequence};
use crate::dom::{impl_try_from_element, DynamicDocument};
use crate::html::{impl_extendable_element, impl_html_element_traits, impl_known_element};
use crate::url::Url;
use crate::window::Window;

#[derive(Clone)]
pub struct HtmlIframeElement {
    inner: web_sys::HtmlIFrameElement,
}

impl HtmlIframeElement {
    delegate! {
        to self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, name: &str);

            pub fn srcdoc(&self) -> String;

            pub fn set_srcdoc(&self, srcdoc: &str);

            pub fn allow_payment_request(&self) -> bool;

            pub fn set_allow_payment_request(&self, allow_payment_request: bool);
        }
    }

    pub fn src(&self) -> Option<Url> {
        Url::parse(self.inner.src().as_ref()).ok()
    }

    pub fn set_src(&self, src: &Url) {
        self.inner.set_src(src.as_ref());
    }

    pub fn width(&self) -> Option<u32> {
        u32::from_str(&self.inner.width()).ok()
    }

    pub fn set_width(&self, width: Option<u32>) {
        self.inner
            .set_width(&width.map(|w| w.to_string()).unwrap_or(String::new()));
    }

    pub fn height(&self) -> Option<u32> {
        u32::from_str(&self.inner.height()).ok()
    }

    pub fn set_height(&self, height: Option<u32>) {
        self.inner
            .set_height(&height.map(|w| w.to_string()).unwrap_or(String::new()));
    }

    pub fn sandbox(&self) -> IframeSandbox {
        IframeSandbox::new(self.inner.sandbox())
    }

    pub fn content_document(&self) -> Option<DynamicDocument> {
        self.inner
            .content_document()
            .map(|document| document.into())
    }

    pub fn content_window(&self) -> Option<Window> {
        self.inner.content_window().map(|w| w.into())
    }
}

impl From<web_sys::HtmlIFrameElement> for HtmlIframeElement {
    fn from(inner: web_sys::HtmlIFrameElement) -> Self {
        HtmlIframeElement { inner }
    }
}

impl AsRef<web_sys::HtmlIFrameElement> for HtmlIframeElement {
    fn as_ref(&self) -> &web_sys::HtmlIFrameElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlIframeElement);
impl_try_from_element!(HtmlIframeElement, HtmlIFrameElement);
impl_known_element!(HtmlIframeElement, HtmlIFrameElement, "IFRAME");
impl_extendable_element!(HtmlIframeElement, "iframe");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IframeSandboxRule {
    AllowForms,
    AllowModals,
    AllowOrientationLock,
    AllowPointerLock,
    AllowPopups,
    AllowPopupsToEscapeSandbox,
    AllowPresentation,
    AllowSameOrigin,
    AllowScripts,
    AllowTopNavigation,
    AllowTopNavigationByUserActivation,
    AllowDownloads,
}

impl IframeSandboxRule {
    fn from_token(token: &str) -> Option<Self> {
        match token {
            "allow-forms" => Some(IframeSandboxRule::AllowForms),
            "allow-modals" => Some(IframeSandboxRule::AllowModals),
            "allow-orientation-lock" => Some(IframeSandboxRule::AllowOrientationLock),
            "allow-pointer-lock" => Some(IframeSandboxRule::AllowPointerLock),
            "allow-popups" => Some(IframeSandboxRule::AllowPopups),
            "allow-popups-to-escape-sandbox" => Some(IframeSandboxRule::AllowPopupsToEscapeSandbox),
            "allow-presentation" => Some(IframeSandboxRule::AllowPresentation),
            "allow-same-origin" => Some(IframeSandboxRule::AllowSameOrigin),
            "allow-scripts" => Some(IframeSandboxRule::AllowScripts),
            "allow-top-navigation" => Some(IframeSandboxRule::AllowTopNavigation),
            "allow-top-navigation-by-user-activation" => {
                Some(IframeSandboxRule::AllowTopNavigationByUserActivation)
            }
            "allow-downloads" => Some(IframeSandboxRule::AllowDownloads),
            _ => None,
        }
    }

    fn serialize(&self) -> &'static str {
        match self {
            IframeSandboxRule::AllowForms => "allow-forms",
            IframeSandboxRule::AllowModals => "allow-modals",
            IframeSandboxRule::AllowOrientationLock => "allow-orientation-lock",
            IframeSandboxRule::AllowPointerLock => "allow-pointer-lock",
            IframeSandboxRule::AllowPopups => "allow-popups",
            IframeSandboxRule::AllowPopupsToEscapeSandbox => "allow-popups-to-escape-sandbox",
            IframeSandboxRule::AllowPresentation => "allow-presentation",
            IframeSandboxRule::AllowSameOrigin => "allow-same-origin",
            IframeSandboxRule::AllowScripts => "allow-scripts",
            IframeSandboxRule::AllowTopNavigation => "allow-top-navigation",
            IframeSandboxRule::AllowTopNavigationByUserActivation => {
                "allow-top-navigation-by-user-activation"
            }
            IframeSandboxRule::AllowDownloads => "allow-downloads",
        }
    }
}

struct IframeSandboxCache {
    raw: String,
    parsed: Vec<IframeSandboxRule>,
}

impl IframeSandboxCache {
    fn refresh(&mut self, mut sandbox_string: String) {
        sandbox_string.make_ascii_lowercase();

        if self.raw != sandbox_string {
            let mut parsed_new = Vec::new();

            for token in sandbox_string.split_ascii_whitespace() {
                if let Some(rule) = IframeSandboxRule::from_token(token) {
                    if !parsed_new.iter().any(|r| r == &rule) {
                        parsed_new.push(rule);
                    }
                }
            }

            self.parsed = parsed_new;
            self.raw = sandbox_string;
        }
    }

    fn contains(&self, rule: IframeSandboxRule) -> bool {
        self.parsed.iter().any(|r| r == &rule)
    }

    fn serialize(&self) -> String {
        self.parsed
            .iter()
            .map(|r| r.serialize())
            .intersperse(" ")
            .collect()
    }
}

pub struct IframeSandbox {
    sandbox: web_sys::DomTokenList,
    cached: RefCell<IframeSandboxCache>,
}

impl IframeSandbox {
    fn new(sandbox: web_sys::DomTokenList) -> Self {
        IframeSandbox {
            sandbox,
            cached: RefCell::new(IframeSandboxCache {
                raw: String::new(),
                parsed: Vec::new(),
            }),
        }
    }

    fn refresh(&self) -> RefMut<IframeSandboxCache> {
        let mut cached = self.cached.borrow_mut();

        cached.refresh(self.sandbox.value());

        cached
    }

    pub fn contains(&self, rule: IframeSandboxRule) -> bool {
        self.refresh().contains(rule)
    }

    pub fn insert(&self, rule: IframeSandboxRule) -> bool {
        let mut cached = self.refresh();

        if !cached.contains(rule) {
            cached.parsed.push(rule);

            let new_sandbox = cached.serialize();

            self.sandbox.set_value(&new_sandbox);

            cached.raw = new_sandbox;

            true
        } else {
            false
        }
    }

    pub fn remove(&self, rule: IframeSandboxRule) -> bool {
        let mut cached = self.refresh();

        if cached.contains(rule) {
            cached.parsed.retain(|r| r != &rule);

            let new_sandbox = cached.serialize();

            self.sandbox.set_value(&new_sandbox);

            cached.raw = new_sandbox;

            true
        } else {
            false
        }
    }

    pub fn toggle(&self, rule: IframeSandboxRule) -> bool {
        let mut cached = self.refresh();

        let output = if cached.contains(rule) {
            cached.parsed.retain(|r| r != &rule);

            false
        } else {
            cached.parsed.push(rule);

            true
        };

        let new_sandbox = cached.serialize();

        self.sandbox.set_value(&new_sandbox);

        cached.raw = new_sandbox;

        output
    }

    pub fn replace(&self, old: IframeSandboxRule, new: IframeSandboxRule) -> bool {
        let mut cached = self.refresh();

        let mut did_replace = false;

        for rule in cached.parsed.iter_mut() {
            if *rule == old {
                *rule = new;

                did_replace = true;

                break;
            }
        }

        if did_replace {
            let new_sandbox = cached.serialize();

            self.sandbox.set_value(&new_sandbox);

            cached.raw = new_sandbox;

            true
        } else {
            false
        }
    }

    pub fn serialize(&self) -> String {
        self.refresh().serialize()
    }

    pub fn deserialize(&self, serialized: &str) {
        let serialized = serialized.to_string();

        let mut cached = self.cached.borrow_mut();

        cached.refresh(serialized);

        self.sandbox.set_value(cached.serialize().as_ref());
    }
}

impl Collection for IframeSandbox {
    fn len(&self) -> u32 {
        self.refresh().parsed.len() as u32
    }
}

impl Sequence for IframeSandbox {
    type Item = IframeSandboxRule;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.refresh().parsed.get(index as usize).copied()
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::JsString::from(self.refresh().serialize()).split(" ")
    }
}
