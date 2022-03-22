use std::any::{Any, TypeId};
use std::ops::Deref;
use std::{marker, mem};

use js_sys::{Array, Function, Reflect, Uint8Array};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_val, JsCast};
use web_sys::HtmlElement;

use crate::dom::{impl_shadow_host_for_element, DynamicElement, Name, ParentNode};
use crate::finalization_registry::FinalizationRegistry;
use crate::html::{impl_html_element_traits, CustomElementName};
use crate::util::type_id_to_u64;
use crate::InvalidCast;
use crate::{dom_exception_wrapper, impl_common_wrapper_traits};

thread_local! {
    static CUSTOM_ELEMENT_FINALIZATION_REGISTRY: FinalizationRegistry = {
        let callback = |held_value: JsValue| {
            // Reconstruct the Box<dyn Any> that holds the data, then drop it.

            let pointer_data: Uint8Array = held_value.unchecked_into();

            // Copy pointer data to WASM linear memory that we can operate on.
            let mut scratch = [0u8; 24];
            let size_of_usize = mem::size_of::<usize>();

            pointer_data.copy_to(&mut scratch[..size_of_usize * 2 + 8]);

            let (address_bytes, rest) = scratch.split_at(size_of_usize);
            let (vtable_bytes, _) = rest.split_at(size_of_usize);

            let address_usize = usize::from_ne_bytes(address_bytes.try_into().unwrap_throw());
            let vtable_usize = usize::from_ne_bytes(vtable_bytes.try_into().unwrap_throw());

            let ptr: *mut dyn Any = unsafe { mem::transmute((address_usize, vtable_usize)) };

            unsafe {
                mem::drop(Box::from_raw(ptr));
            }
        };

        let boxed = Box::new(callback) as Box<dyn FnMut(JsValue)>;
        let closure = Closure::wrap(boxed);
        let registry = FinalizationRegistry::new(&closure);

        closure.forget();

        registry
    };
}

pub(crate) mod extendable_element_seal {
    pub trait Seal {
        const EXTENDED_NAME: Option<&'static str>;

        fn from_web_sys_html_element_unchecked(element: web_sys::HtmlElement) -> Self;

        fn as_web_sys_html_element(&self) -> &web_sys::HtmlElement;
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
        let mut scratch = [0u8; 24];
        let size_of_usize = mem::size_of::<usize>();

        raw.data().copy_to(&mut scratch[0..size_of_usize * 2 + 8]);

        let data_ptr_bits =
            usize::from_ne_bytes(scratch[..size_of_usize].try_into().unwrap_throw());
        let data = <*const T>::from_bits(data_ptr_bits);
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

impl<T, E> TryFrom<DynamicElement> for CustomElement<T, E>
where
    T: 'static,
    E: ExtendableElement + 'static,
{
    type Error = InvalidCast<DynamicElement, CustomElement<T, E>>;

    fn try_from(element: DynamicElement) -> Result<Self, Self::Error> {
        let element: web_sys::Element = element.into();

        if let Ok(value) = Reflect::get(element.as_ref(), &"__arwa_custom_element_data".into()) {
            if !value.is_undefined() {
                let data: Uint8Array = value.unchecked_into();
                let target_type_num = type_id_to_u64(TypeId::of::<CustomElement<T, E>>());

                let mut scratch = [0u8; 24];
                let size_of_usize = mem::size_of::<usize>();
                let type_num_start = size_of_usize * 2;
                let type_num_end = size_of_usize * 2 + 8;

                data.copy_to(&mut scratch[0..size_of_usize * 2 + 8]);

                let type_num = u64::from_ne_bytes(
                    scratch[type_num_start..type_num_end]
                        .try_into()
                        .unwrap_throw(),
                );

                if type_num == target_type_num {
                    let data_ptr_bits =
                        usize::from_ne_bytes(scratch[..size_of_usize].try_into().unwrap_throw());
                    let data = <*const T>::from_bits(data_ptr_bits);
                    let extended = E::from_web_sys_html_element_unchecked(element.unchecked_into());

                    return Ok(CustomElement { data, extended });
                }
            }
        }

        Err(InvalidCast::new(DynamicElement::from(element)))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AttributeChange {
    pub attribute_name: Name,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

pub struct CustomElementDescriptor<T, E>
where
    T: 'static,
    E: ExtendableElement + 'static,
{
    pub constructor: fn(&E) -> T,
    pub connected_callback: fn(&CustomElement<T, E>),
    pub disconnected_callback: fn(&CustomElement<T, E>),
    pub adopted_callback: fn(&CustomElement<T, E>),
    pub attribute_changed_callback: fn(&CustomElement<T, E>, AttributeChange),
    pub observed_attributes: &'static [Name],
}

pub fn default_constructor<T, E>(_extended: &E) -> T
where
    T: Default,
{
    Default::default()
}

pub fn default_connected_callback<T, E>(_custom_element: &CustomElement<T, E>) {}

pub fn default_disconnected_callback<T, E>(_custom_element: &CustomElement<T, E>) {}

pub fn default_adopted_callback<T, E>(_custom_element: &CustomElement<T, E>) {}

pub fn default_attribute_changed_callback<T, E>(
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
    pub fn register<T, E>(
        &self,
        name: &CustomElementName,
        descriptor: CustomElementDescriptor<T, E>,
    ) -> CustomElementDefinition<T, E>
    where
        T: 'static,
        E: ExtendableElement + 'static,
    {
        match self.try_register(name, descriptor) {
            Ok(ok) => ok,
            Err(err) => throw_val(err.inner.into()),
        }
    }

    pub fn try_register<T, E>(
        &self,
        name: &CustomElementName,
        descriptor: CustomElementDescriptor<T, E>,
    ) -> Result<CustomElementDefinition<T, E>, RegisterCustomElementError>
    where
        T: 'static,
        E: ExtendableElement + 'static,
    {
        let CustomElementDescriptor {
            constructor,
            connected_callback,
            disconnected_callback,
            adopted_callback,
            attribute_changed_callback,
            observed_attributes,
        } = descriptor;

        let type_id = TypeId::of::<CustomElement<T, E>>();
        let type_num = type_id_to_u64(type_id);

        let constructor = move |extended: web_sys::HtmlElement| {
            let extended = E::from_web_sys_html_element_unchecked(extended);

            let data = constructor(&extended);

            let data = Box::new(data) as Box<dyn Any>;
            let data_ptr = Box::into_raw(data);
            let (address_ptr, vtable_ptr): (usize, usize) = unsafe { mem::transmute(data_ptr) };

            let mut scratch = [0u8; 24];
            let size_of_usize = mem::size_of::<usize>();
            let type_num_start = size_of_usize * 2;
            let type_num_end = size_of_usize * 2 + 8;

            scratch[0..size_of_usize].copy_from_slice(&address_ptr.to_ne_bytes());
            scratch[size_of_usize..type_num_start].copy_from_slice(&vtable_ptr.to_ne_bytes());
            scratch[type_num_start..type_num_end].copy_from_slice(&type_num.to_ne_bytes());

            let data = Uint8Array::new_with_length(type_num_end as u32);

            data.copy_from(&scratch[..type_num_end]);

            CUSTOM_ELEMENT_FINALIZATION_REGISTRY
                .with(|r| r.register(extended.as_web_sys_html_element().as_ref(), data.as_ref()));

            data
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
        self.inner.upgrade(root.as_web_sys_node());
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
    pub type RawCustomElement;

    #[wasm_bindgen(method, getter, js_name = "__arwa_custom_element_data")]
    pub fn data(this: &RawCustomElement) -> Uint8Array;
}

#[wasm_bindgen(module = "/src/html/define_custom_element.js")]
extern "C" {
    #[wasm_bindgen(catch)]
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
        }

        impl $crate::html::ExtendableElement for $tpe {}
    };
}

pub(crate) use impl_extendable_element;
