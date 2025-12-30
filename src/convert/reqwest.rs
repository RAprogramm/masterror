// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Conversion from [`reqwest::Error`] into [`Error`].
//!
//! Enabled with the `reqwest` feature flag.
//!
//! ## Mapping
//!
//! - [`reqwest::Error::is_timeout`] → `AppErrorKind::Timeout`
//! - [`reqwest::Error::is_connect`] or [`reqwest::Error::is_request`] →
//!   `AppErrorKind::Network`
//! - HTTP status errors are classified by status family:
//!   - `429` → `AppErrorKind::RateLimited`
//!   - `5xx` → `AppErrorKind::DependencyUnavailable`
//!   - `408` → `AppErrorKind::Timeout`
//!   - others → `AppErrorKind::ExternalApi`
//! - All other cases → `AppErrorKind::ExternalApi`
//!
//! Structured metadata captures the upstream endpoint, status code and
//! low-level flags (timeout/connect/request). Potentially sensitive data (URL)
//! is marked for hashing/redaction in public payloads.
//!
//! ## Rationale
//!
//! This mapping treats `reqwest` as a client to an external HTTP API.
//! Timeout and network/connectivity issues are separated from upstream
//! HTTP status failures so they can be handled differently by clients.
//!
//! ## Example
//!
//! ```rust
//! use masterror::{AppErrorKind, Error};
//! use reqwest::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new();
//!     let err = client
//!         .get("http://127.0.0.1:1")
//!         .send()
//!         .await
//!         .expect_err("connection refused");
//!
//!     let app_err: Error = err.into();
//!
//!     match app_err.kind {
//!         AppErrorKind::Network | AppErrorKind::Timeout | AppErrorKind::ExternalApi => {}
//!         _ => panic!("unexpected kind")
//!     }
//! }
//! ```

#[cfg(feature = "reqwest")]
use reqwest::{Error as ReqwestError, StatusCode};

use crate::{AppErrorKind, Context, Error, FieldRedaction, field};

/// Map a [`reqwest::Error`] into an [`struct@crate::Error`] according to its
/// category.
///
/// - Timeout → `Timeout`
/// - Connect or request build error → `Network`
/// - Upstream returned HTTP error status → `ExternalApi`
/// - Fallback for other cases → `ExternalApi`
///
/// # Example
///
/// ```rust
/// use masterror::{AppErrorKind, Error};
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new();
///     let err = client
///         .get("http://127.0.0.1:1")
///         .send()
///         .await
///         .expect_err("connection refused");
///
///     let app_err: Error = err.into();
///     assert_eq!(app_err.kind, AppErrorKind::Network);
/// }
/// ```
#[cfg(feature = "reqwest")]
#[cfg_attr(docsrs, doc(cfg(feature = "reqwest")))]
impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        let (context, retry_after) = classify_reqwest_error(&err);
        let mut error = context.into_error(err);
        if let Some(secs) = retry_after {
            error = error.with_retry_after_secs(secs);
        }
        error
    }
}

#[cfg(feature = "reqwest")]
fn classify_reqwest_error(err: &ReqwestError) -> (Context, Option<u64>) {
    let mut context = Context::new(AppErrorKind::ExternalApi)
        .with(field::bool("reqwest.is_timeout", err.is_timeout()))
        .with(field::bool("reqwest.is_connect", err.is_connect()))
        .with(field::bool("reqwest.is_request", err.is_request()))
        .with(field::bool("reqwest.is_status", err.is_status()))
        .with(field::bool("reqwest.is_body", err.is_body()))
        .with(field::bool("reqwest.is_decode", err.is_decode()))
        .with(field::bool("reqwest.is_redirect", err.is_redirect()));
    let mut retry_after = None;
    if err.is_timeout() {
        context = context.category(AppErrorKind::Timeout);
    } else if err.is_connect() || err.is_request() {
        context = context.category(AppErrorKind::Network);
    }
    if let Some(status) = err.status() {
        let status_code = u16::from(status);
        context = context.with(field::u64("http.status", u64::from(status_code)));
        if let Some(reason) = status.canonical_reason() {
            context = context.with(field::str("http.status_reason", reason));
        }
        context = match status {
            StatusCode::TOO_MANY_REQUESTS => {
                retry_after = Some(1);
                context.category(AppErrorKind::RateLimited)
            }
            StatusCode::REQUEST_TIMEOUT => context.category(AppErrorKind::Timeout),
            s if s.is_server_error() => context.category(AppErrorKind::DependencyUnavailable),
            _ => context
        };
    }
    if let Some(url) = err.url() {
        context = context
            .with(field::str("http.url", url.to_string()))
            .redact_field("http.url", FieldRedaction::Hash);
        if let Some(host) = url.host_str() {
            context = context.with(field::str("http.host", host.to_owned()));
        }
        if let Some(port) = url.port() {
            context = context.with(field::u64("http.port", u64::from(port)));
        }
        let path = url.path();
        if !path.is_empty() {
            context = context.with(field::str("http.path", path.to_owned()));
        }
        let scheme = url.scheme();
        if !scheme.is_empty() {
            context = context.with(field::str("http.scheme", scheme.to_owned()));
        }
    }
    (context, retry_after)
}

#[cfg(all(test, feature = "reqwest", feature = "tokio"))]
mod tests {
    use std::time::Duration;

    use reqwest::Client;
    use tokio::{net::TcpListener, time::sleep};

    use super::*;
    use crate::{AppCode, AppErrorKind, FieldRedaction, FieldValue};

    #[tokio::test]
    async fn timeout_sets_category_and_metadata() {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind listener");
        let addr = listener.local_addr().expect("listener addr");
        let server = tokio::spawn(async move {
            let (_socket, _) = listener.accept().await.expect("accept");
            sleep(Duration::from_secs(5)).await;
        });
        let client = Client::builder()
            .timeout(Duration::from_millis(50))
            .build()
            .expect("client");
        let err = client
            .get(format!("http://{addr}"))
            .send()
            .await
            .expect_err("expected timeout");
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::Timeout);
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("reqwest.is_timeout"),
            Some(&FieldValue::Bool(true))
        );
        assert_eq!(metadata.redaction("http.url"), Some(FieldRedaction::Hash));
        server.abort();
    }

    #[tokio::test]
    async fn status_error_maps_retry_and_rate_limit() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut buf = [0_u8; 1024];
            let _ = socket.read(&mut buf).await;
            let response = b"HTTP/1.1 429 Too Many Requests\r\ncontent-length: 0\r\n\r\n";
            let _ = socket.write_all(response).await;
        });
        let client = Client::new();
        let response = client
            .get(format!("http://{addr}"))
            .send()
            .await
            .expect("send");
        let err = response.error_for_status().expect_err("status error");
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::RateLimited);
        assert_eq!(app_err.code, AppCode::RateLimited);
        assert_eq!(app_err.retry.map(|r| r.after_seconds), Some(1));
        let metadata = app_err.metadata();
        assert_eq!(metadata.get("http.status"), Some(&FieldValue::U64(429)));
        assert_eq!(
            metadata.get("http.port"),
            Some(&FieldValue::U64(u64::from(addr.port())))
        );
        server.abort();
    }

    #[tokio::test]
    async fn server_error_maps_to_dependency_unavailable() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut buf = [0_u8; 1024];
            let _ = socket.read(&mut buf).await;
            let response = b"HTTP/1.1 500 Internal Server Error\r\ncontent-length: 0\r\n\r\n";
            let _ = socket.write_all(response).await;
        });
        let client = Client::new();
        let response = client
            .get(format!("http://{addr}"))
            .send()
            .await
            .expect("send");
        let err = response.error_for_status().expect_err("status error");
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::DependencyUnavailable);
        let metadata = app_err.metadata();
        assert_eq!(metadata.get("http.status"), Some(&FieldValue::U64(500)));
        server.abort();
    }

    #[tokio::test]
    async fn request_timeout_status_maps_to_timeout() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut buf = [0_u8; 1024];
            let _ = socket.read(&mut buf).await;
            let response = b"HTTP/1.1 408 Request Timeout\r\ncontent-length: 0\r\n\r\n";
            let _ = socket.write_all(response).await;
        });
        let client = Client::new();
        let response = client
            .get(format!("http://{addr}"))
            .send()
            .await
            .expect("send");
        let err = response.error_for_status().expect_err("status error");
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::Timeout);
        let metadata = app_err.metadata();
        assert_eq!(metadata.get("http.status"), Some(&FieldValue::U64(408)));
        server.abort();
    }

    #[tokio::test]
    async fn client_error_maps_to_external_api() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut buf = [0_u8; 1024];
            let _ = socket.read(&mut buf).await;
            let response = b"HTTP/1.1 400 Bad Request\r\ncontent-length: 0\r\n\r\n";
            let _ = socket.write_all(response).await;
        });
        let client = Client::new();
        let response = client
            .get(format!("http://{addr}"))
            .send()
            .await
            .expect("send");
        let err = response.error_for_status().expect_err("status error");
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::ExternalApi);
        let metadata = app_err.metadata();
        assert_eq!(metadata.get("http.status"), Some(&FieldValue::U64(400)));
        server.abort();
    }

    #[tokio::test]
    async fn connect_error_maps_to_network() {
        let client = Client::new();
        let err = client
            .get("http://127.0.0.1:1")
            .send()
            .await
            .expect_err("connection refused");
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::Network);
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("reqwest.is_connect"),
            Some(&FieldValue::Bool(true))
        );
    }

    #[tokio::test]
    async fn url_metadata_is_captured() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut buf = [0_u8; 1024];
            let _ = socket.read(&mut buf).await;
            let response = b"HTTP/1.1 404 Not Found\r\ncontent-length: 0\r\n\r\n";
            let _ = socket.write_all(response).await;
        });
        let client = Client::new();
        let response = client
            .get(format!("http://{addr}/api/v1/test"))
            .send()
            .await
            .expect("send");
        let err = response.error_for_status().expect_err("status error");
        let app_err: Error = err.into();
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("http.host"),
            Some(&FieldValue::Str("127.0.0.1".into()))
        );
        assert_eq!(
            metadata.get("http.port"),
            Some(&FieldValue::U64(u64::from(addr.port())))
        );
        assert_eq!(
            metadata.get("http.path"),
            Some(&FieldValue::Str("/api/v1/test".into()))
        );
        assert_eq!(
            metadata.get("http.scheme"),
            Some(&FieldValue::Str("http".into()))
        );
        assert_eq!(metadata.redaction("http.url"), Some(FieldRedaction::Hash));
        server.abort();
    }

    #[tokio::test]
    async fn status_reason_is_captured() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut buf = [0_u8; 1024];
            let _ = socket.read(&mut buf).await;
            let response = b"HTTP/1.1 403 Forbidden\r\ncontent-length: 0\r\n\r\n";
            let _ = socket.write_all(response).await;
        });
        let client = Client::new();
        let response = client
            .get(format!("http://{addr}"))
            .send()
            .await
            .expect("send");
        let err = response.error_for_status().expect_err("status error");
        let app_err: Error = err.into();
        let metadata = app_err.metadata();
        assert_eq!(
            metadata.get("http.status_reason"),
            Some(&FieldValue::Str("Forbidden".into()))
        );
        server.abort();
    }

    #[tokio::test]
    async fn all_reqwest_flags_are_captured() {
        let client = Client::new();
        let err = client
            .get("http://127.0.0.1:1")
            .send()
            .await
            .expect_err("connection refused");
        let app_err: Error = err.into();
        let metadata = app_err.metadata();
        assert!(metadata.get("reqwest.is_timeout").is_some());
        assert!(metadata.get("reqwest.is_connect").is_some());
        assert!(metadata.get("reqwest.is_request").is_some());
        assert!(metadata.get("reqwest.is_status").is_some());
        assert!(metadata.get("reqwest.is_body").is_some());
        assert!(metadata.get("reqwest.is_decode").is_some());
        assert!(metadata.get("reqwest.is_redirect").is_some());
    }

    #[tokio::test]
    async fn service_unavailable_maps_correctly() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut buf = [0_u8; 1024];
            let _ = socket.read(&mut buf).await;
            let response = b"HTTP/1.1 503 Service Unavailable\r\ncontent-length: 0\r\n\r\n";
            let _ = socket.write_all(response).await;
        });
        let client = Client::new();
        let response = client
            .get(format!("http://{addr}"))
            .send()
            .await
            .expect("send");
        let err = response.error_for_status().expect_err("status error");
        let app_err: Error = err.into();
        assert_eq!(app_err.kind, AppErrorKind::DependencyUnavailable);
        server.abort();
    }
}
