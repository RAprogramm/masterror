// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Axum REST API example with masterror integration.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock}
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response}
};
use masterror::{AppError, Error};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

/// User model with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id:    Uuid,
    pub name:  String,
    pub email: String
}

/// User creation request
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name:  String,
    pub email: String
}

/// User update request
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name:  String,
    pub email: String
}

/// Domain-specific user errors
///
/// These errors represent business logic failures that are converted
/// into appropriate HTTP responses via AppError.
#[derive(Debug, Error, Clone)]
pub enum UserError {
    /// User with given ID was not found
    #[error("user not found")]
    NotFound,

    /// Email address is already registered
    #[error("email already exists")]
    DuplicateEmail,

    /// Email format is invalid
    #[error("invalid email format")]
    InvalidEmail,

    /// User name is too short or empty
    #[error("invalid name: must be at least 2 characters")]
    InvalidName
}

/// Convert domain errors to AppError with appropriate HTTP status codes
impl From<UserError> for AppError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::NotFound => AppError::not_found(err.to_string()),
            UserError::DuplicateEmail => AppError::conflict(err.to_string()),
            UserError::InvalidEmail | UserError::InvalidName => {
                AppError::validation(err.to_string())
            }
        }
    }
}

/// Implement IntoResponse to use UserError directly in handlers
impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let app_error: AppError = self.into();
        app_error.into_response()
    }
}

/// In-memory user storage (production would use database)
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<Uuid, User>>>
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new()))
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate email format (simplified)
fn validate_email(email: &str) -> Result<(), UserError> {
    if email.contains('@') && email.len() > 3 {
        Ok(())
    } else {
        Err(UserError::InvalidEmail)
    }
}

/// Validate user name
fn validate_name(name: &str) -> Result<(), UserError> {
    if name.len() >= 2 {
        Ok(())
    } else {
        Err(UserError::InvalidName)
    }
}

/// GET /users/:id - Retrieve user by ID
///
/// Returns 404 if user not found, includes user_id in error metadata.
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>
) -> Result<axum::Json<User>, UserError> {
    let users = state.users.read().unwrap();

    users
        .get(&user_id)
        .cloned()
        .ok_or(UserError::NotFound)
        .map(axum::Json)
}

/// POST /users - Create new user
///
/// Validates email format and checks for duplicates.
/// Returns 201 Created on success.
pub async fn create_user(
    State(state): State<AppState>,
    axum::Json(req): axum::Json<CreateUserRequest>
) -> Result<(StatusCode, axum::Json<User>), UserError> {
    validate_email(&req.email)?;
    validate_name(&req.name)?;

    let mut users = state.users.write().unwrap();

    // Check for duplicate email
    if users.values().any(|u| u.email == req.email) {
        return Err(UserError::DuplicateEmail);
    }

    let user = User {
        id:    Uuid::new_v4(),
        name:  req.name,
        email: req.email
    };

    info!(user_id = %user.id, email = %user.email, "Creating new user");

    users.insert(user.id, user.clone());

    Ok((StatusCode::CREATED, axum::Json(user)))
}

/// PUT /users/:id - Update existing user
///
/// Returns 404 if user not found.
/// Validates email format before update.
pub async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    axum::Json(req): axum::Json<UpdateUserRequest>
) -> Result<axum::Json<User>, UserError> {
    validate_email(&req.email)?;
    validate_name(&req.name)?;

    let mut users = state.users.write().unwrap();

    // Check if user exists and get current email
    let current_email = users
        .get(&user_id)
        .map(|u| u.email.clone())
        .ok_or(UserError::NotFound)?;

    // Check if email is being changed to existing email
    if req.email != current_email && users.values().any(|u| u.email == req.email) {
        return Err(UserError::DuplicateEmail);
    }

    info!(user_id = %user_id, "Updating user");

    // Now update the user
    let user = users.get_mut(&user_id).ok_or(UserError::NotFound)?;
    user.name = req.name;
    user.email = req.email;

    Ok(axum::Json(user.clone()))
}

/// DELETE /users/:id - Delete user
///
/// Returns 404 if user not found, 204 No Content on success.
pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>
) -> Result<StatusCode, UserError> {
    let mut users = state.users.write().unwrap();

    users.remove(&user_id).ok_or(UserError::NotFound)?;

    info!(user_id = %user_id, "Deleted user");

    Ok(StatusCode::NO_CONTENT)
}
