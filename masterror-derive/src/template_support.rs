// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use masterror_template::template::{
    ErrorTemplate, TemplateError, TemplateFormatter, TemplateIdentifier, TemplateSegment
};
use proc_macro2::Span;
use syn::{Error, LitStr};

use crate::span::literal_subspan;

#[derive(Debug, Clone)]
pub struct DisplayTemplate {
    pub segments: Vec<TemplateSegmentSpec>
}

#[derive(Debug, Clone)]
pub enum TemplateSegmentSpec {
    Literal(String),
    Placeholder(TemplatePlaceholderSpec)
}

#[derive(Debug, Clone)]
pub struct TemplatePlaceholderSpec {
    pub span:       Span,
    pub identifier: TemplateIdentifierSpec,
    pub formatter:  TemplateFormatter
}

#[derive(Debug, Clone)]
pub enum TemplateIdentifierSpec {
    Named(String),
    Positional(usize),
    Implicit(usize)
}

pub fn parse_display_template(lit: LitStr) -> Result<DisplayTemplate, Error> {
    let value = lit.value();
    let parsed = ErrorTemplate::parse(&value).map_err(|err| template_error(&lit, err))?;

    let mut segments = Vec::new();
    for segment in parsed.segments() {
        match segment {
            TemplateSegment::Literal(text) => {
                segments.push(TemplateSegmentSpec::Literal(text.to_string()));
            }
            TemplateSegment::Placeholder(placeholder) => {
                let span = placeholder_span(&lit, placeholder.span());
                let identifier = match placeholder.identifier() {
                    TemplateIdentifier::Named(name) => {
                        TemplateIdentifierSpec::Named(name.to_string())
                    }
                    TemplateIdentifier::Positional(index) => {
                        TemplateIdentifierSpec::Positional(*index)
                    }
                    TemplateIdentifier::Implicit(index) => TemplateIdentifierSpec::Implicit(*index)
                };

                segments.push(TemplateSegmentSpec::Placeholder(TemplatePlaceholderSpec {
                    span,
                    identifier,
                    formatter: placeholder.formatter().clone()
                }));
            }
        }
    }

    Ok(DisplayTemplate {
        segments
    })
}

fn placeholder_span(lit: &LitStr, range: core::ops::Range<usize>) -> Span {
    literal_subspan(lit, range).unwrap_or_else(|| lit.span())
}

fn template_error(lit: &LitStr, error: TemplateError) -> Error {
    let message = error.to_string();
    let span = match &error {
        TemplateError::UnmatchedClosingBrace {
            index
        } => literal_subspan(lit, *index..(*index + 1)),
        TemplateError::UnterminatedPlaceholder {
            start
        } => literal_subspan(lit, *start..(*start + 1)),
        TemplateError::NestedPlaceholder {
            index
        } => literal_subspan(lit, *index..(*index + 1)),
        TemplateError::EmptyPlaceholder {
            start
        } => literal_subspan(lit, *start..(*start + 1)),
        TemplateError::InvalidIdentifier {
            span
        } => literal_subspan(lit, span.clone()),
        TemplateError::InvalidIndex {
            span
        } => literal_subspan(lit, span.clone()),
        TemplateError::InvalidFormatter {
            span
        } => literal_subspan(lit, span.clone())
    };

    Error::new(span.unwrap_or_else(|| lit.span()), message)
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn parse_display_template_simple_literal() {
        let lit: LitStr = parse_quote!("hello");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert_eq!(template.segments.len(), 1);
        assert!(matches!(
            &template.segments[0],
            TemplateSegmentSpec::Literal(s) if s == "hello"
        ));
    }

    #[test]
    fn parse_display_template_named_placeholder() {
        let lit: LitStr = parse_quote!("hello {name}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert_eq!(template.segments.len(), 2);
        assert!(matches!(
            &template.segments[0],
            TemplateSegmentSpec::Literal(s) if s == "hello "
        ));
        assert!(matches!(
            &template.segments[1],
            TemplateSegmentSpec::Placeholder(p) if matches!(
                &p.identifier,
                TemplateIdentifierSpec::Named(n) if n == "name"
            )
        ));
    }

    #[test]
    fn parse_display_template_positional_placeholder() {
        let lit: LitStr = parse_quote!("hello {0}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert_eq!(template.segments.len(), 2);
        assert!(matches!(
            &template.segments[1],
            TemplateSegmentSpec::Placeholder(p) if matches!(
                &p.identifier,
                TemplateIdentifierSpec::Positional(idx) if *idx == 0
            )
        ));
    }

    #[test]
    fn parse_display_template_implicit_placeholder() {
        let lit: LitStr = parse_quote!("hello {}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert_eq!(template.segments.len(), 2);
        assert!(matches!(
            &template.segments[1],
            TemplateSegmentSpec::Placeholder(p) if matches!(
                &p.identifier,
                TemplateIdentifierSpec::Implicit(idx) if *idx == 0
            )
        ));
    }

    #[test]
    fn parse_display_template_with_formatter() {
        let lit: LitStr = parse_quote!("hello {name:?}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert_eq!(template.segments.len(), 2);
        if let TemplateSegmentSpec::Placeholder(p) = &template.segments[1] {
            assert!(matches!(
                &p.formatter,
                TemplateFormatter::Debug {
                    alternate: false
                }
            ));
        } else {
            panic!("Expected placeholder segment");
        }
    }

    #[test]
    fn parse_display_template_multiple_placeholders() {
        let lit: LitStr = parse_quote!("hello {0} {name}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert_eq!(template.segments.len(), 4);
        assert!(matches!(
            &template.segments[0],
            TemplateSegmentSpec::Literal(s) if s == "hello "
        ));
        assert!(matches!(
            &template.segments[1],
            TemplateSegmentSpec::Placeholder(p) if matches!(
                &p.identifier,
                TemplateIdentifierSpec::Positional(idx) if *idx == 0
            )
        ));
        assert!(matches!(
            &template.segments[2],
            TemplateSegmentSpec::Literal(s) if s == " "
        ));
        assert!(matches!(
            &template.segments[3],
            TemplateSegmentSpec::Placeholder(p) if matches!(
                &p.identifier,
                TemplateIdentifierSpec::Named(n) if n == "name"
            )
        ));
    }

    #[test]
    fn parse_display_template_escape_sequences() {
        let lit: LitStr = parse_quote!("hello {{world}}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        // "hello {{world}}" parses as: "hello ", "{", "world", "}"
        assert_eq!(template.segments.len(), 4);
        assert!(matches!(
            &template.segments[0],
            TemplateSegmentSpec::Literal(s) if s == "hello "
        ));
        assert!(matches!(
            &template.segments[1],
            TemplateSegmentSpec::Literal(s) if s == "{"
        ));
        assert!(matches!(
            &template.segments[2],
            TemplateSegmentSpec::Literal(s) if s == "world"
        ));
        assert!(matches!(
            &template.segments[3],
            TemplateSegmentSpec::Literal(s) if s == "}"
        ));
    }

    #[test]
    fn parse_display_template_unmatched_closing_brace() {
        let lit: LitStr = parse_quote!("hello }");
        let result = parse_display_template(lit);
        assert!(result.is_err());
        let err = result.err().unwrap();
        let msg = err.to_string();
        assert!(msg.contains("unmatched closing brace"));
    }

    #[test]
    fn parse_display_template_unterminated_placeholder() {
        let lit: LitStr = parse_quote!("hello {name");
        let result = parse_display_template(lit);
        assert!(result.is_err());
        let err = result.err().unwrap();
        let msg = err.to_string();
        assert!(msg.contains("not closed"));
    }

    #[test]
    fn parse_display_template_nested_placeholder() {
        // A truly nested placeholder: "{foo{bar}" - has "{" inside the placeholder
        let lit: LitStr = parse_quote!("{foo{bar}");
        let result = parse_display_template(lit);
        assert!(result.is_err());
        let err = result.err().unwrap();
        let msg = err.to_string();
        assert!(msg.contains("nested"));
    }

    #[test]
    fn parse_display_template_invalid_identifier() {
        let lit: LitStr = parse_quote!("hello {invalid-name}");
        let result = parse_display_template(lit);
        assert!(result.is_err());
        let err = result.err().unwrap();
        let msg = err.to_string();
        assert!(msg.contains("invalid") || msg.contains("identifier"));
    }

    #[test]
    fn parse_display_template_with_debug_formatter() {
        let lit: LitStr = parse_quote!("value: {0:?}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        if let TemplateSegmentSpec::Placeholder(p) = &template.segments[1] {
            assert!(matches!(
                &p.formatter,
                TemplateFormatter::Debug {
                    alternate: false
                }
            ));
        } else {
            panic!("Expected placeholder segment");
        }
    }

    #[test]
    fn parse_display_template_with_alternate_debug_formatter() {
        let lit: LitStr = parse_quote!("value: {0:#?}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        if let TemplateSegmentSpec::Placeholder(p) = &template.segments[1] {
            assert!(matches!(
                &p.formatter,
                TemplateFormatter::Debug {
                    alternate: true
                }
            ));
        } else {
            panic!("Expected placeholder segment");
        }
    }

    #[test]
    fn parse_display_template_with_hex_formatter() {
        let lit: LitStr = parse_quote!("value: {val:x}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        if let TemplateSegmentSpec::Placeholder(p) = &template.segments[1] {
            assert!(matches!(
                &p.formatter,
                TemplateFormatter::LowerHex {
                    alternate: false
                }
            ));
        } else {
            panic!("Expected placeholder segment");
        }
    }

    #[test]
    fn parse_display_template_multiple_implicit_placeholders() {
        let lit: LitStr = parse_quote!("a {} b {} c");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert_eq!(template.segments.len(), 5);
        if let TemplateSegmentSpec::Placeholder(p) = &template.segments[1] {
            assert!(matches!(&p.identifier, TemplateIdentifierSpec::Implicit(0)));
        }
        if let TemplateSegmentSpec::Placeholder(p) = &template.segments[3] {
            assert!(matches!(&p.identifier, TemplateIdentifierSpec::Implicit(1)));
        }
    }

    #[test]
    fn parse_display_template_complex_mixed() {
        let lit: LitStr = parse_quote!("Error: {0} at {location:?} with {1}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert!(template.segments.len() > 3);
    }

    #[test]
    fn parse_display_template_empty_string() {
        let lit: LitStr = parse_quote!("");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert_eq!(template.segments.len(), 0);
    }

    #[test]
    fn parse_display_template_only_placeholder() {
        let lit: LitStr = parse_quote!("{error}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert_eq!(template.segments.len(), 1);
        assert!(matches!(
            &template.segments[0],
            TemplateSegmentSpec::Placeholder(_)
        ));
    }

    #[test]
    fn template_error_unmatched_closing_brace() {
        let lit: LitStr = parse_quote!("test}");
        let error = TemplateError::UnmatchedClosingBrace {
            index: 4
        };
        let syn_error = template_error(&lit, error);
        let msg = syn_error.to_string();
        assert!(msg.contains("unmatched closing brace"));
    }

    #[test]
    fn template_error_unterminated_placeholder() {
        let lit: LitStr = parse_quote!("{test");
        let error = TemplateError::UnterminatedPlaceholder {
            start: 0
        };
        let syn_error = template_error(&lit, error);
        let msg = syn_error.to_string();
        assert!(msg.contains("not closed"));
    }

    #[test]
    fn template_error_nested_placeholder() {
        let lit: LitStr = parse_quote!("{{{test}");
        let error = TemplateError::NestedPlaceholder {
            index: 2
        };
        let syn_error = template_error(&lit, error);
        let msg = syn_error.to_string();
        assert!(msg.contains("nested placeholder"));
    }

    #[test]
    fn template_error_empty_placeholder() {
        let lit: LitStr = parse_quote!("test{}extra");
        let error = TemplateError::EmptyPlaceholder {
            start: 4
        };
        let syn_error = template_error(&lit, error);
        let msg = syn_error.to_string();
        assert!(msg.contains("empty"));
    }

    #[test]
    fn template_error_invalid_identifier() {
        let lit: LitStr = parse_quote!("{invalid-id}");
        let error = TemplateError::InvalidIdentifier {
            span: 0..12
        };
        let syn_error = template_error(&lit, error);
        let msg = syn_error.to_string();
        assert!(msg.contains("invalid") || msg.contains("identifier"));
    }

    #[test]
    fn template_error_invalid_index() {
        let lit: LitStr = parse_quote!("{999999999999999999999}");
        let error = TemplateError::InvalidIndex {
            span: 0..23
        };
        let syn_error = template_error(&lit, error);
        let msg = syn_error.to_string();
        assert!(msg.contains("not a valid unsigned integer") || msg.contains("Invalid"));
    }

    #[test]
    fn template_error_invalid_formatter() {
        let lit: LitStr = parse_quote!("{value:@}");
        let error = TemplateError::InvalidFormatter {
            span: 0..9
        };
        let syn_error = template_error(&lit, error);
        let msg = syn_error.to_string();
        assert!(msg.contains("unsupported formatter") || msg.contains("Invalid"));
    }

    #[test]
    fn placeholder_span_returns_subspan_for_valid_range() {
        let lit: LitStr = parse_quote!("hello {name}");
        let span = placeholder_span(&lit, 6..12);
        // The span should be valid (not equal to the lit span in a meaningful way)
        // We can't directly compare spans, but we can verify the function doesn't panic
        let _ = span;
    }

    #[test]
    fn placeholder_span_returns_lit_span_for_invalid_range() {
        let lit: LitStr = parse_quote!("hello");
        // Invalid range beyond the string length
        let span = placeholder_span(&lit, 100..200);
        // Should return lit.span() without panicking
        let _ = span;
    }

    #[test]
    fn parse_display_template_with_binary_formatter() {
        let lit: LitStr = parse_quote!("{val:b}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        if let TemplateSegmentSpec::Placeholder(p) = &template.segments[0] {
            assert!(matches!(
                &p.formatter,
                TemplateFormatter::Binary {
                    alternate: false
                }
            ));
        }
    }

    #[test]
    fn parse_display_template_with_octal_formatter() {
        let lit: LitStr = parse_quote!("{val:o}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        if let TemplateSegmentSpec::Placeholder(p) = &template.segments[0] {
            assert!(matches!(
                &p.formatter,
                TemplateFormatter::Octal {
                    alternate: false
                }
            ));
        }
    }

    #[test]
    fn parse_display_template_with_pointer_formatter() {
        let lit: LitStr = parse_quote!("{val:p}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        if let TemplateSegmentSpec::Placeholder(p) = &template.segments[0] {
            assert!(matches!(
                &p.formatter,
                TemplateFormatter::Pointer {
                    alternate: false
                }
            ));
        }
    }

    #[test]
    fn parse_display_template_with_lower_exp_formatter() {
        let lit: LitStr = parse_quote!("{val:e}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        if let TemplateSegmentSpec::Placeholder(p) = &template.segments[0] {
            assert!(matches!(
                &p.formatter,
                TemplateFormatter::LowerExp {
                    alternate: false
                }
            ));
        }
    }

    #[test]
    fn parse_display_template_with_upper_exp_formatter() {
        let lit: LitStr = parse_quote!("{val:E}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        if let TemplateSegmentSpec::Placeholder(p) = &template.segments[0] {
            assert!(matches!(
                &p.formatter,
                TemplateFormatter::UpperExp {
                    alternate: false
                }
            ));
        }
    }

    #[test]
    fn parse_display_template_with_upper_hex_formatter() {
        let lit: LitStr = parse_quote!("{val:X}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        if let TemplateSegmentSpec::Placeholder(p) = &template.segments[0] {
            assert!(matches!(
                &p.formatter,
                TemplateFormatter::UpperHex {
                    alternate: false
                }
            ));
        }
    }

    #[test]
    fn parse_display_template_with_alternate_formatters() {
        let cases = vec![
            ("{val:#x}", "LowerHex"),
            ("{val:#X}", "UpperHex"),
            ("{val:#b}", "Binary"),
            ("{val:#o}", "Octal"),
            ("{val:#p}", "Pointer"),
        ];

        for (template_str, formatter_name) in cases {
            let lit: LitStr = parse_quote!(#template_str);
            let result = parse_display_template(lit);
            assert!(result.is_ok(), "Failed for {}", formatter_name);
        }
    }

    #[test]
    fn parse_display_template_leading_trailing_braces() {
        let lit: LitStr = parse_quote!("{{prefix}} {val} {{suffix}}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        // Should have: "{", "prefix", "}", " ", placeholder, " ", "{", "suffix", "}"
        assert!(template.segments.len() >= 3);
    }

    #[test]
    fn parse_display_template_consecutive_placeholders() {
        let lit: LitStr = parse_quote!("{a}{b}{c}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert_eq!(template.segments.len(), 3);
        assert!(matches!(
            &template.segments[0],
            TemplateSegmentSpec::Placeholder(_)
        ));
        assert!(matches!(
            &template.segments[1],
            TemplateSegmentSpec::Placeholder(_)
        ));
        assert!(matches!(
            &template.segments[2],
            TemplateSegmentSpec::Placeholder(_)
        ));
    }

    #[test]
    fn parse_display_template_unicode_content() {
        let lit: LitStr = parse_quote!("Error: {msg} 错误");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert!(template.segments.len() >= 2);
    }

    #[test]
    fn parse_display_template_numbers_in_names() {
        let lit: LitStr = parse_quote!("{error1} and {error2}");
        let result = parse_display_template(lit);
        assert!(result.is_ok());
        let template = result.ok().unwrap();
        assert_eq!(template.segments.len(), 3);
    }
}
