// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Error;

#[derive(Debug, Error)]
enum BadEnum {
    #[error("{source:?} - {extra:?}")]
    WithExtra {
        #[from]
        source: DummyError,
        #[backtrace]
        trace: Option<std::backtrace::Backtrace>,
        extra: DummyError
    }
}

#[derive(Debug, Error)]
#[error("dummy")]
struct DummyError;

fn main() {}
