use std::fmt;
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

use pin_project::pin_project;
use delegate::delegate;
use wasm_bindgen_futures::JsFuture;
use js_sys::JsString;
use wasm_bindgen::{throw_val, JsCast, UnwrapThrowExt};

use crate::collection::{Collection, Sequence};
use crate::cssom::{
    impl_animation_event_target_for_element, impl_transition_event_target_for_element,
};
use crate::dom::{
    impl_child_node, impl_node_traits,impl_owned_node, impl_parent_node, impl_try_from_child_node,
    impl_try_from_node, impl_try_from_parent_node, range_bound_container_seal, Attribute, Name,
    NonColonName, RangeBoundContainer, Selector, Token,
};
use crate::dom_exception_wrapper;
use crate::impl_common_wrapper_traits;
use crate::scroll::{impl_scroll_into_view_for_element, impl_scrollable_for_element};
use crate::ui::impl_ui_event_target_for_element;
use crate::unchecked_cast_array::unchecked_cast_array;

dom_exception_wrapper!(InvalidPointerId);

pub(crate) mod element_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_element(&self) -> &web_sys::Element;
    }
}

pub trait Element: element_seal::Seal {
    // TODO: skip `attach_shadow` here, add it to the specific elements for which it is valid.

    // TODO: implement `request_full_screen` as a future.

    fn matches(&self, selector: &Selector) -> bool {
        self.as_web_sys_element()
            .matches(selector.as_ref())
            .unwrap_throw()
    }

    fn closest<T>(&self, selector: &Selector) -> Option<DynamicElement> {
        self.as_web_sys_element()
            .closest(selector.as_ref())
            .unwrap_throw()
            .map(|e| e.into())
    }

    fn set_pointer_capture(&self, pointer_id: i32) {
        if let Err(err) = self.as_web_sys_element().set_pointer_capture(pointer_id) {
            throw_val(err)
        }
    }

    fn try_set_pointer_capture(&self, pointer_id: i32) -> Result<(), InvalidPointerId> {
        self.as_web_sys_element()
            .set_pointer_capture(pointer_id)
            .map_err(|err| InvalidPointerId::new(err.unchecked_into()))
    }

    fn has_pointer_capture(&self, pointer_id: i32) -> bool {
        self.as_web_sys_element().has_pointer_capture(pointer_id)
    }

    fn release_pointer_capture(&self, pointer_id: i32) {
        self.as_web_sys_element()
            .release_pointer_capture(pointer_id)
            .unwrap_throw()
    }

    fn try_release_pointer_capture(&self, pointer_id: i32) -> Result<(), InvalidPointerId> {
        self.as_web_sys_element()
            .release_pointer_capture(pointer_id)
            .map_err(|err| InvalidPointerId::new(err.unchecked_into()))
    }

    fn request_pointer_lock(&self) {
        self.as_web_sys_element().request_pointer_lock();
    }

    fn try_request_pointer_lock(&self) -> Result<(), RequestPointerLockError> {
        todo!("web-sys does not currently return a Result")
    }

    fn request_fullscreen(&self) -> RequestFullscreen {
        todo!("web-sys incorrectly does not return a Promise")
        // let promise = self.as_web_sys_element().request_fullscreen();
        //
        // RequestFullscreen {
        //     inner: promise.into()
        // }
    }

    fn bounding_client_rect(&self) -> ClientRect {
        ClientRect {
            inner: self.as_web_sys_element().get_bounding_client_rect(),
        }
    }

    fn client_rects(&self) -> ClientRects {
        ClientRects {
            inner: self.as_web_sys_element().get_client_rects(),
        }
    }

    fn attributes(&self) -> Attributes {
        Attributes {
            element: self.as_web_sys_element().clone(),
            attributes: self.as_web_sys_element().attributes(),
        }
    }

    /// Returns a live collection of the set of class labels attached to the element.
    ///
    /// Reflects the value of the `class` attribute (see [class]) as a whitespace delimited set of
    /// unique class labels.
    fn class(&self) -> Class {
        Class {
            inner: self.as_web_sys_element().class_list(),
        }
    }

    fn tag_name(&self) -> Name {
        Name::trusted(self.as_web_sys_element().tag_name())
    }

    fn namespace_uri(&self) -> Option<String> {
        self.as_web_sys_element().namespace_uri()
    }

    fn local_name(&self) -> Option<NonColonName> {
        NonColonName::parse(self.as_web_sys_element().local_name().as_ref()).ok()
    }

    fn prefix(&self) -> Option<NonColonName> {
        self.as_web_sys_element()
            .prefix()
            .map(|n| NonColonName::trusted(n))
    }

    fn client_width(&self) -> i32 {
        self.as_web_sys_element().client_width()
    }

    fn client_height(&self) -> i32 {
        self.as_web_sys_element().client_height()
    }

    fn client_top(&self) -> i32 {
        self.as_web_sys_element().client_top()
    }

    fn client_left(&self) -> i32 {
        self.as_web_sys_element().client_left()
    }

    fn id(&self) -> String {
        self.as_web_sys_element().id()
    }

    fn set_id(&self, id: &str) {
        self.as_web_sys_element().set_id(id);
    }

    fn slot(&self) -> String {
        self.as_web_sys_element().slot()
    }

    fn set_slot(&self, slot: &str) {
        self.as_web_sys_element().set_slot(slot);
    }

    fn scroll_width(&self) -> i32 {
        self.as_web_sys_element().scroll_width()
    }

    fn scroll_height(&self) -> i32 {
        self.as_web_sys_element().scroll_height()
    }

    fn serialize_inner(&self) -> String {
        self.as_web_sys_element().inner_html()
    }

    fn deserialize_inner(&self, serialized: &str) {
        self.as_web_sys_element().set_inner_html(serialized);
    }

    fn serialize_outer(&self) -> String {
        self.as_web_sys_element().outer_html()
    }

    fn deserialize_outer(&self, serialized: &str) {
        self.as_web_sys_element().set_outer_html(serialized);
    }
}

dom_exception_wrapper!(RequestPointerLockError);
dom_exception_wrapper!(RequestFullscreenError);

#[pin_project]
pub struct RequestFullscreen {
    #[pin]
    inner: JsFuture
}

impl Future for RequestFullscreen {
    type Output = Result<(), RequestFullscreenError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().inner.poll(cx).map_ok(|_| ()).map_err(|err| RequestFullscreenError::new(err.unchecked_into()))
    }
}

pub struct Attributes {
    element: web_sys::Element,
    attributes: web_sys::NamedNodeMap,
}

impl Attributes {
    pub fn lookup(&self, name: &Name) -> Option<Attribute> {
        self.attributes
            .get_named_item(name.as_ref())
            .map(|a| Attribute::new(a))
    }

    pub fn lookup_namespaced(&self, local_name: &Name, namespace: &str) -> Option<Attribute> {
        self.attributes
            .get_named_item_ns(Some(namespace), local_name.as_ref())
            .map(|a| Attribute::new(a))
    }

    pub fn contains(&self, name: &Name) -> bool {
        self.element.has_attribute(name.as_ref())
    }

    pub fn contains_namespaced(&self, local_name: &Name, namespace: &str) -> bool {
        self.element
            .has_attribute_ns(Some(namespace), local_name.as_ref())
    }

    pub fn names(&self) -> AttributeNames {
        AttributeNames::new(self.element.get_attribute_names())
    }

    pub fn set(&self, name: &Name, value: &str) {
        self.element
            .set_attribute(name.as_ref(), value)
            .unwrap_throw();
    }

    pub fn set_namespaced(&self, name: &Name, namespace: &str, value: &str) {
        self.element
            .set_attribute_ns(Some(namespace), name.as_ref(), value)
            .unwrap_throw();
    }

    pub fn insert(&self, attribute: &Attribute) -> Option<Attribute> {
        match self.attributes.set_named_item(attribute.as_ref()) {
            Ok(attr) => attr.map(|a| a.into()),
            Err(err) => throw_val(err),
        }
    }

    pub fn try_insert(&self, attribute: &Attribute) -> Result<Option<Attribute>, InUseAttribute> {
        self.attributes
            .set_named_item(attribute.as_ref())
            .map(|ok| ok.map(|attr| attr.into()))
            .map_err(|err| InUseAttribute::new(err.unchecked_into()))
    }

    pub fn toggle(&self, name: &Name) -> bool {
        self.element.toggle_attribute(name.as_ref()).unwrap_throw()
    }

    pub fn toggle_on(&self, name: &Name) {
        self.element
            .toggle_attribute_with_force(name.as_ref(), true)
            .unwrap_throw();
    }

    pub fn toggle_off(&self, name: &Name) {
        self.element
            .toggle_attribute_with_force(name.as_ref(), false)
            .unwrap_throw();
    }

    pub fn remove(&self, name: &Name) -> Option<Attribute> {
        self.attributes
            .remove_named_item(name.as_ref())
            .ok()
            .map(|attr| Attribute::new(attr))
    }

    pub fn remove_namespaced(&self, local_name: &Name, namespace: &str) -> Option<Attribute> {
        self.attributes
            .remove_named_item_ns(Some(namespace), local_name.as_ref())
            .ok()
            .map(|attr| Attribute::new(attr))
    }
}

impl Collection for Attributes {
    fn len(&self) -> u32 {
        self.attributes.length()
    }
}

impl Sequence for Attributes {
    type Item = Attribute;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.attributes.item(index).map(|a| a.into())
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.attributes.as_ref())
    }
}

unchecked_cast_array!(String, JsString, AttributeNames);

pub struct Class {
    inner: web_sys::DomTokenList,
}

impl Class {
    pub fn contains(&self, class: &Token) -> bool {
        self.inner.contains(class.as_ref())
    }

    pub fn insert(&self, class: &Token) -> bool {
        if !self.contains(class) {
            self.inner
                .toggle_with_force(class.as_ref(), true)
                .unwrap_throw();

            true
        } else {
            false
        }
    }

    pub fn remove(&self, class: &Token) -> bool {
        if self.contains(class) {
            self.inner.remove_1(class.as_ref()).unwrap_throw();

            true
        } else {
            false
        }
    }

    pub fn toggle(&self, class: &Token) -> bool {
        self.inner.toggle(class.as_ref()).unwrap_throw()
    }

    pub fn replace(&self, old: &Token, new: &Token) -> bool {
        // It seems the error case covers old browser returning void instead of a bool, but I don't
        // believe there's any overlap between browsers that support WASM and browsers that still
        // return void, so this should never cause an error.
        self.inner
            .replace(old.as_ref(), new.as_ref())
            .unwrap_throw()
    }

    pub fn serialize(&self) -> String {
        self.to_string()
    }

    pub fn deserialize(&self, serialized: &str) {
        self.inner.set_value(serialized);
    }
}

impl Collection for Class {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for Class {
    type Item = Token;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.item(index).map(|t| Token::trusted(t))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

impl ToString for Class {
    fn to_string(&self) -> String {
        self.inner.value()
    }
}

pub struct ClientRect {
    inner: web_sys::DomRect,
}

impl ClientRect {
    delegate! {
        to self.inner {
            pub fn x(&self) -> f64;

            pub fn y(&self) -> f64;

            pub fn width(&self) -> f64;

            pub fn height(&self) -> f64;

            pub fn top(&self) -> f64;

            pub fn bottom(&self) -> f64;

            pub fn left(&self) -> f64;

            pub fn right(&self) -> f64;
        }
    }
}

impl From<web_sys::DomRect> for ClientRect {
    fn from(inner: web_sys::DomRect) -> Self {
        ClientRect { inner }
    }
}

impl AsRef<web_sys::DomRect> for ClientRect {
    fn as_ref(&self) -> &web_sys::DomRect {
        &self.inner
    }
}

impl_common_wrapper_traits!(ClientRect);

pub struct ClientRects {
    inner: web_sys::DomRectList,
}

impl Collection for ClientRects {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for ClientRects {
    type Item = ClientRect;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get(index).map(|r| r.into())
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

#[derive(Clone, PartialEq)]
pub struct DynamicElement {
    inner: web_sys::Element,
}

impl fmt::Debug for DynamicElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.inner.fmt(f)
    }
}

impl From<web_sys::Element> for DynamicElement {
    fn from(inner: web_sys::Element) -> Self {
        DynamicElement { inner }
    }
}

impl From<DynamicElement> for web_sys::Element {
    fn from(value: DynamicElement) -> Self {
        value.inner
    }
}

impl AsRef<web_sys::Element> for DynamicElement {
    fn as_ref(&self) -> &web_sys::Element {
        &self.inner
    }
}

impl element_seal::Seal for DynamicElement {
    fn as_web_sys_element(&self) -> &web_sys::Element {
        &self.inner
    }
}

impl Element for DynamicElement {}

impl range_bound_container_seal::Seal for DynamicElement {
    fn as_web_sys_node(&self) -> &web_sys::Node {
        self.inner.as_ref()
    }
}

impl RangeBoundContainer for DynamicElement {}

impl_node_traits!(DynamicElement);
impl_try_from_node!(DynamicElement, Element);
impl_parent_node!(DynamicElement);
impl_try_from_parent_node!(DynamicElement, Element);
impl_child_node!(DynamicElement);
impl_try_from_child_node!(DynamicElement, Element);
impl_owned_node!(DynamicElement);
impl_scrollable_for_element!(DynamicElement);
impl_scroll_into_view_for_element!(DynamicElement);
impl_ui_event_target_for_element!(DynamicElement);
impl_animation_event_target_for_element!(DynamicElement);
impl_transition_event_target_for_element!(DynamicElement);

dom_exception_wrapper!(InUseAttribute);

macro_rules! impl_element_traits {
    ($tpe:ident) => {
        impl $crate::dom::element_seal::Seal for $tpe {
            fn as_web_sys_element(&self) -> &web_sys::Element {
                &self.inner
            }
        }
        impl $crate::dom::Element for $tpe {}

        impl AsRef<web_sys::Element> for $tpe {
            fn as_ref(&self) -> &web_sys::Element {
                use crate::dom::element_seal::Seal;

                self.as_web_sys_element()
            }
        }

        impl $crate::dom::range_bound_container_seal::Seal for $tpe {
            fn as_web_sys_node(&self) -> &web_sys::Node {
                use crate::dom::element_seal::Seal;

                self.as_web_sys_element().as_ref()
            }
        }

        impl $crate::dom::RangeBoundContainer for $tpe {}

        impl From<$tpe> for $crate::dom::DynamicElement {
            fn from(element: $tpe) -> $crate::dom::DynamicElement {
                use wasm_bindgen::JsCast;

                $crate::dom::DynamicElement::from(element.inner.unchecked_into::<web_sys::Element>())
            }
        }

        impl From<$tpe> for $crate::dom::DynamicChildNode {
            fn from(element: $tpe) -> $crate::dom::DynamicChildNode {
                use wasm_bindgen::JsCast;

                $crate::dom::DynamicChildNode::new(element.inner.unchecked_into())
            }
        }

        impl From<$tpe> for $crate::dom::DynamicParentNode {
            fn from(element: $tpe) -> $crate::dom::DynamicParentNode {
                use wasm_bindgen::JsCast;

                $crate::dom::DynamicParentNode::new(element.inner.unchecked_into())
            }
        }

        $crate::dom::impl_node_traits!($tpe);
        $crate::dom::impl_parent_node!($tpe);
        $crate::dom::impl_child_node!($tpe);
        $crate::dom::impl_owned_node!($tpe);
        $crate::dom::impl_element_sibling_for_element!($tpe);
        $crate::scroll::impl_scrollable_for_element!($tpe);
        $crate::scroll::impl_scroll_into_view_for_element!($tpe);
        $crate::ui::impl_ui_event_target_for_element!($tpe);
        $crate::cssom::impl_animation_event_target_for_element!($tpe);
        $crate::cssom::impl_transition_event_target_for_element!($tpe);
    };
}

pub(crate) use impl_element_traits;

macro_rules! impl_try_from_element {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl TryFrom<$crate::dom::DynamicElement> for $tpe {
            type Error = $crate::InvalidCast<$crate::dom::DynamicElement, $tpe>;

            fn try_from(value: $crate::dom::DynamicElement) -> Result<Self, Self::Error> {
                use wasm_bindgen::JsCast;

                let value: web_sys::Element = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|e| e.into())
                    .map_err(|e| $crate::InvalidCast::new(e.into()))
            }
        }

        $crate::dom::impl_try_from_node!($tpe, $web_sys_tpe);
        $crate::dom::impl_try_from_child_node!($tpe, $web_sys_tpe);
        $crate::dom::impl_try_from_parent_node!($tpe, $web_sys_tpe);
    };
    ($tpe:ident) => {
        $crate::dom::impl_try_from_element!($tpe, $tpe);
    };
}

pub(crate) use impl_try_from_element;

macro_rules! impl_try_from_element_with_tag_check {
    ($tpe:ident, $web_sys_tpe:ident, $tag_name:literal) => {
        impl TryFrom<$crate::dom::DynamicElement> for $tpe {
            type Error = $crate::InvalidCast<$crate::dom::DynamicElement, $tpe>;

            fn try_from(value: $crate::dom::DynamicElement) -> Result<Self, Self::Error> {
                use wasm_bindgen::JsCast;

                let value: web_sys::Element = value.into();

                if value.tag_name().as_str() != $tag_name {
                    return Err($crate::InvalidCast::new($crate::dom::DynamicElement::from(
                        value,
                    )));
                }

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|inner| $tpe { inner })
                    .map_err(|e| $crate::InvalidCast::new($crate::dom::DynamicElement::from(e)))
            }
        }

        impl TryFrom<$crate::dom::DynamicNode> for $tpe {
            type Error = $crate::InvalidCast<$crate::dom::DynamicNode, $tpe>;

            fn try_from(value: $crate::dom::DynamicNode) -> Result<Self, Self::Error> {
                use wasm_bindgen::JsCast;

                let value: web_sys::Node = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map_err(|e| $crate::InvalidCast::new($crate::dom::DynamicNode::from(e)))
                    .and_then(|inner| {
                        if inner.tag_name().as_str() == $tag_name {
                            Ok($tpe { inner })
                        } else {
                            Err($crate::InvalidCast::new($crate::dom::DynamicNode::from(
                                inner.unchecked_into::<web_sys::Node>(),
                            )))
                        }
                    })
            }
        }

        impl TryFrom<$crate::event::DynamicEventTarget> for $tpe {
            type Error = $crate::InvalidCast<$crate::event::DynamicEventTarget, $tpe>;

            fn try_from(value: $crate::event::DynamicEventTarget) -> Result<Self, Self::Error> {
                use wasm_bindgen::JsCast;

                let value: web_sys::EventTarget = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map_err(|e| {
                        $crate::InvalidCast::new($crate::event::DynamicEventTarget::from(e))
                    })
                    .and_then(|inner| {
                        if inner.tag_name().as_str() == $tag_name {
                            Ok($tpe { inner })
                        } else {
                            Err($crate::InvalidCast::new(
                                $crate::event::DynamicEventTarget::from(
                                    inner.unchecked_into::<web_sys::EventTarget>(),
                                ),
                            ))
                        }
                    })
            }
        }
    };
}

pub(crate) use impl_try_from_element_with_tag_check;

