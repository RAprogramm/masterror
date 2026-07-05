// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Edge cases hardened against by thiserror 2.x, verified for masterror.
//!
//! Covers raw-identifier `r#source` fields opting out of implicit source
//! detection, dynamically sized final fields, associated types of generic
//! parameters in display templates, enum-level `#[error(fmt = ...)]` shared
//! across variants, and lint hygiene of generated code.

use std::{error::Error as StdError, fmt};

use masterror::Error;

#[derive(Debug, Error)]
#[error("leaf failure")]
struct LeafError;

#[derive(Debug, Error)]
#[error("raw source: {source}")]
struct RawSourceStruct {
    r#source: String
}

#[derive(Debug, Error)]
enum RawSourceEnum {
    #[error("raw variant: {source}")]
    Named { r#source: String }
}

#[derive(Debug, Error)]
#[error("explicit: {source}")]
struct ExplicitRawSource {
    #[source]
    r#source: LeafError
}

#[derive(Debug, Error)]
#[error("dst message: {msg}, tail: {tail:?}")]
struct DstTail {
    msg:  String,
    tail: [u8]
}

pub trait Repository {
    type Entity: fmt::Debug + fmt::Display;
}

#[derive(Debug)]
pub struct UserRepository;

impl Repository for UserRepository {
    type Entity = String;
}

#[derive(Debug, Error)]
#[error("missing entity: {entity}")]
struct AssocTypeError<T: Repository + fmt::Debug>
where
    T::Entity: fmt::Debug + fmt::Display
{
    entity: T::Entity
}

fn shared_formatter(f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("shared failure")
}

fn value_formatter(value: &u8, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "value failure: {value}")
}

#[derive(Debug, Error)]
#[error(fmt = shared_formatter)]
enum SharedFmtEnum {
    First,
    Second,
    #[error(fmt = value_formatter)]
    Custom(u8),
    #[error("templated failure: {0}")]
    Templated(u8),
    #[error(transparent)]
    Transparent(LeafError)
}

mod hygiene {
    #![deny(
        deprecated,
        unused_qualifications,
        clippy::needless_lifetimes,
        clippy::elidable_lifetime_names,
        clippy::allow_attributes,
        clippy::missing_inline_in_public_items
    )]

    use masterror::Error;

    #[derive(Debug, Error)]
    #[error("lifetime failure: {value}")]
    pub struct LifetimeError<'a> {
        pub value: &'a str
    }

    #[deprecated]
    #[derive(Debug, Error)]
    #[error("deprecated type failure")]
    pub struct DeprecatedError;

    #[derive(Debug, Error)]
    pub enum DeprecatedVariantError {
        #[deprecated]
        #[error("deprecated variant failure")]
        Old,
        #[error("current variant failure")]
        Current
    }
}

fn assert_error_impl<E: StdError + ?Sized>() {}

#[test]
fn raw_source_field_is_not_implicit_source() {
    let error = RawSourceStruct {
        r#source: "payload".to_owned()
    };
    assert_eq!(error.to_string(), "raw source: payload");
    assert!(StdError::source(&error).is_none());
}

#[test]
fn raw_source_variant_field_is_not_implicit_source() {
    let error = RawSourceEnum::Named {
        r#source: "payload".to_owned()
    };
    assert_eq!(error.to_string(), "raw variant: payload");
    assert!(StdError::source(&error).is_none());
}

#[test]
fn raw_source_field_with_explicit_attribute_is_source() {
    let error = ExplicitRawSource {
        r#source: LeafError
    };
    assert_eq!(error.to_string(), "explicit: leaf failure");
    let source = StdError::source(&error).expect("explicit source");
    assert_eq!(source.to_string(), "leaf failure");
}

#[test]
fn dynamically_sized_final_field_compiles() {
    assert_error_impl::<DstTail>();
}

#[test]
fn associated_type_field_uses_declared_bounds() {
    let error = AssocTypeError::<UserRepository> {
        entity: "user".to_owned()
    };
    assert_eq!(error.to_string(), "missing entity: user");
    assert!(StdError::source(&error).is_none());
}

#[test]
fn enum_level_fmt_shared_across_variants() {
    assert_eq!(SharedFmtEnum::First.to_string(), "shared failure");
    assert_eq!(SharedFmtEnum::Second.to_string(), "shared failure");
}

#[test]
fn enum_level_fmt_variant_overrides() {
    assert_eq!(SharedFmtEnum::Custom(7).to_string(), "value failure: 7");
    assert_eq!(
        SharedFmtEnum::Templated(9).to_string(),
        "templated failure: 9"
    );
    assert_eq!(
        SharedFmtEnum::Transparent(LeafError).to_string(),
        "leaf failure"
    );
}

#[test]
fn deprecated_type_derive_does_not_warn() {
    #[allow(deprecated)]
    let error = hygiene::DeprecatedError;
    assert_eq!(error.to_string(), "deprecated type failure");
}

#[test]
fn deprecated_variant_derive_does_not_warn() {
    #[allow(deprecated)]
    let old = hygiene::DeprecatedVariantError::Old;
    assert_eq!(old.to_string(), "deprecated variant failure");
    assert_eq!(
        hygiene::DeprecatedVariantError::Current.to_string(),
        "current variant failure"
    );
}

#[test]
fn lifetime_error_display() {
    let error = hygiene::LifetimeError {
        value: "borrowed"
    };
    assert_eq!(error.to_string(), "lifetime failure: borrowed");
}
