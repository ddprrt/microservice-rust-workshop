use std::io::Read;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use gag::BufferRedirect;
use microservice_rust_workshop::{router, SharedState};
use tower::Service; // for `call`

#[tokio::test]
async fn log_test() {
    // Run with nocapture
    let state = SharedState::default();
    let mut app = router(&state);

    let mut buf = BufferRedirect::stdout().unwrap();

    let response = app
        .call(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"<h1>Hello Axum</h1>");

    let mut output = String::new();
    buf.read_to_string(&mut output).unwrap();

    assert_eq!("processing GET /", output.trim());
}

#[ignore]
#[tokio::test]
async fn advanced_log_test() {
    // Run with nocapture
    let state = SharedState::default();
    let mut app = router(&state);

    let mut buf = BufferRedirect::stdout().unwrap();

    let response = app
        .call(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"<h1>Hello Axum</h1>");

    let mut output = String::new();
    buf.read_to_string(&mut output).unwrap();

    assert_eq!("processing GET /\nend processing GET /", output.trim());
}
