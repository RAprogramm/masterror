// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use axum::{
    Router,
    http::StatusCode,
    routing::{delete, get, post, put}
};
use axum_rest_api::{AppState, User};
use axum_test::TestServer;
use serde_json::json;
use uuid::Uuid;

/// Helper to create test router
fn create_test_router() -> Router {
    let state = AppState::new();

    Router::new()
        .route("/users/{id}", get(axum_rest_api::get_user))
        .route("/users", post(axum_rest_api::create_user))
        .route("/users/{id}", put(axum_rest_api::update_user))
        .route("/users/{id}", delete(axum_rest_api::delete_user))
        .with_state(state)
}

#[tokio::test]
async fn health_check_returns_ok() {
    let app = Router::new().route("/health", get(|| async { "OK" }));
    let server = TestServer::new(app).unwrap();

    let response = server.get("/health").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.text(), "OK");
}

#[tokio::test]
async fn create_user_returns_201() {
    let server = TestServer::new(create_test_router()).unwrap();

    let response = server
        .post("/users")
        .json(&json!({
            "name": "Alice",
            "email": "alice@example.com"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let user: User = response.json();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
}

#[tokio::test]
async fn create_user_with_invalid_email_returns_422() {
    let server = TestServer::new(create_test_router()).unwrap();

    let response = server
        .post("/users")
        .json(&json!({
            "name": "Bob",
            "email": "invalid-email"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);

    let body: serde_json::Value = response.json();
    assert_eq!(body["status"], 422);
    assert!(body["detail"].as_str().unwrap().contains("invalid email"));
}

#[tokio::test]
async fn create_user_with_short_name_returns_422() {
    let server = TestServer::new(create_test_router()).unwrap();

    let response = server
        .post("/users")
        .json(&json!({
            "name": "A",
            "email": "valid@example.com"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);

    let body: serde_json::Value = response.json();
    assert!(
        body["detail"]
            .as_str()
            .unwrap()
            .contains("at least 2 characters")
    );
}

#[tokio::test]
async fn create_duplicate_email_returns_409() {
    let server = TestServer::new(create_test_router()).unwrap();

    // Create first user
    server
        .post("/users")
        .json(&json!({
            "name": "Alice",
            "email": "alice@example.com"
        }))
        .await;

    // Try to create user with same email
    let response = server
        .post("/users")
        .json(&json!({
            "name": "Bob",
            "email": "alice@example.com"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::CONFLICT);

    let body: serde_json::Value = response.json();
    assert_eq!(body["status"], 409);
    assert!(body["detail"].as_str().unwrap().contains("already exists"));
}

#[tokio::test]
async fn get_user_returns_200() {
    let server = TestServer::new(create_test_router()).unwrap();

    // Create user
    let create_response = server
        .post("/users")
        .json(&json!({
            "name": "Charlie",
            "email": "charlie@example.com"
        }))
        .await;

    let created_user: User = create_response.json();

    // Get user
    let response = server.get(&format!("/users/{}", created_user.id)).await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let user: User = response.json();
    assert_eq!(user.id, created_user.id);
    assert_eq!(user.name, "Charlie");
}

#[tokio::test]
async fn get_nonexistent_user_returns_404() {
    let server = TestServer::new(create_test_router()).unwrap();

    let fake_id = Uuid::new_v4();
    let response = server.get(&format!("/users/{}", fake_id)).await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);

    let body: serde_json::Value = response.json();
    assert_eq!(body["status"], 404);
    assert!(body["detail"].as_str().unwrap().contains("not found"));
}

#[tokio::test]
async fn update_user_returns_200() {
    let server = TestServer::new(create_test_router()).unwrap();

    // Create user
    let create_response = server
        .post("/users")
        .json(&json!({
            "name": "Dave",
            "email": "dave@example.com"
        }))
        .await;

    let created_user: User = create_response.json();

    // Update user
    let response = server
        .put(&format!("/users/{}", created_user.id))
        .json(&json!({
            "name": "Dave Updated",
            "email": "dave.updated@example.com"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let user: User = response.json();
    assert_eq!(user.name, "Dave Updated");
    assert_eq!(user.email, "dave.updated@example.com");
}

#[tokio::test]
async fn update_nonexistent_user_returns_404() {
    let server = TestServer::new(create_test_router()).unwrap();

    let fake_id = Uuid::new_v4();
    let response = server
        .put(&format!("/users/{}", fake_id))
        .json(&json!({
            "name": "Ghost",
            "email": "ghost@example.com"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_user_returns_204() {
    let server = TestServer::new(create_test_router()).unwrap();

    // Create user
    let create_response = server
        .post("/users")
        .json(&json!({
            "name": "Eve",
            "email": "eve@example.com"
        }))
        .await;

    let created_user: User = create_response.json();

    // Delete user
    let response = server.delete(&format!("/users/{}", created_user.id)).await;

    assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

    // Verify user is gone
    let get_response = server.get(&format!("/users/{}", created_user.id)).await;
    assert_eq!(get_response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_nonexistent_user_returns_404() {
    let server = TestServer::new(create_test_router()).unwrap();

    let fake_id = Uuid::new_v4();
    let response = server.delete(&format!("/users/{}", fake_id)).await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}
