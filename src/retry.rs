// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Retry advice carried by errors and surfaced to API clients.

use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

/// Retry advice intended for API clients.
///
/// When present, HTTP adapters set the `Retry-After` header with the number of
/// seconds.
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct RetryAdvice {
    /// Number of seconds the client should wait before retrying.
    pub after_seconds: u64
}
