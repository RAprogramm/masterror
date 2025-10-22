// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use core::sync::atomic::Ordering;

#[cfg(feature = "tracing")]
use tracing::callsite::rebuild_interest_cache;
#[cfg(feature = "tracing")]
use tracing::{Level, event};

#[cfg(feature = "backtrace")]
use super::backtrace::capture_backtrace_snapshot;
#[cfg(feature = "tracing")]
use super::types::MessageEditPolicy;
use super::{error::Error, types::CapturedBacktrace};
#[cfg(any(feature = "metrics", feature = "tracing"))]
use crate::AppErrorKind;

impl Error {
    /// Marks the error as dirty, requiring telemetry re-emission.
    ///
    /// This internal method sets the telemetry and tracing dirty flags,
    /// ensuring that the next call to `emit_telemetry()` will process
    /// the error state.
    pub(super) fn mark_dirty(&self) {
        self.telemetry_dirty.store(true, Ordering::Release);
        #[cfg(feature = "tracing")]
        self.mark_tracing_dirty();
    }

    /// Atomically clears and returns the telemetry dirty flag.
    ///
    /// Returns `true` if the flag was set before clearing.
    fn take_dirty(&self) -> bool {
        self.telemetry_dirty.swap(false, Ordering::AcqRel)
    }

    /// Marks the tracing subsystem as dirty.
    ///
    /// Only available when the `tracing` feature is enabled.
    #[cfg(feature = "tracing")]
    fn mark_tracing_dirty(&self) {
        self.tracing_dirty.store(true, Ordering::Release);
    }

    /// Atomically clears and returns the tracing dirty flag.
    ///
    /// Returns `true` if the flag was set before clearing.
    /// Only available when the `tracing` feature is enabled.
    #[cfg(feature = "tracing")]
    fn take_tracing_dirty(&self) -> bool {
        self.tracing_dirty.swap(false, Ordering::AcqRel)
    }

    /// Captures backtrace if configured, returning a reference to it.
    ///
    /// If a backtrace was previously attached via `with_backtrace()`, returns
    /// that. Otherwise, lazily captures a new backtrace based on
    /// `RUST_BACKTRACE` configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "backtrace")]
    /// # {
    /// use masterror::AppError;
    ///
    /// let err = AppError::internal("test");
    /// let bt = err.backtrace();
    /// # }
    /// ```
    #[cfg(feature = "backtrace")]
    pub(super) fn capture_backtrace(&self) -> Option<&CapturedBacktrace> {
        if let Some(backtrace) = self.backtrace.as_deref() {
            return Some(backtrace);
        }

        self.captured_backtrace
            .get_or_init(capture_backtrace_snapshot)
            .as_deref()
    }

    #[cfg(not(feature = "backtrace"))]
    pub(super) fn capture_backtrace(&self) -> Option<&CapturedBacktrace> {
        None
    }

    /// Emits telemetry for this error if the dirty flag is set.
    ///
    /// Captures backtrace (if feature enabled), increments metrics counters,
    /// and emits tracing events. This is called automatically by constructors
    /// and mutation methods.
    pub(crate) fn emit_telemetry(&self) {
        if self.take_dirty() {
            #[cfg(feature = "backtrace")]
            let _ = self.capture_backtrace();

            #[cfg(feature = "metrics")]
            {
                let code_label = self.code.as_str().to_owned();
                let category_label = kind_label(self.kind).to_owned();
                metrics::counter!(
                    "error_total",
                    "code" => code_label,
                    "category" => category_label
                )
                .increment(1);
            }
        }

        #[cfg(feature = "tracing")]
        self.flush_tracing();
    }

    /// Flushes pending tracing events for this error.
    ///
    /// Emits a structured `tracing` event with error metadata if the tracing
    /// dirty flag is set and the subscriber is interested in ERROR-level
    /// events.
    ///
    /// Only available when the `tracing` feature is enabled.
    #[cfg(feature = "tracing")]
    fn flush_tracing(&self) {
        if !self.take_tracing_dirty() {
            return;
        }

        if !tracing::event_enabled!(target: "masterror::error", Level::ERROR) {
            rebuild_interest_cache();

            if !tracing::event_enabled!(target: "masterror::error", Level::ERROR) {
                self.mark_tracing_dirty();
                return;
            }
        }

        let message = self.message.as_deref();
        let retry_seconds = self.retry.map(|value| value.after_seconds);
        let trace_id = log_mdc::get("trace_id", |value| value.map(str::to_owned));
        event!(
            target: "masterror::error",
            Level::ERROR,
            code = self.code.as_str(),
            category = kind_label(self.kind),
            message = message,
            retry_seconds,
            redactable = matches!(self.edit_policy, MessageEditPolicy::Redact),
            metadata_len = self.metadata.len() as u64,
            www_authenticate = self.www_authenticate.as_deref(),
            trace_id = trace_id.as_deref(),
            "app error constructed"
        );
    }
}

/// Converts error kind to a static label for telemetry.
///
/// Returns a string representation of the error category for use in metrics
/// and tracing.
///
/// # Arguments
///
/// * `kind` - The error kind to convert
#[cfg(any(feature = "metrics", feature = "tracing"))]
pub(crate) fn kind_label(kind: AppErrorKind) -> &'static str {
    match kind {
        AppErrorKind::NotFound => "NotFound",
        AppErrorKind::Validation => "Validation",
        AppErrorKind::Conflict => "Conflict",
        AppErrorKind::Unauthorized => "Unauthorized",
        AppErrorKind::Forbidden => "Forbidden",
        AppErrorKind::NotImplemented => "NotImplemented",
        AppErrorKind::Internal => "Internal",
        AppErrorKind::BadRequest => "BadRequest",
        AppErrorKind::TelegramAuth => "TelegramAuth",
        AppErrorKind::InvalidJwt => "InvalidJwt",
        AppErrorKind::Database => "Database",
        AppErrorKind::Service => "Service",
        AppErrorKind::Config => "Config",
        AppErrorKind::Turnkey => "Turnkey",
        AppErrorKind::Timeout => "Timeout",
        AppErrorKind::Network => "Network",
        AppErrorKind::RateLimited => "RateLimited",
        AppErrorKind::DependencyUnavailable => "DependencyUnavailable",
        AppErrorKind::Serialization => "Serialization",
        AppErrorKind::Deserialization => "Deserialization",
        AppErrorKind::ExternalApi => "ExternalApi",
        AppErrorKind::Queue => "Queue",
        AppErrorKind::Cache => "Cache"
    }
}
