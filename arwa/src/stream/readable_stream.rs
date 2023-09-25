use crate::finalization_registry::FinalizationRegistry;
use crate::js_serialize::{js_deserialize, js_serialize};
use crate::stream::{QueuingStrategy, QueuingStrategyIntoWebSys, TransformStream, WritableStream};
use crate::{spawn_local, type_error_wrapper};
use futures::stream::{AbortHandle, Abortable};
use futures::{Stream, StreamExt};
use js_sys::{Object, Uint8Array};
use pin_project::pin_project;
use std::any::Any;
use std::future::Future;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr::DynMetadata;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::{fmt, marker, mem, ptr};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::{throw_str, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

thread_local! {
    static READABLE_STREAM_FINALIZATION_REGISTRY: FinalizationRegistry = {
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

pub(super) mod readable_stream_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys(&self) -> &web_sys::ReadableStream;

        #[doc(hidden)]
        fn from_web_sys(web_sys: web_sys::ReadableStream) -> Self
        where
            Self: Sized;
    }
}

pub trait ReadableStream: readable_stream_seal::Seal + Sized {
    type Chunk: JsCast;

    type Error: JsCast;

    type Reason: JsCast;

    fn is_locked(&self) -> bool {
        self.as_web_sys().locked()
    }

    fn cancel(&self, reason: Self::Reason) -> ReadableStreamCancel<Self::Error> {
        ReadableStreamCancel {
            inner: self.as_web_sys().cancel_with_reason(reason.as_ref()).into(),
            _marker: Default::default(),
        }
    }

    fn get_reader(&self) -> ReadableStreamDefaultReader<Self::Chunk, Self::Error, Self::Reason> {
        let inner = self
            .as_web_sys()
            .get_reader()
            .dyn_into::<web_sys::ReadableStreamDefaultReader>()
            .unwrap_throw();

        ReadableStreamDefaultReader {
            inner,
            _marker: Default::default(),
        }
    }

    fn pipe_to<ESink, CSink>(
        &self,
        sink: &WritableStream<Self::Chunk, ESink, CSink>,
    ) -> ReadableStreamPipeTo<Self::Error, ESink>
    where
        ESink: JsCast,
    {
        ReadableStreamPipeTo {
            inner: self.as_web_sys().pipe_to(&sink.inner).into(),
            _marker: Default::default(),
        }
    }

    fn pipe_through<S>(&self, transform_stream: &S) -> S::Readable
    where
        S: TransformStream,
    {
        use readable_stream_seal::Seal;

        let readable = transform_stream.readable();
        let pair = web_sys::ReadableWritablePair::new(
            readable.as_web_sys(),
            &transform_stream.writable().inner,
        );

        let res = self.as_web_sys().pipe_through(&pair);

        S::Readable::from_web_sys(res)
    }

    fn into_chunks(self) -> ReadableStreamChunks<Self::Chunk, Self::Error, Self::Reason> {
        ReadableStreamChunks {
            reader: self.get_reader(),
            current_read: None,
        }
    }

    fn tee(self) -> (Self, Self) {
        let array = self.as_web_sys().tee();

        let a = Self::from_web_sys(array.get(0).unchecked_into());
        let b = Self::from_web_sys(array.get(1).unchecked_into());

        (a, b)
    }
}

pub trait ReadableByteStream: ReadableStream<Chunk = Uint8Array> {
    fn get_byob_reader(&self) -> ReadableStreamByobReader<Self::Error, Self::Reason> {
        let mut options = web_sys::ReadableStreamGetReaderOptions::new();

        options.mode(web_sys::ReadableStreamReaderMode::Byob);

        let inner = self
            .as_web_sys()
            .get_reader_with_options(&options)
            .dyn_into::<web_sys::ReadableStreamByobReader>()
            .unwrap_throw();

        ReadableStreamByobReader {
            inner,
            _marker: Default::default(),
        }
    }
}

pub fn default_readable_stream_start<T, E>(_: ReadableStreamDefaultController<T, E>) {}
pub fn default_readable_stream_pull<T, E>(_: ReadableStreamDefaultController<T, E>) {}
pub fn default_readable_stream_cancel<C>(_: C) {}

pub struct ReadableStreamSource<Start, Pull, Cancel> {
    pub start: Start,
    pub pull: Pull,
    pub cancel: Cancel,
}

pub struct ReadableByteStreamSource<Start, Pull, Cancel> {
    pub start: Start,
    pub pull: Pull,
    pub cancel: Cancel,
    pub auto_allocate_chunk_size: Option<u32>,
}

#[allow(unused)]
struct ClosureState<Start: ?Sized, Pull: ?Sized, Cancel: ?Sized, Size: ?Sized> {
    start_callback: Closure<Start>,
    pull_callback: Closure<Pull>,
    cancel_callback: Closure<Cancel>,
    size_callback: Option<Closure<Size>>,
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

pub struct CustomReadableStream<T, E = JsValue, C = JsValue> {
    inner: web_sys::ReadableStream,
    _marker: marker::PhantomData<(T, E, C)>,
}

impl<T, E, C> CustomReadableStream<T, E, C>
where
    T: JsCast,
    E: JsCast,
    C: JsCast,
{
    pub fn from_source<Start, Pull, Cancel, Size>(
        source: ReadableStreamSource<Start, Pull, Cancel>,
        queuing_strategy: QueuingStrategy<T, Size>,
    ) -> Self
    where
        Start: FnOnce(ReadableStreamDefaultController<T, E>) + 'static,
        Pull: FnMut(ReadableStreamDefaultController<T, E>) + 'static,
        Cancel: FnMut(C) + 'static,
        Size: FnMut(T) -> u32 + 'static,
    {
        let QueuingStrategyIntoWebSys {
            queuing_strategy,
            size_callback,
        } = queuing_strategy.into_web_sys();

        let underlying_source = Object::new();

        let ReadableStreamSource {
            start,
            mut pull,
            mut cancel,
        } = source;

        let start = move |inner: web_sys::ReadableStreamDefaultController| {
            start(ReadableStreamDefaultController {
                inner,
                _marker: Default::default(),
            })
        };

        let start_callback = Closure::once(start);

        js_sys::Reflect::set(
            underlying_source.as_ref(),
            &JsValue::from_str("start"),
            &start_callback.as_ref(),
        )
        .unwrap_throw();

        let pull = move |inner: web_sys::ReadableStreamDefaultController| {
            pull(ReadableStreamDefaultController {
                inner,
                _marker: Default::default(),
            })
        };

        let pull_callback = Closure::new(pull);

        js_sys::Reflect::set(
            underlying_source.as_ref(),
            &JsValue::from_str("pull"),
            &pull_callback.as_ref(),
        )
        .unwrap_throw();

        let cancel = move |reason: JsValue| {
            if let Ok(reason) = reason.dyn_into() {
                cancel(reason)
            } else {
                throw_str("Stream source could not be cancelled with invalid reason type");
            }
        };

        let cancel_callback = Closure::new(cancel);

        js_sys::Reflect::set(
            underlying_source.as_ref(),
            &JsValue::from_str("cancel"),
            &cancel_callback.as_ref(),
        )
        .unwrap_throw();

        // TODO: unstable in web_sys, use custom binding for now, replace later
        // let inner = web_sys::ReadableStream::new_with_underlying_source_and_strategy(
        //     &underlying_source, queuing_strategy.unchecked_ref());
        let inner = create_readable_stream(&underlying_source, &queuing_strategy).unwrap_throw();

        // Collect closures together in a box and register a cleanup operation with the finalization registry
        let closure_state = Box::new(ClosureState {
            start_callback,
            pull_callback,
            cancel_callback,
            size_callback,
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

        READABLE_STREAM_FINALIZATION_REGISTRY
            .with(|r| r.register(inner.as_ref(), serialized.as_ref()));

        CustomReadableStream {
            inner,
            _marker: Default::default(),
        }
    }
}

struct AbortHandleContainer {
    handle: AbortHandle,
}

impl Drop for AbortHandleContainer {
    fn drop(&mut self) {
        self.handle.abort()
    }
}

impl<T> CustomReadableStream<T>
where
    T: JsCast + 'static,
{
    pub fn from_iterator<I, Size>(mut iter: I, queuing_strategy: QueuingStrategy<T, Size>) -> Self
    where
        I: Iterator<Item = T> + 'static,
        Size: FnMut(T) -> u32 + 'static,
    {
        let pull = move |controller: ReadableStreamDefaultController<T>| {
            if let Some(next) = iter.next() {
                controller.enqueue(next);
            } else {
                controller.close();
            }
        };

        CustomReadableStream::from_source(
            ReadableStreamSource {
                start: default_readable_stream_start,
                pull,
                cancel: default_readable_stream_cancel,
            },
            queuing_strategy,
        )
    }

    pub fn from_async_iterator<I, Size>(
        async_iter: I,
        queuing_strategy: QueuingStrategy<T, Size>,
    ) -> Self
    where
        I: Stream<Item = T> + Unpin + 'static,
        Size: FnMut(T) -> u32 + 'static,
    {
        let (abort_handle, abort_registration) = AbortHandle::new_pair();
        let mut abortable = Abortable::new(async_iter, abort_registration);

        let start = move |controller: ReadableStreamDefaultController<T>| {
            spawn_local(async move {
                while let Some(next) = abortable.next().await {
                    controller.enqueue(next);
                }

                controller.close();
            })
        };

        // Wrap the abort handle in a container that will call `abort` on the handle when it gets dropped. This
        // container will be owned by the cancel closure, which will in turn get tied to the brower-owned ReadableStream
        // via a finalization registry. Therefore, when the ReadableStream gets GCed at some point, `abort` will be
        // called, and if a stream was spawned by `start` it's resources will get cleaned-up.
        let abort_handle = Rc::new(AbortHandleContainer {
            handle: abort_handle,
        });

        let cancel = move |_reason: JsValue| {
            let abort_handle = abort_handle.clone();

            abort_handle.handle.abort();
        };

        CustomReadableStream::from_source(
            ReadableStreamSource {
                start,
                pull: default_readable_stream_pull,
                cancel,
            },
            queuing_strategy,
        )
    }
}

impl<T, E, C> readable_stream_seal::Seal for CustomReadableStream<T, E, C> {
    fn as_web_sys(&self) -> &web_sys::ReadableStream {
        &self.inner
    }

    fn from_web_sys(web_sys: web_sys::ReadableStream) -> Self
    where
        Self: Sized,
    {
        CustomReadableStream {
            inner: web_sys,
            _marker: Default::default(),
        }
    }
}

impl<T, E, C> ReadableStream for CustomReadableStream<T, E, C>
where
    T: JsCast,
    E: JsCast,
    C: JsCast,
{
    type Chunk = T;
    type Error = E;
    type Reason = C;
}

pub struct CustomReadableByteStream<E = JsValue, C = JsValue> {
    inner: web_sys::ReadableStream,
    _marker: marker::PhantomData<(E, C)>,
}

impl<E, C> CustomReadableByteStream<E, C>
where
    E: JsCast,
    C: JsCast,
{
    pub fn from_source<Start, Pull, Cancel, Size>(
        source: ReadableByteStreamSource<Start, Pull, Cancel>,
        queuing_strategy: QueuingStrategy<Uint8Array, Size>,
    ) -> Self
    where
        Start: FnOnce(ReadableByteStreamController<E>) + 'static,
        Pull: FnMut(ReadableByteStreamController<E>) + 'static,
        Cancel: FnMut(C) + 'static,
        Size: FnMut(Uint8Array) -> u32 + 'static,
    {
        let QueuingStrategyIntoWebSys {
            queuing_strategy,
            size_callback,
        } = queuing_strategy.into_web_sys();

        let underlying_source = Object::new();

        let ReadableByteStreamSource {
            start,
            mut pull,
            mut cancel,
            auto_allocate_chunk_size,
        } = source;

        js_sys::Reflect::set(
            underlying_source.as_ref(),
            &JsValue::from_str("type"),
            &JsValue::from_str("bytes"),
        )
        .unwrap_throw();

        if let Some(auto_allocate_chunk_size) = auto_allocate_chunk_size {
            js_sys::Reflect::set(
                underlying_source.as_ref(),
                &JsValue::from_str("autoAllocateChunkSize"),
                &JsValue::from(auto_allocate_chunk_size),
            )
            .unwrap_throw();
        }

        let start = move |inner: web_sys::ReadableByteStreamController| {
            start(ReadableByteStreamController {
                inner,
                _marker: Default::default(),
            })
        };

        let start_callback = Closure::once(start);

        js_sys::Reflect::set(
            underlying_source.as_ref(),
            &JsValue::from_str("start"),
            &start_callback.as_ref(),
        )
        .unwrap_throw();

        let pull = move |inner: web_sys::ReadableByteStreamController| {
            pull(ReadableByteStreamController {
                inner,
                _marker: Default::default(),
            })
        };

        let pull_callback = Closure::new(pull);

        js_sys::Reflect::set(
            underlying_source.as_ref(),
            &JsValue::from_str("pull"),
            &pull_callback.as_ref(),
        )
        .unwrap_throw();

        let cancel = move |reason: JsValue| {
            if let Ok(reason) = reason.dyn_into() {
                cancel(reason)
            } else {
                throw_str("Stream source could not be cancelled with invalid reason type");
            }
        };

        let cancel_callback = Closure::new(cancel);

        js_sys::Reflect::set(
            underlying_source.as_ref(),
            &JsValue::from_str("cancel"),
            &cancel_callback.as_ref(),
        )
        .unwrap_throw();

        // TODO: unstable in web_sys, use custom binding for now, replace later
        // let inner = web_sys::ReadableStream::new_with_underlying_source_and_strategy(
        //     &underlying_source, queuing_strategy.unchecked_ref());
        let inner = create_readable_stream(&underlying_source, &queuing_strategy).unwrap_throw();

        // Collect closures together in a box and register a cleanup operation with the finalization registry
        let closure_state = Box::new(ClosureState {
            start_callback,
            pull_callback,
            cancel_callback,
            size_callback,
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

        READABLE_STREAM_FINALIZATION_REGISTRY
            .with(|r| r.register(inner.as_ref(), serialized.as_ref()));

        CustomReadableByteStream {
            inner,
            _marker: Default::default(),
        }
    }
}

impl<E, C> readable_stream_seal::Seal for CustomReadableByteStream<E, C> {
    fn as_web_sys(&self) -> &web_sys::ReadableStream {
        &self.inner
    }

    fn from_web_sys(web_sys: web_sys::ReadableStream) -> Self
    where
        Self: Sized,
    {
        CustomReadableByteStream {
            inner: web_sys,
            _marker: Default::default(),
        }
    }
}

impl<E, C> ReadableStream for CustomReadableByteStream<E, C>
where
    E: JsCast,
    C: JsCast,
{
    type Chunk = Uint8Array;
    type Error = E;
    type Reason = C;
}

impl<E, C> ReadableByteStream for CustomReadableByteStream<E, C>
where
    E: JsCast,
    C: JsCast,
{
}

type_error_wrapper!(ReadableStreamEnqueueError);
type_error_wrapper!(ReadableStreamCloseError);

mod readable_stream_controller_seal {
    pub trait Seal {}
}

pub trait ReadableStreamController<T, E>: readable_stream_controller_seal::Seal
where
    T: JsCast,
    E: JsCast,
{
    fn desired_size(&self) -> i32;

    fn close(&self);

    fn try_close(&self) -> Result<(), ReadableStreamCloseError>;

    fn enqueue(&self, chunk: T);

    fn try_enqueue(&self, chunk: T) -> Result<(), ReadableStreamEnqueueError>;

    fn error(&self, error: E);
}

pub struct ReadableStreamDefaultController<T, E = JsValue> {
    inner: web_sys::ReadableStreamDefaultController,
    _marker: marker::PhantomData<(T, E)>,
}

impl<T, E> readable_stream_controller_seal::Seal for ReadableStreamDefaultController<T, E>
where
    T: JsCast,
    E: JsCast,
{
}

impl<T, E> ReadableStreamController<T, E> for ReadableStreamDefaultController<T, E>
where
    T: JsCast,
    E: JsCast,
{
    fn desired_size(&self) -> i32 {
        self.inner.desired_size().unwrap_or(0.0) as i32
    }

    fn close(&self) {
        self.inner.close().unwrap_throw();
    }

    fn try_close(&self) -> Result<(), ReadableStreamCloseError> {
        self.inner
            .close()
            .map_err(|err| ReadableStreamCloseError::new(err.unchecked_into()))
    }

    fn enqueue(&self, chunk: T) {
        self.inner.enqueue_with_chunk(chunk.as_ref()).unwrap_throw();
    }

    fn try_enqueue(&self, chunk: T) -> Result<(), ReadableStreamEnqueueError> {
        self.inner
            .enqueue_with_chunk(chunk.as_ref())
            .map_err(|err| ReadableStreamEnqueueError::new(err.unchecked_into()))
    }

    fn error(&self, error: E) {
        self.inner.error_with_e(error.as_ref())
    }
}

pub struct ReadableStreamByobRequest {
    inner: web_sys::ReadableStreamByobRequest,
}

impl ReadableStreamByobRequest {
    pub fn view(&self) -> Uint8Array {
        self.inner.view().unwrap_throw().unchecked_into()
    }

    pub fn respond(self, bytes_written: u32) {
        self.inner.respond_with_u32(bytes_written).unwrap_throw();
    }
}

pub struct ReadableByteStreamController<E = JsValue> {
    inner: web_sys::ReadableByteStreamController,
    _marker: marker::PhantomData<E>,
}

impl<E> ReadableByteStreamController<E> {
    pub fn byob_request(&self) -> Option<ReadableStreamByobRequest> {
        self.inner
            .byob_request()
            .map(|inner| ReadableStreamByobRequest { inner })
    }
}

impl<E> readable_stream_controller_seal::Seal for ReadableByteStreamController<E> where E: JsCast {}

impl<E> ReadableStreamController<Uint8Array, E> for ReadableByteStreamController<E>
where
    E: JsCast,
{
    fn desired_size(&self) -> i32 {
        self.inner.desired_size().unwrap_or(0.0) as i32
    }

    fn close(&self) {
        self.inner.close().unwrap_throw();
    }

    fn try_close(&self) -> Result<(), ReadableStreamCloseError> {
        self.inner
            .close()
            .map_err(|err| ReadableStreamCloseError::new(err.unchecked_into()))
    }

    fn enqueue(&self, chunk: Uint8Array) {
        self.inner
            .enqueue_with_array_buffer_view(chunk.as_ref())
            .unwrap_throw();
    }

    fn try_enqueue(&self, chunk: Uint8Array) -> Result<(), ReadableStreamEnqueueError> {
        self.inner
            .enqueue_with_array_buffer_view(chunk.as_ref())
            .map_err(|err| ReadableStreamEnqueueError::new(err.unchecked_into()))
    }

    fn error(&self, error: E) {
        self.inner.error_with_e(error.as_ref())
    }
}

mod readable_stream_reader_seal {
    pub trait Seal {}
}

pub trait ReadableStreamReader<E, C>: readable_stream_reader_seal::Seal + Sized
where
    E: JsCast,
    C: JsCast,
{
    fn cancel(&self, reason: C) -> ReadableStreamCancel<E>;

    fn on_close(&self) -> ReadableStreamOnClose<E>;

    fn release_lock(self);
}

pub struct ReadableStreamDefaultReader<T, E = JsValue, C = JsValue> {
    inner: web_sys::ReadableStreamDefaultReader,
    _marker: marker::PhantomData<(T, E, C)>,
}

impl<T, E, C> ReadableStreamDefaultReader<T, E, C>
where
    T: JsCast,
    E: JsCast,
    C: JsCast,
{
    pub fn read(&self) -> ReadableStreamRead<T, E> {
        ReadableStreamRead {
            inner: self.inner.read().into(),
            _marker: Default::default(),
        }
    }
}

impl<E, C> readable_stream_reader_seal::Seal for ReadableStreamDefaultReader<E, C>
where
    E: JsCast,
    C: JsCast,
{
}

impl<E, C> ReadableStreamReader<E, C> for ReadableStreamDefaultReader<E, C>
where
    E: JsCast,
    C: JsCast,
{
    fn cancel(&self, reason: C) -> ReadableStreamCancel<E> {
        ReadableStreamCancel {
            inner: self.inner.cancel_with_reason(reason.as_ref()).into(),
            _marker: Default::default(),
        }
    }

    fn on_close(&self) -> ReadableStreamOnClose<E> {
        ReadableStreamOnClose {
            inner: self.inner.closed().into(),
            _marker: Default::default(),
        }
    }

    fn release_lock(self) {
        self.inner.release_lock();
    }
}

pub struct ReadableStreamByobReader<E = JsValue, C = JsValue> {
    inner: web_sys::ReadableStreamByobReader,
    _marker: marker::PhantomData<(E, C)>,
}

impl<E, C> ReadableStreamByobReader<E, C>
where
    E: JsCast,
    C: JsCast,
{
    pub fn read(&self, view: Uint8Array) -> ReadableStreamRead<Uint8Array, E> {
        ReadableStreamRead {
            inner: self.inner.read_with_array_buffer_view(view.as_ref()).into(),
            _marker: Default::default(),
        }
    }
}

impl<E, C> readable_stream_reader_seal::Seal for ReadableStreamByobReader<E, C>
where
    E: JsCast,
    C: JsCast,
{
}

impl<E, C> ReadableStreamReader<E, C> for ReadableStreamByobReader<E, C>
where
    E: JsCast,
    C: JsCast,
{
    fn cancel(&self, reason: C) -> ReadableStreamCancel<E> {
        ReadableStreamCancel {
            inner: self.inner.cancel_with_reason(reason.as_ref()).into(),
            _marker: Default::default(),
        }
    }

    fn on_close(&self) -> ReadableStreamOnClose<E> {
        ReadableStreamOnClose {
            inner: self.inner.closed().into(),
            _marker: Default::default(),
        }
    }

    fn release_lock(self) {
        self.inner.release_lock();
    }
}

pub struct ReadableStreamError<E> {
    inner: JsValue,
    _marker: marker::PhantomData<E>,
}

impl<E> ReadableStreamError<E> {
    fn new(inner: JsValue) -> Self {
        ReadableStreamError {
            inner,
            _marker: Default::default(),
        }
    }
}

impl<E> ReadableStreamError<E>
where
    E: JsCast,
{
    pub fn try_into_source_error(self) -> Result<E, ReadableStreamError<E>> {
        self.inner
            .dyn_into::<E>()
            .map_err(|err| ReadableStreamError::new(err))
    }
}

impl<E> fmt::Debug for ReadableStreamError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<E> fmt::Display for ReadableStreamError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ReadableStreamError: {}",
            self.inner.as_string().unwrap_or_default()
        )
    }
}

pub struct PipeStreamError<ESource, ESink> {
    inner: JsValue,
    _marker: marker::PhantomData<(ESource, ESink)>,
}

impl<ESource, ESink> PipeStreamError<ESource, ESink> {
    fn new(inner: JsValue) -> Self {
        PipeStreamError {
            inner,
            _marker: Default::default(),
        }
    }
}

impl<ESource, ESink> PipeStreamError<ESource, ESink>
where
    ESource: JsCast,
{
    pub fn try_into_source_error(self) -> Result<ESource, PipeStreamError<ESource, ESink>> {
        self.inner
            .dyn_into::<ESource>()
            .map_err(|err| PipeStreamError::new(err))
    }
}

impl<ESource, ESink> PipeStreamError<ESource, ESink>
where
    ESink: JsCast,
{
    pub fn try_into_sink_error(self) -> Result<ESink, PipeStreamError<ESource, ESink>> {
        self.inner
            .dyn_into::<ESink>()
            .map_err(|err| PipeStreamError::new(err))
    }
}

impl<ESource, ESink> fmt::Debug for PipeStreamError<ESource, ESink> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<ESource, ESink> fmt::Display for PipeStreamError<ESource, ESink> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PipeStreamError: {}",
            self.inner.as_string().unwrap_or_default()
        )
    }
}

#[pin_project]
pub struct ReadableStreamRead<T, E = JsValue> {
    #[pin]
    inner: JsFuture,
    _marker: marker::PhantomData<(T, E)>,
}

impl<T, E> Future for ReadableStreamRead<T, E>
where
    T: JsCast,
    E: JsCast,
{
    type Output = Result<Option<T>, ReadableStreamError<E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|ok| {
                let value = ok.unchecked_into::<ReadableStreamReadValue>();

                if !value.done() {
                    Some(value.value().unchecked_into())
                } else {
                    None
                }
            })
            .map_err(|err| ReadableStreamError::new(err))
    }
}

#[pin_project]
pub struct ReadableStreamCancel<E = JsValue> {
    #[pin]
    inner: JsFuture,
    _marker: marker::PhantomData<E>,
}

impl<E> Future for ReadableStreamCancel<E>
where
    E: JsCast,
{
    type Output = Result<(), ReadableStreamError<E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|_| ())
            .map_err(|err| ReadableStreamError::new(err))
    }
}

#[pin_project]
pub struct ReadableStreamOnClose<E> {
    #[pin]
    inner: JsFuture,
    _marker: marker::PhantomData<E>,
}

impl<E> Future for ReadableStreamOnClose<E>
where
    E: JsCast,
{
    type Output = Result<(), ReadableStreamError<E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|_| ())
            .map_err(|err| ReadableStreamError::new(err))
    }
}

#[pin_project]
pub struct ReadableStreamPipeTo<ESource = JsValue, ESink = JsValue> {
    #[pin]
    inner: JsFuture,
    _marker: marker::PhantomData<(ESource, ESink)>,
}

impl<ESource, ESink> Future for ReadableStreamPipeTo<ESource, ESink>
where
    ESource: JsCast,
    ESink: JsCast,
{
    type Output = Result<(), PipeStreamError<ESource, ESink>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|_| ())
            .map_err(|err| PipeStreamError::new(err))
    }
}

pub struct ReadableStreamChunks<T, E, C> {
    reader: ReadableStreamDefaultReader<T, E, C>,
    current_read: Option<ReadableStreamRead<T, E>>,
}

impl<T, E, C> Stream for ReadableStreamChunks<T, E, C>
where
    T: JsCast,
    E: JsCast,
    C: JsCast,
{
    type Item = Result<T, ReadableStreamError<E>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = unsafe { self.get_unchecked_mut() };

        if this.current_read.is_none() {
            this.current_read = Some(this.reader.read());
        }

        Pin::new(this.current_read.as_mut().unwrap())
            .poll(cx)
            .map(|p| {
                this.current_read = Some(this.reader.read());

                match p {
                    Ok(Some(v)) => Some(Ok(v)),
                    Ok(None) => None,
                    Err(err) => Some(Err(err)),
                }
            })
    }
}

#[wasm_bindgen(module = "/src/js_support.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = __arwa_create_readable_stream)]
    fn create_readable_stream(
        underlying_source: &Object,
        queuing_strategy: &Object,
    ) -> Result<web_sys::ReadableStream, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    type ReadableStreamReadValue;

    #[wasm_bindgen(method, getter)]
    fn value(this: &ReadableStreamReadValue) -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn done(this: &ReadableStreamReadValue) -> bool;
}
