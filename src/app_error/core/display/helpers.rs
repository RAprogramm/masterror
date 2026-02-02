// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Helper functions for display formatting.

use core::fmt::{Formatter, Result as FmtResult};

use crate::FieldValue;

#[allow(dead_code)]
/// Writes a string with JSON escaping.
pub(super) fn write_json_escaped(f: &mut Formatter<'_>, s: &str) -> FmtResult {
    for ch in s.chars() {
        match ch {
            '"' => write!(f, "\\\"")?,
            '\\' => write!(f, "\\\\")?,
            '\n' => write!(f, "\\n")?,
            '\r' => write!(f, "\\r")?,
            '\t' => write!(f, "\\t")?,
            ch if ch.is_control() => write!(f, "\\u{:04x}", ch as u32)?,
            ch => write!(f, "{}", ch)?
        }
    }
    Ok(())
}

#[allow(dead_code)]
/// Writes a metadata field value in JSON format.
pub(super) fn write_metadata_value(f: &mut Formatter<'_>, value: &FieldValue) -> FmtResult {
    match value {
        FieldValue::Str(s) => {
            write!(f, "\"")?;
            write_json_escaped(f, s.as_ref())?;
            write!(f, "\"")
        }
        FieldValue::I64(v) => write!(f, "{}", v),
        FieldValue::U64(v) => write!(f, "{}", v),
        FieldValue::F64(v) => {
            if v.is_finite() {
                write!(f, "{}", v)
            } else {
                write!(f, "null")
            }
        }
        FieldValue::Bool(v) => write!(f, "{}", v),
        FieldValue::Uuid(v) => write!(f, "\"{}\"", v),
        FieldValue::Duration(v) => {
            write!(
                f,
                r#"{{"secs":{},"nanos":{}}}"#,
                v.as_secs(),
                v.subsec_nanos()
            )
        }
        FieldValue::Ip(v) => write!(f, "\"{}\"", v),
        #[cfg(feature = "serde_json")]
        FieldValue::Json(v) => write!(f, "{}", v)
    }
}

#[allow(dead_code)]
/// Writes metadata as JSON object, respecting field redaction policies.
pub(super) fn write_metadata_json(f: &mut Formatter<'_>, metadata: &crate::Metadata) -> FmtResult {
    use crate::FieldRedaction;

    if metadata.is_empty() {
        return Ok(());
    }

    let has_public_fields = metadata
        .iter_with_redaction()
        .any(|(_, _, redaction)| !matches!(redaction, FieldRedaction::Redact));

    if !has_public_fields {
        return Ok(());
    }

    write!(f, r#","metadata":{{"#)?;
    let mut first = true;
    for (name, value, redaction) in metadata.iter_with_redaction() {
        if matches!(redaction, FieldRedaction::Redact) {
            continue;
        }
        if !first {
            write!(f, ",")?;
        }
        first = false;
        write!(f, r#""{}":"#, name)?;
        write_metadata_value(f, value)?;
    }
    write!(f, "}}")
}
