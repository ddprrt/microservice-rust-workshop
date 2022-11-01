use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    body::Bytes,
    extract::Query,
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
