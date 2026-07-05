// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Error;

fn shared(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("shared")
}

fn custom(value: &u8, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "custom={value}")
}

#[derive(Debug, Error)]
#[error(fmt = crate::shared)]
enum SharedFormatter {
    First,
    Second,
    #[error(fmt = crate::custom)]
    Custom(u8),
    #[error("template {0}")]
    Template(u8)
}

fn main() {
    assert_eq!(SharedFormatter::First.to_string(), "shared");
    assert_eq!(SharedFormatter::Second.to_string(), "shared");
    assert_eq!(SharedFormatter::Custom(1).to_string(), "custom=1");
    assert_eq!(SharedFormatter::Template(2).to_string(), "template 2");
}
