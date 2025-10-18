// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use core::fmt::{self, Debug, Display, Formatter};

use super::{core::ErrorResponse, problem_json::ProblemJson};

/// Formatter exposing response internals for opt-in diagnostics.
#[derive(Clone, Copy)]
pub struct ErrorResponseFormatter<'a> {
    inner: &'a ErrorResponse
}

impl<'a> ErrorResponseFormatter<'a> {
    pub(crate) fn new(inner: &'a ErrorResponse) -> Self {
        Self {
            inner
        }
    }
}

impl Debug for ErrorResponseFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorResponse")
            .field("status", &self.inner.status)
            .field("code", &self.inner.code)
            .field("message", &self.inner.message)
            .field("details", &self.inner.details)
            .field("retry", &self.inner.retry)
            .field("www_authenticate", &self.inner.www_authenticate)
            .finish()
    }
}

impl Display for ErrorResponseFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.inner, f)
    }
}

/// Formatter exposing problem-json internals for opt-in diagnostics.
#[derive(Clone, Copy)]
pub struct ProblemJsonFormatter<'a> {
    inner: &'a ProblemJson
}

impl<'a> ProblemJsonFormatter<'a> {
    pub(crate) fn new(inner: &'a ProblemJson) -> Self {
        Self {
            inner
        }
    }
}

impl Debug for ProblemJsonFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProblemJson")
            .field("type", &self.inner.type_uri)
            .field("title", &self.inner.title)
            .field("status", &self.inner.status)
            .field("detail", &self.inner.detail)
            .field("details", &self.inner.details)
            .field("code", &self.inner.code)
            .field("grpc", &self.inner.grpc)
            .field("metadata", &self.inner.metadata)
            .finish()
    }
}

impl Display for ProblemJsonFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {:?}",
            self.inner.status, self.inner.code, self.inner.detail
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{AppCode, AppError, ErrorResponse, ProblemJson, response::core::RetryAdvice};

    #[test]
    fn error_response_formatter_debug_includes_all_fields() {
        let mut resp = ErrorResponse::new(404, AppCode::NotFound, "resource missing").unwrap();
        resp.retry = Some(RetryAdvice {
            after_seconds: 30
        });
        resp.www_authenticate = Some("Bearer".to_owned());

        let formatted = format!("{:?}", resp.internal());

        assert!(formatted.contains("ErrorResponse"));
        assert!(formatted.contains("status"));
        assert!(formatted.contains("404"));
        assert!(formatted.contains("code"));
        assert!(formatted.contains("NOT_FOUND"));
        assert!(formatted.contains("message"));
        assert!(formatted.contains("resource missing"));
        assert!(formatted.contains("retry"));
        assert!(formatted.contains("30"));
        assert!(formatted.contains("www_authenticate"));
        assert!(formatted.contains("Bearer"));
    }

    #[test]
    fn error_response_formatter_debug_shows_none_for_optional_fields() {
        let resp = ErrorResponse::new(500, AppCode::Internal, "error").unwrap();
        let formatted = format!("{:?}", resp.internal());

        assert!(formatted.contains("details: None"));
        assert!(formatted.contains("retry: None"));
        assert!(formatted.contains("www_authenticate: None"));
    }

    #[test]
    fn error_response_formatter_display_delegates_to_inner() {
        let resp = ErrorResponse::new(400, AppCode::BadRequest, "invalid input").unwrap();
        let formatter = resp.internal();

        let display_str = format!("{}", formatter);
        let inner_display_str = format!("{}", resp);

        assert_eq!(display_str, inner_display_str);
    }

    #[test]
    fn error_response_formatter_is_copy() {
        let resp = ErrorResponse::new(500, AppCode::Internal, "error").unwrap();
        let formatter1 = resp.internal();
        let formatter2 = formatter1;

        // Both should work
        let _ = format!("{:?}", formatter1);
        let _ = format!("{:?}", formatter2);
    }

    #[test]
    fn problem_json_formatter_debug_includes_all_fields() {
        let error = AppError::not_found("missing resource")
            .with_retry_after_secs(60)
            .with_www_authenticate("Bearer realm=\"api\"");

        let problem = ProblemJson::from_app_error(error);
        let formatted = format!("{:?}", problem.internal());

        assert!(formatted.contains("ProblemJson"));
        assert!(formatted.contains("type"));
        assert!(formatted.contains("title"));
        assert!(formatted.contains("status"));
        assert!(formatted.contains("404"));
        assert!(formatted.contains("detail"));
        assert!(formatted.contains("missing resource"));
        assert!(formatted.contains("code"));
        assert!(formatted.contains("NOT_FOUND"));
    }

    #[test]
    fn problem_json_formatter_display_shows_status_code_detail() {
        let error = AppError::bad_request("validation failed");
        let problem = ProblemJson::from_app_error(error);

        let display_str = format!("{}", problem.internal());

        assert!(display_str.contains("400"));
        assert!(display_str.contains("BAD_REQUEST"));
        assert!(display_str.contains("validation failed"));
    }

    #[test]
    fn problem_json_formatter_display_format_matches_pattern() {
        let error = AppError::internal("server error");
        let problem = ProblemJson::from_app_error(error);

        let display_str = format!("{}", problem.internal());

        // Format: "{status} {code}: {detail:?}"
        assert!(display_str.starts_with("500"));
        assert!(display_str.contains("INTERNAL"));
        assert!(display_str.contains(": "));
        assert!(display_str.contains("server error"));
    }

    #[test]
    fn problem_json_formatter_is_copy() {
        let error = AppError::internal("error");
        let problem = ProblemJson::from_app_error(error);

        let formatter1 = problem.internal();
        let formatter2 = formatter1;

        // Both should work
        let _ = format!("{:?}", formatter1);
        let _ = format!("{:?}", formatter2);
    }

    #[test]
    fn formatters_work_with_various_error_codes() {
        let test_cases = vec![
            (AppError::bad_request("bad"), 400, "BAD_REQUEST"),
            (AppError::unauthorized("auth required"), 401, "UNAUTHORIZED"),
            (AppError::forbidden("access denied"), 403, "FORBIDDEN"),
            (
                AppError::rate_limited("too many requests"),
                429,
                "RATE_LIMITED"
            ),
        ];

        for (error, expected_status, expected_code) in test_cases {
            let problem = ProblemJson::from_app_error(error);
            let display = format!("{}", problem.internal());

            assert!(display.contains(&expected_status.to_string()));
            assert!(display.contains(expected_code));
        }
    }
}
