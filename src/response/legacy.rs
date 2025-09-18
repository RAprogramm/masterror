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
        let msg = message.into();
        Self::new(status, AppCode::Internal, msg.clone()).unwrap_or(Self {
            status:           500,
            code:             AppCode::Internal,
            message:          msg,
            details:          None,
            retry:            None,
            www_authenticate: None
        })
    }
}
