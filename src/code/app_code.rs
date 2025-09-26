use alloc::{boxed::Box, string::String};
use core::{
    error::Error as CoreError,
    fmt::{self, Display},
    str::FromStr
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::kind::AppErrorKind;

/// Error returned when parsing [`AppCode`] from a string fails.
///
/// The parser only accepts SCREAMING_SNAKE_CASE values accepted by
/// [`AppCode::new`] and [`AppCode::try_new`]. Any other value results in this
/// error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseAppCodeError;

impl Display for ParseAppCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid app code")
    }
}

impl CoreError for ParseAppCodeError {}

/// Stable machine-readable error code exposed to clients.
///
/// Values are serialized as **SCREAMING_SNAKE_CASE** strings (e.g.,
/// `"NOT_FOUND"`). This type is part of the public wire contract and supports
/// both built-in constants and caller-defined codes created via
/// [`AppCode::new`] or [`AppCode::try_new`].
///
/// Design rules:
/// - Keep the set small and meaningful.
/// - Prefer adding new variants over overloading existing ones.
/// - Do not encode private/internal details in codes.
/// - Validate custom codes using [`AppCode::try_new`] before exposing them
///   publicly.
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AppCode {
    repr: CodeRepr
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CodeRepr {
    Static(&'static str),
    Owned(Box<str>)
}

#[allow(non_upper_case_globals)]
impl AppCode {
    /// Machine code emitted when a resource is not found.
    pub const NotFound: Self = Self::from_static("NOT_FOUND");
    /// Machine code emitted when validation fails.
    pub const Validation: Self = Self::from_static("VALIDATION");
    /// Machine code emitted when a conflict is detected.
    pub const Conflict: Self = Self::from_static("CONFLICT");
    /// Machine code emitted when attempting to create an existing user.
    pub const UserAlreadyExists: Self = Self::from_static("USER_ALREADY_EXISTS");
    /// Machine code emitted when authentication fails or is required.
    pub const Unauthorized: Self = Self::from_static("UNAUTHORIZED");
    /// Machine code emitted when an operation is not permitted.
    pub const Forbidden: Self = Self::from_static("FORBIDDEN");
    /// Machine code emitted when functionality is missing.
    pub const NotImplemented: Self = Self::from_static("NOT_IMPLEMENTED");
    /// Machine code emitted when a request is malformed.
    pub const BadRequest: Self = Self::from_static("BAD_REQUEST");
    /// Machine code emitted when a caller is throttled.
    pub const RateLimited: Self = Self::from_static("RATE_LIMITED");
    /// Machine code emitted when Telegram authentication fails.
    pub const TelegramAuth: Self = Self::from_static("TELEGRAM_AUTH");
    /// Machine code emitted when a JWT token is invalid.
    pub const InvalidJwt: Self = Self::from_static("INVALID_JWT");
    /// Machine code emitted for internal server failures.
    pub const Internal: Self = Self::from_static("INTERNAL");
    /// Machine code emitted for database-related issues.
    pub const Database: Self = Self::from_static("DATABASE");
    /// Machine code emitted for service-layer failures.
    pub const Service: Self = Self::from_static("SERVICE");
    /// Machine code emitted for configuration issues.
    pub const Config: Self = Self::from_static("CONFIG");
    /// Machine code emitted for Turnkey integration failures.
    pub const Turnkey: Self = Self::from_static("TURNKEY");
    /// Machine code emitted for timeout failures.
    pub const Timeout: Self = Self::from_static("TIMEOUT");
    /// Machine code emitted for network issues.
    pub const Network: Self = Self::from_static("NETWORK");
    /// Machine code emitted when dependencies are unavailable.
    pub const DependencyUnavailable: Self = Self::from_static("DEPENDENCY_UNAVAILABLE");
    /// Machine code emitted for serialization failures.
    pub const Serialization: Self = Self::from_static("SERIALIZATION");
    /// Machine code emitted for deserialization failures.
    pub const Deserialization: Self = Self::from_static("DESERIALIZATION");
    /// Machine code emitted when an external API fails.
    pub const ExternalApi: Self = Self::from_static("EXTERNAL_API");
    /// Machine code emitted for queue processing errors.
    pub const Queue: Self = Self::from_static("QUEUE");
    /// Machine code emitted for cache subsystem failures.
    pub const Cache: Self = Self::from_static("CACHE");

    const fn from_static(code: &'static str) -> Self {
        Self {
            repr: CodeRepr::Static(code)
        }
    }

    fn from_owned(code: String) -> Self {
        Self {
            repr: CodeRepr::Owned(code.into_boxed_str())
        }
    }

    /// Construct an [`AppCode`] from a compile-time string literal.
    ///
    /// # Examples
    /// ```
    /// use masterror::AppCode;
    ///
    /// let code = AppCode::new("INVALID_JSON");
    /// assert_eq!(code.as_str(), "INVALID_JSON");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics when the literal is not SCREAMING_SNAKE_CASE. Use
    /// [`AppCode::try_new`] to validate dynamic strings at runtime.
    #[must_use]
    pub const fn new(code: &'static str) -> Self {
        if !is_valid_literal(code) {
            panic!("AppCode literals must be SCREAMING_SNAKE_CASE");
        }
        Self::from_static(code)
    }

    /// Construct an [`AppCode`] from a dynamically provided string.
    ///
    /// The input must be SCREAMING_SNAKE_CASE. This constructor allocates to
    /// own the string, making it suitable for runtime-defined codes.
    ///
    /// # Errors
    ///
    /// Returns [`ParseAppCodeError`] when the string is empty or contains
    /// characters outside of `A-Z`, `0-9`, and `_`.
    ///
    /// # Examples
    /// ```
    /// use masterror::AppCode;
    ///
    /// let code = AppCode::try_new(String::from("THIRD_PARTY_FAILURE"))?;
    /// assert_eq!(code.as_str(), "THIRD_PARTY_FAILURE");
    /// # Ok::<(), masterror::ParseAppCodeError>(())
    /// ```
    pub fn try_new(code: impl Into<String>) -> Result<Self, ParseAppCodeError> {
        let code = code.into();
        validate_code(&code)?;
        Ok(Self::from_owned(code))
    }

    /// Get the canonical string form of this code (SCREAMING_SNAKE_CASE).
    ///
    /// This matches the JSON serialization.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match &self.repr {
            CodeRepr::Static(value) => value,
            CodeRepr::Owned(value) => value
        }
    }
}

impl Display for AppCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Stable human/machine readable form matching JSON representation.
        f.write_str(self.as_str())
    }
}

/// Parse an [`AppCode`] from its canonical string representation.
///
/// # Errors
///
/// Returns [`ParseAppCodeError`] when the input is not SCREAMING_SNAKE_CASE.
///
/// # Examples
/// ```
/// use std::str::FromStr;
///
/// use masterror::{AppCode, ParseAppCodeError};
///
/// let code = AppCode::from_str("NOT_FOUND")?;
/// assert_eq!(code, AppCode::NotFound);
/// # Ok::<(), ParseAppCodeError>(())
/// ```
impl FromStr for AppCode {
    type Err = ParseAppCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(code) = match_static(s) {
            return Ok(code);
        }

        Self::try_new(s.to_owned())
    }
}

impl From<AppErrorKind> for AppCode {
    /// Map internal taxonomy (`AppErrorKind`) to public machine code
    /// (`AppCode`).
    ///
    /// The mapping is 1:1 today and intentionally conservative.
    fn from(kind: AppErrorKind) -> Self {
        match kind {
            // 4xx
            AppErrorKind::NotFound => Self::NotFound,
            AppErrorKind::Validation => Self::Validation,
            AppErrorKind::Conflict => Self::Conflict,
            AppErrorKind::Unauthorized => Self::Unauthorized,
            AppErrorKind::Forbidden => Self::Forbidden,
            AppErrorKind::NotImplemented => Self::NotImplemented,
            AppErrorKind::BadRequest => Self::BadRequest,
            AppErrorKind::RateLimited => Self::RateLimited,
            AppErrorKind::TelegramAuth => Self::TelegramAuth,
            AppErrorKind::InvalidJwt => Self::InvalidJwt,

            // 5xx
            AppErrorKind::Internal => Self::Internal,
            AppErrorKind::Database => Self::Database,
            AppErrorKind::Service => Self::Service,
            AppErrorKind::Config => Self::Config,
            AppErrorKind::Turnkey => Self::Turnkey,
            AppErrorKind::Timeout => Self::Timeout,
            AppErrorKind::Network => Self::Network,
            AppErrorKind::DependencyUnavailable => Self::DependencyUnavailable,
            AppErrorKind::Serialization => Self::Serialization,
            AppErrorKind::Deserialization => Self::Deserialization,
            AppErrorKind::ExternalApi => Self::ExternalApi,
            AppErrorKind::Queue => Self::Queue,
            AppErrorKind::Cache => Self::Cache
        }
    }
}

impl Serialize for AppCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for AppCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AppCode;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a SCREAMING_SNAKE_CASE code")
            }

            fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                AppCode::from_str(value).map_err(E::custom)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                AppCode::from_str(value).map_err(E::custom)
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                AppCode::try_new(value).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

fn validate_code(value: &str) -> Result<(), ParseAppCodeError> {
    if !is_valid_literal(value) {
        return Err(ParseAppCodeError);
    }

    Ok(())
}

fn match_static(value: &str) -> Option<AppCode> {
    match value {
        "NOT_FOUND" => Some(AppCode::NotFound),
        "VALIDATION" => Some(AppCode::Validation),
        "CONFLICT" => Some(AppCode::Conflict),
        "USER_ALREADY_EXISTS" => Some(AppCode::UserAlreadyExists),
        "UNAUTHORIZED" => Some(AppCode::Unauthorized),
        "FORBIDDEN" => Some(AppCode::Forbidden),
        "NOT_IMPLEMENTED" => Some(AppCode::NotImplemented),
        "BAD_REQUEST" => Some(AppCode::BadRequest),
        "RATE_LIMITED" => Some(AppCode::RateLimited),
        "TELEGRAM_AUTH" => Some(AppCode::TelegramAuth),
        "INVALID_JWT" => Some(AppCode::InvalidJwt),
        "INTERNAL" => Some(AppCode::Internal),
        "DATABASE" => Some(AppCode::Database),
        "SERVICE" => Some(AppCode::Service),
        "CONFIG" => Some(AppCode::Config),
        "TURNKEY" => Some(AppCode::Turnkey),
        "TIMEOUT" => Some(AppCode::Timeout),
        "NETWORK" => Some(AppCode::Network),
        "DEPENDENCY_UNAVAILABLE" => Some(AppCode::DependencyUnavailable),
        "SERIALIZATION" => Some(AppCode::Serialization),
        "DESERIALIZATION" => Some(AppCode::Deserialization),
        "EXTERNAL_API" => Some(AppCode::ExternalApi),
        "QUEUE" => Some(AppCode::Queue),
        "CACHE" => Some(AppCode::Cache),
        _ => None
    }
}

const fn is_valid_literal(value: &str) -> bool {
    let bytes = value.as_bytes();
    let len = bytes.len();
    if len == 0 {
        return false;
    }

    if bytes[0] == b'_' || bytes[len - 1] == b'_' {
        return false;
    }

    let mut index = 0;
    while index < len {
        let byte = bytes[index];
        if !matches!(byte, b'A'..=b'Z' | b'0'..=b'9' | b'_') {
            return false;
        }
        if byte == b'_' && index + 1 < len && bytes[index + 1] == b'_' {
            return false;
        }
        index += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{AppCode, AppErrorKind, ParseAppCodeError};

    #[test]
    fn as_str_matches_json_serde_names() {
        assert_eq!(AppCode::NotFound.as_str(), "NOT_FOUND");
        assert_eq!(AppCode::RateLimited.as_str(), "RATE_LIMITED");
        assert_eq!(
            AppCode::DependencyUnavailable.as_str(),
            "DEPENDENCY_UNAVAILABLE"
        );
    }

    #[test]
    fn mapping_from_kind_is_stable() {
        // Spot checks to guard against accidental remaps.
        assert_eq!(AppCode::from(AppErrorKind::NotFound), AppCode::NotFound);
        assert_eq!(AppCode::from(AppErrorKind::Validation), AppCode::Validation);
        assert_eq!(AppCode::from(AppErrorKind::Internal), AppCode::Internal);
        assert_eq!(AppCode::from(AppErrorKind::Timeout), AppCode::Timeout);
    }

    #[test]
    fn display_uses_screaming_snake_case() {
        assert_eq!(AppCode::BadRequest.to_string(), "BAD_REQUEST");
    }

    #[test]
    fn new_and_try_new_validate_input() {
        let code = AppCode::new("CUSTOM_CODE");
        assert_eq!(code.as_str(), "CUSTOM_CODE");
        assert!(AppCode::try_new(String::from("ANOTHER_CODE")).is_ok());
        assert!(AppCode::try_new(String::from("lower")).is_err());
    }

    #[test]
    #[should_panic]
    fn new_panics_on_invalid_literal() {
        let _ = AppCode::new("not_snake");
    }

    #[test]
    fn from_str_parses_known_codes() {
        for code in [
            AppCode::NotFound,
            AppCode::Validation,
            AppCode::Unauthorized,
            AppCode::Internal,
            AppCode::Timeout
        ] {
            let parsed = AppCode::from_str(code.as_str()).expect("parse");
            assert_eq!(parsed, code);
        }
    }

    #[test]
    fn from_str_allows_dynamic_codes() {
        let parsed = AppCode::from_str("THIRD_PARTY_FAILURE").expect("parse");
        assert_eq!(parsed.as_str(), "THIRD_PARTY_FAILURE");
    }

    #[test]
    fn from_str_rejects_unknown_code_shape() {
        let err = AppCode::from_str("NOT-A-REAL-CODE").unwrap_err();
        assert_eq!(err, ParseAppCodeError);
    }
}
