// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Integration test exercising real `MASTERROR_ENV` detection.
//!
//! Kept as a single test in its own binary so the environment variable is
//! set before the first `DisplayMode::current()` call in this process and
//! the cached detection cannot be influenced by other tests.

use masterror::{AppError, DisplayMode};

#[test]
fn masterror_env_prod_selects_json_display_layout() {
    unsafe {
        std::env::set_var("MASTERROR_ENV", "prod");
    }
    assert_eq!(DisplayMode::current(), DisplayMode::Prod);
    let err = AppError::not_found("missing user");
    let rendered = err.to_string();
    assert!(
        rendered.starts_with(r#"{"kind":"NotFound""#),
        "unexpected layout: {rendered}"
    );
    assert!(rendered.contains(r#""code":"NOT_FOUND""#));
    assert!(rendered.contains(r#""message":"missing user""#));
    assert!(!rendered.contains('\u{1b}'));
}
