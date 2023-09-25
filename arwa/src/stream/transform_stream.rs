use std::any::Any;
use std::mem::MaybeUninit;
use std::ptr::DynMetadata;
use std::{marker, mem, ptr};

use js_sys::{Object, Uint8Array};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsError, JsValue, UnwrapThrowExt};

use crate::finalization_registry::FinalizationRegistry;
use crate::js_serialize::{js_deserialize, js_serialize};
use crate::stream::{
    readable_stream_seal, QueuingStrategy, QueuingStrategyIntoWebSys, ReadableStream,
    WritableStream,
};
use crate::type_error_wrapper;

pub trait TransformStream {
    type Chunk: JsCast;
    type Error: JsCast;
    type AbortReason: JsCast;
    type Readable: ReadableStream;

    fn writable(&self) -> WritableStream<Self::Chunk, Self::Error, Self::AbortReason>;

    fn readable(&self) -> Self::Readable;
}

thread_local! {
    static TRANSFORM_STREAM_FINALIZATION_REGISTRY: FinalizationRegistry = {
        let callback = |held_value: JsValue| {
            // Reconstruct the Box<dyn Any> that holds the data, then drop it.

            let serialized_data: Uint8Array = held_value.unchecked_into();

            let mut uninit_pointer_data = MaybeUninit::<ClosureStatePointerData>::uninit();
            let data_ptr = uninit_pointer_data.as_mut_ptr() as *mut ();

            js_deserialize(&wasm_bindgen::memory(), data_ptr, &serialized_data);

            let pointer_data = unsafe {
                uninit_pointer_data.assume_init()
            };

            unsafe {
                mem::drop(Box::from_raw(pointer_data.to_dyn_any_ptr()));
            }
        };

        let boxed = Box::new(callback) as Box<dyn FnMut(JsValue)>;
        let closure = Closure::wrap(boxed);
        let registry = FinalizationRegistry::new(&closure);

        closure.forget();

        registry
    };
}

pub fn default_transform_stream_start<Out, E>(_: TransformStreamDefaultController<Out, E>) {}
pub fn default_transform_stream_transform<In, Out, E>(
    _: In,
    _: TransformStreamDefaultController<Out, E>,
) {
}
pub fn default_transform_stream_flush<Out, E>(_: TransformStreamDefaultController<Out, E>) {}

pub struct TransformStreamTransformer<Start, Transform, Flush> {
    pub start: Start,
    pub transform: Transform,
    pub flush: Flush,
}

#[allow(unused)]
struct ClosureState<
    Start: ?Sized,
    Transform: ?Sized,
    Flush: ?Sized,
    InSize: ?Sized,
    OutSize: ?Sized,
> {
    start_callback: Closure<Start>,
    transform_callback: Closure<Transform>,
    flush_callback: Closure<Flush>,
    writable_size_callback: Option<Closure<InSize>>,
    readable_size_callback: Option<Closure<OutSize>>,
}

struct ClosureStatePointerData {
    address: *mut (),
    metadata: DynMetadata<dyn Any>,
}

impl ClosureStatePointerData {
    fn to_dyn_any_ptr(&self) -> *mut dyn Any {
        ptr::from_raw_parts_mut(self.address, self.metadata)
    }
}

pub struct CustomTransformStream<In, Out, E = JsValue> {
    inner: web_sys::TransformStream,
    _marker: marker::PhantomData<(In, Out, E)>,
}

impl<In, Out, E> CustomTransformStream<In, Out, E>
where
    In: JsCast,
    Out: JsCast,
    E: JsCast,
{
    pub fn from_source<Start, Transform, Flush, InSize, OutSize>(
        source: TransformStreamTransformer<Start, Transform, Flush>,
        writable_strategy: QueuingStrategy<In, InSize>,
        readable_strategy: QueuingStrategy<Out, OutSize>,
    ) -> Self
    where
        Start: FnOnce(TransformStreamDefaultController<Out, E>) + 'static,
        Transform: FnMut(In, TransformStreamDefaultController<Out, E>) + 'static,
        Flush: FnMut(TransformStreamDefaultController<Out, E>) + 'static,
        InSize: FnMut(In) -> u32 + 'static,
        OutSize: FnMut(Out) -> u32 + 'static,
    {
        let QueuingStrategyIntoWebSys {
            queuing_strategy: writable_strategy,
            size_callback: writable_size_callback,
        } = writable_strategy.into_web_sys();

        let QueuingStrategyIntoWebSys {
            queuing_strategy: readable_strategy,
            size_callback: readable_size_callback,
        } = readable_strategy.into_web_sys();

        let underlying_transformer = Object::new();

        let TransformStreamTransformer {
            start,
            mut transform,
            mut flush,
        } = source;

        let start = move |inner: web_sys::TransformStreamDefaultController| {
            start(TransformStreamDefaultController {
                inner,
                _marker: Default::default(),
            })
        };

        let start_callback = Closure::once(start);

        js_sys::Reflect::set(
            underlying_transformer.as_ref(),
            &JsValue::from_str("start"),
            &start_callback.as_ref(),
        )
        .unwrap_throw();

        let transform = move |chunk: JsValue, inner: web_sys::TransformStreamDefaultController| {
            if let Ok(chunk) = chunk.dyn_into() {
                transform(
                    chunk,
                    TransformStreamDefaultController {
                        inner,
                        _marker: Default::default(),
                    },
                )
            } else {
                inner.error_with_reason(
                    &JsError::new("Transformer could not transform invalid chunk type").into(),
                );
            }
        };

        let transform_callback = Closure::new(transform);

        js_sys::Reflect::set(
            underlying_transformer.as_ref(),
            &JsValue::from_str("transform"),
            &transform_callback.as_ref(),
        )
        .unwrap_throw();

        let flush = move |inner: web_sys::TransformStreamDefaultController| {
            flush(TransformStreamDefaultController {
                inner,
                _marker: Default::default(),
            })
        };

        let flush_callback = Closure::new(flush);

        js_sys::Reflect::set(
            underlying_transformer.as_ref(),
            &JsValue::from_str("flush"),
            &flush_callback.as_ref(),
        )
        .unwrap_throw();

        let inner = create_transform_stream(
            &underlying_transformer,
            &writable_strategy,
            &readable_strategy,
        )
        .unwrap_throw();

        // Collect closures together in a box and register a cleanup operation with the finalization registry
        let closure_state = Box::new(ClosureState {
            start_callback,
            transform_callback,
            flush_callback,
            writable_size_callback,
            readable_size_callback,
        });

        let data = Box::new(closure_state) as Box<dyn Any>;
        let data_ptr = Box::into_raw(data);
        let (address, metadata) = data_ptr.to_raw_parts();
        let mut pointer_data = ClosureStatePointerData { address, metadata };
        let ptr = &mut pointer_data as *mut ClosureStatePointerData as *mut ();

        let serialized = js_serialize(
            &wasm_bindgen::memory(),
            ptr,
            mem::size_of::<ClosureStatePointerData>() as u32,
        );

        // Make sure it doesn't drop early
        mem::drop(pointer_data);

        TRANSFORM_STREAM_FINALIZATION_REGISTRY
            .with(|r| r.register(inner.as_ref(), serialized.as_ref()));

        CustomTransformStream {
            inner,
            _marker: Default::default(),
        }
    }
}

pub struct CustomTransformedReadableStream<T, E = JsValue> {
    inner: web_sys::ReadableStream,
    _marker: marker::PhantomData<(T, E)>,
}

impl<T, E> readable_stream_seal::Seal for CustomTransformedReadableStream<T, E>
where
    T: JsCast,
    E: JsCast,
{
    fn as_web_sys(&self) -> &web_sys::ReadableStream {
        &self.inner
    }

    fn from_web_sys(web_sys: web_sys::ReadableStream) -> Self
    where
        Self: Sized,
    {
        CustomTransformedReadableStream {
            inner: web_sys,
            _marker: Default::default(),
        }
    }
}

impl<T, E> ReadableStream for CustomTransformedReadableStream<T, E>
where
    T: JsCast,
    E: JsCast,
{
    type Chunk = T;
    type Error = E;
    type Reason = JsValue;
}

impl<In, Out, E> TransformStream for CustomTransformStream<In, Out, E>
where
    In: JsCast,
    Out: JsCast,
    E: JsCast,
{
    type Chunk = In;
    type Error = E;
    type AbortReason = JsValue;
    type Readable = CustomTransformedReadableStream<Out, E>;

    fn writable(&self) -> WritableStream<Self::Chunk, Self::Error, Self::AbortReason> {
        WritableStream {
            inner: self.inner.writable(),
            _marker: Default::default(),
        }
    }

    fn readable(&self) -> Self::Readable {
        CustomTransformedReadableStream {
            inner: self.inner.readable(),
            _marker: Default::default(),
        }
    }
}

type_error_wrapper!(TransformStreamEnqueueError);

pub struct TransformStreamDefaultController<T, E = JsValue> {
    inner: web_sys::TransformStreamDefaultController,
    _marker: marker::PhantomData<(T, E)>,
}

impl<T, E> TransformStreamDefaultController<T, E>
where
    T: JsCast,
    E: JsCast,
{
    pub fn desired_size(&self) -> i32 {
        self.inner.desired_size().unwrap_or(0.0) as i32
    }

    pub fn enqueue(&self, chunk: T) {
        self.inner.enqueue_with_chunk(chunk.as_ref()).unwrap_throw();
    }

    pub fn try_enqueue(&self, chunk: T) -> Result<(), TransformStreamEnqueueError> {
        self.inner
            .enqueue_with_chunk(chunk.as_ref())
            .map_err(|err| TransformStreamEnqueueError::new(err.unchecked_into()))
    }

    pub fn error(&self, error: E) {
        self.inner.error_with_reason(error.as_ref());
    }

    pub fn terminate(&self) {
        self.inner.terminate();
    }
}

#[wasm_bindgen(module = "/src/js_support.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = __arwa_create_transform_stream)]
    fn create_transform_stream(
        underlying_source: &Object,
        writable_strategy: &Object,
        readable_strategy: &Object,
    ) -> Result<web_sys::TransformStream, JsValue>;
}
