// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::{AppErrorKind, Error};

#[derive(Debug, Error)]
enum Mixed {
    #[error("with spec")]
    #[app_error(kind = AppErrorKind::NotFound)]
    WithSpec,
    #[error("without")]
    Without,
}

fn main() {}
