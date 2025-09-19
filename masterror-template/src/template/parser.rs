use core::ops::Range;

use super::{
    TemplateError, TemplateFormatter, TemplateIdentifier, TemplatePlaceholder, TemplateSegment
};

pub fn parse_template<'a>(source: &'a str) -> Result<Vec<TemplateSegment<'a>>, TemplateError> {
    let mut segments = Vec::new();
    let mut iter = source.char_indices().peekable();
    let mut literal_start = 0usize;

    while let Some((index, ch)) = iter.next() {
        match ch {
            '{' => {
                if matches!(iter.peek(), Some(&(_, '{'))) {
                    if index > literal_start {
                        segments.push(TemplateSegment::Literal(&source[literal_start..index]));
                    }

                    segments.push(TemplateSegment::Literal(
                        &source[index..index + ch.len_utf8()]
                    ));

                    if let Some((_, escaped)) = iter.next() {
                        literal_start = index + ch.len_utf8() + escaped.len_utf8();
                    } else {
                        return Err(TemplateError::UnterminatedPlaceholder {
                            start: index
                        });
                    }
                    continue;
                }

                if index > literal_start {
                    segments.push(TemplateSegment::Literal(&source[literal_start..index]));
                }

                let parsed = parse_placeholder(source, index)?;
                segments.push(TemplateSegment::Placeholder(parsed.placeholder));

                literal_start = parsed.after;
                while matches!(iter.peek(), Some(&(next_index, _)) if next_index < parsed.after) {
                    iter.next();
                }
            }
            '}' => {
                if matches!(iter.peek(), Some(&(_, '}'))) {
                    if index > literal_start {
                        segments.push(TemplateSegment::Literal(&source[literal_start..index]));
                    }

                    segments.push(TemplateSegment::Literal(
                        &source[index..index + ch.len_utf8()]
                    ));

                    if let Some((_, escaped)) = iter.next() {
                        literal_start = index + ch.len_utf8() + escaped.len_utf8();
                    } else {
                        return Err(TemplateError::UnterminatedPlaceholder {
                            start: index
                        });
                    }
                    continue;
                }

                return Err(TemplateError::UnmatchedClosingBrace {
                    index
                });
            }
            _ => {}
        }
    }

    if literal_start < source.len() {
        segments.push(TemplateSegment::Literal(&source[literal_start..]));
    }

    Ok(segments)
}

struct ParsedPlaceholder<'a> {
    placeholder: TemplatePlaceholder<'a>,
    after:       usize
}

fn parse_placeholder<'a>(
    source: &'a str,
    start: usize
) -> Result<ParsedPlaceholder<'a>, TemplateError> {
    for (offset, ch) in source[start + 1..].char_indices() {
        let absolute = start + 1 + offset;
        match ch {
            '}' => {
                let end = absolute;
                let placeholder = build_placeholder(source, start, end)?;
                return Ok(ParsedPlaceholder {
                    placeholder,
                    after: end + 1
                });
            }
            '{' => {
                return Err(TemplateError::NestedPlaceholder {
                    index: absolute
                });
            }
            _ => {}
        }
    }

    Err(TemplateError::UnterminatedPlaceholder {
        start
    })
}

fn build_placeholder<'a>(
    source: &'a str,
    start: usize,
    end: usize
) -> Result<TemplatePlaceholder<'a>, TemplateError> {
    let span = start..(end + 1);
    let body = &source[start + 1..end];
    let trimmed = body.trim();

    if trimmed.is_empty() {
        return Err(TemplateError::EmptyPlaceholder {
            start
        });
    }

    let (identifier, formatter) = split_placeholder(trimmed, span.clone())?;

    Ok(TemplatePlaceholder {
        span,
        identifier,
        formatter
    })
}

fn split_placeholder<'a>(
    body: &'a str,
    span: Range<usize>
) -> Result<(TemplateIdentifier<'a>, TemplateFormatter), TemplateError> {
    let mut parts = body.splitn(2, ':');
    let identifier_text = parts.next().unwrap_or("").trim();

    let identifier = parse_identifier(identifier_text, span.clone())?;

    let formatter = match parts.next().map(str::trim) {
        None => TemplateFormatter::Display,
        Some("?") => TemplateFormatter::Debug {
            alternate: false
        },
        Some("#?") => TemplateFormatter::Debug {
            alternate: true
        },
        Some("") => {
            return Err(TemplateError::InvalidFormatter {
                span
            });
        }
        Some(_) => {
            return Err(TemplateError::InvalidFormatter {
                span
            });
        }
    };

    Ok((identifier, formatter))
}

fn parse_identifier<'a>(
    text: &'a str,
    span: Range<usize>
) -> Result<TemplateIdentifier<'a>, TemplateError> {
    if text.is_empty() {
        return Err(TemplateError::EmptyPlaceholder {
            start: span.start
        });
    }

    if text.chars().all(|ch| ch.is_ascii_digit()) {
        let value = text
            .parse::<usize>()
            .map_err(|_| TemplateError::InvalidIndex {
                span: span.clone()
            })?;
        return Ok(TemplateIdentifier::Positional(value));
    }

    if text
        .chars()
        .all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
    {
        return Ok(TemplateIdentifier::Named(text));
    }

    Err(TemplateError::InvalidIdentifier {
        span
    })
}
