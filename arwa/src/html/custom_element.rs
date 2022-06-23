use std::any::{Any, TypeId};
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ptr::DynMetadata;
use std::{marker, mem, ptr};

use js_sys::{Array, Function, Reflect, Uint8Array};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_val, JsCast};
use web_sys::HtmlElement;

use crate::dom::{impl_shadow_host_for_element, DynamicElement, Name, ParentNode};
use crate::finalization_registry::FinalizationRegistry;
use crate::html::{impl_html_element_traits, CustomElementName};
use crate::js_serialize::{js_deserialize, js_serialize};
use crate::InvalidCast;
use crate::{dom_exception_wrapper, impl_common_wrapper_traits};

thread_local! {
    static CUSTOM_ELEMENT_FINALIZATION_REGISTRY: FinalizationRegistry = {
        let callback = |held_value: JsValue| {
            // Reconstruct the Box<dyn Any> that holds the data, then drop it.

            let serialized_data: Uint8Array = held_value.unchecked_into();

            let mut uninit_custom_element_data = MaybeUninit::<CustomElementData>::uninit();
            let data_ptr = uninit_custom_element_data.as_mut_ptr() as *mut ();

            js_deserialize(&wasm_bindgen::memory(), data_ptr, &serialized_data);

            let custom_element_data = unsafe {
                uninit_custom_element_data.assume_init()
            };

            unsafe {
                mem::drop(Box::from_raw(custom_element_data.to_dyn_any_ptr()));
            }
        };

        let boxed = Box::new(callback) as Box<dyn FnMut(JsValue)>;
        let closure = Closure::wrap(boxed);
        let registry = FinalizationRegistry::new(&closure);

        closure.forget();

        registry
    };
}

struct CustomElementData {
    address: *mut (),
    metadata: DynMetadata<dyn Any>,
    type_id: TypeId,
}

impl CustomElementData {
    fn to_dyn_any_ptr(&self) -> *mut dyn Any {
        ptr::from_raw_parts_mut(self.address, self.metadata)
    }
}

pub(crate) mod extendable_element_seal {
    pub trait Seal {
        const EXTENDED_NAME: Option<&'static str>;

        fn from_web_sys_html_element_unchecked(element: web_sys::HtmlElement) -> Self;

        fn as_web_sys_html_element(&self) -> &web_sys::HtmlElement;

        fn into_web_sys_html_element(self) -> web_sys::HtmlElement;
    }
}

pub trait ExtendableElement: extendable_element_seal::Seal {}

#[derive(Clone)]
pub struct GenericExtendableElement {
    inner: web_sys::HtmlElement,
}

impl extendable_element_seal::Seal for GenericExtendableElement {
    const EXTENDED_NAME: Option<&'static str> = None;

    fn from_web_sys_html_element_unchecked(inner: web_sys::HtmlElement) -> Self {
        GenericExtendableElement { inner }
    }

    fn as_web_sys_html_element(&self) -> &web_sys::HtmlElement {
        &self.inner
    }

    fn into_web_sys_html_element(self) -> web_sys::HtmlElement {
        self.inner
    }
}
impl ExtendableElement for GenericExtendableElement {}

impl_html_element_traits!(GenericExtendableElement);
impl_shadow_host_for_element!(GenericExtendableElement);

pub struct CustomElement<T, E> {
    data: *const T,
    extended: E,
}

impl<T, E> CustomElement<T, E>
where
    T: 'static,
    E: ExtendableElement,
{
    fn from_raw_unchecked(raw: RawCustomElement) -> Self {
        let mut uninit_custom_element_data = MaybeUninit::<CustomElementData>::uninit();
        let data_ptr = uninit_custom_element_data.as_mut_ptr();

        raw.deserialize_custom_element_data(&wasm_bindgen::memory(), data_ptr);

        let custom_element_data = unsafe { uninit_custom_element_data.assume_init() };

        let data = custom_element_data.address as *const T;
        let extended = E::from_web_sys_html_element_unchecked(raw.into());

        CustomElement { data, extended }
    }
}

impl<T, E> CustomElement<T, E>
where
    T: 'static,
{
    pub fn data(&self) -> &T {
        unsafe { &*self.data }
    }
}

impl<T, E> Deref for CustomElement<T, E>
where
    E: ExtendableElement,
{
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.extended
    }
}

impl<T, E> Clone for CustomElement<T, E>
where
    E: Clone,
{
    fn clone(&self) -> Self {
        CustomElement {
            data: self.data,
            extended: self.extended.clone(),
        }
    }
}

impl<T, E> TryFrom<DynamicElement> for CustomElement<T, E>
where
    T: 'static,
    E: ExtendableElement + 'static,
{
    type Error = InvalidCast<DynamicElement, CustomElement<T, E>>;

    fn try_from(element: DynamicElement) -> Result<Self, Self::Error> {
        let element: web_sys::Element = element.into();

        let is_custom_element = Reflect::has(
            element.as_ref(),
            &"__deserialize_custom_element_data".into(),
        )
        .unwrap_or(false);

        if is_custom_element {
            let raw = element.unchecked_into::<RawCustomElement>();

            let mut uninit_custom_element_data = MaybeUninit::<CustomElementData>::uninit();
            let data_ptr = uninit_custom_element_data.as_mut_ptr();

            raw.deserialize_custom_element_data(&wasm_bindgen::memory(), data_ptr);

            let custom_element_data = unsafe { uninit_custom_element_data.assume_init() };
            let target_type_id = TypeId::of::<CustomElement<T, E>>();

            if custom_element_data.type_id == target_type_id {
                let data = custom_element_data.address as *const T;
                let extended = E::from_web_sys_html_element_unchecked(raw.unchecked_into());

                Ok(CustomElement { data, extended })
            } else {
                Err(InvalidCast::new(DynamicElement::from(
                    raw.unchecked_into::<web_sys::Element>(),
                )))
            }
        } else {
            Err(InvalidCast::new(DynamicElement::from(element)))
        }
    }
}

impl<T, E> From<CustomElement<T, E>> for DynamicElement
where
    T: 'static,
    E: ExtendableElement + 'static,
{
    fn from(element: CustomElement<T, E>) -> Self {
        let web_sys: web_sys::Element = element.extended.into_web_sys_html_element().into();

        DynamicElement::from(web_sys)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AttributeChange {
    pub attribute_name: Name,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

pub struct CustomElementDescriptor<
    'a,
    T,
    E,
    Constructor,
    ConnectedCallback,
    DisconnectedCallback,
    AdoptedCallback,
    AttributeChangedCallback,
> {
    pub constructor: Constructor,
    pub connected_callback: ConnectedCallback,
    pub disconnected_callback: DisconnectedCallback,
    pub adopted_callback: AdoptedCallback,
    pub attribute_changed_callback: AttributeChangedCallback,
    pub observed_attributes: &'a [Name],
    _marker: marker::PhantomData<(*const T, *const E)>,
}

impl CustomElementDescriptor<'_, (), (), (), (), (), (), ()> {
    pub fn new<T, E, Constructor>(
        constructor: Constructor,
    ) -> CustomElementDescriptor<
        'static,
        T,
        E,
        Constructor,
        fn(&CustomElement<T, E>),
        fn(&CustomElement<T, E>),
        fn(&CustomElement<T, E>),
        fn(&CustomElement<T, E>, AttributeChange),
    >
    where
        T: 'static,
        E: ExtendableElement + 'static,
        Constructor: Fn(&E) -> T + 'static,
    {
        CustomElementDescriptor {
            constructor,
            connected_callback: default_connected_callback,
            disconnected_callback: default_disconnected_callback,
            adopted_callback: default_adopted_callback,
            attribute_changed_callback: default_attribute_changed_callback,
            observed_attributes: &[],
            _marker: marker::PhantomData,
        }
    }
}

impl<
        'a,
        T,
        E,
        Constructor,
        ConnectedCallback,
        DisconnectedCallback,
        AdoptedCallback,
        AttributeChangedCallback,
    >
    CustomElementDescriptor<
        'a,
        T,
        E,
        Constructor,
        ConnectedCallback,
        DisconnectedCallback,
        AdoptedCallback,
        AttributeChangedCallback,
    >
{
    pub fn connected_callback<F>(
        self,
        connected_callback: F,
    ) -> CustomElementDescriptor<
        'a,
        T,
        E,
        Constructor,
        F,
        DisconnectedCallback,
        AdoptedCallback,
        AttributeChangedCallback,
    >
    where
        F: FnMut(&CustomElement<T, E>) + 'static,
    {
        let CustomElementDescriptor {
            constructor,
            disconnected_callback,
            adopted_callback,
            attribute_changed_callback,
            observed_attributes,
            ..
        } = self;

        CustomElementDescriptor {
            constructor,
            connected_callback,
            disconnected_callback,
            adopted_callback,
            attribute_changed_callback,
            observed_attributes,
            _marker: marker::PhantomData,
        }
    }

    pub fn disconnected_callback<F>(
        self,
        disconnected_callback: F,
    ) -> CustomElementDescriptor<
        'a,
        T,
        E,
        Constructor,
        ConnectedCallback,
        F,
        AdoptedCallback,
        AttributeChangedCallback,
    >
    where
        F: FnMut(&CustomElement<T, E>) + 'static,
    {
        let CustomElementDescriptor {
            constructor,
            connected_callback,
            adopted_callback,
            attribute_changed_callback,
            observed_attributes,
            ..
        } = self;

        CustomElementDescriptor {
            constructor,
            connected_callback,
            disconnected_callback,
            adopted_callback,
            attribute_changed_callback,
            observed_attributes,
            _marker: marker::PhantomData,
        }
    }

    pub fn adopted_callback<F>(
        self,
        adopted_callback: F,
    ) -> CustomElementDescriptor<
        'a,
        T,
        E,
        Constructor,
        ConnectedCallback,
        DisconnectedCallback,
        F,
        AttributeChangedCallback,
    >
    where
        F: FnMut(&CustomElement<T, E>) + 'static,
    {
        let CustomElementDescriptor {
            constructor,
            connected_callback,
            disconnected_callback,
            attribute_changed_callback,
            observed_attributes,
            ..
        } = self;

        CustomElementDescriptor {
            constructor,
            connected_callback,
            disconnected_callback,
            adopted_callback,
            attribute_changed_callback,
            observed_attributes,
            _marker: marker::PhantomData,
        }
    }

    pub fn attribute_changed_callback<'b, F>(
        self,
        observed_attributes: &'b [Name],
        attribute_changed_callback: F,
    ) -> CustomElementDescriptor<
        'b,
        T,
        E,
        Constructor,
        ConnectedCallback,
        DisconnectedCallback,
        AdoptedCallback,
        F,
    >
    where
        F: FnMut(&CustomElement<T, E>, AttributeChange) + 'static,
    {
        let CustomElementDescriptor {
            constructor,
            connected_callback,
            disconnected_callback,
            adopted_callback,
            ..
        } = self;

        CustomElementDescriptor {
            constructor,
            connected_callback,
            disconnected_callback,
            adopted_callback,
            attribute_changed_callback,
            observed_attributes,
            _marker: marker::PhantomData,
        }
    }
}

fn default_connected_callback<T, E>(_custom_element: &CustomElement<T, E>) {}

fn default_disconnected_callback<T, E>(_custom_element: &CustomElement<T, E>) {}

fn default_adopted_callback<T, E>(_custom_element: &CustomElement<T, E>) {}

fn default_attribute_changed_callback<T, E>(
    _custom_element: &CustomElement<T, E>,
    _change: AttributeChange,
) {
}

pub struct CustomElementDefinition<T, E> {
    _data_marker: marker::PhantomData<T>,
    _extended_marker: marker::PhantomData<E>,
}

#[derive(Clone)]
pub struct CustomElementRegistry {
    inner: web_sys::CustomElementRegistry,
}

impl CustomElementRegistry {
    pub fn register<
        T,
        E,
        Constructor,
        ConnectedCallback,
        DisconnectedCallback,
        AdoptedCallback,
        AttributeChangedCallback,
    >(
        &self,
        name: &CustomElementName,
        descriptor: CustomElementDescriptor<
            T,
            E,
            Constructor,
            ConnectedCallback,
            DisconnectedCallback,
            AdoptedCallback,
            AttributeChangedCallback,
        >,
    ) -> CustomElementDefinition<T, E>
    where
        T: 'static,
        E: ExtendableElement + 'static,
        Constructor: FnMut(&E) -> T + 'static,
        ConnectedCallback: FnMut(&CustomElement<T, E>) + 'static,
        DisconnectedCallback: FnMut(&CustomElement<T, E>) + 'static,
        AdoptedCallback: FnMut(&CustomElement<T, E>) + 'static,
        AttributeChangedCallback: FnMut(&CustomElement<T, E>, AttributeChange) + 'static,
    {
        match self.try_register(name, descriptor) {
            Ok(ok) => ok,
            Err(err) => throw_val(err.inner.into()),
        }
    }

    pub fn try_register<
        T,
        E,
        Constructor,
        ConnectedCallback,
        DisconnectedCallback,
        AdoptedCallback,
        AttributeChangedCallback,
    >(
        &self,
        name: &CustomElementName,
        descriptor: CustomElementDescriptor<
            T,
            E,
            Constructor,
            ConnectedCallback,
            DisconnectedCallback,
            AdoptedCallback,
            AttributeChangedCallback,
        >,
    ) -> Result<CustomElementDefinition<T, E>, RegisterCustomElementError>
    where
        T: 'static,
        E: ExtendableElement + 'static,
        Constructor: FnMut(&E) -> T + 'static,
        ConnectedCallback: FnMut(&CustomElement<T, E>) + 'static,
        DisconnectedCallback: FnMut(&CustomElement<T, E>) + 'static,
        AdoptedCallback: FnMut(&CustomElement<T, E>) + 'static,
        AttributeChangedCallback: FnMut(&CustomElement<T, E>, AttributeChange) + 'static,
    {
        let CustomElementDescriptor {
            mut constructor,
            mut connected_callback,
            mut disconnected_callback,
            mut adopted_callback,
            mut attribute_changed_callback,
            observed_attributes,
            ..
        } = descriptor;

        let type_id = TypeId::of::<CustomElement<T, E>>();

        let constructor = move |extended: web_sys::HtmlElement| {
            let extended = E::from_web_sys_html_element_unchecked(extended);

            let data = constructor(&extended);

            let data = Box::new(data) as Box<dyn Any>;
            let data_ptr = Box::into_raw(data);
            let (address, metadata) = data_ptr.to_raw_parts();
            let mut custom_element_data = CustomElementData {
                address,
                metadata,
                type_id,
            };
            let ptr = &mut custom_element_data as *mut CustomElementData as *mut ();

            let serialized = js_serialize(
                &wasm_bindgen::memory(),
                ptr,
                mem::size_of::<CustomElementData>() as u32,
            );

            // Make sure it doesn't drop early
            mem::drop(custom_element_data);

            CUSTOM_ELEMENT_FINALIZATION_REGISTRY.with(|r| {
                r.register(
                    extended.as_web_sys_html_element().as_ref(),
                    serialized.as_ref(),
                )
            });

            serialized
        };

        let constructor_boxed =
            Box::new(constructor) as Box<dyn FnMut(web_sys::HtmlElement) -> js_sys::Uint8Array>;
        let constructor_closure = Closure::wrap(constructor_boxed);

        let connected_callback = move |custom_element: RawCustomElement| {
            connected_callback(&CustomElement::from_raw_unchecked(custom_element));
        };

        let connected_callback_boxed =
            Box::new(connected_callback) as Box<dyn FnMut(RawCustomElement)>;
        let connected_callback_closure = Closure::wrap(connected_callback_boxed);

        let disconnected_callback = move |custom_element: RawCustomElement| {
            disconnected_callback(&CustomElement::from_raw_unchecked(custom_element));
        };

        let disconnected_callback_boxed =
            Box::new(disconnected_callback) as Box<dyn FnMut(RawCustomElement)>;
        let disconnected_callback_closure = Closure::wrap(disconnected_callback_boxed);

        let adopted_callback = move |custom_element: RawCustomElement| {
            adopted_callback(&CustomElement::from_raw_unchecked(custom_element));
        };

        let adopted_callback_boxed = Box::new(adopted_callback) as Box<dyn FnMut(RawCustomElement)>;
        let adopted_callback_closure = Closure::wrap(adopted_callback_boxed);

        let attribute_changed_callback =
            move |custom_element: RawCustomElement,
                  attribute_name: String,
                  old_value: Option<String>,
                  new_value: Option<String>| {
                let change = AttributeChange {
                    attribute_name: Name::trusted(attribute_name),
                    old_value,
                    new_value,
                };

                attribute_changed_callback(
                    &CustomElement::from_raw_unchecked(custom_element),
                    change,
                );
            };

        let attribute_changed_callback_boxed = Box::new(attribute_changed_callback)
            as Box<dyn FnMut(RawCustomElement, String, Option<String>, Option<String>)>;
        let attribute_changed_callback_closure = Closure::wrap(attribute_changed_callback_boxed);

        let observed_attributes = observed_attributes
            .iter()
            .map(|attr| JsValue::from_str(attr.as_ref()))
            .collect::<js_sys::Array>();

        let result = define_custom_element(
            name.as_ref(),
            E::EXTENDED_NAME,
            constructor_closure.as_ref().unchecked_ref(),
            connected_callback_closure.as_ref().unchecked_ref(),
            disconnected_callback_closure.as_ref().unchecked_ref(),
            adopted_callback_closure.as_ref().unchecked_ref(),
            attribute_changed_callback_closure.as_ref().unchecked_ref(),
            &observed_attributes,
        );

        if result.is_ok() {
            // Custom elements definitions cannot be revoked anyway, so if we just successfully
            // defined our element, then simply forget these closures forever.
            constructor_closure.forget();
            connected_callback_closure.forget();
            disconnected_callback_closure.forget();
            adopted_callback_closure.forget();
            attribute_changed_callback_closure.forget();
        }

        result
            .map(|_| CustomElementDefinition {
                _data_marker: marker::PhantomData,
                _extended_marker: marker::PhantomData,
            })
            .map_err(|err| RegisterCustomElementError::new(err.unchecked_into()))
    }

    pub fn upgrade<T>(&self, root: &T)
    where
        T: ParentNode,
    {
        self.inner.upgrade(root.as_js_parent_node());
    }
}

impl From<web_sys::CustomElementRegistry> for CustomElementRegistry {
    fn from(inner: web_sys::CustomElementRegistry) -> Self {
        CustomElementRegistry { inner }
    }
}

impl_common_wrapper_traits!(CustomElementRegistry);

dom_exception_wrapper!(RegisterCustomElementError);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = HtmlElement)]
    type RawCustomElement;

    #[wasm_bindgen(method, js_name = "__deserialize_custom_element_data")]
    fn deserialize_custom_element_data(
        this: &RawCustomElement,
        wasm_memory: &JsValue,
        ptr: *mut CustomElementData,
    );
}

#[wasm_bindgen(module = "/src/js_support.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = __arwa_define_custom_element)]
    fn define_custom_element(
        name: &str,
        extended_name: Option<&str>,
        constructor: &Function,
        connected_callback: &Function,
        disconnected_callback: &Function,
        adopted_callback: &Function,
        attribute_changed_callback: &Function,
        observed_attributes: &Array,
    ) -> Result<JsValue, JsValue>;
}

macro_rules! impl_extendable_element {
    ($tpe:ident, $tag_name:literal) => {
        impl $crate::html::extendable_element_seal::Seal for $tpe {
            const EXTENDED_NAME: Option<&'static str> = Some($tag_name);

            fn from_web_sys_html_element_unchecked(inner: web_sys::HtmlElement) -> Self {
                use wasm_bindgen::JsCast;

                $tpe {
                    inner: inner.unchecked_into(),
                }
            }

            fn as_web_sys_html_element(&self) -> &web_sys::HtmlElement {
                self.inner.as_ref()
            }

            fn into_web_sys_html_element(self) -> web_sys::HtmlElement {
                self.inner.into()
            }
        }

        impl $crate::html::ExtendableElement for $tpe {}
    };
}

pub(crate) use impl_extendable_element;
