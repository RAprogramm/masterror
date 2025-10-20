// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Field projection code generation for format arguments.
//!
//! This module handles the generation of field access chains (projections) for
//! format arguments. It supports field access by name, tuple indexing, and
//! method calls, allowing complex expressions like `self.field.0.method()` in
//! format templates.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Index;

use crate::input::{FormatArgProjectionMethodCall, FormatArgProjectionSegment};

/// Appends multiple projection segments to an expression.
///
/// This function chains together multiple field accesses, index operations,
/// and method calls to build a complete projection expression.
///
/// # Arguments
///
/// * `expr` - The base expression to project from
/// * `segments` - Slice of projection segments to apply in order
///
/// # Returns
///
/// Token stream representing the complete projection chain
///
/// # Examples
///
/// For segments `[field, 0, method()]` applied to `self`:
/// ```ignore
/// (((self).field).0).method()
/// ```
pub fn append_projection_segments(
    mut expr: TokenStream,
    segments: &[FormatArgProjectionSegment]
) -> TokenStream {
    for segment in segments {
        expr = append_projection_segment(expr, segment);
    }
    expr
}

/// Appends a single projection segment to an expression.
///
/// # Arguments
///
/// * `expr` - The expression to project from
/// * `segment` - The projection segment (field, index, or method call)
///
/// # Returns
///
/// Token stream with the projection segment applied
fn append_projection_segment(
    expr: TokenStream,
    segment: &FormatArgProjectionSegment
) -> TokenStream {
    match segment {
        FormatArgProjectionSegment::Field(ident) => quote!((#expr).#ident),
        FormatArgProjectionSegment::Index {
            index,
            span
        } => {
            let index_token = Index {
                index: *index as u32,
                span:  *span
            };
            quote!((#expr).#index_token)
        }
        FormatArgProjectionSegment::MethodCall(call) => append_method_call(expr, call)
    }
}

/// Appends a method call to an expression.
///
/// Handles both regular method calls and those with turbofish syntax (`::<T>`).
///
/// # Arguments
///
/// * `expr` - The expression to call the method on
/// * `call` - The method call specification including name, generics, and
///   arguments
///
/// # Returns
///
/// Token stream representing the method call expression
///
/// # Examples
///
/// Regular method call:
/// ```ignore
/// (expr).method(args)
/// ```
///
/// With turbofish:
/// ```ignore
/// (expr).method::<T>(args)
/// ```
pub fn append_method_call(expr: TokenStream, call: &FormatArgProjectionMethodCall) -> TokenStream {
    let method = &call.method;
    let args = &call.args;
    if let Some(turbofish) = &call.turbofish {
        let colon2 = turbofish.colon2_token;
        let generics = &turbofish.generics;
        quote!((#expr).#method #colon2 #generics (#args))
    } else {
        quote!((#expr).#method(#args))
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use quote::{format_ident, quote};
    use syn::{punctuated::Punctuated, token::Comma};

    use super::*;

    #[test]
    fn test_append_projection_segments_single_field() {
        let expr = quote!(self);
        let segments = vec![FormatArgProjectionSegment::Field(format_ident!("foo"))];
        let result = append_projection_segments(expr, &segments);
        let expected = quote!((self).foo);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_append_projection_segments_multiple_fields() {
        let expr = quote!(self);
        let segments = vec![
            FormatArgProjectionSegment::Field(format_ident!("foo")),
            FormatArgProjectionSegment::Field(format_ident!("bar")),
        ];
        let result = append_projection_segments(expr, &segments);
        let expected = quote!(((self).foo).bar);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_append_projection_segments_single_index() {
        let expr = quote!(self);
        let segments = vec![FormatArgProjectionSegment::Index {
            index: 0,
            span:  Span::call_site()
        }];
        let result = append_projection_segments(expr, &segments);
        let expected = quote!((self).0);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_append_projection_segments_multiple_indices() {
        let expr = quote!(self);
        let segments = vec![
            FormatArgProjectionSegment::Index {
                index: 0,
                span:  Span::call_site()
            },
            FormatArgProjectionSegment::Index {
                index: 1,
                span:  Span::call_site()
            },
        ];
        let result = append_projection_segments(expr, &segments);
        let expected = quote!(((self).0).1);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_append_projection_segments_field_then_index() {
        let expr = quote!(self);
        let segments = vec![
            FormatArgProjectionSegment::Field(format_ident!("foo")),
            FormatArgProjectionSegment::Index {
                index: 0,
                span:  Span::call_site()
            },
        ];
        let result = append_projection_segments(expr, &segments);
        let expected = quote!(((self).foo).0);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_append_projection_segments_index_then_field() {
        let expr = quote!(self);
        let segments = vec![
            FormatArgProjectionSegment::Index {
                index: 0,
                span:  Span::call_site()
            },
            FormatArgProjectionSegment::Field(format_ident!("bar")),
        ];
        let result = append_projection_segments(expr, &segments);
        let expected = quote!(((self).0).bar);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_append_projection_segments_method_call_no_args() {
        let expr = quote!(self);
        let segments = vec![FormatArgProjectionSegment::MethodCall(
            FormatArgProjectionMethodCall {
                method:    format_ident!("foo"),
                turbofish: None,
                args:      Punctuated::new(),
                span:      Span::call_site()
            }
        )];
        let result = append_projection_segments(expr, &segments);
        let expected = quote!((self).foo());
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_append_projection_segments_method_call_with_args() {
        let expr = quote!(self);
        let mut args = Punctuated::<syn::Expr, Comma>::new();
        args.push(syn::parse_quote!(42));
        args.push(syn::parse_quote!("test"));
        let segments = vec![FormatArgProjectionSegment::MethodCall(
            FormatArgProjectionMethodCall {
                method: format_ident!("process"),
                turbofish: None,
                args,
                span: Span::call_site()
            }
        )];
        let result = append_projection_segments(expr, &segments);
        let expected = quote!((self).process(42, "test"));
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_append_projection_segments_method_call_with_turbofish() {
        let expr = quote!(self);
        let turbofish = crate::input::FormatArgMethodTurbofish {
            colon2_token: syn::parse_quote!(::),
            generics:     syn::parse_quote!(<String>)
        };
        let segments = vec![FormatArgProjectionSegment::MethodCall(
            FormatArgProjectionMethodCall {
                method:    format_ident!("into"),
                turbofish: Some(turbofish),
                args:      Punctuated::new(),
                span:      Span::call_site()
            }
        )];
        let result = append_projection_segments(expr, &segments);
        let expected = quote!((self).into::<String>());
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_append_projection_segments_complex_chain() {
        let expr = quote!(self);
        let mut args = Punctuated::<syn::Expr, Comma>::new();
        args.push(syn::parse_quote!(10));
        let segments = vec![
            FormatArgProjectionSegment::Field(format_ident!("inner")),
            FormatArgProjectionSegment::Index {
                index: 0,
                span:  Span::call_site()
            },
            FormatArgProjectionSegment::MethodCall(FormatArgProjectionMethodCall {
                method: format_ident!("get"),
                turbofish: None,
                args,
                span: Span::call_site()
            }),
            FormatArgProjectionSegment::Field(format_ident!("value")),
        ];
        let result = append_projection_segments(expr, &segments);
        let expected = quote!(((((self).inner).0).get(10)).value);
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_append_projection_segments_empty() {
        let expr = quote!(self);
        let segments = vec![];
        let result = append_projection_segments(expr.clone(), &segments);
        assert_eq!(result.to_string(), expr.to_string());
    }

    #[test]
    fn test_append_method_call_simple() {
        let expr = quote!(value);
        let call = FormatArgProjectionMethodCall {
            method:    format_ident!("to_string"),
            turbofish: None,
            args:      Punctuated::new(),
            span:      Span::call_site()
        };
        let result = append_method_call(expr, &call);
        let expected = quote!((value).to_string());
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_append_method_call_with_turbofish_and_args() {
        let expr = quote!(value);
        let turbofish = crate::input::FormatArgMethodTurbofish {
            colon2_token: syn::parse_quote!(::),
            generics:     syn::parse_quote!(<i32>)
        };
        let mut args = Punctuated::<syn::Expr, Comma>::new();
        args.push(syn::parse_quote!(5));
        let call = FormatArgProjectionMethodCall {
            method: format_ident!("parse"),
            turbofish: Some(turbofish),
            args,
            span: Span::call_site()
        };
        let result = append_method_call(expr, &call);
        let expected = quote!((value).parse::<i32>(5));
        assert_eq!(result.to_string(), expected.to_string());
    }
}
