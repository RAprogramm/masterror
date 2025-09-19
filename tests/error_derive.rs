#![allow(unused_variables, non_shorthand_field_patterns)]

use std::error::Error as StdError;
#[cfg(error_generic_member_access)]
use std::ptr;

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

#[derive(Debug, Error)]
#[error("captured")]
struct StructWithBacktrace {
    #[backtrace]
    trace: std::backtrace::Backtrace
}

#[derive(Debug, Error)]
enum EnumWithBacktrace {
    #[error("tuple {0}")]
    Tuple(&'static str, #[backtrace] std::backtrace::Backtrace),
    #[error("named {message}")]
    Named {
        message: &'static str,
        #[backtrace]
        trace:   std::backtrace::Backtrace
    },
    #[error("unit")]
    Unit
}

#[derive(Debug, Error)]
#[error(
    "x={value:x} X={value:X} #x={value:#x} #X={value:#X} b={value:b} #b={value:#b} \
     o={value:o} #o={value:#o} e={float:e} #e={float:#e} E={float:E} #E={float:#E} \
     p={ptr:p} #p={ptr:#p}"
)]
struct FormatterShowcase {
    value: u32,
    float: f64,
    ptr:   *const u32
}

#[cfg(error_generic_member_access)]
fn assert_backtrace_interfaces<E>(error: &E, expected: &std::backtrace::Backtrace)
where
    E: StdError + ?Sized
{
    let reported = std::error::Error::backtrace(error).expect("backtrace");
    assert!(ptr::eq(expected, reported));
    let provided =
        std::error::request_ref::<std::backtrace::Backtrace>(error).expect("provided backtrace");
    assert!(ptr::eq(reported, provided));
}

#[cfg(not(error_generic_member_access))]
fn assert_backtrace_interfaces<E>(_error: &E, _expected: &std::backtrace::Backtrace)
where
    E: StdError + ?Sized
{
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
    let stored = err.trace.as_ref().expect("trace stored");
    assert_backtrace_interfaces(&err, stored);
    assert_eq!(
        StdError::source(&err).map(|err| err.to_string()),
        Some(String::from("leaf failure"))
    );
}

#[test]
fn enum_from_with_backtrace_field_captures_trace() {
    let err = VariantFromWithBacktrace::from(LeafError);
    let trace = match &err {
        VariantFromWithBacktrace::WithTrace {
            trace, ..
        } => {
            assert!(trace.is_some());
            trace.as_ref().unwrap()
        }
    };
    assert_backtrace_interfaces(&err, trace);
    assert_eq!(
        StdError::source(&err).map(|err| err.to_string()),
        Some(String::from("leaf failure"))
    );
}

#[test]
fn struct_backtrace_field_is_returned() {
    let err = StructWithBacktrace {
        trace: std::backtrace::Backtrace::capture()
    };
    assert_backtrace_interfaces(&err, &err.trace);
    assert!(StdError::source(&err).is_none());
}

#[test]
fn enum_backtrace_field_is_returned() {
    let tuple = EnumWithBacktrace::Tuple("tuple", std::backtrace::Backtrace::capture());
    if let EnumWithBacktrace::Tuple(_, trace) = &tuple {
        assert_backtrace_interfaces(&tuple, trace);
    }

    let named = EnumWithBacktrace::Named {
        message: "named",
        trace:   std::backtrace::Backtrace::capture()
    };
    if let EnumWithBacktrace::Named {
        trace, ..
    } = &named
    {
        assert_backtrace_interfaces(&named, trace);
    }

    let unit = EnumWithBacktrace::Unit;
    #[cfg(error_generic_member_access)]
    {
        assert!(std::error::Error::backtrace(&unit).is_none());
    }
}

#[test]
fn supports_extended_formatters() {
    let value = 0x5A5Au32;
    let float = 1234.5_f64;
    let ptr = core::ptr::null::<u32>();

    let err = FormatterShowcase {
        value,
        float,
        ptr
    };

    let expected = format!(
        "x={value:x} X={value:X} #x={value:#x} #X={value:#X} b={value:b} #b={value:#b} \
         o={value:o} #o={value:#o} e={float:e} #e={float:#e} E={float:E} #E={float:#E} \
         p={ptr:p} #p={ptr:#p}"
    );

    assert_eq!(err.to_string(), expected);
    assert!(StdError::source(&err).is_none());
}
