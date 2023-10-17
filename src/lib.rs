use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    body::Bytes,
    extract::{Query, State},
    headers::ContentType,
    response::IntoResponse,
    routing::get,
    Router,
};
use kv_store::{get_kv, grayscale, post_kv};
use serde::Deserialize;

mod kv_store;

/// Custom type for a shared state
pub type SharedState = Arc<RwLock<AppState>>;

async fn handler() -> impl IntoResponse {
    "<h1>Hello Axum</h1>"
}

#[derive(Deserialize)]
struct Name {
    name: Option<String>,
}

async fn hello_handler(Query(name): Query<Name>) -> impl IntoResponse {
    match name.name {
        Some(name) => format!("<h1>Hello {}</h1>", name),
        None => "<h1>Hello Unknown Visitor</h1>".to_string(),
    }
}

async fn poison(State(state): State<SharedState>) -> impl IntoResponse {
    let _guard = state.write().unwrap();
    panic!("At the disco");
}

#[derive(Default)]
pub struct AppState {
    db: HashMap<String, (String, Bytes)>,
}

pub fn router(state: &SharedState) -> Router<SharedState> {
    Router::with_state(Arc::clone(state))
        .route("/", get(handler))
        .route("/hello", get(hello_handler))
        .route("/kv/:key", get(get_kv).post(post_kv))
        .route("/kv/:key/grayscale", get(grayscale))
        .route("/poison", get(poison))
}
