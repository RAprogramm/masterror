// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Integration tests for environment-based display modes.
//!
//! These tests verify that MASTERROR_ENV correctly controls display output
//! by spawning subprocesses with different environment variables.

use std::process::Command;

#[test]
fn display_mode_prod_via_env() {
    let output = Command::new(env!("CARGO"))
        .args(["run", "--example", "display_mode_test"])
        .env("MASTERROR_ENV", "prod")
        .output()
        .expect("Failed to execute subprocess");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(r#""kind""#), "Prod mode should output JSON");
    assert!(
        stdout.contains(r#""code""#),
        "Prod mode should include code"
    );
}

#[test]
fn display_mode_staging_via_env() {
    let output = Command::new(env!("CARGO"))
        .args(["run", "--example", "display_mode_test"])
        .env("MASTERROR_ENV", "staging")
        .output()
        .expect("Failed to execute subprocess");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(r#""source_chain""#),
        "Staging mode should include source_chain"
    );
}

#[test]
fn display_mode_local_via_env() {
    let output = Command::new(env!("CARGO"))
        .args(["run", "--example", "display_mode_test"])
        .env("MASTERROR_ENV", "local")
        .output()
        .expect("Failed to execute subprocess");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Caused by") || stdout.contains("Error"),
        "Local mode should be human-readable"
    );
}

#[test]
fn display_mode_kubernetes_detection() {
    let output = Command::new(env!("CARGO"))
        .args(["run", "--example", "display_mode_test"])
        .env("KUBERNETES_SERVICE_HOST", "10.0.0.1")
        .env_remove("MASTERROR_ENV")
        .output()
        .expect("Failed to execute subprocess");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(r#""kind""#),
        "Kubernetes env should trigger prod mode"
    );
}
