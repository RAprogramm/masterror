#![allow(unused_variables, non_shorthand_field_patterns)]

#[cfg(error_generic_member_access)]
use std::ptr;
use std::{error::Error as StdError, fmt};

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
#[error("{source:?}")]
struct DelegatedBacktraceFromSource {
    #[from]
    #[source]
    #[backtrace]
    source: StructWithBacktrace
}

#[derive(Debug, Error)]
#[error("{source:?}")]
struct OptionalDelegatedBacktrace {
    #[source]
    #[backtrace]
    source: Option<StructWithBacktrace>
}

#[derive(Debug, Error)]
#[error("auto {source}")]
struct AutoSourceStruct {
    source: LeafError
}

#[derive(Debug, Error)]
enum AutoSourceEnum {
    #[error("named {source}")]
    Named { source: LeafError }
}

#[derive(Debug, Error)]
#[error("captured")]
struct AutoBacktraceStruct {
    trace: std::backtrace::Backtrace
}

#[derive(Debug, Error)]
#[error("optional")]
struct AutoOptionalBacktraceStruct {
    trace: Option<std::backtrace::Backtrace>
}

#[derive(Debug, Error)]
enum AutoBacktraceEnum {
    #[error("named {message}")]
    Named {
        message: &'static str,
        trace:   std::backtrace::Backtrace
    },
    #[error("tuple {0:?}")]
    Tuple(Option<std::backtrace::Backtrace>)
}

#[derive(Debug, Error)]
#[error(
    "display={value} debug={value:?} #debug={value:#?} x={value:x} X={value:X} \
     #x={value:#x} #X={value:#X} b={value:b} #b={value:#b} o={value:o} #o={value:#o} \
     e={float:e} #e={float:#e} E={float:E} #E={float:#E} p={ptr:p} #p={ptr:#p}"
)]
struct FormatterShowcase {
    value: u32,
    float: f64,
    ptr:   *const u32
}

#[derive(Debug)]
struct PrettyDebugValue {
    label: &'static str
}

impl fmt::Display for PrettyDebugValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label)
    }
}

#[derive(Debug, Error)]
#[error("display={value} debug={value:?} #debug={value:#?} tuple={tuple:?} #tuple={tuple:#?}")]
struct FormatterDebugShowcase {
    value: PrettyDebugValue,
    tuple: (&'static str, u8)
}

#[derive(Debug, Error)]
#[error("{formatted}", formatted = self.message.to_uppercase())]
struct FormatArgExpressionError {
    message: &'static str
}

#[derive(Debug, Error)]
#[error("{}, {label}, {}", label = self.label, self.first, self.second)]
struct MixedImplicitArgsError {
    label:  &'static str,
    first:  &'static str,
    second: &'static str
}

#[derive(Debug, Error)]
enum FormatArgEnum {
    #[error("{detail}", detail = detail.to_uppercase())]
    Upper { detail: String }
}

#[derive(Debug, Error)]
#[error("{1}::{0}", self.first, self.second)]
struct ExplicitIndexArgsError {
    first:  &'static str,
    second: &'static str
}

#[derive(Debug, Error)]
#[error("{0}::{label}", label = self.label, self.value)]
struct MixedNamedPositionalArgsError {
    label: &'static str,
    value: &'static str
}

#[derive(Debug, Error)]
#[error("{value}")]
struct DisplayFormatterError {
    value: &'static str
}

#[derive(Debug, Error)]
#[error("debug={value:?} #debug={value:#?}")]
struct DebugFormatterError {
    value: PrettyDebugValue
}

#[derive(Debug, Error)]
#[error("lower={value:x} #lower={value:#x}")]
struct LowerHexFormatterError {
    value: u32
}

#[derive(Debug, Error)]
#[error("upper={value:X} #upper={value:#X}")]
struct UpperHexFormatterError {
    value: u32
}

#[derive(Debug, Error)]
#[error("binary={value:b} #binary={value:#b}")]
struct BinaryFormatterError {
    value: u16
}

#[derive(Debug, Error)]
#[error("octal={value:o} #octal={value:#o}")]
struct OctalFormatterError {
    value: u16
}

#[derive(Debug, Error)]
#[error("pointer={value:p} #pointer={value:#p}")]
struct PointerFormatterError {
    value: *const u32
}

#[derive(Debug, Error)]
#[error("lower={value:e} #lower={value:#e}")]
struct LowerExpFormatterError {
    value: f64
}

#[derive(Debug, Error)]
#[error("upper={value:E} #upper={value:#E}")]
struct UpperExpFormatterError {
    value: f64
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
fn named_format_arg_expression_is_used() {
    let err = FormatArgExpressionError {
        message: "value"
    };
    assert_eq!(err.to_string(), "VALUE");
}

#[test]
fn implicit_format_args_follow_positional_ordering() {
    let err = MixedImplicitArgsError {
        label:  "tag",
        first:  "one",
        second: "two"
    };
    assert_eq!(err.to_string(), "one, tag, two");
}

#[test]
fn explicit_format_arg_indices_resolve() {
    let err = ExplicitIndexArgsError {
        first:  "left",
        second: "right"
    };
    assert_eq!(err.to_string(), "right::left");
}

#[test]
fn mixed_named_and_positional_indices_resolve() {
    let err = MixedNamedPositionalArgsError {
        label: "tag",
        value: "item"
    };
    assert_eq!(err.to_string(), "item::tag");
}

#[test]
fn enum_variant_format_args_resolve_bindings() {
    let err = FormatArgEnum::Upper {
        detail: String::from("variant")
    };
    assert_eq!(err.to_string(), "VARIANT");
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
fn struct_backtrace_attribute_on_source_delegates() {
    let source = StructWithBacktrace {
        trace: std::backtrace::Backtrace::capture()
    };
    let err = DelegatedBacktraceFromSource::from(source);
    let inner = StdError::source(&err)
        .and_then(|source| source.downcast_ref::<StructWithBacktrace>())
        .expect("delegated source");
    assert_backtrace_interfaces(&err, &inner.trace);
}

#[test]
fn optional_source_backtrace_attribute_delegates() {
    let err = OptionalDelegatedBacktrace {
        source: Some(StructWithBacktrace {
            trace: std::backtrace::Backtrace::capture()
        })
    };
    let inner = StdError::source(&err)
        .and_then(|source| source.downcast_ref::<StructWithBacktrace>())
        .expect("optional delegated source");
    assert_backtrace_interfaces(&err, &inner.trace);
}

#[test]
fn optional_source_backtrace_absent_when_none() {
    let err = OptionalDelegatedBacktrace {
        source: None
    };
    assert!(StdError::source(&err).is_none());
    #[cfg(error_generic_member_access)]
    {
        assert!(std::error::Error::backtrace(&err).is_none());
        assert!(std::error::request_ref::<std::backtrace::Backtrace>(&err).is_none());
    }
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
fn supports_display_and_debug_formatters() {
    let value = PrettyDebugValue {
        label: "Alpha"
    };
    let tuple = ("tuple", 7u8);

    let expected = format!(
        "display={value} debug={value:?} #debug={value:#?} tuple={tuple:?} #tuple={tuple:#?}",
    );

    let standard_debug = format!("{value:?}");
    let alternate_debug = format!("{value:#?}");
    assert_ne!(standard_debug, alternate_debug);

    let tuple_debug = format!("{tuple:?}");
    let tuple_alternate_debug = format!("{tuple:#?}");
    assert_ne!(tuple_debug, tuple_alternate_debug);

    let err = FormatterDebugShowcase {
        value,
        tuple
    };

    assert_eq!(err.to_string(), expected);
    assert!(StdError::source(&err).is_none());
}

#[test]
fn struct_named_source_is_inferred() {
    let err = AutoSourceStruct {
        source: LeafError
    };
    assert_eq!(err.to_string(), "auto leaf failure");
    let source = StdError::source(&err).expect("source");
    assert_eq!(source.to_string(), "leaf failure");
}

#[test]
fn enum_named_source_is_inferred() {
    let err = AutoSourceEnum::Named {
        source: LeafError
    };
    assert_eq!(err.to_string(), "named leaf failure");
    let source = StdError::source(&err).expect("source");
    assert_eq!(source.to_string(), "leaf failure");
}

#[test]
fn struct_backtrace_is_inferred_without_attribute() {
    let err = AutoBacktraceStruct {
        trace: std::backtrace::Backtrace::capture()
    };
    assert_backtrace_interfaces(&err, &err.trace);
    assert!(StdError::source(&err).is_none());
}

#[test]
fn struct_optional_backtrace_is_inferred_without_attribute() {
    let err = AutoOptionalBacktraceStruct {
        trace: Some(std::backtrace::Backtrace::capture())
    };
    let stored = err.trace.as_ref().expect("trace stored");
    assert_backtrace_interfaces(&err, stored);
    assert!(StdError::source(&err).is_none());
}

#[test]
fn enum_backtrace_is_inferred_without_attribute() {
    let named = AutoBacktraceEnum::Named {
        message: "named",
        trace:   std::backtrace::Backtrace::capture()
    };
    if let AutoBacktraceEnum::Named {
        trace, ..
    } = &named
    {
        assert_backtrace_interfaces(&named, trace);
    }
    assert!(StdError::source(&named).is_none());

    let tuple = AutoBacktraceEnum::Tuple(Some(std::backtrace::Backtrace::capture()));
    if let AutoBacktraceEnum::Tuple(Some(trace)) = &tuple {
        assert_backtrace_interfaces(&tuple, trace);
    }
    assert!(StdError::source(&tuple).is_none());

    #[cfg(error_generic_member_access)]
    {
        let none = AutoBacktraceEnum::Tuple(None);
        assert!(std::error::Error::backtrace(&none).is_none());
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
        "display={value} debug={value:?} #debug={value:#?} x={value:x} X={value:X} \
         #x={value:#x} #X={value:#X} b={value:b} #b={value:#b} o={value:o} #o={value:#o} \
         e={float:e} #e={float:#e} E={float:E} #E={float:#E} p={ptr:p} #p={ptr:#p}"
    );

    let lower_hex = format!("{value:x}");
    let upper_hex = format!("{value:X}");
    assert_ne!(lower_hex, upper_hex);

    let lower_exp = format!("{float:e}");
    let upper_exp = format!("{float:E}");
    assert_ne!(lower_exp, upper_exp);

    assert_eq!(err.to_string(), expected);
    assert!(StdError::source(&err).is_none());
}

#[test]
fn formatter_variants_render_expected_output() {
    let display = DisplayFormatterError {
        value: "display"
    };
    assert_eq!(display.to_string(), "display");

    let debug = DebugFormatterError {
        value: PrettyDebugValue {
            label: "Debug"
        }
    };
    let debug_expected = format!(
        "debug={value:?} #debug={value:#?}",
        value = PrettyDebugValue {
            label: "Debug"
        }
    );
    assert_eq!(debug.to_string(), debug_expected);
    assert_ne!(
        format!(
            "{value:?}",
            value = PrettyDebugValue {
                label: "Debug"
            }
        ),
        format!(
            "{value:#?}",
            value = PrettyDebugValue {
                label: "Debug"
            }
        )
    );

    const HEX_VALUE: u32 = 0x5A5A;
    let lower_hex = LowerHexFormatterError {
        value: HEX_VALUE
    };
    let lower_hex_expected = format!("lower={value:x} #lower={value:#x}", value = HEX_VALUE);
    assert_eq!(lower_hex.to_string(), lower_hex_expected);
    assert_ne!(format!("{HEX_VALUE:x}"), format!("{HEX_VALUE:#x}"));

    let upper_hex = UpperHexFormatterError {
        value: HEX_VALUE
    };
    let upper_hex_expected = format!("upper={value:X} #upper={value:#X}", value = HEX_VALUE);
    assert_eq!(upper_hex.to_string(), upper_hex_expected);
    assert_ne!(format!("{HEX_VALUE:X}"), format!("{HEX_VALUE:#X}"));
    assert_ne!(format!("{HEX_VALUE:x}"), format!("{HEX_VALUE:X}"));

    const INTEGER_VALUE: u16 = 0b1010_1100;
    let binary = BinaryFormatterError {
        value: INTEGER_VALUE
    };
    let binary_expected = format!("binary={value:b} #binary={value:#b}", value = INTEGER_VALUE);
    assert_eq!(binary.to_string(), binary_expected);
    assert_ne!(format!("{INTEGER_VALUE:b}"), format!("{INTEGER_VALUE:#b}"));

    let octal = OctalFormatterError {
        value: INTEGER_VALUE
    };
    let octal_expected = format!("octal={value:o} #octal={value:#o}", value = INTEGER_VALUE);
    assert_eq!(octal.to_string(), octal_expected);
    assert_ne!(format!("{INTEGER_VALUE:o}"), format!("{INTEGER_VALUE:#o}"));

    let pointer_value = core::ptr::null::<u32>();
    let pointer = PointerFormatterError {
        value: pointer_value
    };
    let pointer_expected = format!(
        "pointer={value:p} #pointer={value:#p}",
        value = pointer_value
    );
    assert_eq!(pointer.to_string(), pointer_expected);
    assert_ne!(format!("{pointer_value:p}"), format!("{pointer_value:#p}"));

    const FLOAT_VALUE: f64 = 1234.5;
    let lower_exp = LowerExpFormatterError {
        value: FLOAT_VALUE
    };
    let lower_exp_expected = format!("lower={value:e} #lower={value:#e}", value = FLOAT_VALUE);
    assert_eq!(lower_exp.to_string(), lower_exp_expected);

    let upper_exp = UpperExpFormatterError {
        value: FLOAT_VALUE
    };
    let upper_exp_expected = format!("upper={value:E} #upper={value:#E}", value = FLOAT_VALUE);
    assert_eq!(upper_exp.to_string(), upper_exp_expected);
    assert_ne!(format!("{FLOAT_VALUE:e}"), format!("{FLOAT_VALUE:E}"));
}
