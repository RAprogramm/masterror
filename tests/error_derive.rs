use std::error::Error as StdError;

use masterror::Error;

#[derive(Debug, Error)]
#[error("{kind}: {message}")]
struct NamedError {
    kind:    &'static str,
    message: &'static str,
    #[source]
    cause:   Option<LeafError>
}

#[derive(Debug, Error)]
#[error("leaf failure")]
struct LeafError;

#[derive(Debug, Error)]
#[error("{0} -> {1:?}")]
struct TupleError(&'static str, u8);

#[derive(Debug, Error)]
enum EnumError {
    #[error("unit failure")]
    Unit,
    #[error("{_code}")]
    Code {
        _code: u16,
        #[source]
        cause: LeafError
    },
    #[error("{0}: {1}")]
    Pair(String, #[source] LeafError)
}

#[test]
fn named_struct_display_and_source() {
    let err = NamedError {
        kind:    "validation",
        message: "invalid email",
        cause:   Some(LeafError)
    };
    assert_eq!(err.to_string(), "validation: invalid email");
    let source = StdError::source(&err).expect("source");
    assert_eq!(source.to_string(), "leaf failure");
}

#[test]
fn tuple_struct_supports_positional_formatting() {
    let err = TupleError("alpha", 42);
    assert_eq!(err.to_string(), "alpha -> 42");
    assert!(StdError::source(&err).is_none());
}

#[test]
fn enum_variants_forward_source() {
    let err = EnumError::Code {
        _code: 503,
        cause: LeafError
    };
    assert_eq!(err.to_string(), "503");
    if let EnumError::Code {
        _code, ..
    } = &err
    {
        assert_eq!(*_code, 503);
    } else {
        panic!("unexpected variant");
    }
    assert_eq!(StdError::source(&err).unwrap().to_string(), "leaf failure");
}

#[test]
fn tuple_variant_with_source() {
    let err = EnumError::Pair("left".into(), LeafError);
    let _unit = EnumError::Unit;
    assert!(err.to_string().starts_with("left"));
    assert_eq!(StdError::source(&err).unwrap().to_string(), "leaf failure");
}
