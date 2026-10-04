use std::path::Path;

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use job_postings_api::{routes, store::PostingStore};
use serde_json::Value;
use tower::ServiceExt;

fn app() -> Router {
    let store = PostingStore::from_json_file(Path::new("data/job_postings.json")).unwrap();
    routes::router(store)
}

async fn get(uri: &str) -> (StatusCode, Value) {
    let request = Request::get(uri).body(Body::empty()).unwrap();
    let response = app().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&bytes).unwrap())
}

#[tokio::test]
async fn list_returns_requested_page() {
    let (status, body) = get("/postings?page=2&per_page=10").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"].as_array().unwrap().len(), 10);
    assert_eq!(body["data"][0]["id"], 11);
    assert_eq!(body["total"], 150);
}

#[tokio::test]
async fn list_rejects_invalid_per_page() {
    let (status, body) = get("/postings?per_page=0").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body["error"].is_string());
}

#[tokio::test]
async fn search_filters_by_role_case_insensitively() {
    let (status, body) = get("/postings/search?role=MANAGER").await;
    assert_eq!(status, StatusCode::OK);
    let results = body["data"].as_array().unwrap();
    assert!(!results.is_empty());
    assert!(results.iter().all(|posting| {
        posting["title"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("manager")
    }));
}

#[tokio::test]
async fn search_without_filters_returns_400() {
    let (status, _) = get("/postings/search").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn get_returns_posting_by_id() {
    let (status, body) = get("/postings/1").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], 1);
}

#[tokio::test]
async fn get_missing_posting_returns_404() {
    let (status, body) = get("/postings/9999").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"], "posting 9999 not found");
}

#[tokio::test]
async fn get_invalid_id_returns_400() {
    let (status, _) = get("/postings/abc").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}
