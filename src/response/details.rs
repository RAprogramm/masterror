// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

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

#[cfg(test)]
mod tests {
    use crate::{AppCode, ErrorResponse};

    #[cfg(not(feature = "serde_json"))]
    #[test]
    fn with_details_text_attaches_string() {
        let resp = ErrorResponse::new(400, AppCode::BadRequest, "error")
            .unwrap()
            .with_details_text("detailed explanation");
        assert_eq!(resp.details.as_deref(), Some("detailed explanation"));
    }

    #[cfg(not(feature = "serde_json"))]
    #[test]
    fn with_details_text_accepts_owned_string() {
        let resp = ErrorResponse::new(422, AppCode::Validation, "invalid")
            .unwrap()
            .with_details_text(String::from("field error"));
        assert_eq!(resp.details.as_deref(), Some("field error"));
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn with_details_json_attaches_value() {
        use serde_json::json;
        let details = json!({"field": "email", "error": "invalid"});
        let resp = ErrorResponse::new(422, AppCode::Validation, "bad input")
            .unwrap()
            .with_details_json(details.clone());
        assert_eq!(resp.details, Some(details));
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn with_details_serializes_struct() {
        use serde::Serialize;

        #[derive(Serialize)]
        struct ErrorInfo {
            field: String,
            code:  u32
        }

        let info = ErrorInfo {
            field: "username".to_owned(),
            code:  1001
        };
        let resp = ErrorResponse::new(400, AppCode::BadRequest, "validation failed")
            .unwrap()
            .with_details(info)
            .unwrap();

        assert!(resp.details.is_some());
        let details = resp.details.unwrap();
        assert_eq!(details["field"], "username");
        assert_eq!(details["code"], 1001);
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn with_details_serializes_nan_as_null() {
        use serde::Serialize;

        // f64::NAN serializes to JSON null
        #[derive(Serialize)]
        struct DataWithNaN {
            value: f64
        }

        let data = DataWithNaN {
            value: f64::NAN
        };
        let resp = ErrorResponse::new(500, AppCode::Internal, "error")
            .unwrap()
            .with_details(data)
            .unwrap();

        // NaN becomes null in JSON
        assert!(resp.details.is_some());
        let details = resp.details.unwrap();
        assert!(details["value"].is_null());
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn with_details_preserves_other_fields() {
        use serde::Serialize;

        #[derive(Serialize)]
        struct Extra {
            info: String
        }

        let mut resp = ErrorResponse::new(429, AppCode::RateLimited, "too many").unwrap();
        resp.retry = Some(crate::response::core::RetryAdvice {
            after_seconds: 60
        });

        let resp = resp
            .with_details(Extra {
                info: "try later".to_owned()
            })
            .unwrap();

        assert!(resp.details.is_some());
        assert!(resp.retry.is_some());
        assert_eq!(resp.status, 429);
        assert_eq!(resp.code, AppCode::RateLimited);
    }

    #[cfg(not(feature = "serde_json"))]
    #[test]
    fn with_details_text_builder_pattern() {
        let resp = ErrorResponse::new(404, AppCode::NotFound, "missing")
            .unwrap()
            .with_details_text("resource not found in database");

        assert_eq!(resp.status, 404);
        assert_eq!(resp.message, "missing");
        assert!(resp.details.is_some());
    }
}
