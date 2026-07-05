// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::{AppCode, AppErrorKind, AppResult, app_error};

#[test]
fn kind_only_form_builds_bare_error() {
    let err = app_error!(AppErrorKind::Timeout);
    assert!(matches!(err.kind, AppErrorKind::Timeout));
    assert!(err.message.is_none());
}

#[test]
fn kind_only_form_accepts_trailing_comma() {
    let err = app_error!(AppErrorKind::NotFound,);
    assert!(matches!(err.kind, AppErrorKind::NotFound));
    assert!(err.message.is_none());
}

#[test]
fn formatted_form_captures_variables() {
    let user_id = 42;
    let err = app_error!(AppErrorKind::Validation, "invalid user {user_id}");
    assert!(matches!(err.kind, AppErrorKind::Validation));
    assert_eq!(err.message.as_deref(), Some("invalid user 42"));
}

#[test]
fn formatted_message_equals_format_output() {
    let name = "payments";
    let attempt = 3;
    let err = app_error!(
        AppErrorKind::ExternalApi,
        "service {name} failed after {attempt} attempts"
    );
    assert_eq!(
        err.message.as_deref(),
        Some(format!("service {name} failed after {attempt} attempts").as_str())
    );
}

#[test]
fn formatted_form_supports_positional_arguments() {
    let err = app_error!(AppErrorKind::BadRequest, "expected {}, got {}", 1, 2);
    assert_eq!(err.message.as_deref(), Some("expected 1, got 2"));
}

#[test]
fn kind_and_code_are_derived_from_kind() {
    let err = app_error!(AppErrorKind::Unauthorized, "token expired");
    assert!(matches!(err.kind, AppErrorKind::Unauthorized));
    assert_eq!(err.code, AppCode::Unauthorized);
}

#[test]
fn works_in_ok_or_else_expression_position() {
    fn find(id: u64) -> AppResult<u64> {
        None::<u64>.ok_or_else(|| app_error!(AppErrorKind::NotFound, "no entity {id}"))
    }
    let err = find(7).unwrap_err();
    assert!(matches!(err.kind, AppErrorKind::NotFound));
    assert_eq!(err.message.as_deref(), Some("no entity 7"));
}

#[test]
fn works_in_map_err_expression_position() {
    fn parse(input: &str) -> AppResult<i64> {
        input
            .parse::<i64>()
            .map_err(|parse_err| app_error!(AppErrorKind::BadRequest, "not a number: {parse_err}"))
    }
    let err = parse("abc").unwrap_err();
    assert!(matches!(err.kind, AppErrorKind::BadRequest));
    assert!(
        err.message
            .as_deref()
            .is_some_and(|msg| msg.starts_with("not a number:"))
    );
}

#[test]
fn composes_with_fail_macro() {
    fn reject() -> AppResult<()> {
        masterror::fail!(app_error!(AppErrorKind::Forbidden, "admin only"));
    }
    let err = reject().unwrap_err();
    assert!(matches!(err.kind, AppErrorKind::Forbidden));
    assert_eq!(err.message.as_deref(), Some("admin only"));
}

#[test]
fn composes_with_ensure_macro() {
    fn guard(flag: bool) -> AppResult<()> {
        masterror::ensure!(flag, app_error!(AppErrorKind::BadRequest, "flag required"));
        Ok(())
    }
    assert!(guard(true).is_ok());
    let err = guard(false).unwrap_err();
    assert!(matches!(err.kind, AppErrorKind::BadRequest));
}
