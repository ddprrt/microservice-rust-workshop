use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use kv_store::{get_kv, grayscale, post_kv};
use serde::Deserialize;
use state::SharedState;

mod kv_store;
pub mod state;

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

pub fn router(state: &SharedState) -> Router<SharedState> {
    Router::with_state(Arc::clone(state))
        .route("/", get(handler))
        .route("/hello", get(hello_handler))
        .route("/kv/:key", get(get_kv).post(post_kv))
        .route("/kv/:key/grayscale", get(grayscale))
        .route("/poison", get(poison))
}
