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
    // message plumbing is AppError-specific; sanity-check only kind here.
}
