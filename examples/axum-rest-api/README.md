<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Axum REST API Example

Full-featured REST API demonstrating masterror integration with Axum framework, RFC 7807 Problem Details, and structured error handling.

## Features

- **RFC 7807 Problem Details** - HTTP API error responses
- **Custom Domain Errors** - User management domain errors with derive macro
- **Metadata Attachment** - request_id and user_id tracking
- **Tracing Integration** - Structured logging with tracing
- **Integration Tests** - Full HTTP test coverage with axum-test

## Running

```bash
cd examples/axum-rest-api
cargo run
```

Server starts on `http://127.0.0.1:3000`.

## API Endpoints

### User Management

```bash
# Get user by ID
curl http://127.0.0.1:3000/users/550e8400-e29b-41d4-a716-446655440000

# Create user
curl -X POST http://127.0.0.1:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "email": "alice@example.com"}'

# Update user
curl -X PUT http://127.0.0.1:3000/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Updated", "email": "alice@example.com"}'

# Delete user
curl -X DELETE http://127.0.0.1:3000/users/550e8400-e29b-41d4-a716-446655440000
```

## Error Response Format

All errors return RFC 7807 Problem Details:

```json
{
  "type": "https://masterror.dev/errors/not-found",
  "title": "Not Found",
  "status": 404,
  "detail": "user not found",
  "instance": "/users/550e8400-e29b-41d4-a716-446655440000",
  "code": "NOT_FOUND",
  "request_id": "req-123",
  "user_id": "user-456"
}
```

## Testing

```bash
cargo test
```

## Key Concepts

### Domain Errors

```rust
#[derive(Debug, Error, Clone)]
pub enum UserError {
    #[error("user not found")]
    NotFound,

    #[error("email already exists")]
    DuplicateEmail,

    #[error("invalid email format")]
    InvalidEmail,
}
```

### Axum Integration

```rust
impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let app_error: AppError = self.into();
        app_error.into_response()
    }
}
```

### Metadata Attachment

```rust
AppError::not_found("user not found")
    .with_field("user_id", user_id.to_string())
    .with_field("request_id", request_id.to_string())
```

## License

MIT
