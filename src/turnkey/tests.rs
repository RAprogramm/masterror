// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{TurnkeyError, TurnkeyErrorKind, classify_turnkey_error, map_turnkey_kind};
use crate::{AppError, AppErrorKind};

#[test]
fn map_is_stable() {
    assert_eq!(
        map_turnkey_kind(TurnkeyErrorKind::UniqueLabel),
        AppErrorKind::Conflict
    );
    assert_eq!(
        map_turnkey_kind(TurnkeyErrorKind::RateLimited),
        AppErrorKind::RateLimited
    );
    assert_eq!(
        map_turnkey_kind(TurnkeyErrorKind::Timeout),
        AppErrorKind::Timeout
    );
    assert_eq!(
        map_turnkey_kind(TurnkeyErrorKind::Auth),
        AppErrorKind::Unauthorized
    );
    assert_eq!(
        map_turnkey_kind(TurnkeyErrorKind::Network),
        AppErrorKind::Network
    );
    assert_eq!(
        map_turnkey_kind(TurnkeyErrorKind::Service),
        AppErrorKind::Turnkey
    );
}

#[test]
fn classifier_unique() {
    for s in [
        "Label must be UNIQUE",
        "already exists: trading-key-foo",
        "duplicate label",
        "unique constraint violation"
    ] {
        assert!(
            matches!(classify_turnkey_error(s), TurnkeyErrorKind::UniqueLabel),
            "failed on: {s}"
        );
    }
}

#[test]
fn classifier_rate_limited() {
    for s in [
        "429 Too Many Requests",
        "rate limit exceeded",
        "throttled by upstream",
        "client ratelimited",
        "rate-limited by upstream",
        "rate limiting in effect"
    ] {
        assert!(
            matches!(classify_turnkey_error(s), TurnkeyErrorKind::RateLimited),
            "failed on: {s}"
        );
    }
}

#[test]
fn classifier_timeout() {
    for s in [
        "request timed out",
        "Timeout while waiting",
        "deadline exceeded"
    ] {
        assert!(
            matches!(classify_turnkey_error(s), TurnkeyErrorKind::Timeout),
            "failed on: {s}"
        );
    }
}

#[test]
fn classifier_auth() {
    for s in ["401 Unauthorized", "403 Forbidden", "unauthor ized"] {
        assert!(
            matches!(classify_turnkey_error(s), TurnkeyErrorKind::Auth),
            "failed on: {s}"
        );
    }
}

#[test]
fn classifier_network() {
    for s in [
        "network error",
        "connection reset",
        "DNS failure",
        "TLS handshake",
        "socket hang up",
        "Corporate network outage"
    ] {
        assert!(
            matches!(classify_turnkey_error(s), TurnkeyErrorKind::Network),
            "failed on: {s}"
        );
    }
}

#[test]
fn classifier_service_fallback() {
    for s in ["unrecognized issue", "operational failure rate"] {
        assert!(
            matches!(classify_turnkey_error(s), TurnkeyErrorKind::Service),
            "failed on: {s}"
        );
    }
}

#[test]
fn from_turnkey_error_into_app_error() {
    let e = TurnkeyError::new(TurnkeyErrorKind::RateLimited, "try later");
    let a: AppError = e.into();
    assert_eq!(a.kind, AppErrorKind::RateLimited);
}

#[test]
fn turnkey_error_kind_display_unique_label() {
    let kind = TurnkeyErrorKind::UniqueLabel;
    assert_eq!(kind.to_string(), "label already exists");
}

#[test]
fn turnkey_error_kind_display_rate_limited() {
    let kind = TurnkeyErrorKind::RateLimited;
    assert_eq!(kind.to_string(), "rate limited or throttled");
}

#[test]
fn turnkey_error_kind_display_timeout() {
    let kind = TurnkeyErrorKind::Timeout;
    assert_eq!(kind.to_string(), "request timed out");
}

#[test]
fn turnkey_error_kind_display_auth() {
    let kind = TurnkeyErrorKind::Auth;
    assert_eq!(kind.to_string(), "authentication/authorization failed");
}

#[test]
fn turnkey_error_kind_display_network() {
    let kind = TurnkeyErrorKind::Network;
    assert_eq!(kind.to_string(), "network error");
}

#[test]
fn turnkey_error_kind_display_service() {
    let kind = TurnkeyErrorKind::Service;
    assert_eq!(kind.to_string(), "service error");
}

#[test]
fn turnkey_error_new_creates_error_with_kind_and_message() {
    let err = TurnkeyError::new(TurnkeyErrorKind::Timeout, "operation timeout");
    assert_eq!(err.kind, TurnkeyErrorKind::Timeout);
    assert_eq!(err.msg, "operation timeout");
}

#[test]
fn turnkey_error_new_accepts_string() {
    let err = TurnkeyError::new(TurnkeyErrorKind::Network, "test".to_string());
    assert_eq!(err.msg, "test");
}

#[test]
fn turnkey_error_new_accepts_str() {
    let err = TurnkeyError::new(TurnkeyErrorKind::Auth, "auth failed");
    assert_eq!(err.msg, "auth failed");
}

#[test]
fn turnkey_error_new_accepts_empty_string() {
    let err = TurnkeyError::new(TurnkeyErrorKind::Service, "");
    assert_eq!(err.msg, "");
}

#[test]
fn turnkey_error_new_accepts_unicode() {
    let err = TurnkeyError::new(TurnkeyErrorKind::UniqueLabel, "ラベルが存在します");
    assert_eq!(err.msg, "ラベルが存在します");
}

#[test]
fn turnkey_error_display_formats_kind_and_message() {
    let err = TurnkeyError::new(TurnkeyErrorKind::RateLimited, "quota exceeded");
    let display = err.to_string();
    assert!(display.contains("rate limited or throttled"));
    assert!(display.contains("quota exceeded"));
    assert_eq!(display, "rate limited or throttled: quota exceeded");
}

#[test]
fn turnkey_error_display_with_empty_message() {
    let err = TurnkeyError::new(TurnkeyErrorKind::Timeout, "");
    let display = err.to_string();
    assert_eq!(display, "request timed out: ");
}

#[test]
fn turnkey_error_clone_creates_identical_copy() {
    let err1 = TurnkeyError::new(TurnkeyErrorKind::Network, "connection lost");
    let err2 = err1.clone();
    assert_eq!(err1.kind, err2.kind);
    assert_eq!(err1.msg, err2.msg);
    assert_eq!(err1, err2);
}

#[test]
fn turnkey_error_partial_eq_compares_kind_and_message() {
    let err1 = TurnkeyError::new(TurnkeyErrorKind::Auth, "invalid token");
    let err2 = TurnkeyError::new(TurnkeyErrorKind::Auth, "invalid token");
    let err3 = TurnkeyError::new(TurnkeyErrorKind::Auth, "different message");
    let err4 = TurnkeyError::new(TurnkeyErrorKind::Service, "invalid token");

    assert_eq!(err1, err2);
    assert_ne!(err1, err3);
    assert_ne!(err1, err4);
}

#[test]
fn turnkey_error_kind_clone_creates_identical_copy() {
    let kind1 = TurnkeyErrorKind::Timeout;
    let kind2 = kind1;
    assert_eq!(kind1, kind2);
}

#[test]
fn turnkey_error_kind_partial_eq_works() {
    assert_eq!(TurnkeyErrorKind::UniqueLabel, TurnkeyErrorKind::UniqueLabel);
    assert_eq!(TurnkeyErrorKind::RateLimited, TurnkeyErrorKind::RateLimited);
    assert_ne!(TurnkeyErrorKind::Timeout, TurnkeyErrorKind::Network);
}

#[test]
fn map_turnkey_kind_is_inline() {
    let kind = TurnkeyErrorKind::Timeout;
    let mapped = map_turnkey_kind(kind);
    assert_eq!(mapped, AppErrorKind::Timeout);
}

#[test]
fn turnkey_error_debug_format() {
    let err = TurnkeyError::new(TurnkeyErrorKind::UniqueLabel, "duplicate key");
    let debug = format!("{:?}", err);
    assert!(debug.contains("TurnkeyError"));
    assert!(debug.contains("UniqueLabel"));
    assert!(debug.contains("duplicate key"));
}
