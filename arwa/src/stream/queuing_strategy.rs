use std::marker;

use js_sys::Object;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{throw_str, JsCast, JsValue, UnwrapThrowExt};

pub struct QueuingStrategy<T, Size> {
    pub(super) internal: QueuingStrategyInternal<Size>,
    _marker: marker::PhantomData<T>,
}

impl<T> QueuingStrategy<T, fn(T) -> u32> {
    pub fn count(high_water_mark: u32) -> Self {
        QueuingStrategy {
            internal: QueuingStrategyInternal::Count(high_water_mark),
            _marker: Default::default(),
        }
    }

    pub fn byte_length(high_water_mark: u32) -> Self {
        QueuingStrategy {
            internal: QueuingStrategyInternal::ByteLength(high_water_mark),
            _marker: Default::default(),
        }
    }
}

impl<T, Size> QueuingStrategy<T, Size>
where
    Size: FnMut(T) -> u32,
{
    pub fn custom(high_water_mark: u32, size: Size) -> Self {
        QueuingStrategy {
            internal: QueuingStrategyInternal::Custom {
                high_water_mark,
                size,
            },
            _marker: Default::default(),
        }
    }
}

pub(super) enum QueuingStrategyInternal<Size> {
    ByteLength(u32),
    Count(u32),
    Custom { high_water_mark: u32, size: Size },
}

impl<T> Default for QueuingStrategy<T, fn(T) -> u32> {
    fn default() -> Self {
        QueuingStrategy::count(1)
    }
}

pub(super) struct QueuingStrategyIntoWebSys {
    pub(super) queuing_strategy: Object,
    pub(super) size_callback: Option<Closure<dyn FnMut(JsValue) -> u32>>,
}

impl<T, Size> QueuingStrategy<T, Size>
where
    T: JsCast,
    Size: FnMut(T) -> u32 + 'static,
{
    pub(super) fn into_web_sys(self) -> QueuingStrategyIntoWebSys {
        let mut size_callback = None;

        let queuing_strategy: Object = match self.internal {
            QueuingStrategyInternal::ByteLength(high_water_mark) => {
                let init = web_sys::QueuingStrategyInit::new(high_water_mark as f64);

                web_sys::ByteLengthQueuingStrategy::new(&init)
                    .unwrap_throw()
                    .into()
            }
            QueuingStrategyInternal::Count(high_water_mark) => {
                let init = web_sys::QueuingStrategyInit::new(high_water_mark as f64);

                web_sys::CountQueuingStrategy::new(&init)
                    .unwrap_throw()
                    .into()
            }
            QueuingStrategyInternal::Custom {
                high_water_mark,
                mut size,
            } => {
                let size = move |chunk: JsValue| {
                    if let Ok(chunk) = chunk.dyn_into() {
                        size(chunk)
                    } else {
                        throw_str("Could not determine size of invalid chunk type");
                    }
                };

                let cb = Closure::new(size);

                // TODO: this is current unstable in web-sys. Build Object for now, but replace
                // when possible
                // let mut strategy = web_sys::QueuingStrategy::new();
                //
                // strategy.high_water_mark(high_water_mark as f64);
                // strategy.size(cb.unchecked_ref());

                let strategy = Object::new();

                js_sys::Reflect::set(
                    strategy.as_ref(),
                    &JsValue::from_str("high_water_mark"),
                    &JsValue::from(high_water_mark),
                )
                .unwrap_throw();
                js_sys::Reflect::set(strategy.as_ref(), &JsValue::from_str("size"), &cb.as_ref())
                    .unwrap_throw();

                size_callback = Some(cb);

                strategy
            }
        };

        QueuingStrategyIntoWebSys {
            queuing_strategy,
            size_callback,
        }
    }
}
