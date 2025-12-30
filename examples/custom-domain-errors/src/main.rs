// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Custom domain errors example for payment processing system.
//!
//! Demonstrates creating domain-specific error types using masterror's derive
//! macro and converting them to AppError.

use masterror::{AppError, Error};

/// Payment processing domain errors
#[derive(Debug, Error, Clone, PartialEq)]
pub enum PaymentError {
    /// Insufficient funds in account
    #[error("insufficient funds: balance=${balance}, required=${required}")]
    InsufficientFunds {
        /// Current account balance in cents
        balance:  u64,
        /// Required amount in cents
        required: u64
    },

    /// Payment method was declined by processor
    #[error("payment method declined: {reason}")]
    PaymentDeclined {
        /// Reason for decline
        reason: String
    },

    /// Invalid payment amount
    #[error("invalid payment amount: {0}")]
    InvalidAmount(String),

    /// Payment processor unavailable
    #[error("payment processor temporarily unavailable")]
    ProcessorUnavailable,

    /// Duplicate transaction detected
    #[error("duplicate transaction: {transaction_id}")]
    DuplicateTransaction {
        /// ID of duplicate transaction
        transaction_id: String
    }
}

/// Authentication and authorization errors
#[derive(Debug, Error, Clone, PartialEq)]
pub enum AuthError {
    /// Invalid username or password
    #[error("invalid credentials")]
    InvalidCredentials,

    /// User session has expired
    #[error("session expired at {expired_at}")]
    SessionExpired {
        /// Timestamp when session expired
        expired_at: String
    },

    /// User lacks required permissions
    #[error("insufficient permissions: requires {required}")]
    Forbidden {
        /// Required permission
        required: String
    },

    /// Account is locked due to too many failed attempts
    #[error("account locked until {unlock_at}")]
    AccountLocked {
        /// Timestamp when account unlocks
        unlock_at: String
    }
}

/// Input validation errors with field-level detail
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ValidationError {
    /// Required field is missing
    #[error("field '{field}' is required")]
    RequiredField {
        /// Name of missing field
        field: String
    },

    /// Field has invalid format
    #[error("field '{field}' has invalid format: {reason}")]
    InvalidFormat {
        /// Field name
        field:  String,
        /// Reason for invalidity
        reason: String
    },

    /// Value is out of allowed range
    #[error("field '{field}' out of range: {min} <= value <= {max}")]
    OutOfRange {
        /// Field name
        field: String,
        /// Minimum allowed value
        min:   String,
        /// Maximum allowed value
        max:   String
    }
}

/// External service integration errors
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ExternalServiceError {
    /// Service returned an error response
    #[error("service '{service}' returned error: {message}")]
    ServiceError {
        /// Service name
        service: String,
        /// Error message from service
        message: String
    },

    /// Service request timed out
    #[error("request to '{service}' timed out after {timeout_ms}ms")]
    Timeout {
        /// Service name
        service:    String,
        /// Timeout duration in milliseconds
        timeout_ms: u64
    },

    /// Network connectivity issue
    #[error("network error connecting to '{service}': {details}")]
    NetworkError {
        /// Service name
        service: String,
        /// Error details
        details: String
    }
}

/// Convert payment errors to HTTP-appropriate AppError
impl From<PaymentError> for AppError {
    fn from(err: PaymentError) -> Self {
        match err {
            PaymentError::InsufficientFunds {
                ..
            }
            | PaymentError::PaymentDeclined {
                ..
            } => AppError::bad_request(err.to_string()),
            PaymentError::InvalidAmount(_) => AppError::validation(err.to_string()),
            PaymentError::ProcessorUnavailable => AppError::external_api(err.to_string()),
            PaymentError::DuplicateTransaction {
                ..
            } => AppError::conflict(err.to_string())
        }
    }
}

/// Convert authentication errors to HTTP-appropriate AppError
impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::InvalidCredentials => AppError::unauthorized(err.to_string()),
            AuthError::SessionExpired {
                ..
            } => AppError::unauthorized(err.to_string()),
            AuthError::Forbidden {
                ..
            } => AppError::forbidden(err.to_string()),
            AuthError::AccountLocked {
                ..
            } => AppError::forbidden(err.to_string())
        }
    }
}

/// Convert validation errors to HTTP 422 Unprocessable Entity
impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        AppError::validation(err.to_string())
    }
}

/// Convert external service errors to HTTP-appropriate AppError
impl From<ExternalServiceError> for AppError {
    fn from(err: ExternalServiceError) -> Self {
        match err {
            ExternalServiceError::ServiceError {
                ..
            } => AppError::external_api(err.to_string()),
            ExternalServiceError::Timeout {
                ..
            } => AppError::timeout(err.to_string()),
            ExternalServiceError::NetworkError {
                ..
            } => AppError::network(err.to_string())
        }
    }
}

/// Simulated payment processing
fn process_payment(amount: u64, balance: u64) -> Result<String, PaymentError> {
    if amount == 0 {
        return Err(PaymentError::InvalidAmount(
            "amount must be greater than 0".to_string()
        ));
    }
    if amount > balance {
        return Err(PaymentError::InsufficientFunds {
            balance,
            required: amount
        });
    }
    Ok(format!("Payment of ${amount} processed successfully"))
}

/// Simulated authentication check
fn authenticate(username: &str, password: &str) -> Result<String, AuthError> {
    if username.is_empty() || password.is_empty() {
        return Err(AuthError::InvalidCredentials);
    }
    if username != "admin" || password != "secret" {
        return Err(AuthError::InvalidCredentials);
    }
    Ok("Authentication successful".to_string())
}

/// Simulated input validation
fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.is_empty() {
        return Err(ValidationError::RequiredField {
            field: "email".to_string()
        });
    }
    if !email.contains('@') {
        return Err(ValidationError::InvalidFormat {
            field:  "email".to_string(),
            reason: "must contain @ symbol".to_string()
        });
    }
    Ok(())
}

fn main() {
    println!("Custom Domain Errors Example\\n");
    println!("=== Payment Processing ===");
    match process_payment(100, 500) {
        Ok(msg) => println!("✓ {msg}"),
        Err(e) => {
            let app_err: AppError = e.into();
            println!("✗ AppError: {app_err}");
        }
    }
    match process_payment(600, 500) {
        Ok(msg) => println!("✓ {msg}"),
        Err(e) => {
            println!("✗ PaymentError: {e}");
            let app_err: AppError = e.into();
            println!(
                "  → AppError kind: {:?}, HTTP: {}",
                app_err.kind,
                app_err.kind.http_status()
            );
        }
    }
    match process_payment(0, 500) {
        Ok(msg) => println!("✓ {msg}"),
        Err(e) => {
            println!("✗ PaymentError: {e}");
            let app_err: AppError = e.into();
            println!("  → AppError kind: {:?}", app_err.kind);
        }
    }
    println!("\\n=== Authentication ===");
    match authenticate("admin", "secret") {
        Ok(msg) => println!("✓ {msg}"),
        Err(e) => println!("✗ {e}")
    }
    match authenticate("user", "wrong") {
        Ok(msg) => println!("✓ {msg}"),
        Err(e) => {
            println!("✗ AuthError: {e}");
            let app_err: AppError = e.into();
            println!(
                "  → AppError kind: {:?}, HTTP: {}",
                app_err.kind,
                app_err.kind.http_status()
            );
        }
    }
    let expired_err = AuthError::SessionExpired {
        expired_at: "2025-01-01T00:00:00Z".to_string()
    };
    println!("✗ AuthError: {expired_err}");
    let app_err: AppError = expired_err.into();
    println!(
        "  → AppError kind: {:?}, HTTP: {}",
        app_err.kind,
        app_err.kind.http_status()
    );
    println!("\\n=== Validation ===");
    match validate_email("user@example.com") {
        Ok(()) => println!("✓ Email is valid"),
        Err(e) => println!("✗ {e}")
    }
    match validate_email("invalid-email") {
        Ok(()) => println!("✓ Email is valid"),
        Err(e) => {
            println!("✗ ValidationError: {e}");
            let app_err: AppError = e.into();
            println!(
                "  → AppError kind: {:?}, HTTP: {}",
                app_err.kind,
                app_err.kind.http_status()
            );
        }
    }
    match validate_email("") {
        Ok(()) => println!("✓ Email is valid"),
        Err(e) => {
            println!("✗ ValidationError: {e}");
            let app_err: AppError = e.into();
            println!(
                "  → AppError kind: {:?}, HTTP: {}",
                app_err.kind,
                app_err.kind.http_status()
            );
        }
    }
    println!("\\n=== External Service Errors ===");
    let service_err = ExternalServiceError::Timeout {
        service:    "payment-gateway".to_string(),
        timeout_ms: 5000
    };
    println!("✗ ExternalServiceError: {service_err}");
    let app_err: AppError = service_err.into();
    println!(
        "  → AppError kind: {:?}, HTTP: {}",
        app_err.kind,
        app_err.kind.http_status()
    );
    let network_err = ExternalServiceError::NetworkError {
        service: "fraud-detection".to_string(),
        details: "connection refused".to_string()
    };
    println!("✗ ExternalServiceError: {network_err}");
    let app_err: AppError = network_err.into();
    println!(
        "  → AppError kind: {:?}, HTTP: {}",
        app_err.kind,
        app_err.kind.http_status()
    );
}
