// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Error;

#[derive(Debug, Error)]
#[error(fmt = crate::format_error, fmt = crate::format_error)]
struct DuplicateFmt;

fn format_error(
    _error: &DuplicateFmt,
    f: &mut core::fmt::Formatter<'_>
) -> core::fmt::Result {
    f.write_str("duplicate")
}

fn main() {}
