// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::{AppCode, Masterror};

#[derive(Debug, Masterror)]
#[error("oops")]
#[masterror(code = AppCode::Internal)]
struct MissingCategory;

fn main() {}
