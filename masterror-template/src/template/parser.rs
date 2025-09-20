use core::ops::Range;

use super::{
    TemplateError, TemplateFormatter, TemplateFormatterKind, TemplateIdentifier,
    TemplatePlaceholder, TemplateSegment
};

pub fn parse_template<'a>(source: &'a str) -> Result<Vec<TemplateSegment<'a>>, TemplateError> {
    let mut segments = Vec::new();
    let mut iter = source.char_indices().peekable();
    let mut literal_start = 0usize;
    let mut implicit_counter = 0usize;

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

                let parsed = parse_placeholder(source, index, &mut implicit_counter)?;
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
    start: usize,
    implicit_counter: &mut usize
) -> Result<ParsedPlaceholder<'a>, TemplateError> {
    for (offset, ch) in source[start + 1..].char_indices() {
        let absolute = start + 1 + offset;
        match ch {
            '}' => {
                let end = absolute;
                let placeholder = build_placeholder(source, start, end, implicit_counter)?;
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
    end: usize,
    implicit_counter: &mut usize
) -> Result<TemplatePlaceholder<'a>, TemplateError> {
    let span = start..(end + 1);
    let body = &source[start + 1..end];

    if body.is_empty() {
        let identifier = next_implicit_identifier(implicit_counter, &span)?;
        return Ok(TemplatePlaceholder {
            span,
            identifier,
            formatter: TemplateFormatter::Display {
                spec: None
            }
        });
    }

    let trimmed = body.trim();

    if trimmed.is_empty() {
        return Err(TemplateError::EmptyPlaceholder {
            start
        });
    }

    let (identifier, formatter) = split_placeholder(trimmed, span.clone(), implicit_counter)?;

    Ok(TemplatePlaceholder {
        span,
        identifier,
        formatter
    })
}

fn split_placeholder<'a>(
    body: &'a str,
    span: Range<usize>,
    implicit_counter: &mut usize
) -> Result<(TemplateIdentifier<'a>, TemplateFormatter), TemplateError> {
    let mut parts = body.splitn(2, ':');
    let identifier_text = parts.next().unwrap_or("").trim();

    let identifier = parse_identifier(identifier_text, span.clone(), implicit_counter)?;

    let formatter = match parts.next().map(str::trim) {
        None => TemplateFormatter::Display {
            spec: None
        },
        Some("") => {
            return Err(TemplateError::InvalidFormatter {
                span
            });
        }
        Some(spec) => parse_formatter(spec, span.clone())?
    };

    Ok((identifier, formatter))
}

fn parse_formatter(spec: &str, span: Range<usize>) -> Result<TemplateFormatter, TemplateError> {
    parse_formatter_spec(spec).ok_or(TemplateError::InvalidFormatter {
        span
    })
}

pub(super) fn parse_formatter_spec(spec: &str) -> Option<TemplateFormatter> {
    let trimmed = spec.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Some((last_index, ty)) = trimmed.char_indices().next_back() {
        if let Some(kind) = TemplateFormatterKind::from_specifier(ty) {
            let prefix = &trimmed[..last_index];
            let alternate = detect_alternate_flag(prefix)?;

            return Some(TemplateFormatter::from_kind(kind, alternate));
        }

        if ty.is_ascii_alphabetic() {
            return None;
        }
    }

    if !display_allows_hash(trimmed) {
        return None;
    }

    if trimmed.chars().any(|ch| matches!(ch, '%' | '{' | '}')) {
        return None;
    }

    Some(TemplateFormatter::Display {
        spec: Some(trimmed.to_owned().into_boxed_str())
    })
}

fn detect_alternate_flag(prefix: &str) -> Option<bool> {
    let mut rest = prefix;

    if rest.len() >= 2 {
        let mut iter = rest.char_indices();
        if let (Some((_, _)), Some((second_index, second))) = (iter.next(), iter.next())
            && matches!(second, '<' | '>' | '^' | '=')
        {
            let skip = second_index + second.len_utf8();
            rest = &rest[skip..];
        }
    }

    if let Some(first) = rest.chars().next()
        && matches!(first, '<' | '>' | '^' | '=')
    {
        rest = &rest[first.len_utf8()..];
    }

    loop {
        let mut chars = rest.chars();
        let Some(ch) = chars.next() else {
            return Some(false);
        };

        match ch {
            '+' | '-' | ' ' => {
                rest = &rest[ch.len_utf8()..];
            }
            '#' => {
                rest = &rest[ch.len_utf8()..];
                if rest.chars().any(|value| value == '#') {
                    return None;
                }
                return Some(true);
            }
            _ => return Some(false)
        }
    }
}

fn display_allows_hash(spec: &str) -> bool {
    match spec.find('#') {
        None => true,
        Some(0) => {
            let mut chars = spec.chars();
            let _ = chars.next();
            let Some(align) = chars.next() else {
                return false;
            };

            if !matches!(align, '<' | '>' | '^' | '=') {
                return false;
            }

            chars.all(|ch| ch != '#')
        }
        Some(_) => false
    }
}

fn parse_identifier<'a>(
    text: &'a str,
    span: Range<usize>,
    implicit_counter: &mut usize
) -> Result<TemplateIdentifier<'a>, TemplateError> {
    if text.is_empty() {
        return next_implicit_identifier(implicit_counter, &span);
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

fn next_implicit_identifier<'a>(
    implicit_counter: &mut usize,
    span: &Range<usize>
) -> Result<TemplateIdentifier<'a>, TemplateError> {
    let index = *implicit_counter;
    *implicit_counter = index
        .checked_add(1)
        .ok_or_else(|| TemplateError::InvalidIdentifier {
            span: span.clone()
        })?;

    Ok(TemplateIdentifier::Implicit(index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_formatter_specs() {
        let cases = [
            (
                "{value:?}",
                TemplateFormatter::Debug {
                    alternate: false
                }
            ),
            (
                "{value:#?}",
                TemplateFormatter::Debug {
                    alternate: true
                }
            ),
            (
                "{value:*>#?}",
                TemplateFormatter::Debug {
                    alternate: true
                }
            ),
            (
                "{value:#>8?}",
                TemplateFormatter::Debug {
                    alternate: false
                }
            ),
            (
                "{value:x}",
                TemplateFormatter::LowerHex {
                    alternate: false
                }
            ),
            (
                "{value:>08x}",
                TemplateFormatter::LowerHex {
                    alternate: false
                }
            ),
            (
                "{value:#x}",
                TemplateFormatter::LowerHex {
                    alternate: true
                }
            ),
            (
                "{value:*<#x}",
                TemplateFormatter::LowerHex {
                    alternate: true
                }
            ),
            (
                "{value:X}",
                TemplateFormatter::UpperHex {
                    alternate: false
                }
            ),
            (
                "{value:*>#X}",
                TemplateFormatter::UpperHex {
                    alternate: true
                }
            ),
            (
                "{value:#X}",
                TemplateFormatter::UpperHex {
                    alternate: true
                }
            ),
            (
                "{value:p}",
                TemplateFormatter::Pointer {
                    alternate: false
                }
            ),
            (
                "{value:>+#18p}",
                TemplateFormatter::Pointer {
                    alternate: true
                }
            ),
            (
                "{value:#p}",
                TemplateFormatter::Pointer {
                    alternate: true
                }
            ),
            (
                "{value:b}",
                TemplateFormatter::Binary {
                    alternate: false
                }
            ),
            (
                "{value:#08b}",
                TemplateFormatter::Binary {
                    alternate: true
                }
            ),
            (
                "{value:#b}",
                TemplateFormatter::Binary {
                    alternate: true
                }
            ),
            (
                "{value:o}",
                TemplateFormatter::Octal {
                    alternate: false
                }
            ),
            (
                "{value:+#o}",
                TemplateFormatter::Octal {
                    alternate: true
                }
            ),
            (
                "{value:#o}",
                TemplateFormatter::Octal {
                    alternate: true
                }
            ),
            (
                "{value:e}",
                TemplateFormatter::LowerExp {
                    alternate: false
                }
            ),
            (
                "{value:#0e}",
                TemplateFormatter::LowerExp {
                    alternate: true
                }
            ),
            (
                "{value:#e}",
                TemplateFormatter::LowerExp {
                    alternate: true
                }
            ),
            (
                "{value:E}",
                TemplateFormatter::UpperExp {
                    alternate: false
                }
            ),
            (
                "{value:#^10E}",
                TemplateFormatter::UpperExp {
                    alternate: false
                }
            ),
            (
                "{value:#E}",
                TemplateFormatter::UpperExp {
                    alternate: true
                }
            )
        ];

        for (source, expected_formatter) in &cases {
            let segments = parse_template(source).expect("template parsed");
            let placeholder = match segments.first() {
                Some(TemplateSegment::Placeholder(placeholder)) => placeholder,
                other => panic!("unexpected segments for {source:?}: {other:?}")
            };

            assert_eq!(
                placeholder.formatter(),
                expected_formatter,
                "case: {source}"
            );
        }
    }

    #[test]
    fn rejects_malformed_formatters() {
        let cases = [
            "{value:}",
            "{value:#}",
            "{value:#4}",
            "{value:>8q}",
            "{value:##x}"
        ];

        for source in &cases {
            let err = parse_template(source).expect_err("expected formatter error");
            assert!(
                matches!(err, TemplateError::InvalidFormatter { span } if span == (0..source.len()))
            );
        }
    }

    #[test]
    fn parses_display_format_specs() {
        let cases = [
            ("{value:>8}", ">8"),
            ("{value:.3}", ".3"),
            ("{value:*<10}", "*<10"),
            ("{value:#>4}", "#>4"),
            ("{value:#>+6}", "#>+6")
        ];

        for (source, expected_spec) in cases {
            let segments = parse_template(source).expect("template parsed");
            let placeholder = match segments.first() {
                Some(TemplateSegment::Placeholder(placeholder)) => placeholder,
                other => panic!("unexpected segments for {source:?}: {other:?}")
            };

            let formatter = placeholder.formatter();
            assert!(formatter.display_spec().is_some());
            assert_eq!(formatter.display_spec(), Some(expected_spec));
        }
    }

    #[test]
    fn parses_empty_braces_as_implicit_display() {
        let segments = parse_template("{}").expect("template parsed");
        let placeholder = match segments.first() {
            Some(TemplateSegment::Placeholder(placeholder)) => placeholder,
            other => panic!("unexpected segments for empty braces: {other:?}")
        };

        assert_eq!(placeholder.identifier(), &TemplateIdentifier::Implicit(0));
        assert_eq!(
            placeholder.formatter(),
            &TemplateFormatter::Display {
                spec: None
            }
        );
    }

    #[test]
    fn increments_implicit_indices_across_placeholders() {
        let segments = parse_template("{}, {value}, {:?}, {}").expect("template parsed");
        let placeholders: Vec<_> = segments
            .iter()
            .filter_map(|segment| match segment {
                TemplateSegment::Placeholder(placeholder) => Some(placeholder),
                TemplateSegment::Literal(_) => None
            })
            .collect();

        assert_eq!(placeholders.len(), 4);
        assert_eq!(
            placeholders[0].identifier(),
            &TemplateIdentifier::Implicit(0)
        );
        assert_eq!(
            placeholders[1].identifier(),
            &TemplateIdentifier::Named("value")
        );
        assert_eq!(
            placeholders[2].identifier(),
            &TemplateIdentifier::Implicit(1)
        );
        assert_eq!(
            placeholders[2].formatter(),
            &TemplateFormatter::Debug {
                alternate: false
            }
        );
        assert_eq!(
            placeholders[3].identifier(),
            &TemplateIdentifier::Implicit(2)
        );
    }

    #[test]
    fn rejects_whitespace_only_placeholders() {
        let err = parse_template("{   }").expect_err("should fail");
        assert!(matches!(
            err,
            TemplateError::EmptyPlaceholder {
                start: 0
            }
        ));
    }
}
