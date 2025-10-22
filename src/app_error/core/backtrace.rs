// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

#[cfg(feature = "backtrace")]
use alloc::sync::Arc;
#[cfg(feature = "backtrace")]
use std::{
    backtrace::Backtrace,
    env,
    sync::atomic::{AtomicU8, Ordering as AtomicOrdering}
};

#[cfg(feature = "backtrace")]
const BACKTRACE_STATE_UNSET: u8 = 0;
#[cfg(feature = "backtrace")]
const BACKTRACE_STATE_ENABLED: u8 = 1;
#[cfg(feature = "backtrace")]
const BACKTRACE_STATE_DISABLED: u8 = 2;

#[cfg(feature = "backtrace")]
static BACKTRACE_STATE: AtomicU8 = AtomicU8::new(BACKTRACE_STATE_UNSET);

/// Captures a backtrace snapshot if enabled by environment configuration.
///
/// Returns `Some(Arc<Backtrace>)` if backtrace capture is enabled via
/// `RUST_BACKTRACE`, otherwise returns `None`.
///
/// Internal function used for lazy backtrace capture in errors.
#[cfg(feature = "backtrace")]
pub(crate) fn capture_backtrace_snapshot() -> Option<Arc<Backtrace>> {
    if should_capture_backtrace() {
        Some(Arc::new(Backtrace::capture()))
    } else {
        None
    }
}

/// Checks whether backtraces should be captured based on configuration.
///
/// Uses cached atomic state to avoid repeated environment variable lookups.
/// The first call reads `RUST_BACKTRACE` and caches the result.
#[cfg(feature = "backtrace")]
fn should_capture_backtrace() -> bool {
    match BACKTRACE_STATE.load(AtomicOrdering::Acquire) {
        BACKTRACE_STATE_ENABLED => true,
        BACKTRACE_STATE_DISABLED => false,
        _ => {
            let enabled = detect_backtrace_preference();
            BACKTRACE_STATE.store(
                if enabled {
                    BACKTRACE_STATE_ENABLED
                } else {
                    BACKTRACE_STATE_DISABLED
                },
                AtomicOrdering::Release
            );
            enabled
        }
    }
}

/// Detects backtrace preference from environment or test override.
///
/// Checks test override first (in test builds), then reads `RUST_BACKTRACE`.
/// Returns `true` if backtrace capture is enabled.
///
/// Valid values for `RUST_BACKTRACE`: any non-empty value except `0`, `off`,
/// or `false`.
#[cfg(feature = "backtrace")]
fn detect_backtrace_preference() -> bool {
    #[cfg(all(test, feature = "backtrace"))]
    if let Some(value) = test_backtrace_override::get() {
        return value;
    }

    match env::var_os("RUST_BACKTRACE") {
        None => false,
        Some(value) => {
            let value = value.to_string_lossy();
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return false;
            }
            let lowered = trimmed.to_ascii_lowercase();
            !(matches!(lowered.as_str(), "0" | "off" | "false"))
        }
    }
}

/// Resets the backtrace preference cache to uninitialized state.
///
/// This function is only available in test builds with the `backtrace` feature.
/// It clears both the global state and test override, forcing the next
/// backtrace capture to re-read configuration.
///
/// # Examples
///
/// ```rust
/// # #[cfg(all(test, feature = "backtrace"))]
/// # {
/// use masterror::app_error::core::backtrace::reset_backtrace_preference;
///
/// reset_backtrace_preference();
/// # }
/// ```
#[cfg(all(test, feature = "backtrace"))]
pub fn reset_backtrace_preference() {
    BACKTRACE_STATE.store(BACKTRACE_STATE_UNSET, AtomicOrdering::Release);
    test_backtrace_override::set(None);
}

/// Overrides backtrace preference for testing purposes.
///
/// This function is only available in test builds with the `backtrace` feature.
/// It allows tests to control backtrace behavior without modifying environment
/// variables.
///
/// # Arguments
///
/// * `value` - `Some(true)` to force enable, `Some(false)` to force disable,
///   `None` to clear override
///
/// # Examples
///
/// ```rust
/// # #[cfg(all(test, feature = "backtrace"))]
/// # {
/// use masterror::app_error::core::backtrace::set_backtrace_preference_override;
///
/// set_backtrace_preference_override(Some(true));
/// # }
/// ```
#[cfg(all(test, feature = "backtrace"))]
pub fn set_backtrace_preference_override(value: Option<bool>) {
    test_backtrace_override::set(value);
}

#[cfg(all(test, feature = "backtrace"))]
mod test_backtrace_override {
    use std::sync::atomic::{AtomicI8, Ordering};

    const OVERRIDE_UNSET: i8 = -1;
    const OVERRIDE_DISABLED: i8 = 0;
    const OVERRIDE_ENABLED: i8 = 1;

    static OVERRIDE_STATE: AtomicI8 = AtomicI8::new(OVERRIDE_UNSET);

    pub(super) fn set(value: Option<bool>) {
        let state = match value {
            Some(true) => OVERRIDE_ENABLED,
            Some(false) => OVERRIDE_DISABLED,
            None => OVERRIDE_UNSET
        };
        OVERRIDE_STATE.store(state, Ordering::Release);
    }

    pub(super) fn get() -> Option<bool> {
        match OVERRIDE_STATE.load(Ordering::Acquire) {
            OVERRIDE_ENABLED => Some(true),
            OVERRIDE_DISABLED => Some(false),
            _ => None
        }
    }
}

#[cfg(all(test, feature = "backtrace"))]
mod tests {
    use super::*;

    #[test]
    fn capture_snapshot_returns_some_when_enabled() {
        reset_backtrace_preference();
        set_backtrace_preference_override(Some(true));
        let result = capture_backtrace_snapshot();
        assert!(result.is_some());
        reset_backtrace_preference();
    }

    #[test]
    fn capture_snapshot_returns_none_when_disabled() {
        reset_backtrace_preference();
        set_backtrace_preference_override(Some(false));
        let result = capture_backtrace_snapshot();
        assert!(result.is_none());
        reset_backtrace_preference();
    }

    #[test]
    fn should_capture_returns_true_when_enabled() {
        reset_backtrace_preference();
        set_backtrace_preference_override(Some(true));
        assert!(should_capture_backtrace());
        reset_backtrace_preference();
    }

    #[test]
    fn should_capture_caches_enabled_state() {
        reset_backtrace_preference();
        set_backtrace_preference_override(Some(true));

        should_capture_backtrace();

        set_backtrace_preference_override(Some(false));
        assert!(
            should_capture_backtrace(),
            "should use cached enabled state"
        );

        reset_backtrace_preference();
    }

    #[test]
    fn detect_preference_respects_override() {
        reset_backtrace_preference();

        set_backtrace_preference_override(Some(true));
        assert!(detect_backtrace_preference());

        set_backtrace_preference_override(Some(false));
        assert!(!detect_backtrace_preference());

        reset_backtrace_preference();
    }

    #[test]
    fn detect_preference_returns_false_by_default() {
        reset_backtrace_preference();
        set_backtrace_preference_override(None);

        let result = detect_backtrace_preference();

        reset_backtrace_preference();
        let _ = result;
    }

    #[test]
    fn reset_clears_state_and_override() {
        set_backtrace_preference_override(Some(true));
        BACKTRACE_STATE.store(BACKTRACE_STATE_ENABLED, AtomicOrdering::Release);

        reset_backtrace_preference();

        assert_eq!(
            BACKTRACE_STATE.load(AtomicOrdering::Acquire),
            BACKTRACE_STATE_UNSET
        );
        assert_eq!(test_backtrace_override::get(), None);
    }
}
