// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror::{AppCode, AppError, ErrorResponse, ProblemJson};

fn error_with_dynamic_code() -> AppError {
    let code = AppCode::try_new("DYNAMIC_REGRESSION_CODE".to_owned())
        .expect("valid SCREAMING_SNAKE_CASE code");
    AppError::internal("boom").with_code(code)
}

#[test]
fn problem_json_reuses_app_code_allocation() {
    let error = error_with_dynamic_code();
    let expected_ptr = error.code.as_str().as_ptr();
    let problem = ProblemJson::from_app_error(error);
    assert_eq!(problem.code.as_str().as_ptr(), expected_ptr);
}

#[test]
fn error_response_reuses_app_code_allocation() {
    let error = error_with_dynamic_code();
    let expected_ptr = error.code.as_str().as_ptr();
    let response = ErrorResponse::from(error);
    assert_eq!(response.code.as_str().as_ptr(), expected_ptr);
}
