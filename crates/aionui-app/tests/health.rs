use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn build_request(method: &str, uri: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .body(Body::empty())
        .expect("failed to build request")
}

async fn response_json(body: Body) -> serde_json::Value {
    let bytes = body.collect().await.expect("failed to read body").to_bytes();
    serde_json::from_slice(&bytes).expect("failed to parse JSON")
}

#[tokio::test]
async fn health_check_returns_ok() {
    let app = aionui_app::create_router();

    let response = app
        .oneshot(build_request("GET", "/health"))
        .await
        .expect("request failed");

    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response.into_body()).await;
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn health_check_post_returns_method_not_allowed() {
    let app = aionui_app::create_router();

    let response = app
        .oneshot(build_request("POST", "/health"))
        .await
        .expect("request failed");

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn unknown_route_returns_not_found() {
    let app = aionui_app::create_router();

    let response = app
        .oneshot(build_request("GET", "/nonexistent"))
        .await
        .expect("request failed");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
