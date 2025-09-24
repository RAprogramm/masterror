use std::{borrow::Cow, collections::BTreeMap, fmt::Write, net::IpAddr};

use http::StatusCode;
use serde::Serialize;
#[cfg(feature = "serde_json")]
use serde_json::Value as JsonValue;
use sha2::{Digest, Sha256};

use super::core::ErrorResponse;
use crate::{
    AppCode, AppError, AppErrorKind, FieldRedaction, FieldValue, MessageEditPolicy, Metadata,
    app_error::duration_to_string
};

/// Canonical mapping for a public [`AppCode`].
///
/// # Examples
///
/// ```rust
/// use masterror::{AppCode, mapping_for_code};
///
/// let mapping = mapping_for_code(AppCode::NotFound);
/// assert_eq!(mapping.http_status(), 404);
/// assert_eq!(
///     mapping.problem_type(),
///     "https://errors.masterror.rs/not-found"
/// );
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeMapping {
    http_status:  u16,
    grpc:         GrpcCode,
    problem_type: &'static str,
    kind:         AppErrorKind
}

impl CodeMapping {
    /// HTTP status code associated with the [`AppCode`].
    #[cfg_attr(not(any(test, feature = "tonic")), allow(dead_code))]
    #[must_use]
    pub const fn http_status(&self) -> u16 {
        self.http_status
    }

    /// gRPC code mapping (`tonic::Code` discriminant).
    #[must_use]
    pub const fn grpc(&self) -> GrpcCode {
        self.grpc
    }

    /// Canonical RFC 7807 problem type URI.
    #[must_use]
    pub const fn problem_type(&self) -> &'static str {
        self.problem_type
    }

    /// Canonical error kind for presentation.
    #[must_use]
    pub const fn kind(&self) -> AppErrorKind {
        self.kind
    }
}

/// gRPC status metadata used in RFC7807 payloads and tonic mapping.
///
/// The `value` matches the discriminant of `tonic::Code`, allowing direct
/// conversion when the `tonic` feature is enabled.
///
/// # Examples
///
/// ```rust
/// use masterror::{AppCode, mapping_for_code};
///
/// let grpc = mapping_for_code(AppCode::Internal).grpc();
/// assert_eq!(grpc.name, "INTERNAL");
/// assert_eq!(grpc.value, 13);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct GrpcCode {
    /// Canonical name (e.g. `"NOT_FOUND"`).
    pub name:  &'static str,
    /// Numeric discriminant matching `tonic::Code`.
    pub value: i32
}

/// RFC7807 `application/problem+json` payload enriched with machine-readable
/// metadata.
///
/// Instances are produced by [`ProblemJson::from_app_error`] or
/// [`ProblemJson::from_ref`]. They power the HTTP adapters and expose
/// transport-neutral data for tests.
///
/// # Examples
///
/// ```rust
/// use masterror::{AppError, ProblemJson};
///
/// let problem = ProblemJson::from_ref(&AppError::not_found("missing"));
/// assert_eq!(problem.status, 404);
/// assert_eq!(problem.code.as_str(), "NOT_FOUND");
/// ```
#[derive(Clone, Debug, Serialize)]
pub struct ProblemJson {
    /// Canonical type URI describing the problem class.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_uri:         Option<Cow<'static, str>>,
    /// Short, human-friendly title describing the error category.
    pub title:            Cow<'static, str>,
    /// HTTP status code returned to the client.
    pub status:           u16,
    /// Optional human-readable detail (redacted when marked private).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail:           Option<Cow<'static, str>>,
    /// Stable machine-readable code.
    pub code:             AppCode,
    /// Optional gRPC mapping for multi-protocol clients.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc:             Option<GrpcCode>,
    /// Structured metadata derived from [`Metadata`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata:         Option<ProblemMetadata>,
    /// Retry advice propagated as the `Retry-After` header.
    #[serde(skip)]
    pub retry_after:      Option<u64>,
    /// Authentication challenge propagated as `WWW-Authenticate`.
    #[serde(skip)]
    pub www_authenticate: Option<String>
}

impl ProblemJson {
    /// Build a problem payload from an owned [`AppError`].
    ///
    /// # Preconditions
    /// - `error.code` must be a public [`AppCode`] (guaranteed by
    ///   construction).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppCode, AppError, ProblemJson};
    ///
    /// let problem = ProblemJson::from_app_error(AppError::conflict("exists"));
    /// assert_eq!(problem.code, AppCode::Conflict);
    /// assert_eq!(problem.status, 409);
    /// ```
    #[must_use]
    pub fn from_app_error(mut error: AppError) -> Self {
        error.emit_telemetry();

        let code = error.code;
        let kind = error.kind;
        let message = error.message.take();
        let metadata = core::mem::take(&mut error.metadata);
        let edit_policy = error.edit_policy;
        let retry = error.retry.take();
        let www_authenticate = error.www_authenticate.take();

        let mapping = mapping_for_code(code);
        let status = kind.http_status();
        let title = Cow::Owned(kind.to_string());
        let detail = sanitize_detail(message, kind, edit_policy);
        let metadata = sanitize_metadata_owned(metadata, edit_policy);

        Self {
            type_uri: Some(Cow::Borrowed(mapping.problem_type())),
            title,
            status,
            detail,
            code,
            grpc: Some(mapping.grpc()),
            metadata,
            retry_after: retry.map(|value| value.after_seconds),
            www_authenticate
        }
    }

    /// Build a problem payload from a borrowed [`AppError`].
    ///
    /// This is useful inside middleware that logs while forwarding the error
    /// downstream without consuming it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppError, ProblemJson};
    ///
    /// let err = AppError::bad_request("invalid");
    /// let problem = ProblemJson::from_ref(&err);
    /// assert_eq!(problem.status, 400);
    /// assert!(problem.detail.is_some());
    /// ```
    #[must_use]
    pub fn from_ref(error: &AppError) -> Self {
        let mapping = mapping_for_code(error.code);
        let status = error.kind.http_status();
        let title = Cow::Owned(error.kind.to_string());
        let detail = sanitize_detail_ref(error);
        let metadata = sanitize_metadata_ref(error.metadata(), error.edit_policy);

        Self {
            type_uri: Some(Cow::Borrowed(mapping.problem_type())),
            title,
            status,
            detail,
            code: error.code,
            grpc: Some(mapping.grpc()),
            metadata,
            retry_after: error.retry.map(|value| value.after_seconds),
            www_authenticate: error.www_authenticate.clone()
        }
    }

    /// Build a problem payload from a plain [`ErrorResponse`].
    ///
    /// Metadata and redaction hints are not available in this conversion.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use masterror::{AppCode, ErrorResponse, ProblemJson};
    ///
    /// let legacy = ErrorResponse::new(404, AppCode::NotFound, "missing").expect("status");
    /// let problem = ProblemJson::from_error_response(legacy);
    /// assert_eq!(problem.status, 404);
    /// assert_eq!(problem.code.as_str(), "NOT_FOUND");
    /// ```
    #[must_use]
    pub fn from_error_response(response: ErrorResponse) -> Self {
        let mapping = mapping_for_code(response.code);
        let detail = if response.message.is_empty() {
            None
        } else {
            Some(Cow::Owned(response.message))
        };

        Self {
            type_uri: Some(Cow::Borrowed(mapping.problem_type())),
            title: Cow::Owned(mapping.kind().to_string()),
            status: response.status,
            detail,
            code: response.code,
            grpc: Some(mapping.grpc()),
            metadata: None,
            retry_after: response.retry.map(|value| value.after_seconds),
            www_authenticate: response.www_authenticate
        }
    }

    /// Convert numeric status into [`StatusCode`].
    ///
    /// Falls back to `500 Internal Server Error` if the value is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use http::StatusCode;
    /// use masterror::{AppError, ProblemJson};
    ///
    /// let problem = ProblemJson::from_app_error(AppError::service("oops"));
    /// assert_eq!(problem.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    /// ```
    #[must_use]
    pub fn status_code(&self) -> StatusCode {
        match StatusCode::from_u16(self.status) {
            Ok(status) => status,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    /// Formatter exposing internals for diagnostic logging.
    #[must_use]
    pub fn internal(&self) -> crate::response::internal::ProblemJsonFormatter<'_> {
        crate::response::internal::ProblemJsonFormatter::new(self)
    }
}

/// Metadata section of a [`ProblemJson`] payload.
///
/// # Examples
///
/// ```rust
/// use masterror::{AppError, ProblemJson};
///
/// let err = AppError::service("retry").with_field(masterror::field::u64("attempt", 1));
/// let problem = ProblemJson::from_ref(&err);
/// assert!(problem.metadata.is_some());
/// ```
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct ProblemMetadata(BTreeMap<Cow<'static, str>, ProblemMetadataValue>);

impl ProblemMetadata {
    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Individual metadata value serialized in problem payloads.
///
/// # Examples
///
/// ```rust
/// use masterror::{ProblemMetadataValue, field};
///
/// let (_name, field_value, _redaction) = field::u64("attempt", 2).into_parts();
/// let value = ProblemMetadataValue::from(field_value);
/// assert!(matches!(value, ProblemMetadataValue::U64(2)));
/// ```
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum ProblemMetadataValue {
    /// String value preserved as-is.
    String(Cow<'static, str>),
    /// Signed 64-bit integer.
    I64(i64),
    /// Unsigned 64-bit integer.
    U64(u64),
    /// Floating-point number.
    F64(f64),
    /// Boolean flag serialized as `true`/`false`.
    Bool(bool),
    /// Duration represented as seconds plus nanoseconds remainder.
    Duration {
        /// Whole seconds component of the duration.
        secs:  u64,
        /// Additional nanoseconds (always less than one second).
        nanos: u32
    },
    /// IP address (v4 or v6).
    Ip(IpAddr),
    /// Structured JSON payload (requires the `serde_json` feature).
    #[cfg(feature = "serde_json")]
    Json(JsonValue)
}

impl From<FieldValue> for ProblemMetadataValue {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::Str(value) => Self::String(value),
            FieldValue::I64(value) => Self::I64(value),
            FieldValue::U64(value) => Self::U64(value),
            FieldValue::F64(value) => Self::F64(value),
            FieldValue::Bool(value) => Self::Bool(value),
            FieldValue::Uuid(value) => Self::String(Cow::Owned(value.to_string())),
            FieldValue::Duration(value) => Self::Duration {
                secs:  value.as_secs(),
                nanos: value.subsec_nanos()
            },
            FieldValue::Ip(value) => Self::Ip(value),
            #[cfg(feature = "serde_json")]
            FieldValue::Json(value) => Self::Json(value)
        }
    }
}

impl From<&FieldValue> for ProblemMetadataValue {
    fn from(value: &FieldValue) -> Self {
        match value {
            FieldValue::Str(value) => Self::String(value.clone()),
            FieldValue::I64(value) => Self::I64(*value),
            FieldValue::U64(value) => Self::U64(*value),
            FieldValue::F64(value) => Self::F64(*value),
            FieldValue::Bool(value) => Self::Bool(*value),
            FieldValue::Uuid(value) => Self::String(Cow::Owned(value.to_string())),
            FieldValue::Duration(value) => Self::Duration {
                secs:  value.as_secs(),
                nanos: value.subsec_nanos()
            },
            FieldValue::Ip(value) => Self::Ip(*value),
            #[cfg(feature = "serde_json")]
            FieldValue::Json(value) => Self::Json(value.clone())
        }
    }
}

fn sanitize_detail(
    message: Option<Cow<'static, str>>,
    kind: AppErrorKind,
    policy: MessageEditPolicy
) -> Option<Cow<'static, str>> {
    if matches!(policy, MessageEditPolicy::Redact) {
        return None;
    }

    Some(message.unwrap_or_else(|| Cow::Owned(kind.to_string())))
}

fn sanitize_detail_ref(error: &AppError) -> Option<Cow<'static, str>> {
    if matches!(error.edit_policy, MessageEditPolicy::Redact) {
        return None;
    }

    Some(Cow::Owned(error.render_message().into_owned()))
}

fn sanitize_metadata_owned(
    metadata: Metadata,
    policy: MessageEditPolicy
) -> Option<ProblemMetadata> {
    if matches!(policy, MessageEditPolicy::Redact) || metadata.is_empty() {
        return None;
    }

    let mut public = BTreeMap::new();
    for field in metadata {
        let (name, value, redaction) = field.into_parts();
        if let Some(sanitized) = sanitize_problem_metadata_value_owned(value, redaction) {
            public.insert(Cow::Borrowed(name), sanitized);
        }
    }

    if public.is_empty() {
        None
    } else {
        Some(ProblemMetadata(public))
    }
}

fn sanitize_metadata_ref(
    metadata: &Metadata,
    policy: MessageEditPolicy
) -> Option<ProblemMetadata> {
    if matches!(policy, MessageEditPolicy::Redact) || metadata.is_empty() {
        return None;
    }

    let mut public = BTreeMap::new();
    for (name, value, redaction) in metadata.iter_with_redaction() {
        if let Some(sanitized) = sanitize_problem_metadata_value_ref(value, redaction) {
            public.insert(Cow::Borrowed(name), sanitized);
        }
    }

    if public.is_empty() {
        None
    } else {
        Some(ProblemMetadata(public))
    }
}

const REDACTED_PLACEHOLDER: &str = "[REDACTED]";

fn sanitize_problem_metadata_value_owned(
    value: FieldValue,
    redaction: FieldRedaction
) -> Option<ProblemMetadataValue> {
    match redaction {
        FieldRedaction::None => Some(ProblemMetadataValue::from(value)),
        FieldRedaction::Redact => Some(ProblemMetadataValue::String(Cow::Borrowed(
            REDACTED_PLACEHOLDER
        ))),
        FieldRedaction::Hash => Some(ProblemMetadataValue::String(Cow::Owned(hash_field_value(
            &value
        )))),
        FieldRedaction::Last4 => mask_last4_field_value(&value)
            .map(|masked| ProblemMetadataValue::String(Cow::Owned(masked)))
    }
}

fn sanitize_problem_metadata_value_ref(
    value: &FieldValue,
    redaction: FieldRedaction
) -> Option<ProblemMetadataValue> {
    match redaction {
        FieldRedaction::None => Some(ProblemMetadataValue::from(value)),
        FieldRedaction::Redact => Some(ProblemMetadataValue::String(Cow::Borrowed(
            REDACTED_PLACEHOLDER
        ))),
        FieldRedaction::Hash => Some(ProblemMetadataValue::String(Cow::Owned(hash_field_value(
            value
        )))),
        FieldRedaction::Last4 => mask_last4_field_value(value)
            .map(|masked| ProblemMetadataValue::String(Cow::Owned(masked)))
    }
}

fn hash_field_value(value: &FieldValue) -> String {
    let mut hasher = Sha256::new();
    match value {
        FieldValue::Str(value) => hasher.update(value.as_ref().as_bytes()),
        FieldValue::I64(value) => {
            let string = value.to_string();
            hasher.update(string.as_bytes());
        }
        FieldValue::U64(value) => {
            let string = value.to_string();
            hasher.update(string.as_bytes());
        }
        FieldValue::F64(value) => hasher.update(value.to_le_bytes()),
        FieldValue::Bool(value) => {
            if *value {
                hasher.update(b"true");
            } else {
                hasher.update(b"false");
            }
        }
        FieldValue::Uuid(value) => {
            let string = value.to_string();
            hasher.update(string.as_bytes());
        }
        FieldValue::Duration(value) => {
            hasher.update(value.as_secs().to_le_bytes());
            hasher.update(value.subsec_nanos().to_le_bytes());
        }
        FieldValue::Ip(value) => match value {
            IpAddr::V4(addr) => hasher.update(addr.octets()),
            IpAddr::V6(addr) => hasher.update(addr.octets())
        },
        #[cfg(feature = "serde_json")]
        FieldValue::Json(value) => {
            if let Ok(serialized) = serde_json::to_vec(value) {
                hasher.update(&serialized);
            }
        }
    }
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        let _ = write!(&mut hex, "{:02x}", byte);
    }
    hex
}

fn mask_last4_field_value(value: &FieldValue) -> Option<String> {
    match value {
        FieldValue::Str(value) => Some(mask_last4(value.as_ref())),
        FieldValue::I64(value) => Some(mask_last4(&value.to_string())),
        FieldValue::U64(value) => Some(mask_last4(&value.to_string())),
        FieldValue::F64(value) => Some(mask_last4(&value.to_string())),
        FieldValue::Uuid(value) => Some(mask_last4(&value.to_string())),
        FieldValue::Duration(value) => Some(mask_last4(&duration_to_string(*value))),
        FieldValue::Ip(value) => Some(mask_last4(&value.to_string())),
        #[cfg(feature = "serde_json")]
        FieldValue::Json(value) => serde_json::to_string(value)
            .ok()
            .map(|text| mask_last4(&text)),
        FieldValue::Bool(_) => None
    }
}

fn mask_last4(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();
    let total = chars.len();
    if total == 0 {
        return String::new();
    }

    let keep = if total <= 4 { 1 } else { 4 };
    let mask_len = total.saturating_sub(keep);
    let mut masked = String::with_capacity(value.len());
    for _ in 0..mask_len {
        masked.push('*');
    }
    for ch in chars.iter().skip(mask_len) {
        masked.push(*ch);
    }
    masked
}

/// Canonical mapping table covering every built-in [`AppCode`].
///
/// # Examples
///
/// ```rust
/// use masterror::CODE_MAPPINGS;
///
/// assert!(
///     CODE_MAPPINGS
///         .iter()
///         .any(|(code, _)| code.as_str() == "NOT_FOUND")
/// );
/// ```
pub const CODE_MAPPINGS: &[(AppCode, CodeMapping)] = &[
    (
        AppCode::NotFound,
        CodeMapping {
            http_status:  404,
            grpc:         GrpcCode {
                name:  "NOT_FOUND",
                value: 5
            },
            problem_type: "https://errors.masterror.rs/not-found",
            kind:         AppErrorKind::NotFound
        }
    ),
    (
        AppCode::Validation,
        CodeMapping {
            http_status:  422,
            grpc:         GrpcCode {
                name:  "INVALID_ARGUMENT",
                value: 3
            },
            problem_type: "https://errors.masterror.rs/validation",
            kind:         AppErrorKind::Validation
        }
    ),
    (
        AppCode::Conflict,
        CodeMapping {
            http_status:  409,
            grpc:         GrpcCode {
                name:  "ALREADY_EXISTS",
                value: 6
            },
            problem_type: "https://errors.masterror.rs/conflict",
            kind:         AppErrorKind::Conflict
        }
    ),
    (
        AppCode::UserAlreadyExists,
        CodeMapping {
            http_status:  409,
            grpc:         GrpcCode {
                name:  "ALREADY_EXISTS",
                value: 6
            },
            problem_type: "https://errors.masterror.rs/user-already-exists",
            kind:         AppErrorKind::Conflict
        }
    ),
    (
        AppCode::Unauthorized,
        CodeMapping {
            http_status:  401,
            grpc:         GrpcCode {
                name:  "UNAUTHENTICATED",
                value: 16
            },
            problem_type: "https://errors.masterror.rs/unauthorized",
            kind:         AppErrorKind::Unauthorized
        }
    ),
    (
        AppCode::Forbidden,
        CodeMapping {
            http_status:  403,
            grpc:         GrpcCode {
                name:  "PERMISSION_DENIED",
                value: 7
            },
            problem_type: "https://errors.masterror.rs/forbidden",
            kind:         AppErrorKind::Forbidden
        }
    ),
    (
        AppCode::NotImplemented,
        CodeMapping {
            http_status:  501,
            grpc:         GrpcCode {
                name:  "UNIMPLEMENTED",
                value: 12
            },
            problem_type: "https://errors.masterror.rs/not-implemented",
            kind:         AppErrorKind::NotImplemented
        }
    ),
    (
        AppCode::BadRequest,
        CodeMapping {
            http_status:  400,
            grpc:         GrpcCode {
                name:  "INVALID_ARGUMENT",
                value: 3
            },
            problem_type: "https://errors.masterror.rs/bad-request",
            kind:         AppErrorKind::BadRequest
        }
    ),
    (
        AppCode::RateLimited,
        CodeMapping {
            http_status:  429,
            grpc:         GrpcCode {
                name:  "RESOURCE_EXHAUSTED",
                value: 8
            },
            problem_type: "https://errors.masterror.rs/rate-limited",
            kind:         AppErrorKind::RateLimited
        }
    ),
    (
        AppCode::TelegramAuth,
        CodeMapping {
            http_status:  401,
            grpc:         GrpcCode {
                name:  "UNAUTHENTICATED",
                value: 16
            },
            problem_type: "https://errors.masterror.rs/telegram-auth",
            kind:         AppErrorKind::TelegramAuth
        }
    ),
    (
        AppCode::InvalidJwt,
        CodeMapping {
            http_status:  401,
            grpc:         GrpcCode {
                name:  "UNAUTHENTICATED",
                value: 16
            },
            problem_type: "https://errors.masterror.rs/invalid-jwt",
            kind:         AppErrorKind::InvalidJwt
        }
    ),
    (
        AppCode::Internal,
        CodeMapping {
            http_status:  500,
            grpc:         GrpcCode {
                name:  "INTERNAL",
                value: 13
            },
            problem_type: "https://errors.masterror.rs/internal",
            kind:         AppErrorKind::Internal
        }
    ),
    (
        AppCode::Database,
        CodeMapping {
            http_status:  500,
            grpc:         GrpcCode {
                name:  "INTERNAL",
                value: 13
            },
            problem_type: "https://errors.masterror.rs/database",
            kind:         AppErrorKind::Database
        }
    ),
    (
        AppCode::Service,
        CodeMapping {
            http_status:  500,
            grpc:         GrpcCode {
                name:  "INTERNAL",
                value: 13
            },
            problem_type: "https://errors.masterror.rs/service",
            kind:         AppErrorKind::Service
        }
    ),
    (
        AppCode::Config,
        CodeMapping {
            http_status:  500,
            grpc:         GrpcCode {
                name:  "INTERNAL",
                value: 13
            },
            problem_type: "https://errors.masterror.rs/config",
            kind:         AppErrorKind::Config
        }
    ),
    (
        AppCode::Turnkey,
        CodeMapping {
            http_status:  500,
            grpc:         GrpcCode {
                name:  "INTERNAL",
                value: 13
            },
            problem_type: "https://errors.masterror.rs/turnkey",
            kind:         AppErrorKind::Turnkey
        }
    ),
    (
        AppCode::Timeout,
        CodeMapping {
            http_status:  504,
            grpc:         GrpcCode {
                name:  "DEADLINE_EXCEEDED",
                value: 4
            },
            problem_type: "https://errors.masterror.rs/timeout",
            kind:         AppErrorKind::Timeout
        }
    ),
    (
        AppCode::Network,
        CodeMapping {
            http_status:  503,
            grpc:         GrpcCode {
                name:  "UNAVAILABLE",
                value: 14
            },
            problem_type: "https://errors.masterror.rs/network",
            kind:         AppErrorKind::Network
        }
    ),
    (
        AppCode::DependencyUnavailable,
        CodeMapping {
            http_status:  503,
            grpc:         GrpcCode {
                name:  "UNAVAILABLE",
                value: 14
            },
            problem_type: "https://errors.masterror.rs/dependency-unavailable",
            kind:         AppErrorKind::DependencyUnavailable
        }
    ),
    (
        AppCode::Serialization,
        CodeMapping {
            http_status:  500,
            grpc:         GrpcCode {
                name:  "INTERNAL",
                value: 13
            },
            problem_type: "https://errors.masterror.rs/serialization",
            kind:         AppErrorKind::Serialization
        }
    ),
    (
        AppCode::Deserialization,
        CodeMapping {
            http_status:  500,
            grpc:         GrpcCode {
                name:  "INTERNAL",
                value: 13
            },
            problem_type: "https://errors.masterror.rs/deserialization",
            kind:         AppErrorKind::Deserialization
        }
    ),
    (
        AppCode::ExternalApi,
        CodeMapping {
            http_status:  500,
            grpc:         GrpcCode {
                name:  "UNAVAILABLE",
                value: 14
            },
            problem_type: "https://errors.masterror.rs/external-api",
            kind:         AppErrorKind::ExternalApi
        }
    ),
    (
        AppCode::Queue,
        CodeMapping {
            http_status:  500,
            grpc:         GrpcCode {
                name:  "UNAVAILABLE",
                value: 14
            },
            problem_type: "https://errors.masterror.rs/queue",
            kind:         AppErrorKind::Queue
        }
    ),
    (
        AppCode::Cache,
        CodeMapping {
            http_status:  500,
            grpc:         GrpcCode {
                name:  "UNAVAILABLE",
                value: 14
            },
            problem_type: "https://errors.masterror.rs/cache",
            kind:         AppErrorKind::Cache
        }
    )
];

const DEFAULT_MAPPING: CodeMapping = CodeMapping {
    http_status:  500,
    grpc:         GrpcCode {
        name:  "INTERNAL",
        value: 13
    },
    problem_type: "https://errors.masterror.rs/internal",
    kind:         AppErrorKind::Internal
};

/// Lookup helper returning canonical mapping for a given [`AppCode`].
///
/// # Examples
///
/// ```rust
/// use masterror::{AppCode, mapping_for_code};
///
/// let mapping = mapping_for_code(AppCode::Timeout);
/// assert_eq!(mapping.grpc().name, "DEADLINE_EXCEEDED");
/// ```
#[must_use]
pub fn mapping_for_code(code: AppCode) -> CodeMapping {
    CODE_MAPPINGS
        .iter()
        .find_map(|(candidate, mapping)| {
            if *candidate == code {
                Some(*mapping)
            } else {
                None
            }
        })
        .unwrap_or(DEFAULT_MAPPING)
}

#[cfg(test)]
mod tests {
    use std::{
        fmt::Write,
        net::{IpAddr, Ipv4Addr},
        time::Duration
    };

    use serde_json::Value;
    use sha2::{Digest, Sha256};

    use super::*;
    use crate::AppError;

    #[test]
    fn metadata_is_skipped_when_redacted() {
        let err = AppError::internal("secret")
            .redactable()
            .with_field(crate::field::str("token", "super-secret"));
        let problem = ProblemJson::from_ref(&err);
        assert!(problem.detail.is_none());
        assert!(problem.metadata.is_none());
    }

    #[test]
    fn metadata_is_serialized_when_allowed() {
        let err = AppError::internal("oops").with_field(crate::field::u64("attempt", 2));
        let problem = ProblemJson::from_ref(&err);
        let metadata = problem.metadata.expect("metadata");
        assert!(!metadata.is_empty());
    }

    #[test]
    fn metadata_preserves_extended_field_types() {
        let mut err = AppError::internal("oops");
        err = err.with_field(crate::field::f64("ratio", 0.25));
        err = err.with_field(crate::field::duration(
            "elapsed",
            Duration::from_millis(1500)
        ));
        err = err.with_field(crate::field::ip(
            "peer",
            IpAddr::from(Ipv4Addr::new(10, 0, 0, 42))
        ));
        #[cfg(feature = "serde_json")]
        {
            err = err.with_field(crate::field::json(
                "payload",
                serde_json::json!({ "status": "ok" })
            ));
        }

        let problem = ProblemJson::from_ref(&err);
        let metadata = problem.metadata.expect("metadata");

        let ratio = metadata.0.get("ratio").expect("ratio metadata");
        assert!(matches!(
            ratio,
            ProblemMetadataValue::F64(value) if (*value - 0.25).abs() < f64::EPSILON
        ));

        let duration = metadata.0.get("elapsed").expect("elapsed metadata");
        assert!(matches!(
            duration,
            ProblemMetadataValue::Duration { secs, nanos }
            if *secs == 1 && *nanos == 500_000_000
        ));

        let ip = metadata.0.get("peer").expect("peer metadata");
        assert!(matches!(ip, ProblemMetadataValue::Ip(addr) if addr.is_ipv4()));

        #[cfg(feature = "serde_json")]
        {
            let payload = metadata.0.get("payload").expect("payload metadata");
            assert!(matches!(
                payload,
                ProblemMetadataValue::Json(value) if value["status"] == "ok"
            ));
        }
    }

    #[test]
    fn redacted_metadata_uses_placeholder() {
        let err = AppError::internal("oops").with_field(crate::field::str("password", "secret"));
        let problem = ProblemJson::from_ref(&err);
        let metadata = problem.metadata.expect("metadata");
        let value = metadata.0.get("password").expect("password field");
        match value {
            ProblemMetadataValue::String(text) => {
                assert_eq!(text.as_ref(), super::REDACTED_PLACEHOLDER);
            }
            other => panic!("unexpected metadata value: {other:?}")
        }
    }

    #[test]
    fn hashed_metadata_masks_original_value() {
        let err = AppError::internal("oops").with_field(crate::field::str("token", "super"));
        let problem = ProblemJson::from_ref(&err);
        let metadata = problem.metadata.expect("metadata");
        let value = metadata.0.get("token").expect("token field");
        match value {
            ProblemMetadataValue::String(text) => {
                assert_eq!(text.len(), 64);
                assert_ne!(text.as_ref(), "super");
            }
            other => panic!("unexpected metadata value: {other:?}")
        }
    }

    #[test]
    fn last4_metadata_preserves_suffix() {
        let err = AppError::internal("oops")
            .with_field(crate::field::str("card_number", "4111111111111111"));
        let problem = ProblemJson::from_ref(&err);
        let metadata = problem.metadata.expect("metadata");
        let value = metadata.0.get("card_number").expect("card number");
        match value {
            ProblemMetadataValue::String(text) => {
                assert!(text.ends_with("1111"));
                assert!(text.starts_with("************"));
            }
            other => panic!("unexpected metadata value: {other:?}")
        }
    }

    #[test]
    fn problem_json_serialization_masks_sensitive_metadata() {
        let secret = "super-secret";
        let err = AppError::internal("oops").with_field(crate::field::str("token", secret));
        let problem = ProblemJson::from_ref(&err);
        let json = serde_json::to_value(&problem).expect("serialize problem");

        let metadata = json
            .get("metadata")
            .and_then(Value::as_object)
            .expect("metadata present");
        let hashed = metadata
            .get("token")
            .and_then(Value::as_str)
            .expect("hashed token");

        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        let digest = hasher.finalize();
        let expected = digest
            .iter()
            .fold(String::with_capacity(64), |mut acc, byte| {
                let _ = write!(&mut acc, "{:02x}", byte);
                acc
            });

        assert_eq!(hashed, expected);
        assert!(!json.to_string().contains(secret));

        let debug_repr = format!("{:?}", problem.internal());
        assert!(debug_repr.contains("metadata"));
        assert!(!debug_repr.contains(secret));
    }

    #[test]
    fn problem_json_serialization_omits_metadata_when_redacted() {
        let secret_value = "sensitive-value";
        let err = AppError::internal("secret")
            .redactable()
            .with_field(crate::field::str("token", secret_value));
        let problem = ProblemJson::from_ref(&err);
        let json = serde_json::to_value(&problem).expect("serialize problem");

        assert!(json.get("metadata").is_none());
        assert!(!json.to_string().contains(secret_value));

        let debug_repr = format!("{:?}", problem.internal());
        assert!(debug_repr.contains("ProblemJson"));
    }

    #[test]
    fn mapping_for_every_code_matches_http_status() {
        for (code, mapping) in CODE_MAPPINGS {
            let status = mapping.http_status();
            let expected = mapping.kind().http_status();
            assert_eq!(status, expected, "status mismatch for {:?}", code);
        }
    }
}
