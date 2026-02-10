// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Display mode detection and configuration.

use core::sync::atomic::{AtomicU8, Ordering};

/// Display mode for error output.
///
/// Controls the structure and verbosity of error messages based on
/// the deployment environment. The mode is determined by the
/// `MASTERROR_ENV` environment variable or auto-detected based on
/// build configuration and runtime environment.
///
/// # Examples
///
/// ```
/// use masterror::DisplayMode;
///
/// let mode = DisplayMode::current();
/// match mode {
///     DisplayMode::Prod => println!("Production mode: JSON output"),
///     DisplayMode::Local => println!("Local mode: Human-readable output"),
///     DisplayMode::Staging => println!("Staging mode: JSON with context")
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    /// Production mode: lightweight JSON, minimal fields, no sensitive data.
    ///
    /// Output includes only: `kind`, `code`, `message` (if not redacted).
    /// Metadata is filtered to exclude sensitive fields.
    /// Source chain and backtrace are excluded.
    ///
    /// # Example Output
    ///
    /// ```json
    /// {"kind":"NotFound","code":"NOT_FOUND","message":"User not found"}
    /// ```
    Prod = 0,

    /// Development mode: human-readable, full context.
    ///
    /// Output includes: error details, full source chain, complete metadata,
    /// and backtrace (if enabled). Supports colored output when the `colored`
    /// feature is enabled and output is a TTY.
    ///
    /// # Example Output
    ///
    /// ```text
    /// Error: NotFound
    /// Code: NOT_FOUND
    /// Message: User not found
    ///
    ///   Caused by: database query failed
    ///   Caused by: connection timeout
    ///
    /// Context:
    ///   user_id: 12345
    /// ```
    Local = 1,

    /// Staging mode: JSON with additional context.
    ///
    /// Output includes: `kind`, `code`, `message`, limited `source_chain`,
    /// and filtered metadata. No backtrace.
    ///
    /// # Example Output
    ///
    /// ```json
    /// {"kind":"NotFound","code":"NOT_FOUND","message":"User not found","source_chain":["database error"],"metadata":{"user_id":12345}}
    /// ```
    Staging = 2
}

impl DisplayMode {
    /// Returns the current display mode based on environment configuration.
    ///
    /// The mode is determined by checking (in order):
    /// 1. `MASTERROR_ENV` environment variable (`prod`, `local`, or `staging`)
    /// 2. Kubernetes environment detection (`KUBERNETES_SERVICE_HOST`)
    /// 3. Build configuration (`cfg!(debug_assertions)`)
    ///
    /// The result is cached for performance.
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
        static CACHED_MODE: AtomicU8 = AtomicU8::new(255);
        let cached = CACHED_MODE.load(Ordering::Relaxed);
        if cached != 255 {
            return match cached {
                0 => Self::Prod,
                1 => Self::Local,
                2 => Self::Staging,
                _ => unreachable!()
            };
        }
        let mode = Self::detect();
        CACHED_MODE.store(mode as u8, Ordering::Relaxed);
        mode
    }

    /// Detects the appropriate display mode from environment.
    ///
    /// This is an internal helper called by [`current()`](Self::current).
    pub(crate) fn detect() -> Self {
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
    pub(crate) fn detect_auto() -> Self {
        if cfg!(debug_assertions) {
            Self::Local
        } else {
            Self::Prod
        }
    }
}
