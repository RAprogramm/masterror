use super::domain::TurnkeyErrorKind;

const STACK_NEEDLE_INLINE_CAP: usize = 64;

/// Heuristic classifier for raw SDK/provider messages (ASCII case-insensitive).
///
/// This helper keeps allocations to a minimum; it performs case-insensitive
/// `contains` checks over the input string to map common upstream texts to
/// stable kinds while reusing stack buffers for the short ASCII patterns we
/// match.
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
///
/// The search avoids heap allocations for needles up to
/// `STACK_NEEDLE_INLINE_CAP` bytes by reusing a stack buffer. Longer needles
/// allocate once to store their lowercased representation.
#[inline]
fn contains_nocase(haystack: &str, needle: &str) -> bool {
    // Fast path: empty needle always matches.
    if needle.is_empty() {
        return true;
    }
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();

    let search = |needle_lower: &[u8]| {
        haystack_bytes.windows(needle_lower.len()).any(|window| {
            window
                .iter()
                .zip(needle_lower.iter())
                .all(|(hay, lower_needle)| ascii_lower(*hay) == *lower_needle)
        })
    };

    if needle_bytes.len() <= STACK_NEEDLE_INLINE_CAP {
        let mut inline = [0u8; STACK_NEEDLE_INLINE_CAP];
        for (idx, byte) in needle_bytes.iter().enumerate() {
            inline[idx] = ascii_lower(*byte);
        }
        search(&inline[..needle_bytes.len()])
    } else {
        let mut lowercased = Vec::with_capacity(needle_bytes.len());
        for byte in needle_bytes {
            lowercased.push(ascii_lower(*byte));
        }
        search(lowercased.as_slice())
    }
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
    fn contains_nocase_matches_ascii_case_insensitively() {
        assert!(contains_nocase("ABCdef", "cDe"));
        assert!(contains_any_nocase("hello world", &["nope", "WORLD"]));
        assert!(!contains_nocase("rustacean", "python"));
        assert!(contains_nocase("", ""));
    }

    #[test]
    fn contains_nocase_handles_long_needles() {
        let haystack = "prefixed".to_owned() + &"A".repeat(128) + "suffix";
        let needle = "a".repeat(128);
        assert!(contains_nocase(&haystack, &needle));
    }
}
