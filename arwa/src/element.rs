use std::convert::TryFrom;
use std::fmt;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::attribute::Attribute;
use crate::console::{Write, Writer};
use crate::error::SyntaxError;
use crate::{
    GenericNode, InvalidCast, InvalidPointerId, Node, PointerId, QuerySelectorAll, ScrollByOptions,
    ScrollIntoViewOptions, ScrollToOptions,
};
use crate::event::GenericEventTarget;

pub trait Element: AsRef<web_sys::Element> {
    // TODO: skip `attach_shadow` here, add it to the specific elements for which it is valid.

    // TODO: implement `request_full_screen` as a future.

    // TODO: is there a (reasonable) way to pre-compile the selector as a typed object and get the
    // syntax error handling out of the way at that stage? Would give cleaner return types.

    // TODO: before/after/insert_adjacent

    // TODO: convert from point/quad/rect? Should that live on Document?

    fn matches(&self, selector: &str) -> Result<bool, SyntaxError> {
        self.as_ref()
            .matches(selector)
            .map_err(|err| SyntaxError::new(err.unchecked_into()))
    }

    fn query_selector_first(&self, selector: &str) -> Result<Option<GenericElement>, SyntaxError> {
        self.as_ref()
            .query_selector(selector)
            .map(|ok| ok.map(|e| e.into()))
            .map_err(|err| SyntaxError::new(err.unchecked_into()))
    }

    fn query_selector_all(&self, selector: &str) -> Result<QuerySelectorAll, SyntaxError> {
        self.as_ref()
            .query_selector_all(selector)
            .map(|inner| QuerySelectorAll::new(inner))
            .map_err(|err| SyntaxError::new(err.unchecked_into()))
    }

    // TODO: I can perhaps be argued that get_elements_by_tag_name and get_elements_by_class_name
    // have been superseded by query_selector_all (especially since the newer DocumentFragment
    // interface does not include them), even though they are not entirely equivalent (live
    // collection vs non-live collection respectively) and perhaps not equally performant (I don't
    // know to what extent browser implementations pre-parse and/or cache parsed selector strings).
    //
    // Omitted for now, but might look like this:
    //
    //    fn query_tag_name(&self, tag_name: &str) -> LiveQueriedElements {
    //        LiveQueriedElements {
    //            inner: self.as_ref().get_elements_by_tag_name(tag_name),
    //        }
    //    }
    //
    //    fn query_tag_name_namespaced(
    //        &self,
    //        namespace: Option<&str>,
    //        local_name: &str,
    //    ) -> LiveQueriedElements {
    //        LiveQueriedElements {
    //            // MDN gives no indication that this could actually fail, so just unwrap for now
    //            inner: self
    //                .as_ref()
    //                .get_elements_by_tag_name_ns(namespace, local_name)
    //                .unwrap(),
    //        }
    //    }
    //
    //    fn query_class_name(&self, class_name: &str) -> LiveQueriedElements {
    //        LiveQueriedElements {
    //            inner: self.as_ref().get_elements_by_class_name(class_name),
    //        }
    //    }

    fn closest(&self, selector: &str) -> Result<Option<GenericElement>, SyntaxError> {
        self.as_ref()
            .closest(selector)
            .map(|ok| ok.map(|e| e.into()))
            .map_err(|err| SyntaxError::new(err.unchecked_into()))
    }

    fn set_pointer_capture(&self, pointer_id: PointerId) -> Result<(), InvalidPointerId> {
        self.as_ref()
            .set_pointer_capture(pointer_id.into())
            .map_err(|_| InvalidPointerId(pointer_id))
    }

    fn has_pointer_capture(&self, pointer_id: PointerId) -> bool {
        self.as_ref().has_pointer_capture(pointer_id.into())
    }

    fn release_pointer_capture(&self, pointer_id: PointerId) -> Result<(), InvalidPointerId> {
        self.as_ref()
            .release_pointer_capture(pointer_id.into())
            .map_err(|_| InvalidPointerId(pointer_id))
    }

    fn request_pointer_lock(&self) {
        self.as_ref().request_pointer_lock();
    }

    fn bounding_client_rect(&self) -> ClientRect {
        ClientRect {
            inner: self.as_ref().get_bounding_client_rect(),
        }
    }

    fn client_rects(&self) -> ClientRects {
        ClientRects {
            inner: self.as_ref().get_client_rects(),
        }
    }

    fn attributes(&self) -> Attributes {
        Attributes {
            element: self.as_ref(),
            attributes: self.as_ref().attributes(),
        }
    }

    fn classes(&self) -> Classes {
        Classes {
            element: &self.as_ref(),
            class_list: self.as_ref().class_list(),
        }
    }

    fn set_classes(&self, classes: &str) {
        self.as_ref().set_class_name(classes);
    }

    fn disconnect(&self) {
        self.as_ref().remove();
    }

    fn previous_sibling_element(&self) -> Option<GenericElement> {
        self.as_ref().previous_element_sibling().map(|e| e.into())
    }

    fn next_sibling_element(&self) -> Option<GenericElement> {
        self.as_ref().next_element_sibling().map(|e| e.into())
    }

    fn child_elements(&self) -> ChildElements {
        ChildElements {
            parent: self.as_ref(),
            children: self.as_ref().children(),
        }
    }

    fn tag_name(&self) -> String {
        self.as_ref().tag_name()
    }

    fn namespace_uri(&self) -> Option<String> {
        self.as_ref().namespace_uri()
    }

    fn local_name(&self) -> String {
        self.as_ref().local_name()
    }

    fn prefix(&self) -> Option<String> {
        self.as_ref().prefix()
    }

    fn client_width(&self) -> i32 {
        self.as_ref().client_width()
    }

    fn client_height(&self) -> i32 {
        self.as_ref().client_height()
    }

    fn client_top(&self) -> i32 {
        self.as_ref().client_top()
    }

    fn client_left(&self) -> i32 {
        self.as_ref().client_left()
    }

    fn id(&self) -> String {
        self.as_ref().id()
    }

    fn set_id(&self, id: &str) {
        self.as_ref().set_id(id);
    }

    fn slot(&self) -> String {
        self.as_ref().slot()
    }

    fn set_slot(&self, slot: &str) {
        self.as_ref().set_slot(slot);
    }

    fn inner_html(&self) -> String {
        self.as_ref().inner_html()
    }

    fn set_inner_html(&self, html: &str) {
        self.as_ref().set_inner_html(html);
    }

    fn outer_html(&self) -> String {
        self.as_ref().outer_html()
    }

    fn set_outer_html(&self, html: &str) {
        self.as_ref().set_outer_html(html);
    }

    fn replace_with<T>(&self, replacement: &T)
    where
        T: ElementReplacement,
        Self: Sized,
    {
        replacement.replace(self);
    }

    fn scroll_left(&self) -> i32 {
        self.as_ref().scroll_left()
    }

    fn scroll_top(&self) -> i32 {
        self.as_ref().scroll_top()
    }

    fn scroll_width(&self) -> i32 {
        self.as_ref().scroll_width()
    }

    fn scroll_height(&self) -> i32 {
        self.as_ref().scroll_height()
    }

    fn scroll_to(&self, options: ScrollToOptions) {
        let mut opts = web_sys::ScrollToOptions::new();

        opts.left(options.left.into());
        opts.top(options.top.into());
        opts.behavior(options.behavior.into());

        self.as_ref().scroll_to_with_scroll_to_options(&opts);
    }

    fn scroll_by(&self, options: ScrollByOptions) {
        let mut opts = web_sys::ScrollToOptions::new();

        opts.left(options.x.into());
        opts.top(options.y.into());
        opts.behavior(options.behavior.into());

        self.as_ref().scroll_by_with_scroll_to_options(&opts);
    }

    fn scroll_into_view(&self, options: ScrollIntoViewOptions) {
        let mut opts = web_sys::ScrollIntoViewOptions::new();

        opts.behavior(options.behavior.into());
        opts.block(options.block.into());
        opts.inline(options.inline.into());

        self.as_ref()
            .scroll_into_view_with_scroll_into_view_options(&opts);
    }
}

pub trait ElementReplacement: element_replacement_seal::Sealed {
    fn replace<E>(&self, element: &E)
    where
        E: Element;
}

impl<T> ElementReplacement for T
where
    T: Element,
{
    fn replace<E>(&self, element: &E)
    where
        E: Element,
    {
        element
            .as_ref()
            .replace_with_with_node_1(self.as_ref())
            .unwrap();
    }
}

impl ElementReplacement for str {
    fn replace<E>(&self, element: &E)
    where
        E: Element,
    {
        element.as_ref().replace_with_with_str_1(self).unwrap();
    }
}

mod element_replacement_seal {
    use super::*;

    pub trait Sealed {}

    impl<T> Sealed for T where T: Element {}
    impl Sealed for str {}
}

pub struct Attributes<'a> {
    element: &'a web_sys::Element,
    attributes: web_sys::NamedNodeMap,
}

impl<'a> Attributes<'a> {
    pub fn get(&self, name: &str) -> Option<Attribute> {
        self.attributes
            .get_named_item(name)
            .map(|a| Attribute::new(a))
    }

    pub fn get_namespaced(&self, namespace: Option<&str>, local_name: &str) -> Option<Attribute> {
        self.attributes
            .get_named_item_ns(namespace, local_name)
            .map(|a| Attribute::new(a))
    }

    pub fn contains_name(&self, name: &str) -> bool {
        self.element.has_attribute(name)
    }

    pub fn contains_name_namespaced(&self, namespace: Option<&str>, local_name: &str) -> bool {
        self.element.has_attribute_ns(namespace, local_name)
    }

    pub fn names(&self) -> AttributeNames {
        AttributeNames {
            inner: self.element.get_attribute_names(),
        }
    }

    pub fn len(&self) -> usize {
        self.attributes.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn toggle(&self, name: &str) -> Result<bool, InvalidAttributeName> {
        self.element
            .toggle_attribute(name)
            .map_err(|_| InvalidAttributeName(name.to_string()))
    }

    pub fn force_toggle(&self, name: &str) -> Result<bool, InvalidAttributeName> {
        self.element
            .toggle_attribute_with_force(name, true)
            .map_err(|_| InvalidAttributeName(name.to_string()))
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

    pub fn remove_node<T>(&self, attribute: &Attribute) -> bool
    where
        T: Node,
    {
        self.element
            .remove_attribute_node(attribute.as_ref())
            .ok()
            .flatten()
            .is_some()
    }
}

impl<'a> Write for Attributes<'a> {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.attributes.as_ref())
    }
}

pub struct AttributeNames {
    inner: js_sys::Array,
}

impl AttributeNames {
    pub fn get(&self, index: usize) -> Option<String> {
        u32::try_from(index).ok().map(|index| {
            let name: js_sys::JsString = self.inner.get(index).unchecked_into();

            String::from(name)
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

    pub fn first(&self) -> Option<String> {
        self.get(0)
    }

    pub fn last(&self) -> Option<String> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> AttributeNamesIter {
        AttributeNamesIter {
            attribute_names: self,
            current: 0,
        }
    }
}

impl Write for AttributeNames {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

impl IntoIterator for AttributeNames {
    type Item = String;
    type IntoIter = AttributeNamesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        AttributeNamesIntoIter {
            attribute_names: self,
            current: 0,
        }
    }
}

pub struct AttributeNamesIter<'a> {
    attribute_names: &'a AttributeNames,
    current: usize,
}

impl<'a> Iterator for AttributeNamesIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.attribute_names.get(current)
    }
}

pub struct AttributeNamesIntoIter {
    attribute_names: AttributeNames,
    current: usize,
}

impl Iterator for AttributeNamesIntoIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.attribute_names.get(current)
    }
}

pub struct Classes<'a> {
    element: &'a web_sys::Element,
    class_list: web_sys::DomTokenList,
}

impl<'a> Classes<'a> {
    pub fn get(&self, index: usize) -> Option<String> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.class_list.item(index))
    }

    pub fn len(&self) -> usize {
        self.class_list.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn contains(&self, class: &str) -> bool {
        self.class_list.contains(class)
    }

    // TODO: make insert and remove add a bool by adding a `contains` check?

    pub fn insert(&self, class: &str) {
        self.class_list.toggle_with_force(class, true).unwrap();
    }

    pub fn remove(&self, class: &str) {
        self.class_list.remove_1(class).unwrap();
    }

    pub fn toggle(&self, class: &str) -> bool {
        self.class_list.toggle(class).unwrap()
    }

    pub fn replace(&self, old: &str, new: &str) -> bool {
        // It seems the error case covers old browser returning void instead of a bool, but I don't
        // believe there's any overlap between browsers that support WASM and browsers that still
        // return void, so this should never cause an error.
        self.class_list.replace(old, new).unwrap()
    }

    pub fn iter(&self) -> ClassesIter {
        ClassesIter {
            classes: self,
            current: 0,
        }
    }
}

impl<'a> Write for Classes<'a> {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.class_list.as_ref())
    }
}

impl<'a> ToString for Classes<'a> {
    fn to_string(&self) -> String {
        self.element.class_name()
    }
}

impl<'a> IntoIterator for Classes<'a> {
    type Item = String;
    type IntoIter = ClassesIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ClassesIntoIter {
            classes: self,
            current: 0,
        }
    }
}

pub struct ClassesIter<'a> {
    classes: &'a Classes<'a>,
    current: usize,
}

impl<'a> Iterator for ClassesIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.classes.get(current)
    }
}

pub struct ClassesIntoIter<'a> {
    classes: Classes<'a>,
    current: usize,
}

impl<'a> Iterator for ClassesIntoIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.classes.get(current)
    }
}

pub struct ChildElements<'a> {
    parent: &'a web_sys::Element,
    children: web_sys::HtmlCollection,
}

impl<'a> ChildElements<'a> {
    pub fn get(&self, index: usize) -> Option<GenericElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.children.item(index))
            .map(|e| {
                let e: web_sys::Element = e.unchecked_into();

                e.into()
            })
    }

    pub fn len(&self) -> usize {
        self.parent.child_element_count() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<GenericElement> {
        self.parent.first_element_child().map(|e| e.into())
    }

    pub fn last(&self) -> Option<GenericElement> {
        self.parent.last_element_child().map(|e| e.into())
    }

    pub fn find_by_id(&self, id: &str) -> Option<GenericElement> {
        self.children.get_with_name(id).map(|e| {
            let e: web_sys::Element = e.unchecked_into();

            e.into()
        })
    }

    pub fn append<C>(&self, child: &C)
    where
        C: ElementChild,
    {
        child.append_to(self);
    }

    pub fn prepend<C>(&self, child: &C)
    where
        C: ElementChild,
    {
        child.prepend_to(self);
    }

    pub fn iter(&self) -> ChildElementsIter {
        ChildElementsIter {
            child_elements: self,
            current: 0,
        }
    }
}

impl<'a> Write for ChildElements<'a> {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.children.as_ref())
    }
}

impl<'a> IntoIterator for ChildElements<'a> {
    type Item = GenericElement;
    type IntoIter = ChildElementsIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChildElementsIntoIter {
            child_elements: self,
            current: 0,
        }
    }
}

pub struct ChildElementsIter<'a> {
    child_elements: &'a ChildElements<'a>,
    current: usize,
}

impl<'a> Iterator for ChildElementsIter<'a> {
    type Item = GenericElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.child_elements.get(current)
    }
}

pub struct ChildElementsIntoIter<'a> {
    child_elements: ChildElements<'a>,
    current: usize,
}

impl<'a> Iterator for ChildElementsIntoIter<'a> {
    type Item = GenericElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.child_elements.get(current)
    }
}

pub trait ElementChild: element_child_seal::Sealed {
    // Note, this construct *should* be locked down enough to avoid any "HierarchyRequestError" and
    // thus these operations should never fail.

    fn prepend_to(&self, parent_children: &ChildElements);

    fn append_to(&self, parent_children: &ChildElements);
}

impl<T> ElementChild for T
where
    T: Element,
{
    // TODO: decide on panic vs error on append/prepend when trying to insert an element into an
    // element that is a descendant. May be argued to be in the same class of errors as indexing
    // error, std handles these with panics (e.g. Vec::insert).

    fn prepend_to(&self, parent_children: &ChildElements) {
        parent_children
            .parent
            .prepend_with_node_1(self.as_ref())
            .expect(
                "Element cannot be an ancestor of the element into which it is being inserted.",
            );
    }

    fn append_to(&self, parent_children: &ChildElements) {
        parent_children
            .parent
            .append_with_node_1(self.as_ref())
            .expect(
                "Element cannot be an ancestor of the element into which it is being inserted.",
            );
    }
}

impl ElementChild for str {
    fn prepend_to(&self, parent_children: &ChildElements) {
        parent_children.parent.prepend_with_str_1(self).unwrap();
    }

    fn append_to(&self, parent_children: &ChildElements) {
        parent_children.parent.append_with_str_1(self).unwrap();
    }
}

mod element_child_seal {
    use super::*;

    pub trait Sealed {}

    impl<T> Sealed for T where T: Element {}
    impl Sealed for str {}
}

#[derive(Clone, PartialEq)]
pub struct GenericElement {
    inner: web_sys::Element,
}

impl fmt::Debug for GenericElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.inner.fmt(f)
    }
}

impl From<web_sys::Element> for GenericElement {
    fn from(inner: web_sys::Element) -> Self {
        GenericElement { inner }
    }
}

impl From<GenericElement> for web_sys::Element {
    fn from(value: GenericElement) -> Self {
        value.inner
    }
}

impl TryFrom<GenericEventTarget> for GenericElement {
    type Error = InvalidCast<GenericEventTarget>;

    fn try_from(value: GenericEventTarget) -> Result<Self, Self::Error> {
        let value: web_sys::EventTarget = value.into();

        value
            .dyn_into::<web_sys::Element>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl TryFrom<GenericNode> for GenericElement {
    type Error = InvalidCast<GenericNode>;

    fn try_from(value: GenericNode) -> Result<Self, Self::Error> {
        let value: web_sys::Node = value.into();

        value
            .dyn_into::<web_sys::Element>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl AsRef<web_sys::Element> for GenericElement {
    fn as_ref(&self) -> &web_sys::Element {
        &self.inner
    }
}

impl Write for GenericElement {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
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

impl Write for ClientRect {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

pub struct ClientRects {
    inner: web_sys::DomRectList,
}

impl ClientRects {
    pub fn get(&self, index: usize) -> Option<ClientRect> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get(index))
            .map(|inner| ClientRect { inner })
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

    pub fn first(&self) -> Option<ClientRect> {
        self.get(0)
    }

    pub fn last(&self) -> Option<ClientRect> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> ClientRectsIter {
        ClientRectsIter {
            client_rects: self,
            current: 0,
        }
    }
}

impl Write for ClientRects {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

impl IntoIterator for ClientRects {
    type Item = ClientRect;
    type IntoIter = ClientRectsIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        ClientRectsIntoIter {
            client_rects: self,
            current: 0,
        }
    }
}

pub struct ClientRectsIntoIter {
    client_rects: ClientRects,
    current: usize,
}

impl Iterator for ClientRectsIntoIter {
    type Item = ClientRect;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.client_rects.get(current)
    }
}

pub struct ClientRectsIter<'a> {
    client_rects: &'a ClientRects,
    current: usize,
}

impl<'a> Iterator for ClientRectsIter<'a> {
    type Item = ClientRect;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.client_rects.get(current)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct InvalidAttributeName(String);
