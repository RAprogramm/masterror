// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Shared redaction helpers applied to metadata field values.
//!
//! Implements the value transformations backing
//! [`FieldRedaction`](crate::FieldRedaction) policies: placeholder
//! substitution, SHA-256 hashing and last-four masking. The same helpers power
//! the `Display` layouts of [`Error`](crate::Error) and the RFC 7807
//! [`ProblemJson`](crate::ProblemJson) payloads so redaction behaves
//! identically everywhere.

use alloc::string::{String, ToString};
use core::{fmt::Write, iter::repeat_n, str::from_utf8};

use itoa::Buffer as IntegerBuffer;
use ryu::Buffer as FloatBuffer;
use sha2::{Digest, Sha256};

use super::{duration_to_string, metadata::FieldValue};

/// Placeholder rendered instead of values redacted with
/// [`FieldRedaction::Redact`](crate::FieldRedaction::Redact).
pub(crate) const REDACTED_PLACEHOLDER: &str = "[REDACTED]";

/// Fixed-capacity stack buffer used to format short values without
/// allocating.
struct StackBuffer<const N: usize> {
    buf: [u8; N],
    len: usize
}

impl<const N: usize> StackBuffer<N> {
    const fn new() -> Self {
        Self {
            buf: [0; N],
            len: 0
        }
    }

    fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    fn as_str(&self) -> Option<&str> {
        from_utf8(self.as_bytes()).ok()
    }
}

impl<const N: usize> Write for StackBuffer<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let remaining = N.saturating_sub(self.len);
        if s.len() > remaining {
            return Err(core::fmt::Error);
        }
        self.buf[self.len..self.len + s.len()].copy_from_slice(s.as_bytes());
        self.len += s.len();
        Ok(())
    }
}

/// Hashes a field value with SHA-256 and returns the lowercase hex digest.
pub(crate) fn hash_field_value(value: &FieldValue) -> String {
    let mut hasher = Sha256::new();
    match value {
        FieldValue::Str(value) => hasher.update(value.as_ref().as_bytes()),
        FieldValue::I64(value) => {
            let mut buffer = IntegerBuffer::new();
            hasher.update(buffer.format(*value).as_bytes());
        }
        FieldValue::U64(value) => {
            let mut buffer = IntegerBuffer::new();
            hasher.update(buffer.format(*value).as_bytes());
        }
        FieldValue::F64(value) => hasher.update(value.to_le_bytes()),
        FieldValue::Bool(value) => {
            if *value {
                hasher.update(b"true");
            } else {
                hasher.update(b"false");
            }
        }
        FieldValue::Uuid(value) => {
            let mut repr = [0u8; 36];
            let text = value.hyphenated().encode_lower(&mut repr);
            hasher.update(text.as_bytes());
        }
        FieldValue::Duration(value) => {
            hasher.update(value.as_secs().to_le_bytes());
            hasher.update(value.subsec_nanos().to_le_bytes());
        }
        FieldValue::Ip(value) => {
            let mut buffer = StackBuffer::<46>::new();
            if write!(&mut buffer, "{value}").is_ok() {
                hasher.update(buffer.as_bytes());
            } else {
                let fallback = value.to_string();
                hasher.update(fallback.as_bytes());
            }
        }
        #[cfg(feature = "serde_json")]
        FieldValue::Json(value) => {
            if let Ok(serialized) = serde_json::to_vec(value) {
                hasher.update(&serialized);
            }
        }
    }
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        let _ = write!(&mut hex, "{:02x}", byte);
    }
    hex
}

/// Masks a field value keeping only its last four characters.
///
/// Returns `None` for values that have no meaningful textual form to mask
/// (currently booleans), in which case the field must be omitted entirely.
pub(crate) fn mask_last4_field_value(value: &FieldValue) -> Option<String> {
    match value {
        FieldValue::Str(value) => Some(mask_last4(value.as_ref())),
        FieldValue::I64(value) => {
            let mut buffer = IntegerBuffer::new();
            Some(mask_last4(buffer.format(*value)))
        }
        FieldValue::U64(value) => {
            let mut buffer = IntegerBuffer::new();
            Some(mask_last4(buffer.format(*value)))
        }
        FieldValue::F64(value) => {
            let mut buffer = FloatBuffer::new();
            Some(mask_last4(buffer.format(*value)))
        }
        FieldValue::Uuid(value) => {
            let mut repr = [0u8; 36];
            let text = value.hyphenated().encode_lower(&mut repr);
            Some(mask_last4(text))
        }
        FieldValue::Duration(value) => Some(mask_last4(&duration_to_string(*value))),
        FieldValue::Ip(value) => {
            let mut buffer = StackBuffer::<46>::new();
            if write!(&mut buffer, "{value}").is_err() {
                return Some(mask_last4(&value.to_string()));
            }
            buffer.as_str().map(mask_last4)
        }
        #[cfg(feature = "serde_json")]
        FieldValue::Json(value) => serde_json::to_string(value)
            .ok()
            .map(|text| mask_last4(&text)),
        FieldValue::Bool(_) => None
    }
}

/// Replaces all but the trailing characters of `value` with `*`.
///
/// Values of four characters or fewer keep only the last character.
pub(crate) fn mask_last4(value: &str) -> String {
    let chars = value.chars();
    let total = chars.clone().count();
    if total == 0 {
        return String::new();
    }
    let keep = if total <= 4 { 1 } else { 4 };
    let mask_len = total.saturating_sub(keep);
    let mut masked = String::with_capacity(value.len());
    masked.extend(repeat_n('*', mask_len));
    masked.extend(chars.skip(mask_len));
    masked
}
