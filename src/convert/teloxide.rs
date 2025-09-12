//! Conversion from [`teloxide_core::RequestError`] into [`AppError`].
//!
//! Enabled with the `teloxide` feature flag.
//!
//! ## Mapping
//!
//! - [`RequestError::Api`] or [`RequestError::MigrateToChatId`] →
//!   `AppErrorKind::ExternalApi`
//! - [`RequestError::RetryAfter`] → `AppErrorKind::RateLimited`
//! - [`RequestError::Network`] → `AppErrorKind::Network`
//! - [`RequestError::InvalidJson`] → `AppErrorKind::Deserialization`
//! - [`RequestError::Io`] → `AppErrorKind::Internal`
//!
//! The original error string is preserved in the `message` for observability.
//!
//! ## Example
//!
//! ```rust,ignore
//! use masterror::{AppError, AppErrorKind};
//! use teloxide_core::{errors::ApiError, RequestError, types::Seconds};
//! use std::{io, sync::Arc};
//!
//! fn map(err: RequestError) -> AppError { err.into() }
//!
//! let err = RequestError::RetryAfter(Seconds::from_seconds(1));
//! let app_err = map(err);
//! assert!(matches!(app_err.kind, AppErrorKind::RateLimited));
//! ```
#[cfg(feature = "teloxide")]
use teloxide_core::RequestError;

#[cfg(feature = "teloxide")]
use crate::AppError;

#[cfg(feature = "teloxide")]
#[cfg_attr(docsrs, doc(cfg(feature = "teloxide")))]
impl From<RequestError> for AppError {
    fn from(err: RequestError) -> Self {
        match err {
            RequestError::Api(api) => AppError::external_api(format!("Telegram API error: {api}")),
            RequestError::MigrateToChatId(id) => {
                AppError::external_api(format!("Group migrated to {id}"))
            }
            RequestError::RetryAfter(secs) => {
                AppError::rate_limited(format!("Retry after {secs}"))
            }
            RequestError::Network(e) => AppError::network(format!("Network error: {e}")),
            RequestError::InvalidJson {
                source, ..
            } => AppError::deserialization(format!("Invalid Telegram JSON: {source}")),
            RequestError::Io(e) => AppError::internal(format!("I/O error: {e}"))
        }
    }
}

#[cfg(all(test, feature = "teloxide"))]
mod tests {
    use std::{io, sync::Arc};

    use teloxide_core::{errors::ApiError, types::Seconds};

    use super::*;
    use crate::AppErrorKind;

    #[test]
    fn api_maps_to_external_api() {
        let err = RequestError::Api(ApiError::BotBlocked);
        let app_err: AppError = err.into();
        assert!(matches!(app_err.kind, AppErrorKind::ExternalApi));
    }

    #[test]
    fn retry_after_maps_to_rate_limited() {
        let err = RequestError::RetryAfter(Seconds::from_seconds(5));
        let app_err: AppError = err.into();
        assert!(matches!(app_err.kind, AppErrorKind::RateLimited));
    }

    #[test]
    fn io_maps_to_internal() {
        let io_err = Arc::new(io::Error::other("disk"));
        let err = RequestError::Io(io_err);
        let app_err: AppError = err.into();
        assert!(matches!(app_err.kind, AppErrorKind::Internal));
    }
}
