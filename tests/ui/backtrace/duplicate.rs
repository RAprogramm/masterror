// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Error;

#[derive(Debug, Error)]
#[error("duplicate backtrace fields")]
struct DuplicateBacktrace {
    #[backtrace]
    first: std::backtrace::Backtrace,
    #[backtrace]
    second: std::backtrace::Backtrace
}

fn main() {}
