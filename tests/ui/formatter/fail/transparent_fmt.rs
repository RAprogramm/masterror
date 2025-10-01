// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Error;

fn format_unit(_f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    Ok(())
}

#[derive(Debug, Error)]
#[error(transparent, fmt = crate::format_unit)]
struct TransparentFormatterWrapper(#[from] std::io::Error);

fn main() {}
