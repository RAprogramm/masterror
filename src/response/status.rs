// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Validation of HTTP status codes used by the wire-level payloads.
//!
//! The check is deliberately independent from the `http` crate: that crate
//! requires the standard library and therefore cannot be part of a `no_std`
//! build. The accepted range is the one implemented by `http` itself —
//! status codes from `100` to `999` inclusive.

/// Check whether `status` is a valid HTTP status code.
///
/// Returns `true` for status codes in the `100..=999` range and `false` for
/// anything outside of it.
#[must_use]
pub(super) const fn is_valid_status(status: u16) -> bool {
    status >= 100 && status <= 999
}

#[cfg(test)]
mod tests {
    use super::is_valid_status;

    #[test]
    fn accepts_the_documented_range() {
        assert!(is_valid_status(100));
        assert!(is_valid_status(404));
        assert!(is_valid_status(599));
        assert!(is_valid_status(999));
    }

    #[test]
    fn rejects_values_outside_the_range() {
        assert!(!is_valid_status(0));
        assert!(!is_valid_status(99));
        assert!(!is_valid_status(1000));
        assert!(!is_valid_status(u16::MAX));
    }
}
