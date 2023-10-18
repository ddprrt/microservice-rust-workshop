use std::io::Cursor;

use axum::response::IntoResponse;
use hyper::body::Bytes;
use image::{DynamicImage, ImageOutputFormat};

use super::kv_error::KVError;

pub struct ImageResponse(Bytes);

impl ImageResponse {
    pub fn new(bytes: impl Into<Bytes>) -> Self {
        Self(bytes.into())
    }
}

impl TryFrom<DynamicImage> for ImageResponse {
    type Error = KVError;

    fn try_from(value: DynamicImage) -> Result<Self, Self::Error> {
        ImageResponse::try_from(&value)
    }
}

impl TryFrom<&DynamicImage> for ImageResponse {
    type Error = KVError;

    fn try_from(value: &DynamicImage) -> Result<Self, Self::Error> {
        let mut vec: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut vec);
        value.write_to(&mut cursor, ImageOutputFormat::Png)?;
        Ok(ImageResponse::new(vec))
    }
}

impl IntoResponse for ImageResponse {
    fn into_response(self) -> axum::response::Response {
        ([("content-type", "image/png")], self.0).into_response()
    }
}
