// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use axum::{
    Router,
    routing::{delete, get, post, put}
};
use axum_rest_api::{AppState, create_user, delete_user, get_user, update_user};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt::layer, layer::SubscriberExt, util::SubscriberInitExt};

/// Health check endpoint
async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(layer())
        .init();
    let state = AppState::new();
    let app = Router::new()
        .route("/health", get(health))
        .route("/users/{id}", get(get_user))
        .route("/users", post(create_user))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
