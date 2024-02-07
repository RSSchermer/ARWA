use std::any::Any;
use std::error::Error;
use std::future::Future;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr::DynMetadata;
use std::task::{Context, Poll};
use std::{fmt, marker, mem, ptr};

use js_sys::{Object, Uint8Array};
use pin_project::pin_project;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{throw_str, JsCast, JsError, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;

use crate::finalization_registry::FinalizationRegistry;
use crate::js_serialize::{js_deserialize, js_serialize};
use crate::stream::{QueuingStrategy, QueuingStrategyIntoWebSys};
use crate::type_error_wrapper;

thread_local! {
    static WRITABLE_STREAM_FINALIZATION_REGISTRY: FinalizationRegistry = {
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

type_error_wrapper!(GetWriterError);

pub(crate) mod writable_stream_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys(&self) -> &web_sys::WritableStream;
    }
}

pub trait WritableStream: writable_stream_seal::Seal + Sized {
    type Chunk: JsCast;

    type Error: JsCast;

    type Reason: JsCast;

    fn is_locked(&self) -> bool {
        self.as_web_sys().locked()
    }

    fn get_writer(&self) -> WritableStreamDefaultWriter<Self::Chunk, Self::Error, Self::Reason> {
        WritableStreamDefaultWriter {
            inner: self.as_web_sys().get_writer().unwrap_throw(),
            _marker: Default::default(),
        }
    }

    fn try_get_writer(
        &self,
    ) -> Result<WritableStreamDefaultWriter<Self::Chunk, Self::Error, Self::Reason>, GetWriterError>
    {
        self.as_web_sys()
            .get_writer()
            .map(|inner| WritableStreamDefaultWriter {
                inner,
                _marker: Default::default(),
            })
            .map_err(|err| GetWriterError::new(err.unchecked_into()))
    }

    fn abort(&self, reason: Self::Reason) -> WritableStreamAbort<Self::Reason, Self::Error> {
        WritableStreamAbort {
            inner: self.as_web_sys().abort_with_reason(reason.as_ref()).into(),
            _marker: Default::default(),
        }
    }

    fn close(&self) -> WritableStreamClose<Self::Error> {
        WritableStreamClose {
            inner: self.as_web_sys().close().into(),
            _marker: Default::default(),
        }
    }
}

pub struct WritableStreamSink<Start, Write, Close, Abort> {
    pub start: Start,
    pub write: Write,
    pub cancel: Close,
    pub abort: Abort,
}

pub fn default_writable_stream_start<E, C>(_: WritableStreamDefaultController<E, C>) {}
pub fn default_writable_stream_write<T, E, C>(_: T, _: WritableStreamDefaultController<E, C>) {}
pub fn default_writable_stream_close<E, C>(_: WritableStreamDefaultController<E, C>) {}
pub fn default_writable_stream_abort<C>(_: C) {}

#[allow(unused)]
struct ClosureState<Start: ?Sized, Write: ?Sized, Close: ?Sized, Abort: ?Sized, Size: ?Sized> {
    start_callback: Closure<Start>,
    write_callback: Closure<Write>,
    close_callback: Closure<Close>,
    abort_callback: Closure<Abort>,
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

pub struct CustomWritableStream<T, E = JsValue, C = JsValue> {
    pub(crate) inner: web_sys::WritableStream,
    pub(crate) _marker: marker::PhantomData<(T, E, C)>,
}

impl<T, E, C> CustomWritableStream<T, E, C>
where
    T: JsCast,
    E: JsCast,
    C: JsCast,
{
    pub(crate) fn from_web_sys(inner: web_sys::WritableStream) -> Self {
        CustomWritableStream {
            inner,
            _marker: Default::default(),
        }
    }

    pub fn from_sink<Start, Write, Close, Abort, Size>(
        sink: WritableStreamSink<Start, Write, Close, Abort>,
        queuing_strategy: QueuingStrategy<T, Size>,
    ) -> Self
    where
        Start: FnOnce(WritableStreamDefaultController<E, C>) + 'static,
        Write: FnMut(T, WritableStreamDefaultController<E, C>) + 'static,
        Close: FnMut(WritableStreamDefaultController<E, C>) + 'static,
        Abort: FnMut(C) + 'static,
        Size: FnMut(T) -> u32 + 'static,
    {
        let QueuingStrategyIntoWebSys {
            queuing_strategy,
            size_callback,
        } = queuing_strategy.into_web_sys();

        let underlying_sink = Object::new();

        let WritableStreamSink {
            start,
            mut write,
            mut cancel,
            mut abort,
        } = sink;

        let start = move |inner: web_sys::WritableStreamDefaultController| {
            start(WritableStreamDefaultController {
                inner,
                _marker: Default::default(),
            })
        };

        let start_callback = Closure::once(start);

        js_sys::Reflect::set(
            underlying_sink.as_ref(),
            &JsValue::from_str("start"),
            &start_callback.as_ref(),
        )
        .unwrap_throw();

        let write = move |chunk: JsValue, inner: web_sys::WritableStreamDefaultController| {
            if let Ok(chunk) = chunk.dyn_into() {
                write(
                    chunk,
                    WritableStreamDefaultController {
                        inner,
                        _marker: Default::default(),
                    },
                )
            } else {
                inner.error_with_e(&JsError::new("Could not write invalid chunk type").into());
            }
        };

        let write_callback = Closure::new(write);

        js_sys::Reflect::set(
            underlying_sink.as_ref(),
            &JsValue::from_str("write"),
            &write_callback.as_ref(),
        )
        .unwrap_throw();

        let close = move |inner: web_sys::WritableStreamDefaultController| {
            cancel(WritableStreamDefaultController {
                inner,
                _marker: Default::default(),
            })
        };

        let close_callback = Closure::new(close);

        js_sys::Reflect::set(
            underlying_sink.as_ref(),
            &JsValue::from_str("close"),
            &close_callback.as_ref(),
        )
        .unwrap_throw();

        let abort = move |reason: JsValue| {
            if let Ok(reason) = reason.dyn_into() {
                abort(reason)
            } else {
                throw_str("Could not abort with invalid reason type");
            }
        };

        let abort_callback = Closure::new(abort);

        js_sys::Reflect::set(
            underlying_sink.as_ref(),
            &JsValue::from_str("abort"),
            &abort_callback.as_ref(),
        )
        .unwrap_throw();

        // TODO: unstable in web_sys, use custom binding for now, replace later
        // let inner = web_sys::ReadableStream::new_with_underlying_source_and_strategy(
        //     &underlying_source, queuing_strategy.unchecked_ref());
        let inner = create_writable_stream(&underlying_sink, &queuing_strategy).unwrap_throw();

        // Collect closures together in a box and register a cleanup operation with the finalization registry
        let closure_state = Box::new(ClosureState {
            start_callback,
            write_callback,
            close_callback,
            abort_callback,
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

        WRITABLE_STREAM_FINALIZATION_REGISTRY
            .with(|r| r.register(inner.as_ref(), serialized.as_ref()));

        CustomWritableStream {
            inner,
            _marker: Default::default(),
        }
    }
}

impl<T, E, C> writable_stream_seal::Seal for CustomWritableStream<T, E, C>
where
    T: JsCast,
    E: JsCast,
    C: JsCast,
{
    fn as_web_sys(&self) -> &web_sys::WritableStream {
        &self.inner
    }
}

impl<T, E, C> WritableStream for CustomWritableStream<T, E, C>
where
    T: JsCast,
    E: JsCast,
    C: JsCast,
{
    type Chunk = T;
    type Error = E;
    type Reason = C;
}

pub struct WritableStreamError<E> {
    inner: JsValue,
    _marker: marker::PhantomData<E>,
}

impl<E> WritableStreamError<E> {
    fn new(inner: JsValue) -> Self {
        WritableStreamError {
            inner,
            _marker: Default::default(),
        }
    }
}

impl<E> WritableStreamError<E>
where
    E: JsCast,
{
    pub fn try_into_sink_error(self) -> Result<E, WritableStreamError<E>> {
        self.inner
            .dyn_into::<E>()
            .map_err(|err| WritableStreamError::new(err))
    }
}

impl<E> fmt::Debug for WritableStreamError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<E> fmt::Display for WritableStreamError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WritableStreamError: {}",
            self.inner.as_string().unwrap_or_default()
        )
    }
}

impl<E> Error for WritableStreamError<E> {}

pub struct WritableStreamDefaultController<E = JsValue, C = JsValue> {
    inner: web_sys::WritableStreamDefaultController,
    _marker: marker::PhantomData<(E, C)>,
}

impl<E, C> WritableStreamDefaultController<E, C> {
    pub fn is_aborted(&self) -> bool {
        self.inner.signal().aborted()
    }
}

pub enum WritableStreamAbortReason<C> {
    Default,
    Custom(C),
}

impl<E, C> WritableStreamDefaultController<E, C>
where
    C: JsCast,
{
    // TODO: reason currently missing on AbortSignal in web_sys
    // pub fn abort_reason(&self) -> Option<WritableStreamAbortReason<C>> {
    //     if self.is_aborted() {
    //         let reason = self.inner.signal().reason();
    //
    //         if let Ok(reason) = reason.dyn_into::<C>() {
    //             Some(WritableStreamAbortReason::Custom(reason))
    //         } else {
    //             Some(WritableStreamAbortReason::Default)
    //         }
    //     } else {
    //         None
    //     }
    // }
}

impl<E, C> WritableStreamDefaultController<E, C>
where
    E: JsCast,
{
    pub fn error(&self, error: E) {
        self.inner.error_with_e(error.as_ref());
    }
}

pub struct WritableStreamDefaultWriter<T, E, C> {
    inner: web_sys::WritableStreamDefaultWriter,
    _marker: marker::PhantomData<(T, E, C)>,
}

impl<T, E, C> WritableStreamDefaultWriter<T, E, C> {
    pub fn desired_size(&self) -> Option<i32> {
        self.inner.desired_size().unwrap_throw().map(|s| s as i32)
    }

    pub fn release_lock(self) {
        self.inner.release_lock();
    }
}

impl<T, E, C> WritableStreamDefaultWriter<T, E, C>
where
    E: JsCast,
{
    pub fn on_close(&self) -> WritableStreamOnClose<E> {
        WritableStreamOnClose {
            inner: self.inner.close().into(),
            _marker: Default::default(),
        }
    }

    pub fn on_ready(&self) -> WritableStreamOnReady<E> {
        WritableStreamOnReady {
            inner: self.inner.ready().into(),
            _marker: Default::default(),
        }
    }
}

impl<T, E, C> WritableStreamDefaultWriter<T, E, C>
where
    T: JsCast,
    E: JsCast,
{
    pub fn write(&self, chunk: T) -> WritableStreamWrite<E> {
        WritableStreamWrite {
            inner: self.inner.write_with_chunk(chunk.as_ref()).into(),
            _marker: Default::default(),
        }
    }
}

impl<T, E, C> WritableStreamDefaultWriter<T, E, C>
where
    E: JsCast,
    C: JsCast,
{
    pub fn abort(&self, reason: C) -> WritableStreamAbort<C, E> {
        WritableStreamAbort {
            inner: self.inner.abort_with_reason(reason.as_ref()).into(),
            _marker: Default::default(),
        }
    }
}

impl<T, E, C> WritableStreamDefaultWriter<T, E, C>
where
    E: JsCast,
{
    pub fn close(&self) -> WritableStreamClose<E> {
        WritableStreamClose {
            inner: self.inner.close().into(),
            _marker: Default::default(),
        }
    }
}

#[pin_project]
pub struct WritableStreamOnClose<E> {
    #[pin]
    inner: JsFuture,
    _marker: marker::PhantomData<E>,
}

impl<E> Future for WritableStreamOnClose<E>
where
    E: JsCast,
{
    type Output = Result<(), WritableStreamError<E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|_| ())
            .map_err(|err| WritableStreamError::new(err))
    }
}

#[pin_project]
pub struct WritableStreamOnReady<E> {
    #[pin]
    inner: JsFuture,
    _marker: marker::PhantomData<E>,
}

impl<E> Future for WritableStreamOnReady<E>
where
    E: JsCast,
{
    type Output = Result<(), WritableStreamError<E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|_| ())
            .map_err(|err| WritableStreamError::new(err))
    }
}

#[pin_project]
pub struct WritableStreamAbort<C, E> {
    #[pin]
    inner: JsFuture,
    _marker: marker::PhantomData<(C, E)>,
}

impl<C, E> Future for WritableStreamAbort<C, E>
where
    C: JsCast,
    E: JsCast,
{
    type Output = Result<C, WritableStreamError<E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|r| r.unchecked_into())
            .map_err(|err| WritableStreamError::new(err))
    }
}

#[pin_project]
pub struct WritableStreamClose<E> {
    #[pin]
    inner: JsFuture,
    _marker: marker::PhantomData<E>,
}

impl<E> Future for WritableStreamClose<E>
where
    E: JsCast,
{
    type Output = Result<(), WritableStreamError<E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|_| ())
            .map_err(|err| WritableStreamError::new(err))
    }
}

#[pin_project]
pub struct WritableStreamWrite<E> {
    #[pin]
    inner: JsFuture,
    _marker: marker::PhantomData<E>,
}

impl<E> Future for WritableStreamWrite<E>
where
    E: JsCast,
{
    type Output = Result<(), WritableStreamError<E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|_| ())
            .map_err(|err| WritableStreamError::new(err))
    }
}

#[wasm_bindgen(module = "/src/js_support.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = __arwa_create_writable_stream)]
    fn create_writable_stream(
        underlying_sink: &Object,
        queuing_strategy: &Object,
    ) -> Result<web_sys::WritableStream, JsValue>;
}
