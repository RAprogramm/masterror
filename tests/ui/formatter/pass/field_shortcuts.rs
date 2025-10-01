// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Error;

#[derive(Debug, Error)]
#[error("{value}", value = .value)]
struct FieldShortcut {
    value: &'static str,
}

#[derive(Debug, Error)]
#[error("{}:{}", .0, .1)]
struct TupleShortcut(&'static str, &'static str);

fn main() {
    let _ = FieldShortcut { value: "alpha" }.to_string();
    let _ = TupleShortcut("left", "right").to_string();
}
