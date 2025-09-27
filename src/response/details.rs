#[cfg(not(feature = "serde_json"))]
use alloc::string::String;

#[cfg(feature = "serde_json")]
use serde::Serialize;
#[cfg(feature = "serde_json")]
use serde_json::{Value as JsonValue, to_value};

use super::core::ErrorResponse;
#[cfg(feature = "serde_json")]
use crate::{AppError, AppResult};

#[cfg(not(feature = "serde_json"))]
impl ErrorResponse {
    /// Attach plain-text details (available when `serde_json` is disabled).
    #[must_use]
    pub fn with_details_text(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

#[cfg(feature = "serde_json")]
impl ErrorResponse {
    /// Attach structured JSON details (available when `serde_json` is enabled).
    #[must_use]
    pub fn with_details_json(mut self, details: JsonValue) -> Self {
        self.details = Some(details);
        self
    }

    /// Serialize and attach structured details from any [`Serialize`] value.
    ///
    /// # Errors
    ///
    /// Returns [`AppError`] if serialization fails.
    ///
    /// # Examples
    /// ```
    /// # #[cfg(feature = "serde_json")]
    /// # {
    /// use masterror::{AppCode, ErrorResponse};
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Extra {
    ///     reason: String
    /// }
    ///
    /// let payload = Extra {
    ///     reason: "missing".into()
    /// };
    /// let resp = ErrorResponse::new(404, AppCode::NotFound, "no user")
    ///     .expect("status")
    ///     .with_details(payload)
    ///     .expect("details");
    /// assert!(resp.details.is_some());
    /// # }
    /// ```
    #[allow(clippy::result_large_err)]
    pub fn with_details<T>(self, payload: T) -> AppResult<Self>
    where
        T: Serialize
    {
        let details = to_value(payload).map_err(|e| AppError::bad_request(e.to_string()))?;
        Ok(self.with_details_json(details))
    }
}
#[cfg(feature = "serde_json")]
use alloc::string::ToString;
