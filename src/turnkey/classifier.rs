// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

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
    const UNIQUE_PATTERNS: &[&str] = &[
        "label must be unique",
        "already exists",
        "duplicate",
        "unique"
    ];
    const RL_PATTERNS: &[&str] = &["429", "throttle", "throttled", "too many requests"];
    const RL_BOUNDARY_PATTERNS: &[&str] = &[
        "rate limit",
        "rate limited",
        "rate limiting",
        "rate-limit",
        "rate-limited",
        "rate-limiting",
        "ratelimit",
        "ratelimited",
        "ratelimiting"
    ];
    const TO_PATTERNS: &[&str] = &["timeout", "timed out", "deadline exceeded"];
    const AUTH_PATTERNS: &[&str] = &["401", "403", "unauthor", "forbidden"];
    const NET_PATTERNS: &[&str] = &["network", "connection", "connect", "dns", "tls", "socket"];
    if contains_any_nocase(msg, UNIQUE_PATTERNS) {
        TurnkeyErrorKind::UniqueLabel
    } else if contains_any_nocase(msg, RL_PATTERNS)
        || contains_any_nocase_with_boundaries(msg, RL_BOUNDARY_PATTERNS)
    {
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
    contains_nocase_with(haystack, needle, |_, _, _| true)
}

/// Check whether `haystack` contains any of the `needles` (ASCII
/// case-insensitive).
#[inline]
fn contains_any_nocase(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|n| contains_nocase(haystack, n))
}

#[inline]
fn contains_nocase_with_boundaries(haystack: &str, needle: &str) -> bool {
    contains_nocase_with(haystack, needle, |start, end, haystack_bytes| {
        let prev_ok = if start == 0 {
            true
        } else {
            !is_ascii_alphanumeric(haystack_bytes[start - 1])
        };
        let next_ok = if end >= haystack_bytes.len() {
            true
        } else {
            !is_ascii_alphanumeric(haystack_bytes[end])
        };
        prev_ok && next_ok
    })
}

#[inline]
fn contains_any_nocase_with_boundaries(haystack: &str, needles: &[&str]) -> bool {
    needles
        .iter()
        .any(|n| contains_nocase_with_boundaries(haystack, n))
}

#[inline]
fn contains_nocase_with(
    haystack: &str,
    needle: &str,
    mut boundary: impl FnMut(usize, usize, &[u8]) -> bool
) -> bool {
    if needle.is_empty() {
        return true;
    }
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    let lowered = LowercasedNeedle::new(needle_bytes);
    let needle_lower = lowered.as_slice();
    if needle_lower.is_empty() {
        return true;
    }
    haystack_bytes
        .windows(needle_lower.len())
        .enumerate()
        .any(|(start, window)| {
            window
                .iter()
                .zip(needle_lower.iter())
                .all(|(hay, lower)| ascii_lower(*hay) == *lower)
                && boundary(start, start + needle_lower.len(), haystack_bytes)
        })
}

struct LowercasedNeedle {
    inline: [u8; STACK_NEEDLE_INLINE_CAP],
    len:    usize,
    heap:   Option<Vec<u8>>
}

impl LowercasedNeedle {
    #[inline]
    fn new(needle_bytes: &[u8]) -> Self {
        if needle_bytes.len() <= STACK_NEEDLE_INLINE_CAP {
            let mut inline = [0u8; STACK_NEEDLE_INLINE_CAP];
            for (idx, byte) in needle_bytes.iter().enumerate() {
                inline[idx] = ascii_lower(*byte);
            }
            Self {
                inline,
                len: needle_bytes.len(),
                heap: None
            }
        } else {
            let mut heap = Vec::with_capacity(needle_bytes.len());
            for byte in needle_bytes {
                heap.push(ascii_lower(*byte));
            }
            Self {
                inline: [0u8; STACK_NEEDLE_INLINE_CAP],
                len:    needle_bytes.len(),
                heap:   Some(heap)
            }
        }
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        match &self.heap {
            Some(heap) => heap.as_slice(),
            None => &self.inline[..self.len]
        }
    }
}

#[inline]
const fn is_ascii_alphanumeric(byte: u8) -> bool {
    (byte >= b'0' && byte <= b'9')
        || (byte >= b'A' && byte <= b'Z')
        || (byte >= b'a' && byte <= b'z')
}

/// Converts ASCII letters to lowercase and leaves other bytes unchanged.
///
/// Uses direct byte comparison instead of `RangeInclusive` to stay
/// const-friendly on MSRV 1.90.
#[inline]
const fn ascii_lower(b: u8) -> u8 {
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

    #[test]
    fn contains_nocase_with_boundaries_respects_word_edges() {
        assert!(contains_nocase_with_boundaries(
            "rate limited",
            "rate limited"
        ));
        assert!(contains_nocase_with_boundaries(
            "429 rate-limit reached",
            "rate-limit"
        ));
        assert!(contains_nocase_with_boundaries(
            "api ratelimited",
            "ratelimited"
        ));
        assert!(!contains_nocase_with_boundaries("corporate policy", "rate"));
        assert!(!contains_nocase_with_boundaries(
            "accelerate limit",
            "rate limit"
        ));
    }
}
