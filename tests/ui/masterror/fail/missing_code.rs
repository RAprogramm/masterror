// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::{AppErrorKind, Masterror};

#[derive(Debug, Masterror)]
#[error("oops")]
#[masterror(category = AppErrorKind::Internal)]
struct MissingCode;

fn main() {}
