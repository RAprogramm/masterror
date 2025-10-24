<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Custom Domain Errors Example

Demonstrates creating domain-specific custom error types using masterror's derive macro for a payment processing system.

## Features

- **Payment Processing Errors** - Domain errors for payment operations
- **Authentication Errors** - User authentication and authorization failures
- **Validation Errors** - Input validation with detailed field information
- **External Service Errors** - Third-party API integration errors
- **Derive Macro Usage** - Full use of `#[derive(Error)]`

## Running

```bash
cd examples/custom-domain-errors
cargo run
```

## Error Domains

### Payment Domain

```rust
#[derive(Debug, Error, Clone)]
pub enum PaymentError {
    #[error("insufficient funds: balance={balance}, required={required}")]
    InsufficientFunds { balance: u64, required: u64 },

    #[error("payment method declined")]
    PaymentDeclined,

    #[error("invalid amount: {0}")]
    InvalidAmount(String),
}
```

### Authentication Domain

```rust
#[derive(Debug, Error, Clone)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("session expired")]
    SessionExpired,

    #[error("insufficient permissions")]
    Forbidden,
}
```

### Validation Domain

```rust
#[derive(Debug, Error, Clone)]
pub enum ValidationError {
    #[error("field '{field}' is required")]
    RequiredField { field: String },

    #[error("field '{field}' has invalid format: {reason}")]
    InvalidFormat { field: String, reason: String },
}
```

## Converting to AppError

```rust
impl From<PaymentError> for AppError {
    fn from(err: PaymentError) -> Self {
        match err {
            PaymentError::InsufficientFunds { .. } => {
                AppError::bad_request(err.to_string())
            }
            PaymentError::PaymentDeclined => {
                AppError::bad_request(err.to_string())
            }
            PaymentError::InvalidAmount(_) => {
                AppError::validation(err.to_string())
            }
        }
    }
}
```

## Testing

```bash
cargo test
```

## Key Concepts

- **Domain Separation** - Each domain (payment, auth, validation) has its own error enum
- **Derive Macro** - Using `#[derive(Error)]` for automatic `Display` and `Error` trait implementation
- **Conversion** - Clean conversion from domain errors to `AppError` with appropriate HTTP status codes
- **Error Context** - Rich error information with structured data

## License

MIT
