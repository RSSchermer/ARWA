use arwa::console;
use arwa::spawn_local;
use arwa::stream::{CustomReadableStream, ReadableStream};
use arwa::timer::{Duration, TimerContext};
use arwa::window::window;
use futures::StreamExt;
use wasm_bindgen::JsValue;

fn main() {
    let source = window()
        .interval(Duration::Milliseconds(5000))
        .map(|_| JsValue::from_str("hello"))
        .take(5);

    let stream = CustomReadableStream::from_async_iterator(source, Default::default());

    spawn_local(async move {
        let mut chunks = stream.into_chunks();

        while let Some(chunk) = chunks.next().await {
            console::log!(chunk.unwrap());
        }
    })
}
