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
