use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct InvalidImageQuality(f64);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ImageQuality(f64);

impl TryFrom<f64> for ImageQuality {
    type Error = InvalidImageQuality;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0.0 || value > 1.0 {
            Err(InvalidImageQuality(value))
        } else {
            Ok(ImageQuality(value))
        }
    }
}

impl Default for ImageQuality {
    fn default() -> Self {
        ImageQuality(0.92)
    }
}

impl From<ImageQuality> for f64 {
    fn from(image_quality: ImageQuality) -> Self {
        image_quality.0
    }
}
