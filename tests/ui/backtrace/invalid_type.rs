// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Error;

#[derive(Debug, Error)]
#[error("invalid backtrace field")]
struct InvalidBacktrace {
    #[backtrace]
    trace: String
}

fn main() {}
