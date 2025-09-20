use core::ops::Range;

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
            let ch = core::str::from_utf8(&bytes[token_pos..content_end])
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
