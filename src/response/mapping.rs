use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result as FmtResult}
};

use super::core::ErrorResponse;
use crate::{AppCode, AppError};

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Concise string form, safe for logs and debugging.
        write!(f, "{} {:?}: {}", self.status, self.code, self.message)
    }
}

impl From<AppError> for ErrorResponse {
    fn from(err: AppError) -> Self {
        let AppError {
            kind,
            message,
            retry,
            www_authenticate
        } = err;

        let status = kind.http_status();
        let code = AppCode::from(kind);
        let message = match message {
            Some(msg) => msg.into_owned(),
            None => String::from("An error occurred")
        };

        Self {
            status,
            code,
            message,
            details: None,
            retry,
            www_authenticate
        }
    }
}

impl From<&AppError> for ErrorResponse {
    fn from(err: &AppError) -> Self {
        let status = err.kind.http_status();
        let code = AppCode::from(err.kind);

        let message = err
            .message
            .clone()
            .unwrap_or(Cow::Borrowed("An error occurred"))
            .into_owned();

        Self {
            status,
            code,
            message,
            details: None,
            retry: err.retry,
            www_authenticate: err.www_authenticate.clone()
        }
    }
}
