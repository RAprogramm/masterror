// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Test helper for display mode integration tests.

use std::io;

use masterror::AppError;

fn main() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let error = AppError::not_found("Resource missing")
        .with_field(masterror::field::str("resource_id", "test-123"))
        .with_source(io_err);

    println!("{}", error);
}
