// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Error;

#[derive(Debug, Error)]
#[error("{value:>width$}", value = .value, width = .width)]
struct DynamicWidthError {
    value: &'static str,
    width: usize,
}

#[derive(Debug, Error)]
#[error("{value:.precision$}", value = .value, precision = .precision)]
struct DynamicPrecisionError {
    value: f64,
    precision: usize,
}

fn main() {
    let _ = DynamicWidthError {
        value: "aligned",
        width: 8,
    }
    .to_string();

    let _ = DynamicPrecisionError {
        value: 42.4242,
        precision: 3,
    }
    .to_string();
}
