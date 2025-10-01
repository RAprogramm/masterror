// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Masterror;

#[derive(Debug, Masterror)]
#[error("no attribute")]
struct Missing;

fn main() {}
