use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use microservice_rust_workshop::{router, SharedState};
use tower::Service; // for `call`

#[tokio::test]
async fn basic_db_test() {
    let state = SharedState::default();
    let mut app = router(&state);

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .call(
            Request::builder()
                .uri("/kv/test")
                .method("POST")
                .body("Hello World".into())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .call(
            Request::builder()
                .uri("/kv/test")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Hello World");
}

#[ignore]
#[tokio::test]
async fn big_request() {
    let state = SharedState::default();
    let mut app = router(&state);
    let bytes = include_bytes!("../crab.png");

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .call(
            Request::builder()
                .uri("/kv/crab")
                .method("POST")
                .body(bytes[..].into())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .call(
            Request::builder()
                .uri("/kv/crab")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(body.len(), bytes.len());
    assert_eq!(&body[..], &bytes[..]);
}
