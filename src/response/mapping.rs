use alloc::string::ToString;
use core::fmt::{Display, Formatter, Result as FmtResult};

use super::core::ErrorResponse;
use crate::AppError;

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Concise string form, safe for logs and debugging.
        write!(f, "{} {:?}: {}", self.status, self.code, self.message)
    }
}

impl From<AppError> for ErrorResponse {
    fn from(mut err: AppError) -> Self {
        let kind = err.kind;
        let code = err.code.clone();
        let retry = err.retry.take();
        let www_authenticate = err.www_authenticate.take();
        let policy = err.edit_policy;

        let status = kind.http_status();
        let message = match err.message.take() {
            Some(msg) if !matches!(policy, crate::MessageEditPolicy::Redact) => msg.into_owned(),
            _ => kind.to_string()
        };
        #[cfg(feature = "serde_json")]
        let details = if matches!(policy, crate::MessageEditPolicy::Redact) {
            None
        } else {
            err.details.take()
        };
        #[cfg(not(feature = "serde_json"))]
        let details = if matches!(policy, crate::MessageEditPolicy::Redact) {
            None
        } else {
            err.details.take()
        };

        Self {
            status,
            code,
            message,
            details,
            retry,
            www_authenticate
        }
    }
}

impl From<&AppError> for ErrorResponse {
    fn from(err: &AppError) -> Self {
        let status = err.kind.http_status();
        let message = if matches!(err.edit_policy, crate::MessageEditPolicy::Redact) {
            err.kind.to_string()
        } else {
            err.render_message().into_owned()
        };
        #[cfg(feature = "serde_json")]
        let details = if matches!(err.edit_policy, crate::MessageEditPolicy::Redact) {
            None
        } else {
            err.details.clone()
        };
        #[cfg(not(feature = "serde_json"))]
        let details = if matches!(err.edit_policy, crate::MessageEditPolicy::Redact) {
            None
        } else {
            err.details.clone()
        };

        Self {
            status,
            code: err.code.clone(),
            message,
            details,
            retry: err.retry,
            www_authenticate: err.www_authenticate.clone()
        }
    }
}
