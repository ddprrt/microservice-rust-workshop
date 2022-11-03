use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use microservice_rust_workshop::{router, SharedState};
use tower::Service; // for `call`

#[tokio::test]
async fn hello_world() {
    let state = SharedState::default();
    let mut app = router(&state);

    let response = app
        .call(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"<h1>Hello Axum</h1>");
}

#[tokio::test]
async fn say_hi_unknown() {
    let state = SharedState::default();
    let mut app = router(&state);

    let response = app
        .call(
            Request::builder()
                .uri("/hello")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"<h1>Hello Unknown Visitor</h1>");
}

#[tokio::test]
async fn say_hi_stefan() {
    let state = SharedState::default();
    let mut app = router(&state);

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .call(
            Request::builder()
                .uri("/hello?name=Stefan")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"<h1>Hello Stefan</h1>");
}
