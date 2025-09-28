use alloc::string::String;

use http::StatusCode;

use super::core::ErrorResponse;
use crate::AppCode;

/// Legacy constructor retained for migration purposes.
///
/// Deprecated: prefer [`ErrorResponse::new`] with an [`AppCode`] argument.
#[deprecated(note = "Use new(status, code, message) instead")]
impl ErrorResponse {
    /// Construct an error response with only `(status, message)`.
    ///
    /// This defaults the code to [`AppCode::Internal`]. Kept temporarily to
    /// ease migration from versions prior to 0.3.0.
    #[must_use]
    pub fn new_legacy(status: u16, message: impl Into<String>) -> Self {
        match StatusCode::from_u16(status) {
            Ok(_) => {
                let message = message.into();
                Self {
                    status,
                    code: AppCode::Internal,
                    message,
                    details: None,
                    retry: None,
                    www_authenticate: None
                }
            }
            Err(_) => {
                let message = message.into();
                Self {
                    status: 500,
                    code: AppCode::Internal,
                    message,
                    details: None,
                    retry: None,
                    www_authenticate: None
                }
            }
        }
    }
}
