// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Template rendering and code generation.
//!
//! This module handles the conversion of display templates into executable
//! formatting code. It processes template segments (literals and placeholders),
//! resolves placeholders to expressions, and generates optimized formatting
//! code using either direct trait calls or the `write!` macro depending on the
//! complexity of formatting requirements.

use std::borrow::Cow;

use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::Error;

use super::{
    format_args::{ResolvedFormatArgument, ResolvedFormatArgumentKind},
    formatter::{format_placeholder, placeholder_requires_format_engine},
    placeholder::ResolvedPlaceholderExpr
};
use crate::template_support::{
    DisplayTemplate, TemplateIdentifierSpec, TemplatePlaceholderSpec, TemplateSegmentSpec
};

/// A rendered template segment.
///
/// Template segments are either literal strings or placeholders that need
/// to be resolved and formatted.
#[derive(Debug)]
pub enum RenderedSegment {
    /// A literal string segment
    Literal(String),
    /// A placeholder with its resolved expression
    Placeholder(PlaceholderRender)
}

/// A rendered placeholder with all resolution information.
///
/// Contains the placeholder specification along with its resolved expression,
/// ready for code generation.
#[derive(Debug)]
pub struct PlaceholderRender {
    /// The placeholder identifier (name, position, or implicit)
    pub identifier: TemplateIdentifierSpec,
    /// The formatter to apply (Display, Debug, etc.)
    pub formatter:  masterror_template::template::TemplateFormatter,
    /// Source span for error reporting
    pub span:       Span,
    /// The resolved expression to format
    pub resolved:   ResolvedPlaceholderExpr
}

/// A named format argument for the `write!` macro.
#[derive(Debug)]
struct NamedArgument {
    name: String,
    span: Span,
    expr: TokenStream
}

/// A positional format argument for the `write!` macro.
#[derive(Debug)]
struct IndexedArgument {
    index: usize,
    expr:  TokenStream
}

/// Renders a display template into formatting code.
///
/// This is the main template rendering function that converts a display
/// template into executable code. It processes all segments, resolves
/// placeholders using the provided resolver function, and generates optimized
/// formatting code.
///
/// # Arguments
///
/// * `template` - The display template to render
/// * `preludes` - Prelude statements to execute before formatting (e.g., let
///   bindings)
/// * `format_args` - Additional format arguments from the error attribute
/// * `resolver` - Function to resolve placeholder identifiers to expressions
///
/// # Returns
///
/// Token stream containing the formatting implementation code
///
/// # Type Parameters
///
/// * `F` - Placeholder resolver function type
pub fn render_template<F>(
    template: &DisplayTemplate,
    preludes: Vec<TokenStream>,
    format_args: Vec<ResolvedFormatArgument>,
    mut resolver: F
) -> Result<TokenStream, Error>
where
    F: FnMut(&TemplatePlaceholderSpec) -> Result<ResolvedPlaceholderExpr, Error>
{
    let mut segments = Vec::new();
    let mut literal_buffer = String::new();
    let mut format_buffer = String::new();
    let mut has_placeholder = false;
    let mut has_implicit_placeholders = false;
    let mut requires_format_engine = false;

    for segment in &template.segments {
        match segment {
            TemplateSegmentSpec::Literal(text) => {
                literal_buffer.push_str(text);
                push_literal_fragment(&mut format_buffer, text);
                segments.push(RenderedSegment::Literal(text.clone()));
            }
            TemplateSegmentSpec::Placeholder(placeholder) => {
                has_placeholder = true;
                if matches!(placeholder.identifier, TemplateIdentifierSpec::Implicit(_)) {
                    has_implicit_placeholders = true;
                }
                if placeholder_requires_format_engine(&placeholder.formatter) {
                    requires_format_engine = true;
                }

                let resolved = resolver(placeholder)?;
                format_buffer.push_str(&placeholder_format_fragment(placeholder));
                segments.push(RenderedSegment::Placeholder(PlaceholderRender {
                    identifier: placeholder.identifier.clone(),
                    formatter: placeholder.formatter.clone(),
                    span: placeholder.span,
                    resolved
                }));
            }
        }
    }

    let has_additional_arguments = !preludes.is_empty() || !format_args.is_empty();

    if !has_placeholder && !has_additional_arguments {
        let literal = Literal::string(&literal_buffer);
        return Ok(quote! {
            #(#preludes)*
            f.write_str(#literal)
        });
    }

    if has_additional_arguments || has_implicit_placeholders || requires_format_engine {
        let format_literal = Literal::string(&format_buffer);
        let args = build_template_arguments(&segments, format_args);
        return Ok(quote! {
            #(#preludes)*
            ::core::write!(f, #format_literal #(, #args)*)
        });
    }

    let mut pieces = preludes;
    for segment in segments {
        match segment {
            RenderedSegment::Literal(text) => {
                pieces.push(quote! { f.write_str(#text)?; });
            }
            RenderedSegment::Placeholder(placeholder) => {
                pieces.push(format_placeholder(
                    placeholder.resolved,
                    placeholder.formatter
                ));
            }
        }
    }
    pieces.push(quote! { Ok(()) });

    Ok(quote! {
        #(#pieces)*
    })
}

/// Builds the argument list for the `write!` macro.
///
/// Collects all arguments (from placeholders and explicit format arguments),
/// deduplicates them, and arranges them in the correct order: positional,
/// implicit, then named arguments.
///
/// # Arguments
///
/// * `segments` - Rendered template segments containing placeholder expressions
/// * `format_args` - Additional format arguments from the error attribute
///
/// # Returns
///
/// Vector of token streams representing the arguments to pass to `write!`
pub fn build_template_arguments(
    segments: &[RenderedSegment],
    format_args: Vec<ResolvedFormatArgument>
) -> Vec<TokenStream> {
    let mut named = Vec::new();
    let mut positional = Vec::new();
    let mut implicit = Vec::new();

    for segment in segments {
        let RenderedSegment::Placeholder(placeholder) = segment else {
            continue;
        };

        match &placeholder.identifier {
            TemplateIdentifierSpec::Named(name) => {
                if named
                    .iter()
                    .any(|argument: &NamedArgument| argument.name == *name)
                {
                    continue;
                }

                named.push(NamedArgument {
                    name: name.clone(),
                    span: placeholder.span,
                    expr: placeholder.resolved.expr_tokens()
                });
            }
            TemplateIdentifierSpec::Positional(index) => {
                if positional
                    .iter()
                    .any(|argument: &IndexedArgument| argument.index == *index)
                {
                    continue;
                }

                positional.push(IndexedArgument {
                    index: *index,
                    expr:  placeholder.resolved.expr_tokens()
                });
            }
            TemplateIdentifierSpec::Implicit(index) => {
                if implicit
                    .iter()
                    .any(|argument: &IndexedArgument| argument.index == *index)
                {
                    continue;
                }

                implicit.push(IndexedArgument {
                    index: *index,
                    expr:  placeholder.resolved.expr_tokens()
                });
            }
        }
    }

    for argument in format_args {
        match argument.kind {
            ResolvedFormatArgumentKind::Named(ident) => {
                let name = ident.to_string();
                if named
                    .iter()
                    .any(|existing: &NamedArgument| existing.name == name)
                {
                    continue;
                }

                let span = ident.span();
                named.push(NamedArgument {
                    name,
                    span,
                    expr: argument.expr
                });
            }
            ResolvedFormatArgumentKind::Positional(index) => {
                if positional
                    .iter()
                    .any(|existing: &IndexedArgument| existing.index == index)
                    || implicit
                        .iter()
                        .any(|existing: &IndexedArgument| existing.index == index)
                {
                    continue;
                }

                positional.push(IndexedArgument {
                    index,
                    expr: argument.expr
                });
            }
            ResolvedFormatArgumentKind::Implicit(index) => {
                if implicit
                    .iter()
                    .any(|existing: &IndexedArgument| existing.index == index)
                {
                    continue;
                }

                implicit.push(IndexedArgument {
                    index,
                    expr: argument.expr
                });
            }
        }
    }

    positional.sort_by_key(|argument| argument.index);
    implicit.sort_by_key(|argument| argument.index);

    let mut arguments = Vec::with_capacity(named.len() + positional.len() + implicit.len());
    for IndexedArgument {
        expr, ..
    } in positional
    {
        arguments.push(expr);
    }
    for IndexedArgument {
        expr, ..
    } in implicit
    {
        arguments.push(expr);
    }
    for NamedArgument {
        name,
        span,
        expr
    } in named
    {
        let ident = format_ident!("{}", name, span = span);
        arguments.push(quote_spanned!(span => #ident = #expr));
    }

    arguments
}

/// Escapes a literal string for use in a format string.
///
/// Doubles all braces (`{` and `}`) so they are treated as literal characters
/// in the format string rather than placeholder delimiters.
///
/// # Arguments
///
/// * `buffer` - The string buffer to append the escaped literal to
/// * `literal` - The literal string to escape and append
pub fn push_literal_fragment(buffer: &mut String, literal: &str) {
    for ch in literal.chars() {
        match ch {
            '{' => buffer.push_str("{{"),
            '}' => buffer.push_str("}}"),
            _ => buffer.push(ch)
        }
    }
}

/// Generates the format string fragment for a placeholder.
///
/// Creates the `{...}` format specifier that will be used in the format string,
/// including the identifier and any format specifications (like `:?` for
/// Debug).
///
/// # Arguments
///
/// * `placeholder` - The placeholder specification
///
/// # Returns
///
/// String containing the format string fragment (e.g., `"{name:?}"` or `"{0}"`)
pub fn placeholder_format_fragment(placeholder: &TemplatePlaceholderSpec) -> String {
    let mut fragment = String::from("{");

    match &placeholder.identifier {
        TemplateIdentifierSpec::Named(name) => fragment.push_str(name),
        TemplateIdentifierSpec::Positional(index) => fragment.push_str(&index.to_string()),
        TemplateIdentifierSpec::Implicit(_) => {}
    }

    if let Some(spec) = formatter_format_fragment(&placeholder.formatter) {
        fragment.push(':');
        fragment.push_str(spec.as_ref());
    }

    fragment.push('}');
    fragment
}

/// Generates the format specification fragment for a formatter.
///
/// Extracts the format specification string from a formatter (e.g., `"?"` for
/// Debug, `"x"` for LowerHex). Returns `None` for formatters that don't require
/// a specification.
///
/// # Arguments
///
/// * `formatter` - The template formatter
///
/// # Returns
///
/// The format specification string, or `None` if not applicable
pub fn formatter_format_fragment<'a>(
    formatter: &'a masterror_template::template::TemplateFormatter
) -> Option<Cow<'a, str>> {
    formatter.format_fragment()
}

#[cfg(test)]
mod tests {
    use masterror_template::template::TemplateFormatter;
    use proc_macro2::Span;
    use quote::quote;

    use super::*;

    #[test]
    fn test_push_literal_fragment_simple_text() {
        let mut buffer = String::new();
        push_literal_fragment(&mut buffer, "Hello, World!");
        assert_eq!(buffer, "Hello, World!");
    }

    #[test]
    fn test_push_literal_fragment_with_opening_brace() {
        let mut buffer = String::new();
        push_literal_fragment(&mut buffer, "foo {");
        assert_eq!(buffer, "foo {{");
    }

    #[test]
    fn test_push_literal_fragment_with_closing_brace() {
        let mut buffer = String::new();
        push_literal_fragment(&mut buffer, "} bar");
        assert_eq!(buffer, "}} bar");
    }

    #[test]
    fn test_push_literal_fragment_with_both_braces() {
        let mut buffer = String::new();
        push_literal_fragment(&mut buffer, "{ value }");
        assert_eq!(buffer, "{{ value }}");
    }

    #[test]
    fn test_push_literal_fragment_multiple_braces() {
        let mut buffer = String::new();
        push_literal_fragment(&mut buffer, "{}{}{");
        assert_eq!(buffer, "{{}}{{}}{{");
    }

    #[test]
    fn test_push_literal_fragment_appends() {
        let mut buffer = String::from("prefix ");
        push_literal_fragment(&mut buffer, "{ value }");
        assert_eq!(buffer, "prefix {{ value }}");
    }

    #[test]
    fn test_placeholder_format_fragment_named_display() {
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("foo".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = placeholder_format_fragment(&placeholder);
        assert_eq!(result, "{foo}");
    }

    #[test]
    fn test_placeholder_format_fragment_named_debug() {
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("bar".to_string()),
            formatter:  TemplateFormatter::Debug {
                alternate: false
            },
            span:       Span::call_site()
        };
        let result = placeholder_format_fragment(&placeholder);
        assert_eq!(result, "{bar:?}");
    }

    #[test]
    fn test_placeholder_format_fragment_named_debug_alternate() {
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("bar".to_string()),
            formatter:  TemplateFormatter::Debug {
                alternate: true
            },
            span:       Span::call_site()
        };
        let result = placeholder_format_fragment(&placeholder);
        assert_eq!(result, "{bar:#?}");
    }

    #[test]
    fn test_placeholder_format_fragment_positional() {
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Positional(0),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = placeholder_format_fragment(&placeholder);
        assert_eq!(result, "{0}");
    }

    #[test]
    fn test_placeholder_format_fragment_positional_hex() {
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Positional(1),
            formatter:  TemplateFormatter::LowerHex {
                alternate: false
            },
            span:       Span::call_site()
        };
        let result = placeholder_format_fragment(&placeholder);
        assert_eq!(result, "{1:x}");
    }

    #[test]
    fn test_placeholder_format_fragment_positional_hex_alternate() {
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Positional(1),
            formatter:  TemplateFormatter::LowerHex {
                alternate: true
            },
            span:       Span::call_site()
        };
        let result = placeholder_format_fragment(&placeholder);
        assert_eq!(result, "{1:#x}");
    }

    #[test]
    fn test_placeholder_format_fragment_implicit() {
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Implicit(0),
            formatter:  TemplateFormatter::Display {
                spec: None
            },
            span:       Span::call_site()
        };
        let result = placeholder_format_fragment(&placeholder);
        assert_eq!(result, "{}");
    }

    #[test]
    fn test_placeholder_format_fragment_implicit_binary() {
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Implicit(0),
            formatter:  TemplateFormatter::Binary {
                alternate: false
            },
            span:       Span::call_site()
        };
        let result = placeholder_format_fragment(&placeholder);
        assert_eq!(result, "{:b}");
    }

    #[test]
    fn test_placeholder_format_fragment_display_with_spec() {
        let placeholder = TemplatePlaceholderSpec {
            identifier: TemplateIdentifierSpec::Named("value".to_string()),
            formatter:  TemplateFormatter::Display {
                spec: Some(">10".into())
            },
            span:       Span::call_site()
        };
        let result = placeholder_format_fragment(&placeholder);
        assert_eq!(result, "{value:>10}");
    }

    #[test]
    fn test_formatter_format_fragment_display_no_spec() {
        let formatter = TemplateFormatter::Display {
            spec: None
        };
        assert_eq!(formatter_format_fragment(&formatter), None);
    }

    #[test]
    fn test_formatter_format_fragment_display_with_spec() {
        let formatter = TemplateFormatter::Display {
            spec: Some(">10".into())
        };
        assert_eq!(
            formatter_format_fragment(&formatter),
            Some(Cow::Borrowed(">10"))
        );
    }

    #[test]
    fn test_formatter_format_fragment_debug() {
        let formatter = TemplateFormatter::Debug {
            alternate: false
        };
        assert_eq!(
            formatter_format_fragment(&formatter),
            Some(Cow::Borrowed("?"))
        );
    }

    #[test]
    fn test_formatter_format_fragment_debug_alternate() {
        let formatter = TemplateFormatter::Debug {
            alternate: true
        };
        assert_eq!(
            formatter_format_fragment(&formatter),
            Some(Cow::Borrowed("#?"))
        );
    }

    #[test]
    fn test_formatter_format_fragment_lower_hex() {
        let formatter = TemplateFormatter::LowerHex {
            alternate: false
        };
        assert_eq!(
            formatter_format_fragment(&formatter),
            Some(Cow::Borrowed("x"))
        );
    }

    #[test]
    fn test_formatter_format_fragment_lower_hex_alternate() {
        let formatter = TemplateFormatter::LowerHex {
            alternate: true
        };
        assert_eq!(
            formatter_format_fragment(&formatter),
            Some(Cow::Borrowed("#x"))
        );
    }

    #[test]
    fn test_formatter_format_fragment_upper_hex() {
        let formatter = TemplateFormatter::UpperHex {
            alternate: false
        };
        assert_eq!(
            formatter_format_fragment(&formatter),
            Some(Cow::Borrowed("X"))
        );
    }

    #[test]
    fn test_formatter_format_fragment_pointer() {
        let formatter = TemplateFormatter::Pointer {
            alternate: false
        };
        assert_eq!(
            formatter_format_fragment(&formatter),
            Some(Cow::Borrowed("p"))
        );
    }

    #[test]
    fn test_formatter_format_fragment_binary() {
        let formatter = TemplateFormatter::Binary {
            alternate: false
        };
        assert_eq!(
            formatter_format_fragment(&formatter),
            Some(Cow::Borrowed("b"))
        );
    }

    #[test]
    fn test_formatter_format_fragment_octal() {
        let formatter = TemplateFormatter::Octal {
            alternate: false
        };
        assert_eq!(
            formatter_format_fragment(&formatter),
            Some(Cow::Borrowed("o"))
        );
    }

    #[test]
    fn test_formatter_format_fragment_lower_exp() {
        let formatter = TemplateFormatter::LowerExp {
            alternate: false
        };
        assert_eq!(
            formatter_format_fragment(&formatter),
            Some(Cow::Borrowed("e"))
        );
    }

    #[test]
    fn test_formatter_format_fragment_upper_exp() {
        let formatter = TemplateFormatter::UpperExp {
            alternate: false
        };
        assert_eq!(
            formatter_format_fragment(&formatter),
            Some(Cow::Borrowed("E"))
        );
    }

    #[test]
    fn test_build_template_arguments_deduplicates_named() {
        let segments = vec![
            RenderedSegment::Placeholder(PlaceholderRender {
                identifier: TemplateIdentifierSpec::Named("foo".to_string()),
                formatter:  TemplateFormatter::Display {
                    spec: None
                },
                span:       Span::call_site(),
                resolved:   ResolvedPlaceholderExpr::new(quote!(value1))
            }),
            RenderedSegment::Placeholder(PlaceholderRender {
                identifier: TemplateIdentifierSpec::Named("foo".to_string()),
                formatter:  TemplateFormatter::Display {
                    spec: None
                },
                span:       Span::call_site(),
                resolved:   ResolvedPlaceholderExpr::new(quote!(value2))
            }),
        ];
        let result = build_template_arguments(&segments, Vec::new());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_build_template_arguments_deduplicates_positional() {
        let segments = vec![
            RenderedSegment::Placeholder(PlaceholderRender {
                identifier: TemplateIdentifierSpec::Positional(0),
                formatter:  TemplateFormatter::Display {
                    spec: None
                },
                span:       Span::call_site(),
                resolved:   ResolvedPlaceholderExpr::new(quote!(value1))
            }),
            RenderedSegment::Placeholder(PlaceholderRender {
                identifier: TemplateIdentifierSpec::Positional(0),
                formatter:  TemplateFormatter::Display {
                    spec: None
                },
                span:       Span::call_site(),
                resolved:   ResolvedPlaceholderExpr::new(quote!(value2))
            }),
        ];
        let result = build_template_arguments(&segments, Vec::new());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_build_template_arguments_orders_correctly() {
        let segments = vec![
            RenderedSegment::Placeholder(PlaceholderRender {
                identifier: TemplateIdentifierSpec::Named("foo".to_string()),
                formatter:  TemplateFormatter::Display {
                    spec: None
                },
                span:       Span::call_site(),
                resolved:   ResolvedPlaceholderExpr::new(quote!(named_val))
            }),
            RenderedSegment::Placeholder(PlaceholderRender {
                identifier: TemplateIdentifierSpec::Implicit(0),
                formatter:  TemplateFormatter::Display {
                    spec: None
                },
                span:       Span::call_site(),
                resolved:   ResolvedPlaceholderExpr::new(quote!(implicit_val))
            }),
            RenderedSegment::Placeholder(PlaceholderRender {
                identifier: TemplateIdentifierSpec::Positional(0),
                formatter:  TemplateFormatter::Display {
                    spec: None
                },
                span:       Span::call_site(),
                resolved:   ResolvedPlaceholderExpr::new(quote!(positional_val))
            }),
        ];
        let result = build_template_arguments(&segments, Vec::new());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].to_string(), "positional_val");
        assert_eq!(result[1].to_string(), "implicit_val");
        assert!(result[2].to_string().contains("foo"));
    }

    #[test]
    fn test_build_template_arguments_handles_literals() {
        let segments = vec![
            RenderedSegment::Literal("Hello".to_string()),
            RenderedSegment::Placeholder(PlaceholderRender {
                identifier: TemplateIdentifierSpec::Named("foo".to_string()),
                formatter:  TemplateFormatter::Display {
                    spec: None
                },
                span:       Span::call_site(),
                resolved:   ResolvedPlaceholderExpr::new(quote!(value))
            }),
            RenderedSegment::Literal("World".to_string()),
        ];
        let result = build_template_arguments(&segments, Vec::new());
        assert_eq!(result.len(), 1);
    }
}
