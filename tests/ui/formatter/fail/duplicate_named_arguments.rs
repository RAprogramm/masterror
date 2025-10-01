// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Error;

#[derive(Debug, Error)]
#[error("{value}", value = self.value, value = self.value)]
struct DuplicateNamedArgumentError {
    value: &'static str,
}

fn main() {}
