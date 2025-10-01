// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Error;

#[derive(Debug, Error)]
#[error(transparent, code = 42)]
struct TransparentWithArgs(#[from] std::io::Error);

fn main() {}
