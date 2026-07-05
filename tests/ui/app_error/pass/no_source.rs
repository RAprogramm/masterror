// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::rc::Rc;

use masterror::{AppError, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("non-send payload {payload}")]
#[app_error(kind = AppErrorKind::Internal, no_source)]
struct NonSendError {
    payload: Rc<String>,
}

#[derive(Debug, Error)]
enum NonSendEnumError {
    #[error("non-send variant {0}")]
    #[app_error(kind = AppErrorKind::Service, no_source, message)]
    Payload(Rc<u8>),
}

fn main() {
    let err = NonSendError {
        payload: Rc::new("data".to_owned()),
    };
    let app: AppError = err.into();
    assert!(matches!(app.kind, AppErrorKind::Internal));
    assert!(app.source_ref().is_none());

    let app: AppError = NonSendEnumError::Payload(Rc::new(7)).into();
    assert!(matches!(app.kind, AppErrorKind::Service));
    assert_eq!(app.message.as_deref(), Some("non-send variant 7"));
    assert!(app.source_ref().is_none());
}
