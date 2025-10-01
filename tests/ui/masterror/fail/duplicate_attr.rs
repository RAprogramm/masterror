// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::{AppCode, AppErrorKind, Masterror};

#[derive(Debug, Masterror)]
#[error("dup")]
#[masterror(code = AppCode::Internal, category = AppErrorKind::Internal)]
#[masterror(code = AppCode::Internal, category = AppErrorKind::Internal)]
struct Duplicate;

fn main() {}
