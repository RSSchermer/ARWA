use std::convert::TryFrom;
use std::fmt;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::collection::{Collection, Sequence};
use crate::console::{Write, Writer};
use crate::dom::attribute::Attribute;
use crate::dom::selector::{CompiledSelector, Selector};
use crate::dom::InvalidAttributeName;
use crate::error::SyntaxError;
use crate::event::GenericEventTarget;
use crate::{
    DynamicNode, GlobalEventHandlers, InvalidCast, InvalidPointerId, Node, PointerId,
    QuerySelectorAll, ScrollByOptions, ScrollIntoViewOptions, ScrollToOptions,
};
use std::fmt::Formatter;

pub(crate) mod element_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_element(&self) -> &web_sys::Element;
    }
}

pub trait Element: element_seal::Seal {
    // TODO: skip `attach_shadow` here, add it to the specific elements for which it is valid.

    // TODO: implement `request_full_screen` as a future.

    fn matches(&self, selector: &CompiledSelector) -> bool {
        self.as_web_sys_element()
            .matches(selector.as_ref())
            .unwrap_throw()
    }

    fn closest<T>(&self, selector: &CompiledSelector) -> Option<DynamicElement> {
        self.as_web_sys_element()
            .closest(selector)
            .unwrap_throw()
            .into()
    }

    fn set_pointer_capture(&self, pointer_id: PointerId) -> Result<(), InvalidPointerId> {
        self.as_web_sys_element()
            .set_pointer_capture(pointer_id.into())
            .map_err(|_| InvalidPointerId(pointer_id))
    }

    fn has_pointer_capture(&self, pointer_id: PointerId) -> bool {
        self.as_web_sys_element()
            .has_pointer_capture(pointer_id.into())
    }

    fn release_pointer_capture(&self, pointer_id: PointerId) -> Result<(), InvalidPointerId> {
        self.as_web_sys_element()
            .release_pointer_capture(pointer_id.into())
            .map_err(|_| InvalidPointerId(pointer_id))
    }

    fn request_pointer_lock(&self) {
        self.as_web_sys_element().request_pointer_lock();
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
            element: self.as_web_sys_element(),
            attributes: self.as_web_sys_element().attributes(),
        }
    }

    /// Returns a live collection of the current classes attached to the element.
    ///
    /// Reflects the value of the `class` attribute (see [class]) as a whitespace delimited set of
    /// unique class labels. Modifying the value of the `class` attribute (e.g. by calling
    /// [set_class]), will change the classes in the collection. Conversely, toggling classes in
    /// the collection or removing classes from the collection will update the value of the `class`
    /// attribute.
    fn classes(&self) -> Classes {
        Classes {
            class_list: self.as_web_sys_element().class_list(),
        }
    }

    fn tag_name(&self) -> String {
        self.as_web_sys_element().tag_name()
    }

    fn namespace_uri(&self) -> Option<String> {
        self.as_web_sys_element().namespace_uri()
    }

    fn local_name(&self) -> String {
        self.as_web_sys_element().local_name()
    }

    fn prefix(&self) -> Option<String> {
        self.as_web_sys_element().prefix()
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

    /// Returns the current value of the `class` attribute.
    fn class(&self) -> String {
        self.as_web_sys_element().class_name()
    }

    /// Sets the value of the `class` attribute to the given string.
    ///
    /// Setting this modifies the set of [classes] attached to the element.
    fn set_class(&self, value: &str) {
        self.as_web_sys_element().set_class_name(value);
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

pub struct Attributes {
    element: web_sys::Element,
    attributes: web_sys::NamedNodeMap,
}

impl Attributes {
    pub fn lookup(&self, name: &str) -> Option<Attribute> {
        self.attributes
            .get_named_item(name)
            .map(|a| Attribute::new(a))
    }

    pub fn lookup_namespaced(
        &self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Option<Attribute> {
        self.attributes
            .get_named_item_ns(namespace, local_name)
            .map(|a| Attribute::new(a))
    }

    pub fn contains(&self, name: &str) -> bool {
        self.element.has_attribute(name)
    }

    pub fn contains_namespaced(&self, namespace: Option<&str>, local_name: &str) -> bool {
        self.element.has_attribute_ns(namespace, local_name)
    }

    pub fn names(&self) -> AttributeNames {
        AttributeNames {
            inner: self.element.get_attribute_names(),
        }
    }

    pub fn insert(&self, attribute: &Attribute) -> Option<Attribute> {
        self.attributes
            .set_named_item(attribute.as_ref())
            .unwrap_throw()
            .map(|attr| attr.into())
    }

    pub fn try_insert(&self, attribute: &Attribute) -> Result<Option<Attribute>, InUseAttribute> {
        self.attributes
            .set_named_item(attribute.as_ref())
            .map_ok(|ok| ok.map(|attr| attr.into()))
            .map_err(|err| InUseAttribute::new(err.unchecked_into()))
    }

    pub fn insert_namespaced(&self, attribute: &Attribute) -> Option<Attribute> {
        self.attributes
            .set_named_item_ns(attribute.as_ref())
            .unwrap_throw()
            .map(|attr| attr.into())
    }

    pub fn try_insert_namespaced(
        &self,
        attribute: &Attribute,
    ) -> Result<Option<Attribute>, InUseAttribute> {
        self.attributes
            .set_named_item_ns(attribute.as_ref())
            .map_ok(|ok| ok.map(|attr| attr.into()))
            .map_err(|err| InUseAttribute::new(err.unchecked_into()))
    }

    pub fn toggle(&self, qualified_name: &str) -> bool {
        self.element.toggle_attribute(qualified_name).unwrap_throw()
    }

    pub fn try_toggle(&self, qualified_name: &str) -> Result<bool, InvalidAttributeName> {
        self.element
            .toggle_attribute(qualified_name)
            .map_err(|_| InvalidAttributeName(qualified_name.to_string()))
    }

    pub fn toggle_on(&self, qualified_name: &str) -> bool {
        self.element.toggle_attribute(qualified_name).unwrap_throw()
    }

    pub fn try_toggle_on(&self, qualified_name: &str) -> Result<bool, InvalidAttributeName> {
        self.element
            .toggle_attribute_with_force(qualified_name, true)
            .map_err(|_| InvalidAttributeName(qualified_name.to_string()))
    }

    pub fn toggle_off(&self, qualified_name: &str) -> bool {
        self.element.toggle_attribute(qualified_name).unwrap_throw()
    }

    pub fn try_toggle_off(&self, qualified_name: &str) -> Result<bool, InvalidAttributeName> {
        self.element
            .toggle_attribute_with_force(qualified_name, false)
            .map_err(|_| InvalidAttributeName(qualified_name.to_string()))
    }

    pub fn remove(&self, name: &str) -> Option<Attribute> {
        self.attributes
            .remove_named_item(name)
            .ok()
            .map(|attr| Attribute::new(attr))
    }

    pub fn remove_namespaced(
        &self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Option<Attribute> {
        self.attributes
            .remove_named_item_ns(namespace, local_name)
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

unchecked_cast_array_wrapper!(String, js_sys::JsString, AttributeNames, AttributeNamesIter);

pub struct Classes {
    class_list: web_sys::DomTokenList,
}

impl Classes {
    pub fn contains(&self, class: &str) -> bool {
        self.class_list.contains(class)
    }

    pub fn insert(&self, class: &str) -> bool {
        if !self.contains(class) {
            self.class_list
                .toggle_with_force(class, true)
                .unwrap_throw();

            true
        } else {
            false
        }
    }

    pub fn try_insert(&self, class: &str) -> Result<bool, InvalidClassName> {
        if !self.contains(class) {
            self.class_list
                .toggle_with_force(class, true)
                .map_ok(|ok| Ok(true))
                .map_err(|err| InvalidClassName::new(err.unchecked_into()))
        } else {
            Ok(false)
        }
    }

    pub fn remove(&self, class: &str) -> bool {
        if self.contains(class) {
            self.class_list.remove_1(class).unwrap_throw();

            true
        } else {
            false
        }
    }

    pub fn toggle(&self, class: &str) -> bool {
        self.class_list.toggle(class).unwrap_throw()
    }

    pub fn try_toggle(&self, class: &str) -> Result<bool, InvalidClassName> {
        self.class_list
            .toggle(class)
            .map_err(|err| InvalidClassName::new(err.unchecked_into()))
    }

    pub fn replace(&self, old: &str, new: &str) -> bool {
        // It seems the error case covers old browser returning void instead of a bool, but I don't
        // believe there's any overlap between browsers that support WASM and browsers that still
        // return void, so this should never cause an error.
        self.class_list
            .replace(old, new)
            .map_err(|err| InvalidClassName::new(err.unchecked_into()))
    }
}

impl Collection for Classes {
    fn len(&self) -> u32 {
        self.class_list.length()
    }
}

impl Sequence for Classes {
    type Item = String;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.class_list.item(index)
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.class_list.as_ref())
    }
}

pub struct ClientRect {
    inner: web_sys::DomRect,
}

impl ClientRect {
    delegate! {
        target self.inner {
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

impl_node_traits!(DynamicElement, web_sys::Element);
impl_parent_node_for_element!(DynamicElement);
impl_child_node_for_element!(DynamicElement);
impl_owned_node!(DynamicElement);
impl_scrollable_for_element!(DynamicElement);
impl_scroll_into_view_for_element!(DynamicElement);
impl_ui_event_target_for_element!(DynamicElement);

#[derive(Clone)]
pub struct InUseAttribute {
    inner: web_sys::DomException,
}

impl InUseAttribute {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        InUseAttribute { inner }
    }
}

impl fmt::Debug for InUseAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InUseAttributeError: {}", self.inner.message())
    }
}

#[derive(Clone)]
pub struct InvalidClassName {
    inner: web_sys::DomException,
}

impl InvalidClassName {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        InvalidClassName { inner }
    }
}

impl fmt::Debug for InvalidClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidClassName: {}", self.inner.message())
    }
}

macro_rules! impl_element_traits {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl AsRef<web_sys::Element> for DynamicElement {
            fn as_ref(&self) -> &web_sys::Element {
                &self.inner
            }
        }

        impl $crate::dom::element_seal::Seal for DynamicElement {}
        impl $crate::dom::Element for DynamicElement {}

        $crate::dom::impl_node_traits!(DynamicElement, web_sys::Element);
        $crate::dom::impl_parent_node_for_element!(DynamicElement);
        $crate::dom::impl_child_node_for_element!(DynamicElement);
        $crate::dom::impl_owned_node!(DynamicElement);
        $crate::scroll::impl_scrollable_for_element!(DynamicElement);
        $crate::scroll::impl_scroll_into_view_for_element!(DynamicElement);
    };
}
