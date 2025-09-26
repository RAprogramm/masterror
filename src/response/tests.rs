use super::ErrorResponse;
use crate::{AppCode, AppError, AppErrorKind, ProblemJson};

// --- Basic constructors and fields --------------------------------------

#[test]
fn new_sets_status_code_and_message() {
    let e = ErrorResponse::new(404, AppCode::NotFound, "missing").expect("status");
    assert_eq!(e.status, 404);
    assert!(matches!(e.code, AppCode::NotFound));
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
    assert!(matches!(e.code, AppCode::NotFound));
    assert_eq!(e.message, "user");
    assert!(e.retry.is_none());
}

#[test]
fn from_app_error_uses_default_message_when_none() {
    let app = AppError::bare(AppErrorKind::Internal);
    let e: ErrorResponse = (&app).into();
    assert_eq!(e.status, 500);
    assert!(matches!(e.code, AppCode::Internal));
    assert_eq!(e.message, AppErrorKind::Internal.to_string());
}

#[test]
fn from_owned_app_error_moves_message_and_metadata() {
    let err = AppError::unauthorized(String::from("owned message"))
        .with_retry_after_secs(5)
        .with_www_authenticate("Bearer");

    let resp: ErrorResponse = err.into();

    assert_eq!(resp.status, 401);
    assert!(matches!(resp.code, AppCode::Unauthorized));
    assert_eq!(resp.message, "owned message");
    assert_eq!(resp.retry.unwrap().after_seconds, 5);
    assert_eq!(resp.www_authenticate.as_deref(), Some("Bearer"));
}

#[test]
fn from_owned_app_error_defaults_message_when_absent() {
    let resp: ErrorResponse = AppError::bare(AppErrorKind::Internal).into();

    assert_eq!(resp.status, 500);
    assert!(matches!(resp.code, AppCode::Internal));
    assert_eq!(resp.message, AppErrorKind::Internal.to_string());
}

#[test]
fn from_app_error_bare_uses_kind_display_as_message() {
    let app = AppError::bare(AppErrorKind::Timeout);
    let resp: ErrorResponse = app.into();

    assert_eq!(resp.status, 504);
    assert!(matches!(resp.code, AppCode::Timeout));
    assert_eq!(resp.message, AppErrorKind::Timeout.to_string());
}

#[test]
fn from_app_error_redacts_message_when_policy_allows() {
    let app = AppError::internal("sensitive").redactable();
    let resp: ErrorResponse = app.into();

    assert_eq!(resp.message, AppErrorKind::Internal.to_string());

    let borrowed = AppError::internal("private").redactable();
    let resp_ref: ErrorResponse = (&borrowed).into();
    assert_eq!(resp_ref.message, AppErrorKind::Internal.to_string());
}

#[test]
fn error_response_serialization_hides_redacted_message() {
    let secret = "super-secret";
    let resp: ErrorResponse = AppError::internal(secret).redactable().into();
    let json = serde_json::to_value(&resp).expect("serialize response");

    let fallback = AppErrorKind::Internal.to_string();
    assert_eq!(
        json.get("message").and_then(|value| value.as_str()),
        Some(fallback.as_str())
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
    let e = ErrorResponse::new_legacy(500, "boom");
    assert_eq!(e.status, 500);
    assert!(matches!(e.code, AppCode::Internal));
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
    assert_eq!(headers.get(RETRY_AFTER).unwrap(), "7");
    assert_eq!(
        headers.get(WWW_AUTHENTICATE).unwrap(),
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

    // Build ErrorResponse with both headers
    let resp = ErrorResponse::new(429, AppCode::RateLimited, "slow down")
        .expect("status")
        .with_retry_after_secs(42)
        .with_www_authenticate("Bearer");

    // Build a minimal HttpRequest for Responder::respond_to
    let req = TestRequest::default().to_http_request();

    // `respond_to` builds HttpResponse synchronously; we can inspect it.
    let http = resp.respond_to(&req);
    assert_eq!(http.status(), StatusCode::TOO_MANY_REQUESTS);

    let headers = http.headers();
    assert_eq!(headers.get(RETRY_AFTER).unwrap(), "42");
    assert_eq!(headers.get(WWW_AUTHENTICATE).unwrap(), "Bearer");
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
    // Fast contract sanity checks without tying to exact field order
    assert!(s.contains("\"status\":404"));
    assert!(s.contains("\"code\":\"NOT_FOUND\""));
    assert!(s.contains("\"message\":\"nope\""));
    // Retry advice is serialized as nested object
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
