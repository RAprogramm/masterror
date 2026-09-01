// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Deployment-environment detection driving error `Display` layouts.
//!
//! [`DisplayMode`] is the single source of truth for the environment the
//! process runs in; the `Display` implementation for [`struct@crate::Error`]
//! consumes it through [`DisplayMode::current`].

use core::sync::atomic::{AtomicU8, Ordering};

/// Sentinel stored in [`CACHED_MODE`] while no mode has been detected yet.
const MODE_CACHE_UNSET: u8 = 255;

/// Process-wide cache holding the detected [`DisplayMode`] discriminant.
static CACHED_MODE: AtomicU8 = AtomicU8::new(MODE_CACHE_UNSET);

/// Detected deployment environment driving the `Display` layout of
/// [`struct@crate::Error`].
///
/// [`DisplayMode::current`] identifies the environment the process runs in,
/// based on the `MASTERROR_ENV` environment variable, Kubernetes detection
/// (`KUBERNETES_SERVICE_HOST`) or build configuration, and caches the result.
///
/// The `Display` implementation for [`struct@crate::Error`] dispatches on
/// this mode: `Local` renders a multi-line human-readable report, while
/// `Prod` and `Staging` render compact single-line JSON. Set
/// `MASTERROR_ENV=local` to force the human-readable layout in any
/// environment.
///
/// # Examples
///
/// ```
/// use masterror::DisplayMode;
///
/// let mode = DisplayMode::current();
/// match mode {
///     DisplayMode::Prod => println!("Production environment"),
///     DisplayMode::Local => println!("Local development environment"),
///     DisplayMode::Staging => println!("Staging environment")
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    /// Production environment.
    ///
    /// Selected when `MASTERROR_ENV` is `prod`/`production`, when
    /// `KUBERNETES_SERVICE_HOST` is set, or for release builds by default.
    /// Errors render as compact JSON without a source chain.
    Prod = 0,

    /// Local development environment.
    ///
    /// Selected when `MASTERROR_ENV` is `local`/`dev`/`development`, or for
    /// debug builds by default. Errors render as a multi-line
    /// human-readable report; the `colored` feature adds ANSI styling to
    /// this layout only.
    Local = 1,

    /// Staging environment.
    ///
    /// Selected when `MASTERROR_ENV` is `staging`/`stage`. Errors render as
    /// compact JSON extended with the source chain.
    Staging = 2
}

impl DisplayMode {
    /// Returns the detected environment based on configuration.
    ///
    /// The mode is determined by checking (in order):
    /// 1. `MASTERROR_ENV` environment variable (`prod`, `local`, or `staging`)
    /// 2. Kubernetes environment detection (`KUBERNETES_SERVICE_HOST`)
    /// 3. Build configuration (`cfg!(debug_assertions)`)
    ///
    /// Without the `std` feature only step 3 applies. The result is cached
    /// on first access, so the environment is read once per process and
    /// later changes to the variables have no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror::DisplayMode;
    ///
    /// let mode = DisplayMode::current();
    /// assert!(matches!(
    ///     mode,
    ///     DisplayMode::Prod | DisplayMode::Local | DisplayMode::Staging
    /// ));
    /// ```
    #[must_use]
    pub fn current() -> Self {
        #[cfg(test)]
        if let Some(mode) = test_display_mode_override::get() {
            return mode;
        }
        let cached = CACHED_MODE.load(Ordering::Relaxed);
        if cached != MODE_CACHE_UNSET {
            return Self::from_discriminant(cached);
        }
        let mode = Self::detect();
        CACHED_MODE.store(mode as u8, Ordering::Relaxed);
        mode
    }

    /// Converts a cached discriminant back into a mode.
    const fn from_discriminant(value: u8) -> Self {
        match value {
            0 => Self::Prod,
            2 => Self::Staging,
            _ => Self::Local
        }
    }

    /// Detects the appropriate display mode from environment.
    ///
    /// This is an internal helper called by [`current()`](Self::current).
    fn detect() -> Self {
        #[cfg(feature = "std")]
        {
            use std::env::var;
            if let Ok(env) = var("MASTERROR_ENV") {
                return match env.as_str() {
                    "prod" | "production" => Self::Prod,
                    "local" | "dev" | "development" => Self::Local,
                    "staging" | "stage" => Self::Staging,
                    _ => Self::detect_auto()
                };
            }
            if var("KUBERNETES_SERVICE_HOST").is_ok() {
                return Self::Prod;
            }
        }
        Self::detect_auto()
    }

    /// Auto-detects mode based on build configuration.
    const fn detect_auto() -> Self {
        if cfg!(debug_assertions) {
            Self::Local
        } else {
            Self::Prod
        }
    }
}

/// Overrides the detected display mode for testing purposes.
///
/// Setting an override clears the process-wide cache so the next
/// [`DisplayMode::current`] call observes the new value. Overridden modes are
/// consulted before the cache and never stored in it, keeping detection
/// deterministic for tests that do not override.
///
/// # Arguments
///
/// * `mode` - `Some(mode)` to force a mode, `None` to clear the override
#[cfg(test)]
pub fn set_display_mode_override(mode: Option<DisplayMode>) {
    test_display_mode_override::set(mode);
    CACHED_MODE.store(MODE_CACHE_UNSET, Ordering::Relaxed);
}

/// Resets the display mode cache and override to the uninitialized state.
///
/// Forces the next [`DisplayMode::current`] call to re-run detection. Tests
/// call it after overriding the mode, mirroring
/// `reset_backtrace_preference`.
#[cfg(test)]
pub fn reset_display_mode() {
    test_display_mode_override::set(None);
    CACHED_MODE.store(MODE_CACHE_UNSET, Ordering::Relaxed);
}

#[cfg(test)]
mod test_display_mode_override {
    use core::sync::atomic::{AtomicU8, Ordering};

    use super::DisplayMode;

    const OVERRIDE_UNSET: u8 = 255;

    static OVERRIDE_STATE: AtomicU8 = AtomicU8::new(OVERRIDE_UNSET);

    pub(super) fn set(mode: Option<DisplayMode>) {
        let state = mode.map_or(OVERRIDE_UNSET, |mode| mode as u8);
        OVERRIDE_STATE.store(state, Ordering::Release);
    }

    pub(super) fn get() -> Option<DisplayMode> {
        match OVERRIDE_STATE.load(Ordering::Acquire) {
            OVERRIDE_UNSET => None,
            value => Some(DisplayMode::from_discriminant(value))
        }
    }
}

#[cfg(test)]
pub use test_support::force_display_mode;

#[cfg(test)]
mod test_support {
    use std::sync::{Mutex, MutexGuard, PoisonError};

    use super::{DisplayMode, reset_display_mode, set_display_mode_override};

    static DISPLAY_MODE_LOCK: Mutex<()> = Mutex::new(());

    /// Guard restoring the default display mode detection on drop.
    pub struct DisplayModeGuard {
        _lock: MutexGuard<'static, ()>
    }

    impl Drop for DisplayModeGuard {
        fn drop(&mut self) {
            reset_display_mode();
        }
    }

    /// Forces the display mode for the lifetime of the returned guard.
    ///
    /// Serializes tests that override the mode so concurrent overrides do
    /// not observe each other.
    pub fn force_display_mode(mode: DisplayMode) -> DisplayModeGuard {
        let lock = DISPLAY_MODE_LOCK
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        set_display_mode_override(Some(mode));
        DisplayModeGuard {
            _lock: lock
        }
    }
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::Ordering;

    use super::{
        CACHED_MODE, DisplayMode, MODE_CACHE_UNSET, reset_display_mode,
        test_support::force_display_mode
    };

    #[test]
    fn display_mode_current_returns_valid_mode() {
        let mode = DisplayMode::current();
        assert!(matches!(
            mode,
            DisplayMode::Prod | DisplayMode::Local | DisplayMode::Staging
        ));
    }

    #[test]
    fn display_mode_detect_auto_returns_local_in_debug() {
        if cfg!(debug_assertions) {
            assert_eq!(DisplayMode::detect_auto(), DisplayMode::Local);
        } else {
            assert_eq!(DisplayMode::detect_auto(), DisplayMode::Prod);
        }
    }

    #[test]
    fn display_mode_detect_auto_returns_prod_in_release() {
        if !cfg!(debug_assertions) {
            assert_eq!(DisplayMode::detect_auto(), DisplayMode::Prod);
        }
    }

    #[test]
    fn display_mode_current_caches_result() {
        let _guard = force_display_mode(DisplayMode::Staging);
        reset_display_mode();
        assert_eq!(CACHED_MODE.load(Ordering::Relaxed), MODE_CACHE_UNSET);
        let first = DisplayMode::current();
        assert_eq!(CACHED_MODE.load(Ordering::Relaxed), first as u8);
        let second = DisplayMode::current();
        assert_eq!(first, second);
    }
}
