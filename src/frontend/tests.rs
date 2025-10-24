// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{BrowserConsoleError, BrowserConsoleExt};
use crate::{AppCode, AppError, ErrorResponse};

#[test]
fn context_returns_optional_message() {
    let serialization = BrowserConsoleError::Serialization {
        message: "encode failed".to_owned()
    };
    assert_eq!(serialization.context(), Some("encode failed"));

    let invocation = BrowserConsoleError::ConsoleInvocation {
        message: "js error".to_owned()
    };
    assert_eq!(invocation.context(), Some("js error"));

    assert_eq!(
        BrowserConsoleError::ConsoleMethodNotCallable.context(),
        None
    );
    assert_eq!(BrowserConsoleError::UnsupportedTarget.context(), None);
}

#[test]
fn context_returns_message_for_console_unavailable() {
    let err = BrowserConsoleError::ConsoleUnavailable {
        message: "missing console".to_owned()
    };
    assert_eq!(err.context(), Some("missing console"));
}

#[test]
fn context_returns_message_for_console_error_unavailable() {
    let err = BrowserConsoleError::ConsoleErrorUnavailable {
        message: "error method missing".to_owned()
    };
    assert_eq!(err.context(), Some("error method missing"));
}

#[test]
fn display_formats_serialization_error() {
    let err = BrowserConsoleError::Serialization {
        message: "json fail".to_owned()
    };
    let display = format!("{}", err);
    assert!(display.contains("failed to serialize"));
    assert!(display.contains("json fail"));
}

#[test]
fn display_formats_console_unavailable() {
    let err = BrowserConsoleError::ConsoleUnavailable {
        message: "no console".to_owned()
    };
    let display = format!("{}", err);
    assert!(display.contains("not available"));
    assert!(display.contains("no console"));
}

#[test]
fn display_formats_console_error_unavailable() {
    let err = BrowserConsoleError::ConsoleErrorUnavailable {
        message: "no error fn".to_owned()
    };
    let display = format!("{}", err);
    assert!(display.contains("failed to access"));
    assert!(display.contains("no error fn"));
}

#[test]
fn display_formats_console_method_not_callable() {
    let err = BrowserConsoleError::ConsoleMethodNotCallable;
    let display = format!("{}", err);
    assert!(display.contains("not callable"));
}

#[test]
fn display_formats_console_invocation_error() {
    let err = BrowserConsoleError::ConsoleInvocation {
        message: "call failed".to_owned()
    };
    let display = format!("{}", err);
    assert!(display.contains("failed to invoke"));
    assert!(display.contains("call failed"));
}

#[test]
fn display_formats_unsupported_target() {
    let err = BrowserConsoleError::UnsupportedTarget;
    let display = format!("{}", err);
    assert!(display.contains("not supported"));
}

#[test]
fn debug_trait_works() {
    let err = BrowserConsoleError::Serialization {
        message: "test".to_owned()
    };
    let debug = format!("{:?}", err);
    assert!(debug.contains("Serialization"));
}

#[test]
fn partial_eq_works() {
    let err1 = BrowserConsoleError::UnsupportedTarget;
    let err2 = BrowserConsoleError::UnsupportedTarget;
    assert_eq!(err1, err2);

    let err3 = BrowserConsoleError::ConsoleMethodNotCallable;
    assert_ne!(err1, err3);
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::*;

    #[test]
    fn to_js_value_is_unsupported_on_native_targets() {
        let response = ErrorResponse::new(404, AppCode::NotFound, "missing user").expect("status");
        assert!(matches!(
            response.to_js_value(),
            Err(BrowserConsoleError::UnsupportedTarget)
        ));

        let err = AppError::conflict("already exists");
        assert!(matches!(
            err.to_js_value(),
            Err(BrowserConsoleError::UnsupportedTarget)
        ));
    }

    #[test]
    fn console_logging_returns_unsupported_on_native_targets() {
        let err = AppError::internal("boom");
        let result = err.log_to_browser_console();
        assert!(matches!(
            result,
            Err(BrowserConsoleError::UnsupportedTarget)
        ));
    }

    #[test]
    fn error_response_log_to_browser_console_unsupported() {
        let response = ErrorResponse::new(500, AppCode::Internal, "crash").expect("status");
        let result = response.log_to_browser_console();
        assert!(matches!(
            result,
            Err(BrowserConsoleError::UnsupportedTarget)
        ));
    }

    #[test]
    fn to_js_value_error_response_with_various_error_kinds() {
        let errors = vec![
            (404, AppCode::NotFound, "not found"),
            (409, AppCode::Conflict, "conflict"),
            (500, AppCode::Internal, "internal"),
            (401, AppCode::Unauthorized, "unauthorized"),
        ];

        for (status, code, message) in errors {
            let response = ErrorResponse::new(status, code, message).expect("status");
            assert!(matches!(
                response.to_js_value(),
                Err(BrowserConsoleError::UnsupportedTarget)
            ));
        }
    }

    #[test]
    fn to_js_value_app_error_with_all_error_kinds() {
        let errors = vec![
            AppError::not_found("not found"),
            AppError::validation("invalid"),
            AppError::unauthorized("no auth"),
            AppError::forbidden("forbidden"),
            AppError::conflict("exists"),
            AppError::bad_request("bad"),
            AppError::rate_limited("limited"),
            AppError::internal("internal"),
            AppError::timeout("timeout"),
            AppError::network("network"),
        ];

        for err in errors {
            assert!(matches!(
                err.to_js_value(),
                Err(BrowserConsoleError::UnsupportedTarget)
            ));
        }
    }

    #[test]
    fn log_to_browser_console_propagates_to_js_value_error() {
        let err = AppError::not_found("missing");
        let result = err.log_to_browser_console();
        assert!(matches!(
            result,
            Err(BrowserConsoleError::UnsupportedTarget)
        ));
    }

    #[test]
    fn error_response_to_js_value_with_empty_message() {
        let response = ErrorResponse::new(500, AppCode::Internal, "").expect("status");
        assert!(matches!(
            response.to_js_value(),
            Err(BrowserConsoleError::UnsupportedTarget)
        ));
    }

    #[test]
    fn app_error_to_js_value_with_empty_message() {
        let err = AppError::internal("");
        assert!(matches!(
            err.to_js_value(),
            Err(BrowserConsoleError::UnsupportedTarget)
        ));
    }

    #[test]
    fn error_response_to_js_value_with_unicode_message() {
        let response =
            ErrorResponse::new(404, AppCode::NotFound, "見つかりません").expect("status");
        assert!(matches!(
            response.to_js_value(),
            Err(BrowserConsoleError::UnsupportedTarget)
        ));
    }

    #[test]
    fn app_error_to_js_value_with_unicode_message() {
        let err = AppError::not_found("Ошибка поиска");
        assert!(matches!(
            err.to_js_value(),
            Err(BrowserConsoleError::UnsupportedTarget)
        ));
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use serde_wasm_bindgen::from_value;

    use super::*;
    use crate::AppErrorKind;

    #[test]
    fn error_response_to_js_value_roundtrip() {
        let response = ErrorResponse::new(404, AppCode::NotFound, "missing user").expect("status");
        let js = response.to_js_value().expect("serialize");
        let decoded: ErrorResponse = from_value(js).expect("decode");
        assert_eq!(decoded.status, 404);
        assert_eq!(decoded.code, AppCode::NotFound);
        assert_eq!(decoded.message, "missing user");
    }

    #[test]
    fn app_error_to_js_value_roundtrip() {
        let err = AppError::conflict("already exists");
        let js = err.to_js_value().expect("serialize");
        let decoded: ErrorResponse = from_value(js).expect("decode");
        assert_eq!(decoded.code, AppCode::Conflict);
        assert_eq!(decoded.message, "already exists");
        assert_eq!(decoded.status, AppErrorKind::Conflict.http_status());
    }
}
