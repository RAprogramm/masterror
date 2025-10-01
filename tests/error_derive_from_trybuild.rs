// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use trybuild::TestCases;

#[test]
fn from_attribute_compile_failures() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/from/*.rs");
}

#[test]
fn transparent_attribute_compile_failures() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/transparent/*.rs");
}

#[test]
fn backtrace_attribute_compile_failures() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/backtrace/*.rs");
}

#[test]
fn formatter_attribute_passes() {
    let t = TestCases::new();
    t.pass("tests/ui/formatter/pass/*.rs");
}

#[test]
fn formatter_attribute_compile_failures() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/formatter/fail/*.rs");
}

#[test]
fn app_error_attribute_passes() {
    let t = TestCases::new();
    t.pass("tests/ui/app_error/pass/*.rs");
}

#[test]
fn app_error_attribute_compile_failures() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/app_error/fail/*.rs");
}

#[test]
fn masterror_attribute_passes() {
    let t = TestCases::new();
    t.pass("tests/ui/masterror/pass/*.rs");
}

#[test]
fn masterror_attribute_compile_failures() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/masterror/fail/*.rs");
}
