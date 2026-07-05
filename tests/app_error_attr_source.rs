// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::io;

use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("bare failure")]
#[app_error(kind = AppErrorKind::Service)]
struct BareDomainError;

#[derive(Debug, Error)]
#[error("missing flag: {name}")]
#[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
struct MessageDomainError {
    name: &'static str
}

#[derive(Debug, Error)]
#[error("database connection failed")]
#[app_error(kind = AppErrorKind::Internal)]
struct ChainedDomainError {
    #[source]
    source: io::Error
}

#[derive(Debug, Error)]
enum DomainEnumError {
    #[error("missing resource {id}")]
    #[app_error(kind = AppErrorKind::NotFound, code = AppCode::NotFound, message)]
    Missing { id: u64 },
    #[error("backend unavailable")]
    #[app_error(kind = AppErrorKind::Service, code = AppCode::Service)]
    Backend
}

#[test]
fn struct_without_message_attaches_source() {
    let app: AppError = BareDomainError.into();
    assert!(app.message.is_none());
    assert!(app.source_ref().is_some());
    assert!(app.is::<BareDomainError>());
    assert!(app.downcast_ref::<BareDomainError>().is_some());
}

#[test]
fn struct_with_message_keeps_message_and_source() {
    let domain = MessageDomainError {
        name: "feature"
    };
    let rendered = domain.to_string();
    let app: AppError = domain.into();
    assert_eq!(app.message.as_deref(), Some(rendered.as_str()));
    let source = app
        .downcast_ref::<MessageDomainError>()
        .expect("domain source");
    assert_eq!(source.name, "feature");
}

#[test]
fn enum_variant_with_message_attaches_source() {
    let app: AppError = DomainEnumError::Missing {
        id: 7
    }
    .into();
    assert!(matches!(app.kind, AppErrorKind::NotFound));
    assert_eq!(app.message.as_deref(), Some("missing resource 7"));
    assert!(matches!(
        app.downcast_ref::<DomainEnumError>(),
        Some(DomainEnumError::Missing {
            id: 7
        })
    ));
}

#[test]
fn enum_variant_without_message_attaches_source() {
    let app: AppError = DomainEnumError::Backend.into();
    assert!(matches!(app.kind, AppErrorKind::Service));
    assert!(app.message.is_none());
    assert!(matches!(
        app.downcast_ref::<DomainEnumError>(),
        Some(DomainEnumError::Backend)
    ));
}

#[test]
fn source_chain_reaches_innermost_error() {
    let app: AppError = ChainedDomainError {
        source: io::Error::other("disk offline")
    }
    .into();
    assert_eq!(app.chain().count(), 3);
    assert_eq!(app.root_cause().to_string(), "disk offline");
}

#[test]
fn app_code_conversion_is_unchanged() {
    let code: AppCode = MessageDomainError {
        name: "other"
    }
    .into();
    assert_eq!(code, AppCode::BadRequest);
    let code: AppCode = DomainEnumError::Backend.into();
    assert_eq!(code, AppCode::Service);
}

#[test]
fn problem_json_does_not_serialize_source() {
    use masterror::ProblemJson;

    let app: AppError = ChainedDomainError {
        source: io::Error::other("disk offline")
    }
    .into();
    let problem = ProblemJson::from_app_error(app);
    let payload = serde_json::to_string(&problem).expect("problem json");
    assert!(!payload.contains("disk offline"));
    assert!(!payload.contains("database connection failed"));
    assert!(!payload.contains("source"));
}
