use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, PoisonError, RwLock},
};

use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

/// Custom type for a shared state
pub type SharedState = Arc<RwLock<AppState>>;
#[derive(Default)]
pub struct AppState {
    db: HashMap<String, Bytes>,
}

pub fn router(state: &SharedState) -> Router<SharedState> {
    Router::with_state(Arc::clone(&state))
        .route("/", get(hello))
        .route("/hello", get(say_hi))
        .route("/kv/:key", get(kv_get).post(kv_set))
}

#[derive(Debug)]
struct ErrorStatus(StatusCode);

impl Display for ErrorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<StatusCode> for ErrorStatus {
    fn from(val: StatusCode) -> Self {
        Self(val)
    }
}

impl<T> From<PoisonError<T>> for ErrorStatus {
    fn from(_: PoisonError<T>) -> Self {
        ErrorStatus(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IntoResponse for ErrorStatus {
    fn into_response(self) -> axum::response::Response {
        let status = self.0;
        (status, self.to_string()).into_response()
    }
}

async fn kv_set(
    Path(key): Path<String>,
    State(state): State<SharedState>,
    bytes: Bytes,
) -> Result<(), ErrorStatus> {
    state.write()?.db.insert(key, bytes);
    Ok(())
}

async fn kv_get(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<Bytes, ErrorStatus> {
    let db = &state.read()?.db;
    if let Some(val) = db.get(&key) {
        Ok(val.to_owned())
    } else {
        Err(StatusCode::NOT_FOUND.into())
    }
}

async fn say_hi(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    Html(format!(
        "<h1>Hello {}</h1>",
        params.get("name").unwrap_or(&"Unknown Visitor".to_string())
    ))
}

async fn hello() -> impl IntoResponse {
    Html("<h1>Hello Axum</h1>")
}
