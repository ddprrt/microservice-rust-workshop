use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, PoisonError, RwLock},
};

use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Path, Query, State},
    handler::Handler,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get},
    Router,
};

use tower::ServiceBuilder;
use tower_http::{
    auth::RequireAuthorizationLayer, limit::RequestBodyLimitLayer, trace::TraceLayer,
};
use tracing::{event, instrument, span, Level};
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;

/// Custom type for a shared state
pub type SharedState = Arc<RwLock<AppState>>;
#[derive(Default, Debug)]
pub struct AppState {
    db: HashMap<String, Bytes>,
}

pub fn router(state: &SharedState) -> Router<SharedState> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "microservice_rust_workshop=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    Router::with_state(Arc::clone(&state))
        .route("/", get(hello))
        .route("/hello", get(say_hi))
        .nest("/kv", key_value_store(state))
        .nest("/admin", admin_routes(state))
        .layer(TraceLayer::new_for_http())
}

pub fn key_value_store(state: &SharedState) -> Router<SharedState> {
    let kv_set_service = ServiceBuilder::new()
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(1024 * 8000))
        .service(kv_set.with_state(Arc::clone(state)));
    Router::with_state(Arc::clone(state)).route("/:key", get(kv_get).post_service(kv_set_service))
}

pub fn admin_routes(state: &SharedState) -> Router<SharedState> {
    async fn remove_key(
        Path(key): Path<String>,
        State(state): State<SharedState>,
    ) -> Result<(), ErrorStatus> {
        state.write()?.db.remove(&key);
        Ok(())
    }

    async fn delete_all_keys(State(state): State<SharedState>) -> Result<(), ErrorStatus> {
        state.write()?.db.clear();
        Ok(())
    }

    Router::with_state(Arc::clone(state))
        .route("/kv/:key", delete(remove_key))
        .route("/kv", delete(delete_all_keys))
        .layer(RequireAuthorizationLayer::bearer("secret"))
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

#[instrument]
async fn kv_get(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<Bytes, ErrorStatus> {
    let span = span!(Level::DEBUG, "enter kv_get");
    let _guard = span.enter();
    let db = &state.read()?.db;
    if let Some(val) = db.get(&key) {
        event!(Level::DEBUG, "Found");
        Ok(val.to_owned())
    } else {
        event!(Level::DEBUG, "Not found");
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
