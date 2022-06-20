use std::cell::Cell;

use arwa::console;
use arwa::dom::{name, selector, Element, ParentNode, ShadowHost, ShadowRootOptions, Name};
use arwa::html::{AttributeChange, CustomElement, CustomElementDescriptor, GenericExtendableElement, HtmlDocument, HtmlTemplateElement, CustomElementRegistry, CustomElementName, CustomElementDefinition};
use arwa::window::window;

thread_local! {
    static TEMPLATE: HtmlTemplateElement = {
        let document: HtmlDocument = window().document().try_into().expect("Window document it not a HTML document");

        let template_element: HtmlTemplateElement = document.create_known_element();

        template_element.deserialize_inner(include_str!("template.html"));

        template_element
    }
}

pub struct MyElementData {
    connected_count: Cell<u32>,
}

impl Drop for MyElementData {
    fn drop(&mut self) {
        console::log!("Dropping element data...")
    }
}

pub type MyElement = CustomElement<MyElementData, GenericExtendableElement>;

pub trait MyElementExt {
    fn message(&self) -> Option<String>;

    fn set_message(&self, message: &str);
}

impl MyElementExt for MyElement {
    fn message(&self) -> Option<String> {
        self.attributes()
            .lookup(&name!("message"))
            .map(|a| a.value())
    }

    fn set_message(&self, message: &str) {
        self.attributes().set(&name!("message"), message);
    }
}

fn constructor(extended: &GenericExtendableElement) -> MyElementData {
    let shadow_root = extended.attach_shadow(ShadowRootOptions::default());

    TEMPLATE.with(|template| {
        let content = ParentNode::duplicate_deep(&template.content());

        shadow_root.append_fragment(&content)
    });

    MyElementData {
        connected_count: Cell::new(0),
    }
}

fn connected_callback(element: &MyElement) {
    let connected_count = &element.data().connected_count;
    let count = connected_count.get() + 1;

    connected_count.set(count);

    console::log!("Custom element has been connected %i time(s)!", count);
}

fn disconnected_callback(_element: &MyElement) {
    console::log!("Disconnecting custom element...");
}

fn attribute_changed_callback(element: &MyElement, change: AttributeChange) {
    if change.attribute_name == "message" {
        if let Some(message_container) = element
            .shadow_root()
            .and_then(|r| r.query_selector(&selector!("#message_container")))
        {
            message_container.deserialize_inner(&change.new_value.unwrap_or_default());
        }
    }
}

const OBSERVED_ATTRIBUTES: &'static [Name] = &[name!("message")];

pub fn register(name: &CustomElementName, registry: &CustomElementRegistry) -> CustomElementDefinition<MyElementData, GenericExtendableElement> {
    let descriptor = CustomElementDescriptor::new(constructor)
        .connected_callback(connected_callback)
        .disconnected_callback(disconnected_callback)
        .attribute_changed_callback(OBSERVED_ATTRIBUTES, attribute_changed_callback);

    registry.register(name, descriptor)
}
