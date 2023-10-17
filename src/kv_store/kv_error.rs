use std::sync::PoisonError;

use axum::response::IntoResponse;
use hyper::StatusCode;
use image::ImageError;

#[derive(Debug)]
pub struct KVError(StatusCode, String);

impl KVError {
    pub fn new(status: StatusCode, message: impl ToString) -> Self {
        Self(status, message.to_string())
    }
}

impl std::fmt::Display for KVError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.0.as_str(), self.1)
    }
}

impl std::error::Error for KVError {}

impl<T> From<PoisonError<T>> for KVError {
    fn from(_value: PoisonError<T>) -> Self {
        Self(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error writing to DB".to_string(),
        )
    }
}

fn _internal_server_error(_err: impl std::error::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error".to_string(),
    )
}

impl IntoResponse for KVError {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}

impl From<ImageError> for KVError {
    fn from(value: ImageError) -> Self {
        match value {
            ImageError::IoError(err) => Self::new(StatusCode::BAD_REQUEST, err),
            ImageError::Unsupported(err) => Self::new(StatusCode::BAD_REQUEST, err),
            _ => Self::new(StatusCode::BAD_REQUEST, ":-("),
        }
    }
}
