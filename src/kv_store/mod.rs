use std::io::Cursor;

use axum::{
    extract::{Path, State},
    headers::ContentType,
    response::IntoResponse,
    TypedHeader,
};
use hyper::{body::Bytes, StatusCode};
use image::ImageOutputFormat;

use crate::SharedState;

use self::kv_error::KVError;

mod kv_error;

pub async fn post_kv(
    Path(key): Path<String>,
    TypedHeader(content_type): TypedHeader<ContentType>,
    State(state): State<SharedState>,
    data: Bytes,
) -> Result<String, KVError> {
    state
        .write()?
        .db
        .insert(key, (content_type.to_string(), data));
    Ok("OK".to_string())
}

pub async fn get_kv(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, KVError> {
    match state.read()?.db.get(&key) {
        Some((content_type, data)) => Ok(([("content-type", content_type.clone())], data.clone())),
        None => Err(KVError::new(StatusCode::NOT_FOUND, "Key not found")),
    }
}

type KVResult<T: IntoResponse> = Result<T, KVError>;

// get rid of the unwraps, use error propagation!
// don't change the signature of the function
pub async fn grayscale(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, KVError> {
    let image = match state.read()?.db.get(&key) {
        Some((content_type, data)) => {
            if content_type == "image/png" {
                image::load_from_memory(&data)?
            } else {
                return Err(KVError::new(
                    StatusCode::FORBIDDEN,
                    "Not possible to grayscale this type of data",
                ));
            }
        }
        None => return Err(KVError::new(StatusCode::NOT_FOUND, "Key not found")),
    };

    // Get rid of all the manual labor here and go from DynamicImage to ImageResponse
    // Step 1: Create a struct `ImageResponse` that can hold `Bytes`
    // Step 2: Convert from `DynamicImage` to `ImageResponse`
    //         Do all the allocations, writings, Error Handlings
    //         Hint: TryFrom
    // Step 3: Go from ImageResponse to `impl IntoResponse`
    //         Hint: impl IntoResponse for ...

    let mut vec: Vec<u8> = Vec::new();

    let mut cursor = Cursor::new(&mut vec);
    image
        .grayscale()
        .write_to(&mut cursor, ImageOutputFormat::Png)?;
    let bytes: Bytes = vec.into();

    Ok(([("content-type", "image/png")], bytes).into_response())
}
