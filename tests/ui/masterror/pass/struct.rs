// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::{AppCode, AppErrorKind, Masterror};

#[derive(Debug, Masterror)]
#[error("simple {value}")]
#[masterror(
    code = AppCode::Internal,
    category = AppErrorKind::Internal,
    telemetry(),
    map.problem = "urn:example:internal"
)]
struct Simple {
    value: u8
}

fn main() {
    let err = Simple { value: 1 };
    let converted: masterror::Error = err.into();
    assert_eq!(converted.code, AppCode::Internal);
}
