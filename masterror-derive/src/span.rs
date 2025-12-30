// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use core::{ops::Range, str::from_utf8};

use proc_macro2::Span;
use syn::LitStr;

/// Computes the span of a substring within a string literal.
///
/// The range is expressed in byte indices over the interpreted contents of the
/// literal (after unescaping). The function maps it back to a span over the
/// original source code when possible.
pub fn literal_subspan(lit: &LitStr, range: Range<usize>) -> Option<Span> {
    if range.start > range.end {
        return None;
    }
    let value = lit.value();
    if range.end > value.len() {
        return None;
    }
    let token = lit.token();
    let repr = token.to_string();
    if repr.starts_with('r') {
        raw_range(&repr, range).and_then(|sub| token.subspan(sub))
    } else {
        escaped_range(&repr, &value, range).and_then(|sub| token.subspan(sub))
    }
}

fn raw_range(repr: &str, range: Range<usize>) -> Option<Range<usize>> {
    let bytes = repr.as_bytes();
    let mut idx = 0usize;
    if bytes.get(idx)? != &b'r' {
        return None;
    }
    idx += 1;
    while matches!(bytes.get(idx), Some(b'#')) {
        idx += 1;
    }
    if bytes.get(idx)? != &b'"' {
        return None;
    }
    let hash_count = idx - 1;
    let start_content = idx + 1;
    let end_content = repr.len().checked_sub(hash_count + 1)?;
    if start_content > end_content || range.end > end_content - start_content {
        return None;
    }
    let start = start_content + range.start;
    let end = start_content + range.end;
    Some(start..end)
}

fn escaped_range(repr: &str, value: &str, range: Range<usize>) -> Option<Range<usize>> {
    let bytes = repr.as_bytes();
    if bytes.first()? != &b'"' || bytes.last()? != &b'"' {
        return None;
    }
    let mut mapping = vec![0usize; value.len() + 1];
    let mut token_pos = 1usize;
    let content_end = repr.len() - 1;
    let mut value_pos = 0usize;
    mapping[value_pos] = token_pos;
    while token_pos < content_end && value_pos < value.len() {
        if bytes[token_pos] == b'\\' {
            let escape_len = escape_sequence_len(&bytes[token_pos..content_end])?;
            let ch = value[value_pos..].chars().next()?;
            let produced = ch.len_utf8();
            for offset in 0..produced {
                mapping[value_pos + offset] = token_pos;
            }
            value_pos += produced;
            token_pos += escape_len;
            mapping[value_pos] = token_pos;
        } else {
            let ch = from_utf8(&bytes[token_pos..content_end])
                .ok()?
                .chars()
                .next()?;
            let char_len = ch.len_utf8();
            for offset in 0..char_len {
                mapping[value_pos + offset] = token_pos;
            }
            value_pos += ch.len_utf8();
            token_pos += char_len;
            mapping[value_pos] = token_pos;
        }
    }
    if value_pos != value.len() {
        return None;
    }
    mapping[value_pos] = content_end;
    if range.end > value.len() {
        return None;
    }
    Some(mapping[range.start]..mapping[range.end])
}

fn escape_sequence_len(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < 2 || bytes[0] != b'\\' {
        return None;
    }
    match bytes[1] {
        b'\\' | b'"' | b'\'' | b'n' | b'r' | b't' | b'0' => Some(2),
        b'x' => {
            if bytes.len() >= 4 {
                Some(4)
            } else {
                None
            }
        }
        b'u' => {
            let mut idx = 2usize;
            if bytes.get(idx)? != &b'{' {
                return None;
            }
            idx += 1;
            while idx < bytes.len() && bytes[idx] != b'}' {
                idx += 1;
            }
            if idx >= bytes.len() {
                return None;
            }
            Some(idx + 1)
        }
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    // Tests for literal_subspan()
    #[test]
    fn test_literal_subspan_valid_range_regular_string() {
        let lit: LitStr = parse_quote!("hello world");
        let result = literal_subspan(&lit, 0..5);
        assert!(result.is_some());
    }

    #[test]
    fn test_literal_subspan_valid_range_raw_string() {
        let lit: LitStr = parse_quote!(r"hello world");
        let result = literal_subspan(&lit, 0..5);
        assert!(result.is_some());
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    fn test_literal_subspan_invalid_range_start_greater_than_end() {
        let lit: LitStr = parse_quote!("hello");
        let result = literal_subspan(&lit, 5..2);
        assert!(result.is_none());
    }

    #[test]
    fn test_literal_subspan_invalid_range_end_exceeds_length() {
        let lit: LitStr = parse_quote!("hello");
        let result = literal_subspan(&lit, 0..100);
        assert!(result.is_none());
    }

    #[test]
    fn test_literal_subspan_empty_range_at_start() {
        let lit: LitStr = parse_quote!("hello");
        let result = literal_subspan(&lit, 0..0);
        assert!(result.is_some());
    }

    #[test]
    fn test_literal_subspan_empty_range_at_end() {
        let lit: LitStr = parse_quote!("hello");
        let result = literal_subspan(&lit, 5..5);
        assert!(result.is_some());
    }

    // Tests for raw_range()
    #[test]
    fn test_raw_range_simple() {
        let result = raw_range(r#"r"hello""#, 0..5);
        assert_eq!(result, Some(2..7));
    }

    #[test]
    fn test_raw_range_with_one_hash() {
        let result = raw_range(r##"r#"hello"#"##, 0..5);
        assert_eq!(result, Some(3..8));
    }

    #[test]
    fn test_raw_range_with_two_hashes() {
        let result = raw_range(r###"r##"hello"##"###, 0..5);
        assert_eq!(result, Some(4..9));
    }

    #[test]
    fn test_raw_range_not_starting_with_r() {
        let result = raw_range(r#""hello""#, 0..5);
        assert!(result.is_none());
    }

    #[test]
    fn test_raw_range_missing_opening_quote() {
        let result = raw_range("r#hello#", 0..5);
        assert!(result.is_none());
    }

    #[test]
    fn test_raw_range_exceeds_content() {
        let result = raw_range(r#"r"hi""#, 0..10);
        assert!(result.is_none());
    }

    #[test]
    fn test_raw_range_empty_content() {
        let result = raw_range(r#"r"""#, 0..0);
        assert_eq!(result, Some(2..2));
    }

    #[test]
    fn test_raw_range_empty_content_with_hash() {
        let result = raw_range(r##"r#""#"##, 0..0);
        assert_eq!(result, Some(3..3));
    }

    // Tests for escaped_range()
    #[test]
    fn test_escaped_range_no_escapes() {
        let result = escaped_range(r#""hello""#, "hello", 0..5);
        assert_eq!(result, Some(1..6));
    }

    #[test]
    fn test_escaped_range_newline_escape() {
        let result = escaped_range(r#""hello\nworld""#, "hello\nworld", 0..11);
        assert_eq!(result, Some(1..13));
    }

    #[test]
    fn test_escaped_range_tab_escape() {
        let result = escaped_range(r#""a\tb""#, "a\tb", 0..3);
        assert_eq!(result, Some(1..5));
    }

    #[test]
    fn test_escaped_range_carriage_return() {
        let result = escaped_range(r#""a\rb""#, "a\rb", 0..3);
        assert_eq!(result, Some(1..5));
    }

    #[test]
    fn test_escaped_range_backslash_escape() {
        let result = escaped_range(r#""a\\b""#, "a\\b", 0..3);
        assert_eq!(result, Some(1..5));
    }

    #[test]
    fn test_escaped_range_quote_escape() {
        let result = escaped_range(r#""a\"b""#, "a\"b", 0..3);
        assert_eq!(result, Some(1..5));
    }

    #[test]
    fn test_escaped_range_hex_escape() {
        let result = escaped_range(r#""\x41BC""#, "ABC", 0..3);
        assert_eq!(result, Some(1..7));
    }

    #[test]
    fn test_escaped_range_unicode_escape() {
        let result = escaped_range(r#""\u{1F4A9}""#, "\u{1F4A9}", 0..4);
        assert_eq!(result, Some(1..10));
    }

    #[test]
    fn test_escaped_range_unicode_escape_short() {
        let result = escaped_range(r#""\u{41}""#, "\u{41}", 0..1);
        assert_eq!(result, Some(1..7));
    }

    #[test]
    fn test_escaped_range_missing_quotes() {
        let result = escaped_range("hello", "hello", 0..5);
        assert!(result.is_none());
    }

    #[test]
    fn test_escaped_range_exceeds_content() {
        let result = escaped_range(r#""hi""#, "hi", 0..10);
        assert!(result.is_none());
    }

    #[test]
    fn test_escaped_range_multibyte_utf8() {
        let result = escaped_range(r#""café""#, "café", 0..5);
        assert_eq!(result, Some(1..6));
    }

    #[test]
    fn test_escaped_range_emoji() {
        let result = escaped_range(r#""😀""#, "😀", 0..4);
        assert_eq!(result, Some(1..5));
    }

    #[test]
    fn test_escaped_range_partial_with_escape() {
        let result = escaped_range(r#""hello\nworld""#, "hello\nworld", 0..6);
        assert_eq!(result, Some(1..8));
    }

    #[test]
    fn test_escaped_range_partial_after_escape() {
        let result = escaped_range(r#""hello\nworld""#, "hello\nworld", 6..11);
        assert_eq!(result, Some(8..13));
    }

    // Tests for escape_sequence_len()
    #[test]
    fn test_escape_sequence_len_backslash() {
        let result = escape_sequence_len(b"\\\\");
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_escape_sequence_len_double_quote() {
        let result = escape_sequence_len(b"\\\"");
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_escape_sequence_len_single_quote() {
        let result = escape_sequence_len(b"\\'");
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_escape_sequence_len_newline() {
        let result = escape_sequence_len(b"\\n");
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_escape_sequence_len_carriage_return() {
        let result = escape_sequence_len(b"\\r");
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_escape_sequence_len_tab() {
        let result = escape_sequence_len(b"\\t");
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_escape_sequence_len_null() {
        let result = escape_sequence_len(b"\\0");
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_escape_sequence_len_hex() {
        let result = escape_sequence_len(b"\\x41");
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_escape_sequence_len_hex_with_extra() {
        let result = escape_sequence_len(b"\\x41ABC");
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_escape_sequence_len_unicode_short() {
        let result = escape_sequence_len(b"\\u{41}");
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_escape_sequence_len_unicode_long() {
        let result = escape_sequence_len(b"\\u{1F4A9}");
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_escape_sequence_len_not_starting_with_backslash() {
        let result = escape_sequence_len(b"abc");
        assert!(result.is_none());
    }

    #[test]
    fn test_escape_sequence_len_incomplete_hex() {
        let result = escape_sequence_len(b"\\x4");
        assert!(result.is_none());
    }

    #[test]
    fn test_escape_sequence_len_incomplete_unicode_no_brace() {
        let result = escape_sequence_len(b"\\u41");
        assert!(result.is_none());
    }

    #[test]
    fn test_escape_sequence_len_incomplete_unicode_no_closing() {
        let result = escape_sequence_len(b"\\u{41");
        assert!(result.is_none());
    }

    #[test]
    fn test_escape_sequence_len_too_short() {
        let result = escape_sequence_len(b"\\");
        assert!(result.is_none());
    }

    #[test]
    fn test_escape_sequence_len_invalid_escape() {
        let result = escape_sequence_len(b"\\q");
        assert!(result.is_none());
    }

    #[test]
    fn test_escape_sequence_len_empty() {
        let result = escape_sequence_len(b"");
        assert!(result.is_none());
    }
}
