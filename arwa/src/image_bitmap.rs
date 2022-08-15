use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use delegate::delegate;
use pin_project::pin_project;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::file::Blob;
use crate::html::{HtmlCanvasElement, HtmlImgElement, HtmlVideoElement};
use crate::{dom_exception_wrapper, impl_common_wrapper_traits, impl_js_cast};

#[derive(Clone)]
pub struct ImageBitmap {
    inner: web_sys::ImageBitmap,
}

impl ImageBitmap {
    delegate! {
        to self.inner {
            pub fn width(&self) -> u32;

            pub fn height(&self) -> u32;

            pub fn close(&self);
        }
    }
}

impl AsRef<web_sys::ImageBitmap> for ImageBitmap {
    fn as_ref(&self) -> &web_sys::ImageBitmap {
        &self.inner
    }
}

impl From<web_sys::ImageBitmap> for ImageBitmap {
    fn from(inner: web_sys::ImageBitmap) -> ImageBitmap {
        ImageBitmap { inner }
    }
}

impl Into<web_sys::ImageBitmap> for ImageBitmap {
    fn into(self) -> web_sys::ImageBitmap {
        self.inner
    }
}

impl_common_wrapper_traits!(ImageBitmap);
impl_js_cast!(ImageBitmap);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ImageRegion {
    pub x: u32,
    pub y: u32,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ImageOrientation {
    None,
    FlipY,
}

impl ImageOrientation {
    fn to_str(&self) -> &str {
        match self {
            ImageOrientation::None => "none",
            ImageOrientation::FlipY => "flipY",
        }
    }
}

impl Default for ImageOrientation {
    fn default() -> Self {
        ImageOrientation::None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PremultiplyAlpha {
    None,
    Premultiply,
    Default,
}

impl PremultiplyAlpha {
    fn to_str(&self) -> &str {
        match self {
            PremultiplyAlpha::None => "none",
            PremultiplyAlpha::Premultiply => "premultiply",
            PremultiplyAlpha::Default => "default",
        }
    }
}

impl Default for PremultiplyAlpha {
    fn default() -> Self {
        PremultiplyAlpha::Default
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorSpaceConversion {
    None,
    Default,
}

impl ColorSpaceConversion {
    fn to_str(&self) -> &str {
        match self {
            ColorSpaceConversion::None => "none",
            ColorSpaceConversion::Default => "default",
        }
    }
}

impl Default for ColorSpaceConversion {
    fn default() -> Self {
        ColorSpaceConversion::Default
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResizeQuality {
    Pixelated,
    Low,
    Medium,
    High,
}

impl ResizeQuality {
    fn to_str(&self) -> &str {
        match self {
            ResizeQuality::Pixelated => "pixelated",
            ResizeQuality::Low => "low",
            ResizeQuality::Medium => "medium",
            ResizeQuality::High => "high",
        }
    }
}

impl Default for ResizeQuality {
    fn default() -> Self {
        ResizeQuality::Low
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ImageBitmapOptions {
    pub image_orientation: ImageOrientation,
    pub premultiply_alpha: PremultiplyAlpha,
    pub color_space_conversion: ColorSpaceConversion,
    pub resize_width: Option<u32>,
    pub resize_height: Option<u32>,
    pub resize_quality: ResizeQuality,
}

impl ImageBitmapOptions {
    fn to_js_value(&self) -> JsValue {
        let value = js_sys::Object::new().into();

        let ImageBitmapOptions {
            image_orientation,
            premultiply_alpha,
            color_space_conversion,
            resize_width,
            resize_height,
            resize_quality,
        } = *self;

        if image_orientation != ImageOrientation::default() {
            let _ = js_sys::Reflect::set(
                &value,
                &JsValue::from_str("imageOrientation"),
                &JsValue::from_str(image_orientation.to_str()),
            );
        }

        if premultiply_alpha != PremultiplyAlpha::default() {
            let _ = js_sys::Reflect::set(
                &value,
                &JsValue::from_str("premultiplyAlpha"),
                &JsValue::from_str(premultiply_alpha.to_str()),
            );
        }

        if color_space_conversion != ColorSpaceConversion::default() {
            let _ = js_sys::Reflect::set(
                &value,
                &JsValue::from_str("colorSpaceConversion"),
                &JsValue::from_str(color_space_conversion.to_str()),
            );
        }

        if let Some(resize_width) = resize_width {
            let _ = js_sys::Reflect::set(
                &value,
                &JsValue::from_str("resizeWidth"),
                &JsValue::from(resize_width),
            );
        }

        if let Some(resize_height) = resize_height {
            let _ = js_sys::Reflect::set(
                &value,
                &JsValue::from_str("resizeHeight"),
                &JsValue::from(resize_height),
            );
        }

        if resize_quality != ResizeQuality::default() {
            let _ = js_sys::Reflect::set(
                &value,
                &JsValue::from_str("resizeQuality"),
                &JsValue::from_str(resize_quality.to_str()),
            );
        }

        value
    }
}

mod create_image_bitmap_source_seal {
    use wasm_bindgen::JsValue;

    pub trait Seal {
        #[doc(hidden)]
        fn as_js_value(&self) -> &JsValue;
    }
}

pub trait CreateImageBitmapSource: create_image_bitmap_source_seal::Seal {}

impl create_image_bitmap_source_seal::Seal for HtmlImgElement {
    fn as_js_value(&self) -> &JsValue {
        self.as_ref()
    }
}
impl CreateImageBitmapSource for HtmlImgElement {}

impl create_image_bitmap_source_seal::Seal for HtmlCanvasElement {
    fn as_js_value(&self) -> &JsValue {
        self.as_ref()
    }
}
impl CreateImageBitmapSource for HtmlCanvasElement {}

impl create_image_bitmap_source_seal::Seal for HtmlVideoElement {
    fn as_js_value(&self) -> &JsValue {
        self.as_ref()
    }
}
impl CreateImageBitmapSource for HtmlVideoElement {}

impl create_image_bitmap_source_seal::Seal for Blob {
    fn as_js_value(&self) -> &JsValue {
        self.as_ref()
    }
}
impl CreateImageBitmapSource for Blob {}

impl create_image_bitmap_source_seal::Seal for ImageBitmap {
    fn as_js_value(&self) -> &JsValue {
        self.as_ref()
    }
}
impl CreateImageBitmapSource for ImageBitmap {}

pub fn create_image_bitmap<S>(
    image: &S,
    region: ImageRegion,
    options: ImageBitmapOptions,
) -> CreateImageBitmap
where
    S: CreateImageBitmapSource,
{
    let ImageRegion {
        x,
        y,
        width,
        height,
    } = region;

    let promise = js_create_image_bitmap(
        image.as_js_value(),
        x,
        y,
        width,
        height,
        &options.to_js_value(),
    );

    CreateImageBitmap {
        inner: promise.into(),
    }
}

dom_exception_wrapper!(CreateImageBitmapError);

#[pin_project]
pub struct CreateImageBitmap {
    #[pin]
    inner: JsFuture,
}

impl Future for CreateImageBitmap {
    type Output = Result<ImageBitmap, CreateImageBitmapError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|ok| ImageBitmap {
                inner: ok.unchecked_into(),
            })
            .map_err(|err| CreateImageBitmapError::new(err.unchecked_into()))
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = createImageBitmap)]
    fn js_create_image_bitmap(
        image: &JsValue,
        x: u32,
        y: u32,
        width: i32,
        height: i32,
        options: &JsValue,
    ) -> js_sys::Promise;
}
