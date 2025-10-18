// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use alloc::string::String;

use http::StatusCode;

use super::core::ErrorResponse;
use crate::AppCode;

/// Legacy constructor retained for migration purposes.
///
/// Deprecated: prefer [`ErrorResponse::new`] with an [`AppCode`] argument.
///
/// # Examples
///
/// ```
/// # use masterror::ErrorResponse;
/// #[allow(deprecated)]
/// let response = ErrorResponse::new_legacy(404, "Not found");
/// assert_eq!(response.status, 404);
/// assert_eq!(response.message, "Not found");
/// ```
#[deprecated(note = "Use new(status, code, message) instead")]
impl ErrorResponse {
    /// Construct an error response with only `(status, message)`.
    ///
    /// This defaults the code to [`AppCode::Internal`]. Kept temporarily to
    /// ease migration from versions prior to 0.3.0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use masterror::{AppCode, ErrorResponse};
    /// #[allow(deprecated)]
    /// let response = ErrorResponse::new_legacy(500, "Internal error");
    /// assert_eq!(response.status, 500);
    /// assert_eq!(response.code, AppCode::Internal);
    /// assert_eq!(response.message, "Internal error");
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn new_legacy_creates_response_with_valid_status() {
        let resp = ErrorResponse::new_legacy(404, "not found");
        assert_eq!(resp.status, 404);
        assert_eq!(resp.message, "not found");
        assert_eq!(resp.code, AppCode::Internal);
    }

    #[test]
    #[allow(deprecated)]
    fn new_legacy_defaults_code_to_internal() {
        let resp = ErrorResponse::new_legacy(200, "success");
        assert_eq!(resp.code, AppCode::Internal);
    }

    #[test]
    #[allow(deprecated)]
    fn new_legacy_accepts_string_reference() {
        let resp = ErrorResponse::new_legacy(400, "bad request");
        assert_eq!(resp.message, "bad request");
    }

    #[test]
    #[allow(deprecated)]
    fn new_legacy_accepts_owned_string() {
        let msg = String::from("server error");
        let resp = ErrorResponse::new_legacy(500, msg);
        assert_eq!(resp.message, "server error");
    }

    #[test]
    #[allow(deprecated)]
    fn new_legacy_falls_back_to_500_on_invalid_status() {
        let resp = ErrorResponse::new_legacy(1000, "invalid status");
        assert_eq!(resp.status, 500);
        assert_eq!(resp.code, AppCode::Internal);
        assert_eq!(resp.message, "invalid status");
    }

    #[test]
    #[allow(deprecated)]
    fn new_legacy_preserves_message_on_fallback() {
        let resp = ErrorResponse::new_legacy(9999, "fallback test");
        assert_eq!(resp.message, "fallback test");
    }

    #[test]
    #[allow(deprecated)]
    fn new_legacy_sets_optional_fields_to_none() {
        let resp = ErrorResponse::new_legacy(200, "ok");
        assert!(resp.details.is_none());
        assert!(resp.retry.is_none());
        assert!(resp.www_authenticate.is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn new_legacy_works_with_common_status_codes() {
        let test_cases = vec![
            (200, "OK"),
            (201, "Created"),
            (400, "Bad Request"),
            (401, "Unauthorized"),
            (403, "Forbidden"),
            (404, "Not Found"),
            (500, "Internal Server Error"),
            (502, "Bad Gateway"),
            (503, "Service Unavailable"),
        ];

        for (status, message) in test_cases {
            let resp = ErrorResponse::new_legacy(status, message);
            assert_eq!(resp.status, status);
            assert_eq!(resp.message, message);
            assert_eq!(resp.code, AppCode::Internal);
        }
    }

    #[test]
    #[allow(deprecated)]
    fn new_legacy_handles_edge_case_status_codes() {
        // Minimum valid HTTP status
        let resp = ErrorResponse::new_legacy(100, "continue");
        assert_eq!(resp.status, 100);

        // Maximum valid HTTP status
        let resp = ErrorResponse::new_legacy(599, "custom");
        assert_eq!(resp.status, 599);
    }

    #[test]
    #[allow(deprecated)]
    fn new_legacy_handles_empty_message() {
        let resp = ErrorResponse::new_legacy(500, "");
        assert_eq!(resp.message, "");
        assert_eq!(resp.status, 500);
    }

    #[test]
    #[allow(deprecated)]
    fn new_legacy_handles_unicode_message() {
        let resp = ErrorResponse::new_legacy(400, "Ошибка запроса 错误");
        assert_eq!(resp.message, "Ошибка запроса 错误");
    }

    #[test]
    #[allow(deprecated)]
    fn new_legacy_zero_status_falls_back() {
        let resp = ErrorResponse::new_legacy(0, "zero status");
        assert_eq!(resp.status, 500);
        assert_eq!(resp.message, "zero status");
    }
}
