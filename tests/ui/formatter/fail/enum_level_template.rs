// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Error;

#[derive(Debug, Error)]
#[error("enum-level template")]
enum EnumLevelTemplate {
    #[error("variant")]
    Variant
}

fn main() {}
