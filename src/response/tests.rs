// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::borrow::Cow;

use super::ErrorResponse;
use crate::{AppCode, AppError, AppErrorKind, ProblemJson};

// --- Basic constructors and fields --------------------------------------

#[test]
fn new_sets_status_code_and_message() {
    let e = ErrorResponse::new(404, AppCode::NotFound, "missing").expect("status");
    assert_eq!(e.status, 404);
    assert_eq!(e.code, AppCode::NotFound);
    assert_eq!(e.message, "missing");
    assert!(e.retry.is_none());
    assert!(e.www_authenticate.is_none());
}

#[test]
fn new_rejects_invalid_status() {
    let err = ErrorResponse::new(0, AppCode::Internal, "boom").expect_err("invalid");
    assert!(matches!(err.kind, AppErrorKind::BadRequest));
}

#[test]
fn with_retry_and_www_authenticate_attach_metadata() {
    let e = ErrorResponse::new(401, AppCode::Unauthorized, "auth required")
        .expect("status")
        .with_retry_after_secs(15)
        .with_www_authenticate(r#"Bearer realm="api""#);
    assert_eq!(e.status, 401);
    assert_eq!(e.retry.unwrap().after_seconds, 15);
    assert_eq!(e.www_authenticate.as_deref(), Some(r#"Bearer realm="api""#));
}

#[test]
fn with_retry_after_duration_attaches_advice() {
    use std::time::Duration;
    let e = ErrorResponse::new(429, AppCode::RateLimited, "slow down")
        .expect("status")
        .with_retry_after_duration(Duration::from_secs(42));
    assert_eq!(e.retry.unwrap().after_seconds, 42);
}

#[test]
fn with_retry_after_secs_zero() {
    let e = ErrorResponse::new(503, AppCode::Internal, "unavailable")
        .expect("status")
        .with_retry_after_secs(0);
    assert_eq!(e.retry.unwrap().after_seconds, 0);
}

#[test]
fn with_retry_after_secs_large_value() {
    let e = ErrorResponse::new(503, AppCode::Internal, "unavailable")
        .expect("status")
        .with_retry_after_secs(u64::MAX);
    assert_eq!(e.retry.unwrap().after_seconds, u64::MAX);
}

#[test]
fn with_retry_after_duration_zero() {
    use std::time::Duration;
    let e = ErrorResponse::new(503, AppCode::Internal, "unavailable")
        .expect("status")
        .with_retry_after_duration(Duration::from_secs(0));
    assert_eq!(e.retry.unwrap().after_seconds, 0);
}

#[test]
fn with_retry_after_duration_subsecond_rounds_down() {
    use std::time::Duration;
    let e = ErrorResponse::new(503, AppCode::Internal, "unavailable")
        .expect("status")
        .with_retry_after_duration(Duration::from_millis(999));
    assert_eq!(e.retry.unwrap().after_seconds, 0);
}

#[test]
fn with_www_authenticate_accepts_string() {
    let e = ErrorResponse::new(401, AppCode::Unauthorized, "auth required")
        .expect("status")
        .with_www_authenticate("Basic realm=\"test\"".to_string());
    assert_eq!(e.www_authenticate.as_deref(), Some("Basic realm=\"test\""));
}

#[test]
fn with_www_authenticate_accepts_str() {
    let e = ErrorResponse::new(401, AppCode::Unauthorized, "auth required")
        .expect("status")
        .with_www_authenticate("Bearer");
    assert_eq!(e.www_authenticate.as_deref(), Some("Bearer"));
}

#[test]
fn with_www_authenticate_empty_string() {
    let e = ErrorResponse::new(401, AppCode::Unauthorized, "auth required")
        .expect("status")
        .with_www_authenticate("");
    assert_eq!(e.www_authenticate.as_deref(), Some(""));
}

#[test]
fn with_www_authenticate_unicode() {
    let e = ErrorResponse::new(401, AppCode::Unauthorized, "auth required")
        .expect("status")
        .with_www_authenticate("Bearer realm=\"認証\"");
    assert_eq!(e.www_authenticate.as_deref(), Some("Bearer realm=\"認証\""));
}

#[test]
fn with_www_authenticate_special_characters() {
    let challenge = r#"Bearer realm="api", error="invalid_token", error_description="<>&\""#;
    let e = ErrorResponse::new(401, AppCode::Unauthorized, "auth required")
        .expect("status")
        .with_www_authenticate(challenge);
    assert_eq!(e.www_authenticate.as_deref(), Some(challenge));
}

#[test]
fn metadata_methods_are_chainable() {
    use std::time::Duration;
    let e = ErrorResponse::new(503, AppCode::Internal, "unavailable")
        .expect("status")
        .with_retry_after_duration(Duration::from_secs(30))
        .with_www_authenticate("Bearer")
        .with_retry_after_secs(60);
    assert_eq!(e.retry.unwrap().after_seconds, 60);
    assert_eq!(e.www_authenticate.as_deref(), Some("Bearer"));
}

#[test]
fn with_retry_after_secs_overwrites_previous() {
    let e = ErrorResponse::new(503, AppCode::Internal, "unavailable")
        .expect("status")
        .with_retry_after_secs(10)
        .with_retry_after_secs(20);
    assert_eq!(e.retry.unwrap().after_seconds, 20);
}

#[test]
fn with_www_authenticate_overwrites_previous() {
    let e = ErrorResponse::new(401, AppCode::Unauthorized, "auth required")
        .expect("status")
        .with_www_authenticate("Basic")
        .with_www_authenticate("Bearer");
    assert_eq!(e.www_authenticate.as_deref(), Some("Bearer"));
}

#[test]
fn status_code_maps_invalid_to_internal_server_error() {
    use http::StatusCode;
    let valid = ErrorResponse::new(404, AppCode::NotFound, "missing").expect("status");
    assert_eq!(valid.status_code(), StatusCode::NOT_FOUND);
    let invalid = ErrorResponse {
        status:           1000,
        code:             AppCode::Internal,
        message:          "oops".into(),
        details:          None,
        retry:            None,
        www_authenticate: None
    };
    assert_eq!(invalid.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
}

// --- Details: JSON vs text ----------------------------------------------

#[cfg(feature = "serde_json")]
#[test]
fn details_json_are_attached() {
    let payload = serde_json::json!({"field": "email", "error": "invalid"});
    let e = ErrorResponse::new(422, AppCode::Validation, "invalid")
        .expect("status")
        .with_details_json(payload.clone());
    assert_eq!(e.status, 422);
    assert!(e.details.is_some());
    assert_eq!(e.details.unwrap(), payload);
}

#[cfg(feature = "serde_json")]
#[test]
fn custom_codes_roundtrip_via_json() {
    let custom = AppCode::new("INVALID_JSON");
    let response = ErrorResponse::new(400, custom.clone(), "invalid body").expect("status");
    let json = serde_json::to_string(&response).expect("serialize");
    let decoded: ErrorResponse = serde_json::from_str(&json).expect("decode");
    assert_eq!(decoded.code, custom);
    assert_eq!(decoded.code.as_str(), "INVALID_JSON");
}

#[cfg(feature = "serde_json")]
#[test]
fn with_details_serializes_custom_struct() {
    use serde::Serialize;
    use serde_json::json;
    #[derive(Serialize)]
    struct Extra {
        value: i32
    }
    let resp = ErrorResponse::new(400, AppCode::BadRequest, "bad")
        .expect("status")
        .with_details(Extra {
            value: 7
        })
        .expect("details");
    assert_eq!(resp.details.unwrap(), json!({"value": 7}));
}

#[cfg(feature = "serde_json")]
#[test]
fn with_details_propagates_serialization_errors() {
    use serde::{Serialize, Serializer};
    struct Failing;
    impl Serialize for Failing {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
        {
            Err(serde::ser::Error::custom("nope"))
        }
    }
    let err = ErrorResponse::new(400, AppCode::BadRequest, "bad")
        .expect("status")
        .with_details(Failing)
        .expect_err("serialization should fail");
    assert!(matches!(err.kind, AppErrorKind::BadRequest));
}

#[cfg(not(feature = "serde_json"))]
#[test]
fn details_text_are_attached() {
    let e = ErrorResponse::new(503, AppCode::DependencyUnavailable, "down")
        .expect("status")
        .with_details_text("retry later");
    assert_eq!(e.status, 503);
    assert_eq!(e.details.as_deref(), Some("retry later"));
}

#[cfg(feature = "serde_json")]
#[test]
fn app_error_mappings_propagate_json_details() {
    use serde_json::json;
    let payload = json!({"hint": "enable"});
    let resp: ErrorResponse = AppError::validation("invalid")
        .with_details_json(payload.clone())
        .into();
    assert_eq!(resp.details, Some(payload.clone()));
    let borrowed = AppError::validation("invalid").with_details_json(payload.clone());
    let resp_ref: ErrorResponse = (&borrowed).into();
    assert_eq!(resp_ref.details, Some(payload.clone()));
    let problem_owned = ProblemJson::from_app_error(
        AppError::validation("invalid").with_details_json(payload.clone())
    );
    assert_eq!(problem_owned.details, Some(payload.clone()));
    let problem_ref = ProblemJson::from_ref(&borrowed);
    assert_eq!(problem_ref.details, Some(payload));
}

#[cfg(feature = "serde_json")]
#[test]
fn redacted_app_error_strips_json_details() {
    use serde_json::json;
    let resp: ErrorResponse = AppError::internal("boom")
        .with_details_json(json!({"private": true}))
        .redactable()
        .into();
    assert!(resp.details.is_none());
    let borrowed = AppError::internal("boom")
        .with_details_json(json!({"private": true}))
        .redactable();
    let resp_ref: ErrorResponse = (&borrowed).into();
    assert!(resp_ref.details.is_none());
    let problem = ProblemJson::from_ref(&borrowed);
    assert!(problem.details.is_none());
    let owned_problem = ProblemJson::from_app_error(
        AppError::internal("boom")
            .with_details_json(json!({"private": true}))
            .redactable()
    );
    assert!(owned_problem.details.is_none());
}

#[cfg(not(feature = "serde_json"))]
#[test]
fn app_error_mappings_propagate_text_details() {
    let resp: ErrorResponse = AppError::validation("invalid")
        .with_details_text("enable feature")
        .into();
    assert_eq!(resp.details.as_deref(), Some("enable feature"));
    let borrowed = AppError::validation("invalid").with_details_text("enable feature");
    let resp_ref: ErrorResponse = (&borrowed).into();
    assert_eq!(resp_ref.details.as_deref(), Some("enable feature"));
    let problem_owned = ProblemJson::from_app_error(
        AppError::validation("invalid").with_details_text("enable feature")
    );
    assert_eq!(problem_owned.details.as_deref(), Some("enable feature"));
    let problem_ref = ProblemJson::from_ref(&borrowed);
    assert_eq!(problem_ref.details.as_deref(), Some("enable feature"));
}

#[cfg(not(feature = "serde_json"))]
#[test]
fn redacted_app_error_strips_text_details() {
    let resp: ErrorResponse = AppError::internal("boom")
        .with_details_text("private")
        .redactable()
        .into();
    assert!(resp.details.is_none());
    let borrowed = AppError::internal("boom")
        .with_details_text("private")
        .redactable();
    let resp_ref: ErrorResponse = (&borrowed).into();
    assert!(resp_ref.details.is_none());
    let problem = ProblemJson::from_ref(&borrowed);
    assert!(problem.details.is_none());
    let owned_problem = ProblemJson::from_app_error(
        AppError::internal("boom")
            .with_details_text("private")
            .redactable()
    );
    assert!(owned_problem.details.is_none());
}

// --- From<&AppError> mapping --------------------------------------------

#[test]
fn from_app_error_preserves_status_and_sets_code() {
    let app = AppError::new(AppErrorKind::NotFound, "user");
    let e: ErrorResponse = (&app).into();
    assert_eq!(e.status, 404);
    assert_eq!(e.code, AppCode::NotFound);
    assert_eq!(e.message, "user");
    assert!(e.retry.is_none());
}

#[test]
fn from_app_error_uses_default_message_when_none() {
    let app = AppError::bare(AppErrorKind::Internal);
    let e: ErrorResponse = (&app).into();
    assert_eq!(e.status, 500);
    assert_eq!(e.code, AppCode::Internal);
    assert_eq!(e.message, AppErrorKind::Internal.label());
}

#[test]
fn from_owned_app_error_moves_message_and_metadata() {
    let err = AppError::unauthorized(String::from("owned message"))
        .with_retry_after_secs(5)
        .with_www_authenticate("Bearer");
    let resp: ErrorResponse = err.into();
    assert_eq!(resp.status, 401);
    assert_eq!(resp.code, AppCode::Unauthorized);
    assert_eq!(resp.message, "owned message");
    assert_eq!(resp.retry.unwrap().after_seconds, 5);
    assert_eq!(resp.www_authenticate.as_deref(), Some("Bearer"));
}

#[test]
fn from_owned_app_error_defaults_message_when_absent() {
    let resp: ErrorResponse = AppError::bare(AppErrorKind::Internal).into();
    assert_eq!(resp.status, 500);
    assert_eq!(resp.code, AppCode::Internal);
    assert_eq!(resp.message, AppErrorKind::Internal.label());
}

#[test]
fn from_app_error_bare_uses_kind_display_as_message() {
    let app = AppError::bare(AppErrorKind::Timeout);
    let resp: ErrorResponse = app.into();
    assert_eq!(resp.status, 504);
    assert_eq!(resp.code, AppCode::Timeout);
    assert_eq!(resp.message, AppErrorKind::Timeout.label());
}

#[test]
fn problem_json_fallbacks_borrow_bare_labels() {
    let owned = ProblemJson::from_app_error(AppError::bare(AppErrorKind::Internal));
    assert!(matches!(
        owned.title,
        Cow::Borrowed(label) if label == AppErrorKind::Internal.label()
    ));
    assert!(matches!(
        owned.detail,
        Some(Cow::Borrowed(label)) if label == AppErrorKind::Internal.label()
    ));
    let borrowed_error = AppError::bare(AppErrorKind::Timeout);
    let borrowed_problem = ProblemJson::from_ref(&borrowed_error);
    assert!(matches!(
        borrowed_problem.title,
        Cow::Borrowed(label) if label == AppErrorKind::Timeout.label()
    ));
    assert!(matches!(
        borrowed_problem.detail,
        Some(Cow::Borrowed(label)) if label == AppErrorKind::Timeout.label()
    ));
}

#[test]
fn from_app_error_redacts_message_when_policy_allows() {
    let app = AppError::internal("sensitive").redactable();
    let resp: ErrorResponse = app.into();
    assert_eq!(resp.message, AppErrorKind::Internal.label());
    let borrowed = AppError::internal("private").redactable();
    let resp_ref: ErrorResponse = (&borrowed).into();
    assert_eq!(resp_ref.message, AppErrorKind::Internal.label());
}

#[test]
fn error_response_serialization_hides_redacted_message() {
    let secret = "super-secret";
    let resp: ErrorResponse = AppError::internal(secret).redactable().into();
    let json = serde_json::to_value(&resp).expect("serialize response");
    let fallback = AppErrorKind::Internal.label();
    assert_eq!(
        json.get("message").and_then(|value| value.as_str()),
        Some(fallback)
    );
    assert!(!json.to_string().contains(secret));
}

// --- Display formatting --------------------------------------------------

#[test]
fn display_is_concise_and_does_not_leak_details() {
    let e = ErrorResponse::new(400, AppCode::BadRequest, "bad").expect("status");
    let s = format!("{}", e);
    assert!(s.contains("400"), "status should be present");
    assert!(
        s.to_lowercase().contains("badrequest")
            || s.contains("BAD_REQUEST")
            || s.contains("BadRequest"),
        "code should be present in some form"
    );
    assert!(s.contains("bad"), "message should be present");
}

// --- Legacy constructor (migration shim) --------------------------------

#[allow(deprecated)]
#[test]
fn new_legacy_defaults_to_internal_code() {
    let e = ErrorResponse::new_legacy(404, "boom");
    assert_eq!(e.status, 404);
    assert_eq!(e.code, AppCode::Internal);
    assert_eq!(e.message, "boom");
}

#[allow(deprecated)]
#[test]
fn new_legacy_invalid_status_falls_back_to_internal_error() {
    let e = ErrorResponse::new_legacy(0, "boom");
    assert_eq!(e.status, 500);
    assert_eq!(e.code, AppCode::Internal);
    assert_eq!(e.message, "boom");
}

// --- Axum adapter: headers and status -----------------------------------

#[cfg(feature = "axum")]
#[test]
fn axum_into_response_sets_headers_and_status() {
    use axum::{
        http::header::{RETRY_AFTER, WWW_AUTHENTICATE},
        response::IntoResponse
    };
    let resp = ErrorResponse::new(401, AppCode::Unauthorized, "no token")
        .expect("status")
        .with_retry_after_secs(7)
        .with_www_authenticate(r#"Bearer realm="api", error="invalid_token""#)
        .into_response();
    assert_eq!(resp.status(), 401);
    let headers = resp.headers();
    let retry_after = headers.get(RETRY_AFTER).expect("Retry-After");
    assert_eq!(retry_after.to_str().expect("ASCII value"), "7");
    let www_authenticate = headers
        .get(WWW_AUTHENTICATE)
        .expect("WWW-Authenticate header");
    assert_eq!(
        www_authenticate.to_str().expect("ASCII challenge"),
        r#"Bearer realm="api", error="invalid_token""#
    );
}

// --- Actix adapter: headers and status ----------------------------------

#[cfg(feature = "actix")]
#[test]
fn actix_responder_sets_headers_and_status() {
    use actix_web::{
        Responder,
        http::{
            StatusCode,
            header::{RETRY_AFTER, WWW_AUTHENTICATE}
        },
        test::TestRequest
    };
    let resp = ErrorResponse::new(429, AppCode::RateLimited, "slow down")
        .expect("status")
        .with_retry_after_secs(42)
        .with_www_authenticate("Bearer");
    let req = TestRequest::default().to_http_request();
    let http = resp.respond_to(&req);
    assert_eq!(http.status(), StatusCode::TOO_MANY_REQUESTS);
    let headers = http.headers();
    let retry_after = headers.get(RETRY_AFTER).expect("Retry-After");
    assert_eq!(retry_after.to_str().expect("ASCII value"), "42");
    let www_authenticate = headers
        .get(WWW_AUTHENTICATE)
        .expect("WWW-Authenticate header");
    assert_eq!(
        www_authenticate.to_str().expect("ASCII challenge"),
        "Bearer"
    );
}

#[cfg(feature = "actix")]
#[test]
fn actix_responder_no_optional_headers_by_default() {
    use actix_web::{
        Responder,
        http::header::{RETRY_AFTER, WWW_AUTHENTICATE},
        test::TestRequest
    };
    let resp = ErrorResponse::new(500, AppCode::Internal, "boom").expect("status");
    let req = TestRequest::default().to_http_request();
    let http = resp.respond_to(&req);
    let headers = http.headers();
    assert!(headers.get(RETRY_AFTER).is_none());
    assert!(headers.get(WWW_AUTHENTICATE).is_none());
}

// --- Serde snapshot-ish check -------------------------------------------

#[cfg(feature = "serde_json")]
#[test]
fn serialized_json_contains_core_fields() {
    let e = ErrorResponse::new(404, AppCode::NotFound, "nope")
        .expect("status")
        .with_retry_after_secs(1);
    let s = serde_json::to_string(&e).expect("serialize");
    assert!(s.contains("\"status\":404"));
    assert!(s.contains("\"code\":\"NOT_FOUND\""));
    assert!(s.contains("\"message\":\"nope\""));
    assert!(s.contains("\"retry\""));
    assert!(s.contains("\"after_seconds\":1"));
}

#[test]
fn internal_formatters_are_opt_in() {
    let resp = ErrorResponse::new(404, AppCode::NotFound, "missing").expect("status");
    let formatted = format!("{:?}", resp.internal());
    assert!(formatted.contains("ErrorResponse"));
    let problem = ProblemJson::from_ref(&AppError::not_found("missing"));
    let formatted_problem = format!("{:?}", problem.internal());
    assert!(formatted_problem.contains("ProblemJson"));
}

#[cfg(feature = "axum")]
#[test]
fn app_error_into_response_maps_status() {
    use axum::response::IntoResponse;
    let app = AppError::new(AppErrorKind::Unauthorized, "no token");
    let resp = app.into_response();
    assert_eq!(resp.status(), 401);
}

// --- Mapping: Display trait ---------------------------------------------

#[test]
fn display_formats_status_code_and_message() {
    let resp = ErrorResponse::new(404, AppCode::NotFound, "user not found").expect("status");
    let display = resp.to_string();
    assert!(display.contains("404"));
    assert!(display.contains("NOT_FOUND"));
    assert!(display.contains("user not found"));
}

#[test]
fn display_works_with_empty_message() {
    let resp = ErrorResponse::new(500, AppCode::Internal, "").expect("status");
    let display = resp.to_string();
    assert!(display.contains("500"));
    assert!(display.contains("INTERNAL"));
}

#[test]
fn display_works_with_unicode_message() {
    let resp =
        ErrorResponse::new(404, AppCode::NotFound, "ユーザーが見つかりません").expect("status");
    let display = resp.to_string();
    assert!(display.contains("404"));
    assert!(display.contains("ユーザーが見つかりません"));
}

#[test]
fn display_works_with_long_message() {
    let long_msg = "x".repeat(1000);
    let resp = ErrorResponse::new(400, AppCode::BadRequest, &long_msg).expect("status");
    let display = resp.to_string();
    assert!(display.contains(&long_msg));
}

#[test]
fn display_works_with_special_characters() {
    let resp =
        ErrorResponse::new(400, AppCode::BadRequest, "Error: \"test\" <>&").expect("status");
    let display = resp.to_string();
    assert!(display.contains("Error: \"test\" <>&"));
}

#[test]
fn display_uses_custom_code() {
    let custom = AppCode::new("CUSTOM_ERROR");
    let resp = ErrorResponse::new(418, custom, "teapot").expect("status");
    let display = resp.to_string();
    assert!(display.contains("418"));
    assert!(display.contains("CUSTOM_ERROR"));
    assert!(display.contains("teapot"));
}

// --- Mapping: From<AppError> --------------------------------------------

#[test]
fn from_owned_app_error_with_custom_code() {
    let custom = AppCode::new("PAYMENT_FAILED");
    let err = AppError::bad_request("transaction declined").with_code(custom.clone());
    let resp: ErrorResponse = err.into();
    assert_eq!(resp.status, 400);
    assert_eq!(resp.code, custom);
    assert_eq!(resp.message, "transaction declined");
}

#[test]
fn from_owned_app_error_with_empty_message() {
    let err = AppError::internal("");
    let resp: ErrorResponse = err.into();
    assert_eq!(resp.status, 500);
    assert_eq!(resp.message, "");
}

#[test]
fn from_owned_app_error_with_unicode_message() {
    let err = AppError::not_found("Ошибка поиска");
    let resp: ErrorResponse = err.into();
    assert_eq!(resp.status, 404);
    assert_eq!(resp.message, "Ошибка поиска");
}

#[test]
fn from_owned_app_error_with_special_characters() {
    let err = AppError::validation("Error: \"invalid\" <>&");
    let resp: ErrorResponse = err.into();
    assert_eq!(resp.message, "Error: \"invalid\" <>&");
}

#[test]
fn from_owned_app_error_transfers_code_ownership() {
    let custom = AppCode::new("DUPLICATE_KEY");
    let err = AppError::conflict("already exists").with_code(custom.clone());
    let resp: ErrorResponse = err.into();
    assert_eq!(resp.code, custom);
    assert_eq!(resp.code.as_str(), "DUPLICATE_KEY");
}

// --- Mapping: From<&AppError> -------------------------------------------

#[test]
fn from_borrowed_app_error_preserves_original() {
    let err = AppError::forbidden("access denied");
    let resp: ErrorResponse = (&err).into();
    assert_eq!(resp.status, 403);
    assert_eq!(resp.message, "access denied");
    assert_eq!(err.message.as_deref(), Some("access denied"));
    assert_eq!(err.kind, AppErrorKind::Forbidden);
}

#[test]
fn from_borrowed_app_error_with_metadata() {
    let err = AppError::rate_limited("slow down")
        .with_retry_after_secs(120)
        .with_www_authenticate("Bearer realm=\"api\"");
    let resp: ErrorResponse = (&err).into();
    assert_eq!(resp.status, 429);
    assert_eq!(resp.message, "slow down");
    assert_eq!(resp.retry.unwrap().after_seconds, 120);
    assert_eq!(
        resp.www_authenticate.as_deref(),
        Some("Bearer realm=\"api\"")
    );
    assert_eq!(err.retry.unwrap().after_seconds, 120);
    assert_eq!(
        err.www_authenticate.as_deref(),
        Some("Bearer realm=\"api\"")
    );
}

#[test]
fn from_borrowed_app_error_clones_custom_code() {
    let custom = AppCode::new("SESSION_EXPIRED");
    let err = AppError::unauthorized("login again").with_code(custom.clone());
    let resp: ErrorResponse = (&err).into();
    assert_eq!(resp.code, custom);
    assert_eq!(err.code, custom);
}

#[test]
fn from_borrowed_app_error_with_empty_message() {
    let err = AppError::timeout("");
    let resp: ErrorResponse = (&err).into();
    assert_eq!(resp.status, 504);
    assert_eq!(resp.message, "");
}

#[test]
fn from_borrowed_app_error_with_unicode() {
    let err = AppError::validation("無効な入力");
    let resp: ErrorResponse = (&err).into();
    assert_eq!(resp.message, "無効な入力");
    assert_eq!(err.message.as_deref(), Some("無効な入力"));
}

#[test]
fn from_borrowed_app_error_redacts_message() {
    let err = AppError::internal("database password: secret123").redactable();
    let resp: ErrorResponse = (&err).into();
    assert_eq!(resp.message, AppErrorKind::Internal.label());
    assert!(!resp.message.contains("secret123"));
    assert_eq!(err.message.as_deref(), Some("database password: secret123"));
}

// --- ProblemJson tests -------------------------------------------------------

#[test]
fn problem_json_from_error_response_empty_message() {
    let resp = ErrorResponse::new(500, AppCode::Internal, "").expect("status");
    let problem = ProblemJson::from_error_response(resp);
    assert!(problem.detail.is_none());
}

#[test]
fn problem_json_from_error_response_with_message() {
    let resp = ErrorResponse::new(404, AppCode::NotFound, "user not found").expect("status");
    let problem = ProblemJson::from_error_response(resp);
    assert_eq!(problem.detail.as_deref(), Some("user not found"));
}

#[test]
fn problem_json_status_code_valid() {
    let problem = ProblemJson::from_app_error(AppError::not_found("missing"));
    assert_eq!(problem.status_code(), http::StatusCode::NOT_FOUND);
}

#[test]
fn problem_json_grpc_code() {
    let problem = ProblemJson::from_app_error(AppError::not_found("missing"));
    assert!(problem.grpc.is_some());
    let grpc = problem.grpc.unwrap();
    assert_eq!(grpc.name, "NOT_FOUND");
    assert_eq!(grpc.value, 5);
}

#[test]
fn problem_json_type_uri() {
    let problem = ProblemJson::from_app_error(AppError::not_found("missing"));
    assert!(problem.type_uri.is_some());
    assert!(problem.type_uri.unwrap().contains("not-found"));
}

#[test]
fn problem_json_with_metadata() {
    use crate::field;
    let err = AppError::service("failed").with_field(field::u64("attempt", 3));
    let problem = ProblemJson::from_app_error(err);
    assert!(problem.metadata.is_some());
}

#[test]
fn problem_json_with_redacted_metadata() {
    use crate::field;
    let err = AppError::internal("error")
        .with_field(field::str("password", "secret"))
        .with_field(field::str("user", "john"));
    let problem = ProblemJson::from_app_error(err);
    assert!(problem.metadata.is_some());
}

#[test]
fn problem_json_redacts_metadata_when_redactable() {
    use crate::field;
    let err = AppError::internal("error")
        .with_field(field::str("data", "value"))
        .redactable();
    let problem = ProblemJson::from_app_error(err);
    assert!(problem.metadata.is_none());
    assert!(problem.detail.is_none());
}

#[test]
fn problem_json_from_ref_with_retry() {
    let err = AppError::rate_limited("slow down").with_retry_after_secs(60);
    let problem = ProblemJson::from_ref(&err);
    assert_eq!(problem.retry_after, Some(60));
}

#[test]
fn problem_json_from_ref_with_www_authenticate() {
    let err = AppError::unauthorized("need auth").with_www_authenticate("Bearer");
    let problem = ProblemJson::from_ref(&err);
    assert_eq!(problem.www_authenticate.as_deref(), Some("Bearer"));
}

#[test]
fn problem_json_metadata_hash_redaction() {
    use crate::field;
    let mut err = AppError::service("test");
    err = err.with_field(field::str("api_token", "secret_token_value"));
    let problem = ProblemJson::from_app_error(err);
    let metadata = problem.metadata.expect("metadata");
    let serialized = serde_json::to_string(&metadata).expect("serialize");
    assert!(!serialized.contains("secret_token_value"));
}

#[test]
fn problem_json_metadata_last4_redaction() {
    use crate::field;
    let err = AppError::service("test").with_field(field::str("card_number", "4111111111111111"));
    let problem = ProblemJson::from_app_error(err);
    let metadata = problem.metadata.expect("metadata");
    let serialized = serde_json::to_string(&metadata).expect("serialize");
    assert!(serialized.contains("1111"));
    assert!(!serialized.contains("4111111111111111"));
}

#[test]
fn problem_json_internal_formatter() {
    let problem = ProblemJson::from_app_error(AppError::not_found("user"));
    let internal = problem.internal();
    let debug = format!("{:?}", internal);
    assert!(debug.contains("ProblemJson"));
}

#[test]
fn problem_metadata_value_from_field_value() {
    use std::{borrow::Cow, net::IpAddr, time::Duration};

    use uuid::Uuid;

    use crate::{FieldValue, ProblemMetadataValue};

    let str_val = ProblemMetadataValue::from(FieldValue::Str(Cow::Borrowed("test")));
    assert!(matches!(str_val, ProblemMetadataValue::String(_)));

    let i64_val = ProblemMetadataValue::from(FieldValue::I64(-42));
    assert!(matches!(i64_val, ProblemMetadataValue::I64(-42)));

    let u64_val = ProblemMetadataValue::from(FieldValue::U64(100));
    assert!(matches!(u64_val, ProblemMetadataValue::U64(100)));

    let f64_val = ProblemMetadataValue::from(FieldValue::F64(1.5));
    assert!(matches!(f64_val, ProblemMetadataValue::F64(_)));

    let bool_val = ProblemMetadataValue::from(FieldValue::Bool(true));
    assert!(matches!(bool_val, ProblemMetadataValue::Bool(true)));

    let uuid = Uuid::nil();
    let uuid_val = ProblemMetadataValue::from(FieldValue::Uuid(uuid));
    assert!(matches!(uuid_val, ProblemMetadataValue::String(_)));

    let dur_val = ProblemMetadataValue::from(FieldValue::Duration(Duration::from_secs(5)));
    assert!(matches!(dur_val, ProblemMetadataValue::Duration { .. }));

    let ip: IpAddr = "127.0.0.1".parse().unwrap();
    let ip_val = ProblemMetadataValue::from(FieldValue::Ip(ip));
    assert!(matches!(ip_val, ProblemMetadataValue::Ip(_)));
}

#[cfg(feature = "serde_json")]
#[test]
fn problem_metadata_value_from_json() {
    use serde_json::json;

    use crate::{FieldValue, ProblemMetadataValue};

    let json_val = ProblemMetadataValue::from(FieldValue::Json(json!({"key": "value"})));
    assert!(matches!(json_val, ProblemMetadataValue::Json(_)));
}

#[test]
fn code_mapping_accessors() {
    use crate::mapping_for_code;
    let mapping = mapping_for_code(&AppCode::NotFound);
    assert_eq!(mapping.http_status(), 404);
    assert_eq!(mapping.kind(), AppErrorKind::NotFound);
    assert!(mapping.problem_type().contains("not-found"));
    let grpc = mapping.grpc();
    assert_eq!(grpc.name, "NOT_FOUND");
}
