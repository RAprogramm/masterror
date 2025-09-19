#![allow(unused_variables, non_shorthand_field_patterns)]

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
#[error("{0}")]
struct TransparentInner(#[source] LeafError);

#[derive(Debug, Error)]
#[error(transparent)]
struct TransparentWrapper(TransparentInner);

#[derive(Debug, Error)]
#[error(transparent)]
struct TransparentFromWrapper(#[from] TransparentInner);

#[derive(Debug, Error)]
#[error("{0} -> {1:?}")]
struct TupleError(&'static str, u8);

#[derive(Debug, Error)]
enum EnumError {
    #[error("unit failure")]
    Unit,
    #[error("{code}")]
    Code {
        code:  u16,
        #[source]
        cause: LeafError
    },
    #[error("{0}: {1}")]
    Pair(String, #[source] LeafError)
}

#[derive(Debug, Error)]
#[error("primary failure")]
struct PrimaryError;

#[derive(Debug, Error)]
#[error("secondary failure")]
struct SecondaryError;

#[derive(Debug, Error)]
#[error("tuple wrapper -> {0}")]
struct TupleWrapper(
    #[from]
    #[source]
    LeafError
);

#[derive(Debug, Error)]
#[error("message: {message}")]
struct MessageWrapper {
    message: String
}

impl From<String> for MessageWrapper {
    fn from(message: String) -> Self {
        Self {
            message
        }
    }
}

#[derive(Debug, Error)]
enum MixedFromError {
    #[error("tuple variant {0}")]
    Tuple(
        #[from]
        #[source]
        LeafError
    ),
    #[error("variant attr {0}")]
    VariantAttr(
        #[from]
        #[source]
        PrimaryError
    ),
    #[error("named variant {source:?}")]
    Named {
        #[from]
        #[source]
        source: SecondaryError
    }
}

#[derive(Debug, Error)]
enum TransparentEnum {
    #[error("opaque {0}")]
    Opaque(&'static str),
    #[error(transparent)]
    TransparentVariant(#[from] TransparentInner)
}

#[derive(Debug, Error)]
#[error("{source:?}")]
struct StructFromWithBacktrace {
    #[from]
    source: LeafError,
    #[backtrace]
    trace:  Option<std::backtrace::Backtrace>
}

#[derive(Debug, Error)]
enum VariantFromWithBacktrace {
    #[error("{source:?}")]
    WithTrace {
        #[from]
        source: LeafError,
        #[backtrace]
        trace:  Option<std::backtrace::Backtrace>
    }
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
fn enum_variants_cover_display_and_source() {
    let unit = EnumError::Unit;
    assert_eq!(unit.to_string(), "unit failure");
    assert!(StdError::source(&unit).is_none());

    let code = EnumError::Code {
        code:  503,
        cause: LeafError
    };
    assert_eq!(code.to_string(), "503");
    assert_eq!(StdError::source(&code).unwrap().to_string(), "leaf failure");

    let pair = EnumError::Pair("left".into(), LeafError);
    assert!(pair.to_string().starts_with("left"));
    assert_eq!(StdError::source(&pair).unwrap().to_string(), "leaf failure");
}

#[test]
fn tuple_struct_from_wraps_source() {
    let err = TupleWrapper::from(LeafError);
    assert_eq!(err.to_string(), "tuple wrapper -> leaf failure");
    let source = StdError::source(&err).expect("source present");
    assert_eq!(source.to_string(), "leaf failure");
}

#[test]
fn named_struct_from_without_source() {
    let err = MessageWrapper::from(String::from("payload"));
    assert_eq!(err.to_string(), "message: payload");
    assert!(StdError::source(&err).is_none());
}

#[test]
fn enum_from_variants_generate_impls() {
    let tuple = MixedFromError::from(LeafError);
    assert!(matches!(&tuple, MixedFromError::Tuple(_)));
    assert_eq!(
        StdError::source(&tuple).unwrap().to_string(),
        "leaf failure"
    );

    let variant_attr = MixedFromError::from(PrimaryError);
    assert!(matches!(&variant_attr, MixedFromError::VariantAttr(_)));
    assert_eq!(
        StdError::source(&variant_attr).unwrap().to_string(),
        "primary failure"
    );

    let named = MixedFromError::from(SecondaryError);
    assert!(matches!(
        &named,
        MixedFromError::Named {
            source: SecondaryError
        }
    ));
    assert_eq!(
        StdError::source(&named).unwrap().to_string(),
        "secondary failure"
    );
}

#[test]
fn transparent_struct_delegates_display_and_source() {
    let inner = TransparentInner(LeafError);
    let inner_display = inner.to_string();
    let inner_source = StdError::source(&inner).map(|err| err.to_string());
    let wrapper = TransparentWrapper(inner);
    assert_eq!(wrapper.to_string(), inner_display);
    assert_eq!(
        StdError::source(&wrapper).map(|err| err.to_string()),
        inner_source
    );
}

#[test]
fn transparent_struct_from_impl() {
    let wrapper = TransparentFromWrapper::from(TransparentInner(LeafError));
    assert_eq!(wrapper.to_string(), "leaf failure");
    assert_eq!(
        StdError::source(&wrapper).map(|err| err.to_string()),
        Some(String::from("leaf failure"))
    );
}

#[test]
fn transparent_enum_variant_from_impl() {
    let _unused = TransparentEnum::Opaque("noop");
    let variant = TransparentEnum::from(TransparentInner(LeafError));
    assert!(matches!(variant, TransparentEnum::TransparentVariant(_)));
    assert_eq!(variant.to_string(), "leaf failure");
    assert_eq!(
        StdError::source(&variant).map(|err| err.to_string()),
        Some(String::from("leaf failure"))
    );
}

#[test]
fn struct_from_with_backtrace_field_captures_trace() {
    let err = StructFromWithBacktrace::from(LeafError);
    assert!(err.trace.is_some());
    assert_eq!(
        StdError::source(&err).map(|err| err.to_string()),
        Some(String::from("leaf failure"))
    );
}

#[test]
fn enum_from_with_backtrace_field_captures_trace() {
    let err = VariantFromWithBacktrace::from(LeafError);
    match &err {
        VariantFromWithBacktrace::WithTrace {
            trace, ..
        } => {
            assert!(trace.is_some());
        }
    }
    assert_eq!(
        StdError::source(&err).map(|err| err.to_string()),
        Some(String::from("leaf failure"))
    );
}
