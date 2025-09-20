use super::domain::TurnkeyErrorKind;

/// Heuristic classifier for raw SDK/provider messages (ASCII case-insensitive).
///
/// This helper **does not allocate**; it performs case-insensitive `contains`
/// checks over the input string to map common upstream texts to stable kinds.
///
/// The classifier is intentionally minimal; providers can and will change
/// messages. Prefer returning structured errors from adapters whenever
/// possible.
///
/// # Examples
/// ```rust
/// use masterror::turnkey::{TurnkeyErrorKind, classify_turnkey_error};
/// assert!(matches!(
///     classify_turnkey_error("429 Too Many Requests"),
///     TurnkeyErrorKind::RateLimited
/// ));
/// assert!(matches!(
///     classify_turnkey_error("label must be unique"),
///     TurnkeyErrorKind::UniqueLabel
/// ));
/// assert!(matches!(
///     classify_turnkey_error("request timed out"),
///     TurnkeyErrorKind::Timeout
/// ));
/// ```
#[must_use]
pub fn classify_turnkey_error(msg: &str) -> TurnkeyErrorKind {
    // Patterns grouped by kind. Keep short, ASCII, and conservative.
    const UNIQUE_PATTERNS: &[&str] = &[
        "label must be unique",
        "already exists",
        "duplicate",
        "unique"
    ];
    const RL_PATTERNS: &[&str] = &["429", "rate", "throttle"];
    const TO_PATTERNS: &[&str] = &["timeout", "timed out", "deadline exceeded"];
    const AUTH_PATTERNS: &[&str] = &["401", "403", "unauthor", "forbidden"];
    const NET_PATTERNS: &[&str] = &["network", "connection", "connect", "dns", "tls", "socket"];

    if contains_any_nocase(msg, UNIQUE_PATTERNS) {
        TurnkeyErrorKind::UniqueLabel
    } else if contains_any_nocase(msg, RL_PATTERNS) {
        TurnkeyErrorKind::RateLimited
    } else if contains_any_nocase(msg, TO_PATTERNS) {
        TurnkeyErrorKind::Timeout
    } else if contains_any_nocase(msg, AUTH_PATTERNS) {
        TurnkeyErrorKind::Auth
    } else if contains_any_nocase(msg, NET_PATTERNS) {
        TurnkeyErrorKind::Network
    } else {
        TurnkeyErrorKind::Service
    }
}

/// Returns true if `haystack` contains `needle` ignoring ASCII case.
/// Performs the search without allocating.
#[inline]
fn contains_nocase(haystack: &str, needle: &str) -> bool {
    // Fast path: empty needle always matches.
    if needle.is_empty() {
        return true;
    }
    // Walk haystack windows and compare ASCII case-insensitively.
    haystack.as_bytes().windows(needle.len()).any(|w| {
        w.iter()
            .copied()
            .map(ascii_lower)
            .eq(needle.as_bytes().iter().copied().map(ascii_lower))
    })
}

/// Check whether `haystack` contains any of the `needles` (ASCII
/// case-insensitive).
#[inline]
fn contains_any_nocase(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|n| contains_nocase(haystack, n))
}

/// Converts ASCII letters to lowercase and leaves other bytes unchanged.
#[inline]
const fn ascii_lower(b: u8) -> u8 {
    // ASCII-only fold without RangeInclusive to keep const-friendly on MSRV 1.89
    if b >= b'A' && b <= b'Z' { b + 32 } else { b }
}

#[cfg(test)]
pub(super) mod internal_tests {
    use super::*;

    #[test]
    fn contains_nocase_works_without_alloc() {
        assert!(contains_nocase("ABCdef", "cDe"));
        assert!(contains_any_nocase("hello world", &["nope", "WORLD"]));
        assert!(!contains_nocase("rustacean", "python"));
        assert!(contains_nocase("", ""));
    }
}
