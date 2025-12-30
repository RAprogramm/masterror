// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Error attachment and metadata generation.
//!
//! This module provides functionality for generating code that attaches
//! additional context and metadata to errors during conversion:
//!
//! - Source error chain attachment (#[source] fields)
//! - Backtrace capture and attachment
//! - Telemetry metadata (spans, events, custom fields)
//! - Field redaction policies (message, field-level)
//!
//! The attachment system respects type safety and handles both owned and
//! Arc-wrapped error sources.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

use super::binding::BoundField;
use crate::input::{
    FieldRedactionKind, FieldRedactionSpec, Fields, RedactSpec, is_arc_type, is_option_type,
    option_inner_type
};

/// Generates source error attachment tokens.
///
/// Searches bound fields for a field marked with `#[source]` and generates
/// code to attach it to the error. Handles both owned and Arc-wrapped sources,
/// as well as optional sources.
///
/// # Arguments
///
/// * `bound_fields` - List of bound fields to search
///
/// # Returns
///
/// A `TokenStream` containing the source attachment code, or empty if no
/// source.
///
/// # Examples
///
/// ```ignore
/// // For owned source:
/// struct Error {
///     #[source]
///     inner: std::io::Error,
/// }
/// // Generates: __masterror_error = __masterror_error.with_source(inner);
///
/// // For Arc-wrapped source:
/// struct Error {
///     #[source]
///     inner: Arc<dyn std::error::Error>,
/// }
/// // Generates: __masterror_error = __masterror_error.with_source_arc(inner);
///
/// // For optional source:
/// struct Error {
///     #[source]
///     inner: Option<std::io::Error>,
/// }
/// // Generates:
/// // if let Some(source) = inner {
/// //     __masterror_error = __masterror_error.with_source(source);
/// // }
/// ```
pub fn source_attachment_tokens(bound_fields: &[BoundField<'_>]) -> TokenStream {
    for bound in bound_fields {
        if bound.field.attrs.has_source() {
            let binding = &bound.binding;
            let ty = &bound.field.ty;
            if is_option_type(ty) {
                let arc_inner = option_inner_type(ty).is_some_and(is_arc_type);
                if arc_inner {
                    return quote! {
                        if let Some(source) = #binding {
                            __masterror_error = __masterror_error.with_source_arc(source);
                        }
                    };
                }
                return quote! {
                    if let Some(source) = #binding {
                        __masterror_error = __masterror_error.with_source(source);
                    }
                };
            } else {
                if is_arc_type(ty) {
                    return quote! {
                        __masterror_error = __masterror_error.with_source_arc(#binding);
                    };
                }
                return quote! {
                    __masterror_error = __masterror_error.with_source(#binding);
                };
            }
        }
    }
    TokenStream::new()
}

/// Generates backtrace attachment tokens.
///
/// Searches for a backtrace field (marked with `#[backtrace]` or named
/// `backtrace`) and generates code to attach it to the error. Handles both
/// required and optional backtraces.
///
/// # Arguments
///
/// * `fields` - The complete field set to search
/// * `bound_fields` - List of bound fields with their bindings
///
/// # Returns
///
/// A `TokenStream` containing the backtrace attachment code, or empty if none.
///
/// # Examples
///
/// ```ignore
/// // For required backtrace:
/// struct Error {
///     backtrace: std::backtrace::Backtrace,
/// }
/// // Generates: __masterror_error = __masterror_error.with_backtrace(backtrace);
///
/// // For optional backtrace:
/// struct Error {
///     backtrace: Option<std::backtrace::Backtrace>,
/// }
/// // Generates:
/// // if let Some(trace) = backtrace {
/// //     __masterror_error = __masterror_error.with_backtrace(trace);
/// // }
/// ```
pub fn backtrace_attachment_tokens(
    fields: &Fields,
    bound_fields: &[BoundField<'_>]
) -> TokenStream {
    let Some(backtrace_field) = fields.backtrace_field() else {
        return TokenStream::new();
    };
    let index = backtrace_field.index();
    let Some(binding) = bound_fields
        .iter()
        .find(|bound| bound.field.index == index)
        .map(|bound| &bound.binding)
    else {
        return TokenStream::new();
    };
    if is_option_type(&backtrace_field.field().ty) {
        quote! {
            if let Some(trace) = #binding {
                __masterror_error = __masterror_error.with_backtrace(trace);
            }
        }
    } else {
        quote! {
            __masterror_error = __masterror_error.with_backtrace(#binding);
        }
    }
}

/// Generates telemetry metadata initialization tokens.
///
/// Creates code to build a metadata object from configured telemetry entries.
/// Each entry is an optional field that's inserted if present.
///
/// # Arguments
///
/// * `entries` - List of telemetry field expressions
///
/// # Returns
///
/// A `TokenStream` initializing the `__masterror_metadata` variable.
///
/// # Examples
///
/// ```ignore
/// // For telemetry = [trace_id(), span_id()]:
/// // Generates:
/// let mut __masterror_metadata_inner = masterror::Metadata::new();
/// if let Some(field) = (trace_id()) {
///     __masterror_metadata_inner.insert(field);
/// }
/// if let Some(field) = (span_id()) {
///     __masterror_metadata_inner.insert(field);
/// }
/// let __masterror_metadata = if __masterror_metadata_inner.is_empty() {
///     None
/// } else {
///     Some(__masterror_metadata_inner)
/// };
///
/// // For no telemetry:
/// // Generates: let __masterror_metadata: Option<masterror::Metadata> = None;
/// ```
pub fn telemetry_initialization(entries: &[Expr]) -> TokenStream {
    if entries.is_empty() {
        quote!(let __masterror_metadata: Option<masterror::Metadata> = None;)
    } else {
        let inserts = entries.iter().map(|expr| {
            quote! {
                if let Some(field) = (#expr) {
                    __masterror_metadata_inner.insert(field);
                }
            }
        });
        quote! {
            let mut __masterror_metadata_inner = masterror::Metadata::new();
            #(#inserts)*
            let __masterror_metadata = if __masterror_metadata_inner.is_empty() {
                None
            } else {
                Some(__masterror_metadata_inner)
            };
        }
    }
}

/// Generates metadata attachment tokens.
///
/// Creates code to attach the metadata object to the error if it's not empty.
///
/// # Returns
///
/// A `TokenStream` that conditionally attaches metadata.
///
/// # Examples
///
/// ```ignore
/// // Generates:
/// if let Some(metadata) = __masterror_metadata {
///     __masterror_error = __masterror_error.with_metadata(metadata);
/// }
/// ```
pub fn metadata_attach_tokens() -> TokenStream {
    quote! {
        if let Some(metadata) = __masterror_metadata {
            __masterror_error = __masterror_error.with_metadata(metadata);
        }
    }
}

/// Generates redaction policy application tokens.
///
/// Creates code to apply redaction policies to the error message and specific
/// fields based on the redaction specification.
///
/// # Arguments
///
/// * `spec` - The redaction specification with message and field policies
///
/// # Returns
///
/// A `TokenStream` applying all redaction policies.
///
/// # Examples
///
/// ```ignore
/// // For redact(message, fields = [("password", Hash)]):
/// // Generates:
/// __masterror_error = __masterror_error.redactable();
/// __masterror_error = __masterror_error.redact_field("password", masterror::FieldRedaction::Hash);
/// ```
pub fn redact_tokens(spec: &RedactSpec) -> TokenStream {
    let message = if spec.message {
        quote!(
            __masterror_error = __masterror_error.redactable();
        )
    } else {
        TokenStream::new()
    };
    let field_updates = spec.fields.iter().map(|field_spec: &FieldRedactionSpec| {
        let name = &field_spec.name;
        let policy = field_redaction_tokens(field_spec.policy);
        quote!(
            __masterror_error = __masterror_error.redact_field(#name, #policy);
        )
    });
    quote! {
        #message
        #( #field_updates )*
    }
}

/// Generates field redaction policy tokens.
///
/// Converts a field redaction kind to its corresponding enum variant.
///
/// # Arguments
///
/// * `kind` - The redaction policy kind
///
/// # Returns
///
/// A `TokenStream` containing the `FieldRedaction` enum variant.
///
/// # Examples
///
/// ```ignore
/// // FieldRedactionKind::Hash -> masterror::FieldRedaction::Hash
/// // FieldRedactionKind::Last4 -> masterror::FieldRedaction::Last4
/// ```
fn field_redaction_tokens(kind: FieldRedactionKind) -> TokenStream {
    match kind {
        FieldRedactionKind::None => quote!(masterror::FieldRedaction::None),
        FieldRedactionKind::Redact => quote!(masterror::FieldRedaction::Redact),
        FieldRedactionKind::Hash => quote!(masterror::FieldRedaction::Hash),
        FieldRedactionKind::Last4 => quote!(masterror::FieldRedaction::Last4)
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn test_telemetry_initialization_empty() {
        let entries = vec![];
        let result = telemetry_initialization(&entries);
        let expected = quote!(let __masterror_metadata: Option<masterror::Metadata> = None;);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_telemetry_initialization_with_entries() {
        use syn::parse_quote;
        let entries: Vec<Expr> = vec![parse_quote!(trace_id()), parse_quote!(span_id())];
        let result = telemetry_initialization(&entries);
        let result_str = result.to_string();
        assert!(result_str.contains("__masterror_metadata_inner"));
        assert!(result_str.contains("trace_id"));
        assert!(result_str.contains("span_id"));
        assert!(result_str.contains("is_empty"));
    }

    #[test]
    fn test_metadata_attach_tokens() {
        let result = metadata_attach_tokens();
        let result_str = result.to_string();
        assert!(result_str.contains("if let Some"));
        assert!(result_str.contains("metadata"));
        assert!(result_str.contains("with_metadata"));
    }

    #[test]
    fn test_field_redaction_tokens_all_variants() {
        let none = field_redaction_tokens(FieldRedactionKind::None);
        assert!(none.to_string().contains("None"));
        let redact = field_redaction_tokens(FieldRedactionKind::Redact);
        assert!(redact.to_string().contains("Redact"));
        let hash = field_redaction_tokens(FieldRedactionKind::Hash);
        assert!(hash.to_string().contains("Hash"));
        let last4 = field_redaction_tokens(FieldRedactionKind::Last4);
        assert!(last4.to_string().contains("Last4"));
    }

    #[test]
    fn test_redact_tokens_message_only() {
        use crate::input::RedactSpec;
        let spec = RedactSpec {
            message: true,
            fields:  vec![]
        };
        let result = redact_tokens(&spec);
        let result_str = result.to_string();
        assert!(result_str.contains("redactable"));
    }

    #[test]
    fn test_redact_tokens_with_fields() {
        use syn::LitStr;

        use crate::input::{FieldRedactionSpec, RedactSpec};
        let spec = RedactSpec {
            message: false,
            fields:  vec![FieldRedactionSpec {
                name:   LitStr::new("password", proc_macro2::Span::call_site()),
                policy: FieldRedactionKind::Hash
            }]
        };
        let result = redact_tokens(&spec);
        let result_str = result.to_string();
        assert!(result_str.contains("password"));
        assert!(result_str.contains("Hash"));
        assert!(result_str.contains("redact_field"));
    }

    #[test]
    fn test_source_attachment_tokens_empty() {
        let bound_fields = vec![];
        let result = source_attachment_tokens(&bound_fields);
        assert!(result.is_empty());
    }

    #[test]
    fn test_backtrace_attachment_tokens_none() {
        let fields = Fields::Unit;
        let bound_fields = vec![];
        let result = backtrace_attachment_tokens(&fields, &bound_fields);
        assert!(result.is_empty());
    }

    #[test]
    fn test_source_attachment_arc() {
        use proc_macro2::Span;
        use quote::format_ident;
        use syn::parse_quote;

        use crate::input::{Field, FieldAttrs};
        let mut attrs = FieldAttrs::default();
        attrs.source = Some(parse_quote!(#[source]));
        let field = Field {
            ident: Some(format_ident!("inner")),
            member: syn::Member::Named(format_ident!("inner")),
            ty: parse_quote!(std::sync::Arc<dyn std::error::Error>),
            index: 0,
            attrs,
            span: Span::call_site()
        };
        let binding = format_ident!("inner");
        let bound = vec![BoundField {
            field: &field,
            binding
        }];
        let result = source_attachment_tokens(&bound);
        let result_str = result.to_string();
        assert!(result_str.contains("with_source_arc"));
    }

    #[test]
    fn test_source_attachment_option_arc() {
        use proc_macro2::Span;
        use quote::format_ident;
        use syn::parse_quote;

        use crate::input::{Field, FieldAttrs};
        let mut attrs = FieldAttrs::default();
        attrs.source = Some(parse_quote!(#[source]));
        let field = Field {
            ident: Some(format_ident!("inner")),
            member: syn::Member::Named(format_ident!("inner")),
            ty: parse_quote!(Option<std::sync::Arc<dyn std::error::Error>>),
            index: 0,
            attrs,
            span: Span::call_site()
        };
        let binding = format_ident!("inner");
        let bound = vec![BoundField {
            field: &field,
            binding
        }];
        let result = source_attachment_tokens(&bound);
        let result_str = result.to_string();
        assert!(result_str.contains("with_source_arc"));
        assert!(result_str.contains("if let Some"));
    }

    #[test]
    fn test_field_redaction_tokens_all_kinds() {
        let result_none = field_redaction_tokens(FieldRedactionKind::None);
        assert!(result_none.to_string().contains("None"));
        let result_redact = field_redaction_tokens(FieldRedactionKind::Redact);
        assert!(result_redact.to_string().contains("Redact"));
        let result_hash = field_redaction_tokens(FieldRedactionKind::Hash);
        assert!(result_hash.to_string().contains("Hash"));
        let result_last4 = field_redaction_tokens(FieldRedactionKind::Last4);
        assert!(result_last4.to_string().contains("Last4"));
    }
}
