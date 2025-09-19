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
                    formatter: placeholder.formatter()
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
