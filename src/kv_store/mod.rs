use axum::{
    extract::{Path, State},
    headers::ContentType,
    response::{IntoResponse, Response},
    TypedHeader,
};
use hyper::{body::Bytes, StatusCode};

use crate::{state::StoredType, SharedState};

use self::{image_response::ImageResponse, kv_error::KVError};

mod image_response;
mod kv_error;

fn get_stored_type(content_type: impl ToString, data: Bytes) -> KVResult<StoredType> {
    if content_type.to_string().starts_with("image") {
        let image = image::load_from_memory(&data)?;
        Ok(StoredType::Image(image))
    } else {
        Ok(StoredType::Other(data))
    }
}

pub async fn post_kv(
    Path(key): Path<String>,
    TypedHeader(content_type): TypedHeader<ContentType>,
    State(state): State<SharedState>,
    data: Bytes,
) -> Result<String, KVError> {
    let stored_type = get_stored_type(content_type, data)?;
    state.write()?.db.insert(key, stored_type);
    Ok("OK".to_string())
}

pub async fn get_kv(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<Response, KVError> {
    match state.read()?.db.get(&key) {
        Some(StoredType::Image(image)) => Ok(ImageResponse::try_from(image)?.into_response()),
        Some(StoredType::Other(bytes)) => Ok(bytes.clone().into_response()),
        None => Err(KVError::new(StatusCode::NOT_FOUND, "Key not found")),
    }
}

type KVResult<T> = Result<T, KVError>;

// get rid of the unwraps, use error propagation!
// don't change the signature of the function
pub async fn grayscale(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> KVResult<impl IntoResponse> {
    match state.read()?.db.get(&key) {
        Some(StoredType::Image(image)) => ImageResponse::try_from(image.grayscale()),
        _ => Err(KVError::new(StatusCode::NOT_FOUND, "Key not found")),
    }
}
