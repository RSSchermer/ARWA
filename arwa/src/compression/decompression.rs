use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

use crate::stream::{
    readable_stream_seal, writable_stream_seal, ReadableStream, TransformStream, WritableStream,
};

pub struct DecompressionStream {
    inner: DecompressionStreamInternal,
}

impl DecompressionStream {
    pub fn gzip() -> Self {
        DecompressionStream {
            inner: DecompressionStreamInternal::new("gzip"),
        }
    }

    pub fn deflate() -> Self {
        DecompressionStream {
            inner: DecompressionStreamInternal::new("deflate"),
        }
    }

    pub fn deflate_raw() -> Self {
        DecompressionStream {
            inner: DecompressionStreamInternal::new("deflate-raw"),
        }
    }
}

pub struct DecompressionWritableStream {
    inner: web_sys::WritableStream,
}

impl writable_stream_seal::Seal for DecompressionWritableStream {
    fn as_web_sys(&self) -> &web_sys::WritableStream {
        &self.inner
    }
}

impl WritableStream for DecompressionWritableStream {
    type Chunk = Uint8Array;
    type Error = JsValue;
    type Reason = JsValue;
}

pub struct DecompressionReadableStream {
    inner: web_sys::ReadableStream,
}

impl readable_stream_seal::Seal for DecompressionReadableStream {
    fn as_web_sys(&self) -> &web_sys::ReadableStream {
        &self.inner
    }

    fn from_web_sys(web_sys: web_sys::ReadableStream) -> Self
    where
        Self: Sized,
    {
        DecompressionReadableStream { inner: web_sys }
    }
}

impl ReadableStream for DecompressionReadableStream {
    type Chunk = Uint8Array;
    type Error = JsValue;
    type Reason = JsValue;
}

impl TransformStream for DecompressionStream {
    type In = Uint8Array;

    type Writable = DecompressionWritableStream;
    type Readable = DecompressionReadableStream;

    fn writable(&self) -> Self::Writable {
        DecompressionWritableStream {
            inner: self.inner.writable(),
        }
    }

    fn readable(&self) -> Self::Readable {
        DecompressionReadableStream {
            inner: self.inner.readable(),
        }
    }
}

// TODO: no web_sys bindings currently, custom bindings for now, replace later

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = DecompressionStream)]
    type DecompressionStreamInternal;

    #[wasm_bindgen(constructor)]
    fn new(format: &str) -> DecompressionStreamInternal;

    #[wasm_bindgen(method, getter)]
    fn readable(this: &DecompressionStreamInternal) -> web_sys::ReadableStream;

    #[wasm_bindgen(method, getter)]
    fn writable(this: &DecompressionStreamInternal) -> web_sys::WritableStream;
}
