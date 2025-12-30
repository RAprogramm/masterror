// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::Error;

#[derive(Debug, Error)]
#[error("{label}::{name}", label = self.label, name = self.name)]
struct NamedArgumentUsage {
    label: &'static str,
    name:  &'static str,
}

#[derive(Debug, Error)]
#[error("{1}::{0}", self.first, self.second)]
struct PositionalArgumentUsage {
    first:  &'static str,
    second: &'static str,
}

#[derive(Debug, Error)]
#[error("{}, {label}, {}", label = self.label, self.first, self.second)]
struct MixedImplicitUsage {
    label:  &'static str,
    first:  &'static str,
    second: &'static str,
}

fn main() {
    let _ = NamedArgumentUsage {
        label: "left",
        name:  "right",
    }
    .to_string();
    let _ = PositionalArgumentUsage {
        first:  "positional-0",
        second: "positional-1",
    }
    .to_string();
    let _ = MixedImplicitUsage {
        label:  "tag",
        first:  "one",
        second: "two",
    }
    .to_string();
}
