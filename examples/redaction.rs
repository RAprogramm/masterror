// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Redaction example showing GDPR-compliant field masking.

use masterror::{AppError, FieldRedaction, field};

fn main() {
    let err = AppError::bad_request("Invalid credentials")
        .with_field(field::str("email", "user@example.com").with_redaction(FieldRedaction::Hash))
        .with_field(field::str("ip", "192.168.1.100").with_redaction(FieldRedaction::Redact))
        .with_field(field::str("session_id", "abc123"));
    println!("=== Redacted Metadata ===\n");
    for (key, value, redaction) in err.metadata().iter_with_redaction() {
        println!("{key}: {value:?} [{redaction:?}]");
    }
}
