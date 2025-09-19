use core::{fmt, ops::Range};

mod parser;

/// Parsed representation of an `#[error("...")]` template.
///
/// Templates are represented as a sequence of literal segments and
/// placeholders.  The structure mirrors the internal representation used by
/// formatting machinery, but keeps the slices borrowed from the original input
/// to avoid unnecessary allocations.
///
/// # Examples
///
/// ```
/// use masterror_template::template::{ErrorTemplate, TemplateIdentifier};
///
/// let template = ErrorTemplate::parse("{code}: {message}").expect("parse");
/// let rendered = format!(
///     "{}",
///     template.display_with(|placeholder, f| match placeholder.identifier() {
///         TemplateIdentifier::Named("code") => write!(f, "{}", 404),
///         TemplateIdentifier::Named("message") => f.write_str("Not Found"),
///         _ => Ok(())
///     })
/// );
///
/// assert_eq!(rendered, "404: Not Found");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorTemplate<'a> {
    source:   &'a str,
    segments: Vec<TemplateSegment<'a>>
}

impl<'a> ErrorTemplate<'a> {
    /// Parses an error display template.
    pub fn parse(source: &'a str) -> Result<Self, TemplateError> {
        let segments = parser::parse_template(source)?;
        Ok(Self {
            source,
            segments
        })
    }

    /// Returns the original template string.
    pub const fn source(&self) -> &'a str {
        self.source
    }

    /// Returns the parsed segments.
    pub fn segments(&self) -> &[TemplateSegment<'a>] {
        &self.segments
    }

    /// Iterates over placeholder segments in order of appearance.
    pub fn placeholders(&self) -> impl Iterator<Item = &TemplatePlaceholder<'a>> {
        self.segments.iter().filter_map(|segment| match segment {
            TemplateSegment::Placeholder(placeholder) => Some(placeholder),
            TemplateSegment::Literal(_) => None
        })
    }

    /// Produces a display implementation that delegates placeholder rendering
    /// to the provided resolver.
    pub fn display_with<F>(&'a self, resolver: F) -> DisplayWith<'a, 'a, F>
    where
        F: Fn(&TemplatePlaceholder<'a>, &mut fmt::Formatter<'_>) -> fmt::Result
    {
        DisplayWith {
            template: self,
            resolver
        }
    }
}

/// A lazily formatted view over a template.
#[derive(Debug)]
pub struct DisplayWith<'a, 't, F>
where
    F: Fn(&TemplatePlaceholder<'a>, &mut fmt::Formatter<'_>) -> fmt::Result
{
    template: &'t ErrorTemplate<'a>,
    resolver: F
}

impl<'a, 't, F> fmt::Display for DisplayWith<'a, 't, F>
where
    F: Fn(&TemplatePlaceholder<'a>, &mut fmt::Formatter<'_>) -> fmt::Result
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.template.segments {
            match segment {
                TemplateSegment::Literal(literal) => f.write_str(literal)?,
                TemplateSegment::Placeholder(placeholder) => {
                    (self.resolver)(placeholder, f)?;
                }
            }
        }

        Ok(())
    }
}

/// A single segment of the parsed template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateSegment<'a> {
    /// Literal text copied verbatim.
    Literal(&'a str),
    /// Placeholder (`{name}` or `{0}`) that needs formatting.
    Placeholder(TemplatePlaceholder<'a>)
}

/// Placeholder metadata extracted from a template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplatePlaceholder<'a> {
    span:       Range<usize>,
    identifier: TemplateIdentifier<'a>,
    formatter:  TemplateFormatter
}

impl<'a> TemplatePlaceholder<'a> {
    /// Byte range (inclusive start, exclusive end) of the placeholder within
    /// the original template.
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    /// Returns the parsed identifier.
    pub const fn identifier(&self) -> &TemplateIdentifier<'a> {
        &self.identifier
    }

    /// Returns the requested formatter.
    pub const fn formatter(&self) -> TemplateFormatter {
        self.formatter
    }
}

/// Placeholder identifier parsed from the template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateIdentifier<'a> {
    /// Positional index (`{0}` / `{1:?}` / etc.).
    Positional(usize),
    /// Named field (`{name}` / `{kind:?}` / etc.).
    Named(&'a str)
}

impl<'a> TemplateIdentifier<'a> {
    /// Returns the identifier as a string when it is named.
    pub const fn as_str(&self) -> Option<&'a str> {
        match self {
            Self::Named(value) => Some(value),
            Self::Positional(_) => None
        }
    }
}

/// Formatter traits recognised within placeholders.
///
/// # Examples
///
/// ```
/// use masterror_template::template::{TemplateFormatter, TemplateFormatterKind};
///
/// let formatter = TemplateFormatter::LowerHex {
///     alternate: true
/// };
///
/// assert_eq!(formatter.kind(), TemplateFormatterKind::LowerHex);
/// assert_eq!(TemplateFormatterKind::LowerHex.specifier(), Some('x'));
/// assert!(TemplateFormatterKind::LowerHex.supports_alternate());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateFormatterKind {
    /// Default `Display` trait (`{value}`).
    Display,
    /// `Debug` trait (`{value:?}` / `{value:#?}`).
    Debug,
    /// `LowerHex` trait (`{value:x}` / `{value:#x}`).
    LowerHex,
    /// `UpperHex` trait (`{value:X}` / `{value:#X}`).
    UpperHex,
    /// `Pointer` trait (`{value:p}` / `{value:#p}`).
    Pointer,
    /// `Binary` trait (`{value:b}` / `{value:#b}`).
    Binary,
    /// `Octal` trait (`{value:o}` / `{value:#o}`).
    Octal,
    /// `LowerExp` trait (`{value:e}` / `{value:#e}`).
    LowerExp,
    /// `UpperExp` trait (`{value:E}` / `{value:#E}`).
    UpperExp
}

impl TemplateFormatterKind {
    /// Maps a format specifier character to a formatter kind.
    ///
    /// Returns `None` when the specifier is unsupported.
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror_template::template::TemplateFormatterKind;
    ///
    /// assert_eq!(
    ///     TemplateFormatterKind::from_specifier('?'),
    ///     Some(TemplateFormatterKind::Debug)
    /// );
    /// assert_eq!(TemplateFormatterKind::from_specifier('Q'), None);
    /// ```
    pub const fn from_specifier(specifier: char) -> Option<Self> {
        match specifier {
            '?' => Some(Self::Debug),
            'x' => Some(Self::LowerHex),
            'X' => Some(Self::UpperHex),
            'p' => Some(Self::Pointer),
            'b' => Some(Self::Binary),
            'o' => Some(Self::Octal),
            'e' => Some(Self::LowerExp),
            'E' => Some(Self::UpperExp),
            _ => None
        }
    }

    /// Returns the canonical format specifier character, if any.
    ///
    /// The default `Display` kind has no dedicated specifier and therefore
    /// returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror_template::template::TemplateFormatterKind;
    ///
    /// assert_eq!(TemplateFormatterKind::LowerHex.specifier(), Some('x'));
    /// assert_eq!(TemplateFormatterKind::Display.specifier(), None);
    /// ```
    pub const fn specifier(self) -> Option<char> {
        match self {
            Self::Display => None,
            Self::Debug => Some('?'),
            Self::LowerHex => Some('x'),
            Self::UpperHex => Some('X'),
            Self::Pointer => Some('p'),
            Self::Binary => Some('b'),
            Self::Octal => Some('o'),
            Self::LowerExp => Some('e'),
            Self::UpperExp => Some('E')
        }
    }

    /// Indicates whether the formatter kind supports the alternate (`#`) flag.
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror_template::template::TemplateFormatterKind;
    ///
    /// assert!(TemplateFormatterKind::Binary.supports_alternate());
    /// assert!(!TemplateFormatterKind::Display.supports_alternate());
    /// ```
    pub const fn supports_alternate(self) -> bool {
        !matches!(self, Self::Display)
    }
}

/// Formatting mode requested by the placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateFormatter {
    /// Default `Display` formatting (`{value}`).
    Display,
    /// `Debug` formatting (`{value:?}` or `{value:#?}`).
    Debug {
        /// Whether `{value:#?}` (alternate debug) was requested.
        alternate: bool
    },
    /// Lower-hexadecimal formatting (`{value:x}` / `{value:#x}`).
    LowerHex {
        /// Whether alternate formatting (`{value:#x}`) was requested.
        alternate: bool
    },
    /// Upper-hexadecimal formatting (`{value:X}` / `{value:#X}`).
    UpperHex {
        /// Whether alternate formatting (`{value:#X}`) was requested.
        alternate: bool
    },
    /// Pointer formatting (`{value:p}` / `{value:#p}`).
    Pointer {
        /// Whether alternate formatting (`{value:#p}`) was requested.
        alternate: bool
    },
    /// Binary formatting (`{value:b}` / `{value:#b}`).
    Binary {
        /// Whether alternate formatting (`{value:#b}`) was requested.
        alternate: bool
    },
    /// Octal formatting (`{value:o}` / `{value:#o}`).
    Octal {
        /// Whether alternate formatting (`{value:#o}`) was requested.
        alternate: bool
    },
    /// Lower exponential formatting (`{value:e}` / `{value:#e}`).
    LowerExp {
        /// Whether alternate formatting (`{value:#e}`) was requested.
        alternate: bool
    },
    /// Upper exponential formatting (`{value:E}` / `{value:#E}`).
    UpperExp {
        /// Whether alternate formatting (`{value:#E}`) was requested.
        alternate: bool
    }
}

impl TemplateFormatter {
    /// Constructs a formatter from a [`TemplateFormatterKind`] and `alternate`
    /// flag.
    ///
    /// The `alternate` flag is ignored for [`TemplateFormatterKind::Display`].
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror_template::template::{TemplateFormatter, TemplateFormatterKind};
    ///
    /// let formatter = TemplateFormatter::from_kind(TemplateFormatterKind::Binary, true);
    ///
    /// assert!(matches!(
    ///     formatter,
    ///     TemplateFormatter::Binary {
    ///         alternate: true
    ///     }
    /// ));
    /// ```
    pub const fn from_kind(kind: TemplateFormatterKind, alternate: bool) -> Self {
        match kind {
            TemplateFormatterKind::Display => Self::Display,
            TemplateFormatterKind::Debug => Self::Debug {
                alternate
            },
            TemplateFormatterKind::LowerHex => Self::LowerHex {
                alternate
            },
            TemplateFormatterKind::UpperHex => Self::UpperHex {
                alternate
            },
            TemplateFormatterKind::Pointer => Self::Pointer {
                alternate
            },
            TemplateFormatterKind::Binary => Self::Binary {
                alternate
            },
            TemplateFormatterKind::Octal => Self::Octal {
                alternate
            },
            TemplateFormatterKind::LowerExp => Self::LowerExp {
                alternate
            },
            TemplateFormatterKind::UpperExp => Self::UpperExp {
                alternate
            }
        }
    }

    /// Returns the underlying formatter kind.
    ///
    /// # Examples
    ///
    /// ```
    /// use masterror_template::template::{TemplateFormatter, TemplateFormatterKind};
    ///
    /// let formatter = TemplateFormatter::Pointer {
    ///     alternate: false
    /// };
    ///
    /// assert_eq!(formatter.kind(), TemplateFormatterKind::Pointer);
    /// ```
    pub const fn kind(&self) -> TemplateFormatterKind {
        match self {
            Self::Display => TemplateFormatterKind::Display,
            Self::Debug {
                ..
            } => TemplateFormatterKind::Debug,
            Self::LowerHex {
                ..
            } => TemplateFormatterKind::LowerHex,
            Self::UpperHex {
                ..
            } => TemplateFormatterKind::UpperHex,
            Self::Pointer {
                ..
            } => TemplateFormatterKind::Pointer,
            Self::Binary {
                ..
            } => TemplateFormatterKind::Binary,
            Self::Octal {
                ..
            } => TemplateFormatterKind::Octal,
            Self::LowerExp {
                ..
            } => TemplateFormatterKind::LowerExp,
            Self::UpperExp {
                ..
            } => TemplateFormatterKind::UpperExp
        }
    }

    /// Parses a formatting specifier (the portion after `:`) into a formatter.
    pub fn from_format_spec(spec: &str) -> Option<Self> {
        Self::parse_specifier(spec)
    }

    pub(crate) fn parse_specifier(spec: &str) -> Option<Self> {
        parser::parse_formatter_spec(spec)
    }

    /// Returns `true` when alternate formatting (`#`) was requested.
    pub const fn is_alternate(&self) -> bool {
        match self {
            Self::Display => false,
            Self::Debug {
                alternate
            }
            | Self::LowerHex {
                alternate
            }
            | Self::UpperHex {
                alternate
            }
            | Self::Pointer {
                alternate
            }
            | Self::Binary {
                alternate
            }
            | Self::Octal {
                alternate
            }
            | Self::LowerExp {
                alternate
            }
            | Self::UpperExp {
                alternate
            } => *alternate
        }
    }
}

/// Parsing errors produced when validating a template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateError {
    /// Encountered a stray closing brace.
    UnmatchedClosingBrace {
        /// Byte index of the stray `}` in the original template.
        index: usize
    },
    /// Placeholder without a matching closing brace.
    UnterminatedPlaceholder {
        /// Byte index where the unterminated placeholder starts.
        start: usize
    },
    /// Encountered `{{` or `}}` imbalance inside a placeholder.
    NestedPlaceholder {
        /// Byte index of the unexpected brace.
        index: usize
    },
    /// Placeholder without an identifier.
    EmptyPlaceholder {
        /// Byte index where the empty placeholder starts.
        start: usize
    },
    /// Identifier is malformed (contains illegal characters).
    InvalidIdentifier {
        /// Span (byte indices) covering the invalid identifier.
        span: Range<usize>
    },
    /// Positional identifier is not a valid unsigned integer.
    InvalidIndex {
        /// Span (byte indices) covering the invalid positional identifier.
        span: Range<usize>
    },
    /// Unsupported formatting specifier.
    InvalidFormatter {
        /// Span (byte indices) covering the unsupported formatter.
        span: Range<usize>
    }
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnmatchedClosingBrace {
                index
            } => {
                write!(f, "unmatched closing brace at byte {}", index)
            }
            Self::UnterminatedPlaceholder {
                start
            } => {
                write!(f, "placeholder starting at byte {} is not closed", start)
            }
            Self::NestedPlaceholder {
                index
            } => {
                write!(
                    f,
                    "nested placeholder starting at byte {} is not supported",
                    index
                )
            }
            Self::EmptyPlaceholder {
                start
            } => {
                write!(f, "placeholder starting at byte {} is empty", start)
            }
            Self::InvalidIdentifier {
                span
            } => {
                write!(
                    f,
                    "invalid placeholder identifier spanning bytes {}..{}",
                    span.start, span.end
                )
            }
            Self::InvalidIndex {
                span
            } => {
                write!(
                    f,
                    "positional placeholder spanning bytes {}..{} is not a valid unsigned integer",
                    span.start, span.end
                )
            }
            Self::InvalidFormatter {
                span
            } => {
                write!(
                    f,
                    "placeholder spanning bytes {}..{} uses an unsupported formatter",
                    span.start, span.end
                )
            }
        }
    }
}

impl std::error::Error for TemplateError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(name: &str) -> TemplateIdentifier<'_> {
        TemplateIdentifier::Named(name)
    }

    #[test]
    fn parses_basic_template() {
        let template = ErrorTemplate::parse("{code}: {message}").expect("parse");
        let segments = template.segments();

        assert_eq!(segments.len(), 3);
        assert!(matches!(segments[0], TemplateSegment::Placeholder(_)));
        assert!(matches!(segments[1], TemplateSegment::Literal(": ")));
        assert!(matches!(segments[2], TemplateSegment::Placeholder(_)));

        let placeholders: Vec<_> = template.placeholders().collect();
        assert_eq!(placeholders.len(), 2);
        assert_eq!(placeholders[0].identifier(), &named("code"));
        assert_eq!(placeholders[1].identifier(), &named("message"));
    }

    #[test]
    fn parses_debug_formatter() {
        let template = ErrorTemplate::parse("{0:#?}").expect("parse");
        let placeholders: Vec<_> = template.placeholders().collect();

        assert_eq!(placeholders.len(), 1);
        assert_eq!(
            placeholders[0].identifier(),
            &TemplateIdentifier::Positional(0)
        );
        assert_eq!(
            placeholders[0].formatter(),
            TemplateFormatter::Debug {
                alternate: true
            }
        );
        assert!(placeholders[0].formatter().is_alternate());
    }

    #[test]
    fn parses_extended_formatters() {
        let cases = [
            (
                "{value:x}",
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
                "{value:X}",
                TemplateFormatter::UpperHex {
                    alternate: false
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
                "{value:#E}",
                TemplateFormatter::UpperExp {
                    alternate: true
                }
            )
        ];

        for (template_str, expected) in &cases {
            let template = ErrorTemplate::parse(template_str).expect("parse");
            let placeholder = template.placeholders().next().expect("placeholder present");
            assert_eq!(placeholder.formatter(), *expected, "case: {template_str}");
        }
    }

    #[test]
    fn formatter_kind_helpers_cover_all_variants() {
        let table = [
            (TemplateFormatterKind::Debug, '?'),
            (TemplateFormatterKind::LowerHex, 'x'),
            (TemplateFormatterKind::UpperHex, 'X'),
            (TemplateFormatterKind::Pointer, 'p'),
            (TemplateFormatterKind::Binary, 'b'),
            (TemplateFormatterKind::Octal, 'o'),
            (TemplateFormatterKind::LowerExp, 'e'),
            (TemplateFormatterKind::UpperExp, 'E')
        ];

        for (kind, specifier) in table {
            assert_eq!(TemplateFormatterKind::from_specifier(specifier), Some(kind));
            assert_eq!(kind.specifier(), Some(specifier));

            let with_alternate = TemplateFormatter::from_kind(kind, true);
            let without_alternate = TemplateFormatter::from_kind(kind, false);

            assert_eq!(with_alternate.kind(), kind);
            assert_eq!(without_alternate.kind(), kind);

            if kind.supports_alternate() {
                assert!(with_alternate.is_alternate());
                assert!(!without_alternate.is_alternate());
            } else {
                assert!(!with_alternate.is_alternate());
                assert!(!without_alternate.is_alternate());
            }
        }

        let display = TemplateFormatter::from_kind(TemplateFormatterKind::Display, true);
        assert_eq!(display.kind(), TemplateFormatterKind::Display);
        assert!(!display.is_alternate());
        assert_eq!(TemplateFormatterKind::Display.specifier(), None);
        assert!(!TemplateFormatterKind::Display.supports_alternate());
    }

    #[test]
    fn handles_brace_escaping() {
        let template = ErrorTemplate::parse("{{}} -> {value}").expect("parse");
        let mut iter = template.segments().iter();

        assert!(matches!(iter.next(), Some(TemplateSegment::Literal("{"))));
        assert!(matches!(iter.next(), Some(TemplateSegment::Literal("}"))));
        assert!(matches!(
            iter.next(),
            Some(TemplateSegment::Literal(" -> "))
        ));
        assert!(matches!(
            iter.next(),
            Some(TemplateSegment::Placeholder(TemplatePlaceholder { .. }))
        ));
        assert!(iter.next().is_none());
    }

    #[test]
    fn rejects_unmatched_closing_brace() {
        let err = ErrorTemplate::parse("oops}").expect_err("should fail");
        assert!(matches!(
            err,
            TemplateError::UnmatchedClosingBrace {
                index: 4
            }
        ));
    }

    #[test]
    fn rejects_unterminated_placeholder() {
        let err = ErrorTemplate::parse("{oops").expect_err("should fail");
        assert!(matches!(
            err,
            TemplateError::UnterminatedPlaceholder {
                start: 0
            }
        ));
    }

    #[test]
    fn rejects_invalid_identifier() {
        let err = ErrorTemplate::parse("{invalid-name}").expect_err("should fail");
        assert!(matches!(err, TemplateError::InvalidIdentifier { span } if span == (0..14)));
    }

    #[test]
    fn rejects_unknown_formatter() {
        let err = ErrorTemplate::parse("{value:%}").expect_err("should fail");
        assert!(matches!(err, TemplateError::InvalidFormatter { span } if span == (0..9)));
    }

    #[test]
    fn display_with_resolves_placeholders() {
        let template = ErrorTemplate::parse("{code}: {message}").expect("parse");
        let code = 418;
        let message = "I'm a teapot";

        let rendered = format!(
            "{}",
            template.display_with(|placeholder, f| match placeholder.identifier() {
                TemplateIdentifier::Named("code") => write!(f, "{}", code),
                TemplateIdentifier::Named("message") => f.write_str(message),
                other => panic!("unexpected placeholder: {:?}", other)
            })
        );

        assert_eq!(rendered, "418: I'm a teapot");
    }
}
