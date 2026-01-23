// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Test file with common Rust errors for masterror-cli testing.
//!
//! This file contains INTENTIONAL compile errors to test masterror-cli.
//! It should NOT be compiled directly - only used with `masterror check`.
//!
//! Run: `cd examples/cli-test-errors && cargo run -p masterror-cli -- check`

// This file is excluded from normal compilation.
// See Cargo.toml `[[bin]]` section.

fn main() {
    // E0382 - Use of moved value
    let s = String::from("hello");
    let s2 = s;
    println!("{}", s); // error: value moved

    // E0502 - Cannot borrow as mutable (already borrowed as immutable)
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4); // error: mutable borrow while immutable exists
    println!("{}", first);

    // E0499 - Cannot borrow as mutable more than once
    let mut x = 5;
    let r1 = &mut x;
    let r2 = &mut x; // error: second mutable borrow
    println!("{} {}", r1, r2);
}
