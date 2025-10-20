// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Format argument parsing for error display templates.
//!
//! Handles parsing of format arguments in `#[error(...)]` attributes,
//! including projections, method calls, and turbofish syntax.

use std::collections::HashSet;

use proc_macro2::Span;
use syn::{
    AngleBracketedGenericArguments, Error, Expr, Ident, LitInt, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Paren
};

use super::types::{
    FormatArg, FormatArgMethodTurbofish, FormatArgProjection, FormatArgProjectionMethodCall,
    FormatArgProjectionSegment, FormatArgShorthand, FormatArgValue, FormatArgsSpec,
    FormatBindingKind, MethodCallSuffix
};

/// Parses format arguments from input stream.
///
/// Handles both named and positional arguments with optional leading comma.
pub(crate) fn parse_format_args(input: ParseStream) -> Result<FormatArgsSpec, Error> {
    let mut args = FormatArgsSpec::default();

    if input.is_empty() {
        return Ok(args);
    }

    let leading_comma = if input.peek(Token![,]) {
        let comma: Token![,] = input.parse()?;
        Some(comma.span)
    } else {
        None
    };

    if input.is_empty() {
        if let Some(span) = leading_comma {
            return Err(Error::new(span, "expected format argument after comma"));
        }
        return Ok(args);
    }

    let parsed = syn::punctuated::Punctuated::<RawFormatArg, Token![,]>::parse_terminated(input)?;

    let mut seen_named = HashSet::new();

    let mut positional_index = 0usize;

    for raw in parsed {
        match raw {
            RawFormatArg::Named {
                ident,
                value,
                span
            } => {
                let name_key = ident.to_string();
                if !seen_named.insert(name_key) {
                    return Err(Error::new(
                        ident.span(),
                        format!("duplicate format argument `{ident}`")
                    ));
                }

                args.args.push(FormatArg {
                    value,
                    kind: FormatBindingKind::Named(ident),
                    span
                });
            }
            RawFormatArg::Positional {
                value,
                span
            } => {
                let index = positional_index;
                positional_index += 1;
                args.args.push(FormatArg {
                    value,
                    kind: FormatBindingKind::Positional(index),
                    span
                });
            }
        }
    }

    Ok(args)
}

/// Raw format argument (named or positional).
pub(crate) enum RawFormatArg {
    Named {
        ident: Ident,
        value: FormatArgValue,
        span:  Span
    },
    Positional {
        value: FormatArgValue,
        span:  Span
    }
}

impl Parse for RawFormatArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) && input.peek2(Token![=]) {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value = parse_format_arg_value(input)?;
            let value_span = format_arg_value_span(&value);
            let span = ident.span().join(value_span).unwrap_or(value_span);
            Ok(Self::Named {
                ident,
                value,
                span
            })
        } else {
            let value = parse_format_arg_value(input)?;
            let span = format_arg_value_span(&value);
            Ok(Self::Positional {
                value,
                span
            })
        }
    }
}

/// Parses format argument value (expression or projection shorthand).
fn parse_format_arg_value(input: ParseStream) -> syn::Result<FormatArgValue> {
    if input.peek(Token![.]) {
        let dot: Token![.] = input.parse()?;
        let projection = parse_projection_segments(input, dot.span)?;
        Ok(FormatArgValue::Shorthand(FormatArgShorthand::Projection(
            projection
        )))
    } else {
        let expr: Expr = input.parse()?;
        Ok(FormatArgValue::Expr(expr))
    }
}

/// Parses projection segments (field accesses, indexing, method calls).
fn parse_projection_segments(
    input: ParseStream,
    dot_span: Span
) -> syn::Result<FormatArgProjection> {
    let first = parse_projection_segment(input, true)?;
    let mut segments = vec![first];

    while input.peek(Token![.]) {
        input.parse::<Token![.]>()?;
        segments.push(parse_projection_segment(input, false)?);
    }

    let mut span = join_spans(dot_span, segments[0].span());
    for segment in segments.iter().skip(1) {
        span = join_spans(span, segment.span());
    }

    Ok(FormatArgProjection {
        segments,
        span
    })
}

/// Parses single projection segment.
fn parse_projection_segment(
    input: ParseStream,
    first: bool
) -> syn::Result<FormatArgProjectionSegment> {
    if input.peek(LitInt) {
        let literal: LitInt = input.parse()?;
        let index = literal.base10_parse::<usize>()?;
        return Ok(FormatArgProjectionSegment::Index {
            index,
            span: literal.span()
        });
    }

    if input.peek(Ident) {
        let ident: Ident = input.parse()?;
        if let Some((turbofish, paren_token, args)) = parse_method_call_suffix(input)? {
            let span = method_call_span(&ident, turbofish.as_ref(), &paren_token);
            return Ok(FormatArgProjectionSegment::MethodCall(
                FormatArgProjectionMethodCall {
                    method: ident,
                    turbofish,
                    args,
                    span
                }
            ));
        }

        return Ok(FormatArgProjectionSegment::Field(ident));
    }

    let span = input.span();
    if first {
        Err(syn::Error::new(
            span,
            "expected field, index, or method call after `.`"
        ))
    } else {
        Err(syn::Error::new(
            span,
            "expected field, index, or method call in projection"
        ))
    }
}

/// Parses method call suffix with optional turbofish.
fn parse_method_call_suffix(input: ParseStream) -> syn::Result<MethodCallSuffix> {
    let ahead = input.fork();

    let has_turbofish = ahead.peek(Token![::]);
    if has_turbofish {
        let _: Token![::] = ahead.parse()?;
        let _: AngleBracketedGenericArguments = ahead.parse()?;
    }

    if !ahead.peek(Paren) {
        return Ok(None);
    }

    let turbofish = if has_turbofish {
        let colon2_token = input.parse::<Token![::]>()?;
        let generics = input.parse::<AngleBracketedGenericArguments>()?;
        Some(FormatArgMethodTurbofish {
            colon2_token,
            generics
        })
    } else {
        None
    };

    let content;
    let paren_token = syn::parenthesized!(content in input);
    let args = Punctuated::<Expr, Token![,]>::parse_terminated(&content)?;

    Ok(Some((turbofish, paren_token, args)))
}

/// Computes span for method call.
fn method_call_span(
    method: &Ident,
    turbofish: Option<&FormatArgMethodTurbofish>,
    paren_token: &Paren
) -> Span {
    let mut span = method.span();
    if let Some(turbofish) = turbofish {
        span = join_spans(span, turbofish.generics.gt_token.span);
    }
    join_spans(span, paren_token.span.close())
}

/// Joins two spans into one.
fn join_spans(lhs: Span, rhs: Span) -> Span {
    lhs.join(rhs).unwrap_or(lhs)
}

/// Returns span of format argument value.
fn format_arg_value_span(value: &FormatArgValue) -> Span {
    match value {
        FormatArgValue::Expr(expr) => expr.span(),
        FormatArgValue::Shorthand(FormatArgShorthand::Projection(projection)) => projection.span
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse::ParseStream, parse_quote};

    use super::*;

    #[test]
    fn format_arg_value_span_expr() {
        let expr: Expr = parse_quote! { 42 };
        let value = FormatArgValue::Expr(expr);
        let _ = format_arg_value_span(&value);
    }

    #[test]
    fn format_arg_value_span_projection() {
        let projection = FormatArgProjection {
            segments: vec![],
            span:     Span::call_site()
        };
        let value = FormatArgValue::Shorthand(FormatArgShorthand::Projection(projection));
        let _ = format_arg_value_span(&value);
    }

    #[test]
    fn join_spans_works() {
        let span1 = Span::call_site();
        let span2 = Span::call_site();
        let _ = join_spans(span1, span2);
    }

    #[test]
    fn parse_format_args_empty() {
        let result = syn::parse2::<syn::Expr>(quote::quote! { "" })
            .and_then(|_| syn::parse::Parser::parse2(parse_format_args, quote::quote! {}));
        assert!(result.is_ok());
        assert!(result.unwrap().args.is_empty());
    }

    #[test]
    fn parse_format_args_leading_comma_only() {
        let result = syn::parse::Parser::parse2(parse_format_args, quote::quote! { , });
        assert!(result.is_err());
    }

    #[test]
    fn parse_format_args_positional() {
        let result = syn::parse::Parser::parse2(parse_format_args, quote::quote! { , value });
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert_eq!(spec.args.len(), 1);
    }

    #[test]
    fn parse_format_args_named() {
        let result =
            syn::parse::Parser::parse2(parse_format_args, quote::quote! { , name = value });
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert_eq!(spec.args.len(), 1);
    }

    #[test]
    fn parse_format_args_duplicate_named() {
        let result =
            syn::parse::Parser::parse2(parse_format_args, quote::quote! { , x = a, x = b });
        assert!(result.is_err());
    }

    #[test]
    fn parse_format_args_mixed() {
        let result = syn::parse::Parser::parse2(
            parse_format_args,
            quote::quote! { , pos1, name = val, pos2 }
        );
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert_eq!(spec.args.len(), 3);
    }

    #[test]
    fn raw_format_arg_named() {
        let result: Result<RawFormatArg, _> = syn::parse2(quote::quote! { name = value });
        assert!(result.is_ok());
        match result.unwrap() {
            RawFormatArg::Named {
                ..
            } => {}
            _ => panic!("expected Named variant")
        }
    }

    #[test]
    fn raw_format_arg_positional() {
        let result: Result<RawFormatArg, _> = syn::parse2(quote::quote! { value });
        assert!(result.is_ok());
        match result.unwrap() {
            RawFormatArg::Positional {
                ..
            } => {}
            _ => panic!("expected Positional variant")
        }
    }

    #[test]
    fn parse_format_arg_value_expr() {
        let result = syn::parse::Parser::parse2(parse_format_arg_value, quote::quote! { value });
        assert!(result.is_ok());
        match result.unwrap() {
            FormatArgValue::Expr(_) => {}
            _ => panic!("expected Expr variant")
        }
    }

    #[test]
    fn parse_format_arg_value_projection() {
        let result = syn::parse::Parser::parse2(parse_format_arg_value, quote::quote! { .field });
        assert!(result.is_ok());
        match result.unwrap() {
            FormatArgValue::Shorthand(FormatArgShorthand::Projection(_)) => {}
            _ => panic!("expected Projection variant")
        }
    }

    #[test]
    fn parse_projection_segments_single_field() {
        let result = syn::parse::Parser::parse2(
            |input: ParseStream| parse_projection_segments(input, Span::call_site()),
            quote::quote! { field }
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().segments.len(), 1);
    }

    #[test]
    fn parse_projection_segments_multiple() {
        let result = syn::parse::Parser::parse2(
            |input: ParseStream| parse_projection_segments(input, Span::call_site()),
            quote::quote! { field.nested.value }
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().segments.len(), 3);
    }

    #[test]
    fn parse_projection_segment_field() {
        let result = syn::parse::Parser::parse2(
            |input: ParseStream| parse_projection_segment(input, false),
            quote::quote! { field }
        );
        assert!(result.is_ok());
        match result.unwrap() {
            FormatArgProjectionSegment::Field(_) => {}
            _ => panic!("expected Field variant")
        }
    }

    #[test]
    fn parse_projection_segment_index() {
        let result = syn::parse::Parser::parse2(
            |input: ParseStream| parse_projection_segment(input, false),
            quote::quote! { 0 }
        );
        assert!(result.is_ok());
        match result.unwrap() {
            FormatArgProjectionSegment::Index {
                index, ..
            } => assert_eq!(index, 0),
            _ => panic!("expected Index variant")
        }
    }

    #[test]
    fn parse_projection_segment_method_call() {
        let result = syn::parse::Parser::parse2(
            |input: ParseStream| parse_projection_segment(input, false),
            quote::quote! { method() }
        );
        assert!(result.is_ok());
        match result.unwrap() {
            FormatArgProjectionSegment::MethodCall(_) => {}
            _ => panic!("expected MethodCall variant")
        }
    }

    #[test]
    fn parse_projection_segment_method_with_args() {
        let result = syn::parse::Parser::parse2(
            |input: ParseStream| parse_projection_segment(input, false),
            quote::quote! { method(a, b) }
        );
        assert!(result.is_ok());
        match result.unwrap() {
            FormatArgProjectionSegment::MethodCall(call) => assert_eq!(call.args.len(), 2),
            _ => panic!("expected MethodCall variant")
        }
    }

    #[test]
    fn parse_projection_segment_method_with_turbofish() {
        let result = syn::parse::Parser::parse2(
            |input: ParseStream| parse_projection_segment(input, false),
            quote::quote! { method::<T>() }
        );
        assert!(result.is_ok());
        match result.unwrap() {
            FormatArgProjectionSegment::MethodCall(call) => assert!(call.turbofish.is_some()),
            _ => panic!("expected MethodCall variant")
        }
    }

    #[test]
    fn parse_projection_segment_error_first() {
        let result = syn::parse::Parser::parse2(
            |input: ParseStream| parse_projection_segment(input, true),
            quote::quote! {}
        );
        assert!(result.is_err());
    }

    #[test]
    fn parse_projection_segment_error_not_first() {
        let result = syn::parse::Parser::parse2(
            |input: ParseStream| parse_projection_segment(input, false),
            quote::quote! {}
        );
        assert!(result.is_err());
    }

    #[test]
    fn parse_method_call_suffix_none() {
        let result = syn::parse::Parser::parse2(parse_method_call_suffix, quote::quote! {});
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn parse_method_call_suffix_parens() {
        let result = syn::parse::Parser::parse2(parse_method_call_suffix, quote::quote! { () });
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn parse_method_call_suffix_with_turbofish() {
        let result =
            syn::parse::Parser::parse2(parse_method_call_suffix, quote::quote! { ::<T>() });
        assert!(result.is_ok());
        let suffix = result.unwrap().unwrap();
        assert!(suffix.0.is_some());
    }

    #[test]
    fn parse_method_call_suffix_with_args() {
        let result =
            syn::parse::Parser::parse2(parse_method_call_suffix, quote::quote! { (a, b) });
        assert!(result.is_ok());
        let suffix = result.unwrap().unwrap();
        assert_eq!(suffix.2.len(), 2);
    }
}
