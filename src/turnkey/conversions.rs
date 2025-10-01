// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::domain::{TurnkeyError, TurnkeyErrorKind, map_turnkey_kind};
use crate::{AppError, AppErrorKind};

impl From<TurnkeyErrorKind> for AppErrorKind {
    #[inline]
    fn from(k: TurnkeyErrorKind) -> Self {
        map_turnkey_kind(k)
    }
}

impl From<TurnkeyError> for AppError {
    #[inline]
    fn from(e: TurnkeyError) -> Self {
        // Prefer explicit constructors to keep transport mapping consistent.
        match e.kind {
            TurnkeyErrorKind::UniqueLabel => AppError::conflict(e.msg),
            TurnkeyErrorKind::RateLimited => AppError::rate_limited(e.msg),
            TurnkeyErrorKind::Timeout => AppError::timeout(e.msg),
            TurnkeyErrorKind::Auth => AppError::unauthorized(e.msg),
            TurnkeyErrorKind::Network => AppError::network(e.msg),
            TurnkeyErrorKind::Service => AppError::turnkey(e.msg)
        }
    }
}
