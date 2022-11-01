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
use futures::future::BoxFuture;
use hyper::Request;
use tower::{Layer, Service, ServiceBuilder};
use tower_http::{auth::RequireAuthorizationLayer, limit::RequestBodyLimitLayer};

/// Custom type for a shared state
pub type SharedState = Arc<RwLock<AppState>>;
#[derive(Default)]
pub struct AppState {
    db: HashMap<String, Bytes>,
}

pub fn router(state: &SharedState) -> Router<SharedState> {
    Router::with_state(Arc::clone(state))
        .route("/", get(hello))
        .route("/hello", get(say_hi))
        .route(
            "/kv/:key",
            get(kv_get).post_service(
                ServiceBuilder::new()
                    .layer(DefaultBodyLimit::disable())
                    .layer(RequestBodyLimitLayer::new(1024 * 8_000))
                    .service(kv_set.with_state(Arc::clone(state))),
            ),
        )
        .nest("/admin", admin_routes(&state))
        .layer(LogLayer::new())
}

fn admin_routes(state: &SharedState) -> Router<SharedState> {
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
        .route("/kv", delete(delete_all_keys))
        .route("/kv/:key", delete(remove_key))
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

#[derive(Clone, Copy)]
struct LogService<S> {
    inner: S,
}

impl<S> LogService<S> {
    fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<B>> for LogService<S>
where
    S: Service<Request<B>> + Clone + Send + 'static,
    S::Future: Send,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<S::Response, S::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let this = self.inner.clone();
        let mut this = std::mem::replace(&mut self.inner, this);
        let method = req.method().to_owned();
        let path = req.uri().path().to_owned();

        println!("processing {} {}", method, path);
        Box::pin(async move {
            let res = this.call(req).await;
            println!("end processing {} {}", method, path);
            res
        })
    }
}

struct LogLayer;

impl LogLayer {
    fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LogService::new(inner)
    }
}
