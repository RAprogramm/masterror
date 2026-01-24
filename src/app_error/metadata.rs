// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use alloc::{borrow::Cow, string::String};
use core::{
    fmt::{Display, Formatter, Result as FmtResult, Write},
    net::IpAddr,
    time::Duration
};

use super::inline_vec::InlineVec;

/// Redaction policy associated with a metadata [`Field`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldRedaction {
    /// Preserve the value as-is.
    #[default]
    None,
    /// Remove the value from public payloads.
    Redact,
    /// Hash the value with a cryptographic digest before exposure.
    Hash,
    /// Preserve only the last four characters (mask the rest).
    Last4
}

#[cfg(feature = "serde_json")]
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// Value stored inside [`Metadata`].
///
/// The enum keeps the most common telemetry-friendly primitives without forcing
/// callers to allocate temporary strings. Strings use [`Cow`] so `'static`
/// literals avoid allocation while owned [`String`]s are supported when
/// necessary.
#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    /// Human-readable string.
    Str(Cow<'static, str>),
    /// Signed 64-bit integer.
    I64(i64),
    /// Unsigned 64-bit integer.
    U64(u64),
    /// Floating-point value.
    F64(f64),
    /// Boolean flag.
    Bool(bool),
    /// UUID represented with the canonical binary type.
    Uuid(Uuid),
    /// Elapsed duration captured with nanosecond precision.
    Duration(Duration),
    /// IP address (v4 or v6).
    Ip(IpAddr),
    /// Structured JSON payload (requires the `serde_json` feature).
    #[cfg(feature = "serde_json")]
    Json(JsonValue)
}

impl Display for FieldValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Str(value) => Display::fmt(value, f),
            Self::I64(value) => Display::fmt(value, f),
            Self::U64(value) => Display::fmt(value, f),
            Self::F64(value) => Display::fmt(value, f),
            Self::Bool(value) => Display::fmt(value, f),
            Self::Uuid(value) => Display::fmt(value, f),
            Self::Duration(value) => format_duration(*value, f),
            Self::Ip(value) => Display::fmt(value, f),
            #[cfg(feature = "serde_json")]
            Self::Json(value) => Display::fmt(value, f)
        }
    }
}

#[derive(Clone, Copy)]
struct TrimmedFraction {
    value: u32,
    width: u8
}

fn duration_parts(duration: Duration) -> (u64, Option<TrimmedFraction>) {
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();
    if nanos == 0 {
        return (secs, None);
    }
    let mut fraction = nanos;
    let mut width = 9u8;
    loop {
        let divided = fraction / 10;
        if divided * 10 != fraction {
            break;
        }
        fraction = divided;
        width -= 1;
    }
    (
        secs,
        Some(TrimmedFraction {
            value: fraction,
            width
        })
    )
}

fn format_duration(duration: Duration, f: &mut Formatter<'_>) -> FmtResult {
    let (secs, fraction) = duration_parts(duration);
    if let Some(fraction) = fraction {
        write!(
            f,
            "{}.{:0width$}s",
            secs,
            fraction.value,
            width = fraction.width as usize
        )
    } else {
        write!(f, "{}s", secs)
    }
}

pub(crate) fn duration_to_string(duration: Duration) -> String {
    let (secs, fraction) = duration_parts(duration);
    let mut output = String::new();
    if let Some(fraction) = fraction {
        let _ = write!(
            &mut output,
            "{}.{:0width$}s",
            secs,
            fraction.value,
            width = fraction.width as usize
        );
    } else {
        let _ = write!(&mut output, "{}s", secs);
    }
    output
}

/// Single metadata field – name plus value.
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    name:      &'static str,
    value:     FieldValue,
    redaction: FieldRedaction
}

impl Field {
    /// Create a new [`Field`].
    #[must_use]
    pub fn new(name: &'static str, value: FieldValue) -> Self {
        let redaction = infer_default_redaction(name);
        Self {
            name,
            value,
            redaction
        }
    }

    /// Field name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Field value.
    #[must_use]
    pub const fn value(&self) -> &FieldValue {
        &self.value
    }

    /// Field redaction policy.
    #[must_use]
    pub const fn redaction(&self) -> FieldRedaction {
        self.redaction
    }

    /// Override the redaction policy while consuming the field.
    #[must_use]
    pub fn with_redaction(mut self, redaction: FieldRedaction) -> Self {
        self.redaction = redaction;
        self
    }

    /// Update the redaction policy in place.
    pub fn set_redaction(&mut self, redaction: FieldRedaction) {
        self.redaction = redaction;
    }

    /// Consume the field and return owned components.
    #[must_use]
    pub fn into_parts(self) -> (&'static str, FieldValue, FieldRedaction) {
        (self.name, self.value, self.redaction)
    }

    /// Consume the field and return only the value.
    #[must_use]
    pub fn into_value(self) -> FieldValue {
        self.value
    }
}

fn infer_default_redaction(name: &str) -> FieldRedaction {
    if contains_ascii_case_insensitive(name, "password")
        || contains_ascii_case_insensitive(name, "passphrase")
        || contains_ascii_case_insensitive(name, "secret")
        || contains_ascii_case_insensitive(name, "authorization")
        || contains_ascii_case_insensitive(name, "cookie")
        || contains_ascii_case_insensitive(name, "session")
        || contains_ascii_case_insensitive(name, "jwt")
        || contains_ascii_case_insensitive(name, "bearer")
        || contains_ascii_case_insensitive(name, "otp")
        || contains_ascii_case_insensitive(name, "pin")
    {
        return FieldRedaction::Redact;
    }
    let mut card_like = false;
    let mut number_like = false;
    let has_token = contains_ascii_case_insensitive(name, "token");
    let has_key = contains_ascii_case_insensitive(name, "key");
    for segment in name.split(['.', '_', '-', ':', '/']) {
        if segment.is_empty() {
            continue;
        }
        if segment.eq_ignore_ascii_case("token")
            || segment.eq_ignore_ascii_case("apikey")
            || segment.eq_ignore_ascii_case("api") && has_key
            || ends_with_ascii_case_insensitive(segment, "token")
            || segment.eq_ignore_ascii_case("key")
            || segment.eq_ignore_ascii_case("access") && has_token
            || segment.eq_ignore_ascii_case("refresh") && has_token
        {
            return FieldRedaction::Hash;
        }
        if segment.eq_ignore_ascii_case("card")
            || segment.eq_ignore_ascii_case("iban")
            || segment.eq_ignore_ascii_case("pan")
            || segment.eq_ignore_ascii_case("account")
            || segment.eq_ignore_ascii_case("acct")
        {
            card_like = true;
        }
        if segment.eq_ignore_ascii_case("number")
            || segment.eq_ignore_ascii_case("no")
            || segment.eq_ignore_ascii_case("id")
        {
            number_like = true;
        }
    }
    if card_like && number_like {
        FieldRedaction::Last4
    } else {
        FieldRedaction::None
    }
}

fn ends_with_ascii_case_insensitive(value: &str, suffix: &str) -> bool {
    let value_bytes = value.as_bytes();
    let suffix_bytes = suffix.as_bytes();
    value_bytes.len() >= suffix_bytes.len()
        && eq_ascii_case_insensitive_bytes(
            &value_bytes[value_bytes.len() - suffix_bytes.len()..],
            suffix_bytes
        )
}

fn contains_ascii_case_insensitive(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    haystack_bytes.len() >= needle_bytes.len()
        && haystack_bytes
            .windows(needle_bytes.len())
            .any(|window| eq_ascii_case_insensitive_bytes(window, needle_bytes))
}

fn eq_ascii_case_insensitive_bytes(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(&lhs, &rhs)| lhs.eq_ignore_ascii_case(&rhs))
}

/// Structured metadata attached to [`crate::AppError`].
///
/// Internally backed by a sorted inline vector for optimal performance with
/// small field counts. Fields are kept sorted by name for deterministic
/// iteration and O(log n) lookup via binary search.
///
/// # Performance
///
/// Most errors have 0-4 metadata fields. For these cases, all storage is
/// inline (no heap allocation), saving ~100-200ns per error creation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Metadata {
    /// Fields stored sorted by name for binary search lookup.
    fields: InlineVec<Field>
}

impl Metadata {
    /// Create an empty metadata container.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            fields: InlineVec::new()
        }
    }

    /// Build metadata from an iterator of [`Field`] values.
    #[must_use]
    pub fn from_fields(fields: impl IntoIterator<Item = Field>) -> Self {
        let mut meta = Self::new();
        for field in fields {
            meta.insert(field);
        }
        meta
    }

    /// Number of fields stored in the metadata.
    #[must_use]
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Whether the metadata is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Insert or replace a field and return the previous value.
    ///
    /// Fields are kept sorted by name for efficient lookup.
    pub fn insert(&mut self, field: Field) -> Option<FieldValue> {
        let name = field.name;
        match self.fields.binary_search_by_key(&name, |f| f.name) {
            Ok(idx) => {
                // Replace existing field
                let old = core::mem::replace(&mut self.fields[idx], field);
                Some(old.into_value())
            }
            Err(idx) => {
                // Insert at sorted position
                self.fields.insert(idx, field);
                None
            }
        }
    }

    /// Extend metadata with additional fields.
    pub fn extend(&mut self, fields: impl IntoIterator<Item = Field>) {
        for field in fields {
            self.insert(field);
        }
    }

    /// Borrow a field value by name.
    #[must_use]
    pub fn get(&self, name: &'static str) -> Option<&FieldValue> {
        self.fields
            .binary_search_by_key(&name, |f| f.name)
            .ok()
            .map(|idx| self.fields[idx].value())
    }

    /// Borrow the full field entry by name.
    #[must_use]
    pub fn get_field(&self, name: &'static str) -> Option<&Field> {
        self.fields
            .binary_search_by_key(&name, |f| f.name)
            .ok()
            .map(|idx| &self.fields[idx])
    }

    /// Override the redaction policy for a specific field.
    pub fn set_redaction(&mut self, name: &'static str, redaction: FieldRedaction) {
        if let Ok(idx) = self.fields.binary_search_by_key(&name, |f| f.name) {
            self.fields[idx].set_redaction(redaction);
        }
    }

    /// Retrieve the redaction policy for a field if present.
    #[must_use]
    pub fn redaction(&self, name: &'static str) -> Option<FieldRedaction> {
        self.fields
            .binary_search_by_key(&name, |f| f.name)
            .ok()
            .map(|idx| self.fields[idx].redaction())
    }

    /// Iterator over metadata fields in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &FieldValue)> {
        self.fields.iter().map(|f| (f.name, f.value()))
    }

    /// Iterator over metadata entries including the redaction policy.
    pub fn iter_with_redaction(
        &self
    ) -> impl Iterator<Item = (&'static str, &FieldValue, FieldRedaction)> {
        self.fields
            .iter()
            .map(|f| (f.name, f.value(), f.redaction()))
    }
}

impl IntoIterator for Metadata {
    type Item = Field;
    type IntoIter = super::inline_vec::IntoIter<Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

/// Factories for [`Field`] values.
pub mod field {
    use alloc::borrow::Cow;
    use core::{net::IpAddr, time::Duration};

    #[cfg(feature = "serde_json")]
    use serde_json::Value as JsonValue;
    use uuid::Uuid;

    use super::{Field, FieldValue};

    /// Build a string metadata field.
    #[must_use]
    pub fn str(name: &'static str, value: impl Into<Cow<'static, str>>) -> Field {
        Field::new(name, FieldValue::Str(value.into()))
    }

    /// Build an `i64` metadata field.
    #[must_use]
    pub fn i64(name: &'static str, value: i64) -> Field {
        Field::new(name, FieldValue::I64(value))
    }

    /// Build a `u64` metadata field.
    #[must_use]
    pub fn u64(name: &'static str, value: u64) -> Field {
        Field::new(name, FieldValue::U64(value))
    }

    /// Build an `f64` metadata field.
    ///
    /// ```
    /// use masterror::{field, FieldValue};
    ///
    /// let (_, value, _) = field::f64("ratio", 0.5).into_parts();
    /// assert!(matches!(value, FieldValue::F64(ratio) if ratio.to_bits() == 0.5f64.to_bits()));
    /// ```
    #[must_use]
    pub fn f64(name: &'static str, value: f64) -> Field {
        Field::new(name, FieldValue::F64(value))
    }

    /// Build a boolean metadata field.
    #[must_use]
    pub fn bool(name: &'static str, value: bool) -> Field {
        Field::new(name, FieldValue::Bool(value))
    }

    /// Build a UUID metadata field.
    #[must_use]
    pub fn uuid(name: &'static str, value: Uuid) -> Field {
        Field::new(name, FieldValue::Uuid(value))
    }

    /// Build a duration metadata field.
    ///
    /// ```
    /// use core::time::Duration;
    /// use masterror::{field, FieldValue};
    ///
    /// let (_, value, _) = field::duration("elapsed", Duration::from_millis(1500)).into_parts();
    /// assert!(matches!(value, FieldValue::Duration(duration) if duration == Duration::from_millis(1500)));
    /// ```
    #[must_use]
    pub fn duration(name: &'static str, value: Duration) -> Field {
        Field::new(name, FieldValue::Duration(value))
    }

    /// Build an IP address metadata field.
    ///
    /// ```
    /// use core::net::{IpAddr, Ipv4Addr};
    /// use masterror::{field, FieldValue};
    ///
    /// let (_, value, _) = field::ip("peer", IpAddr::from(Ipv4Addr::LOCALHOST)).into_parts();
    /// assert!(matches!(value, FieldValue::Ip(addr) if addr.is_ipv4()));
    /// ```
    #[must_use]
    pub fn ip(name: &'static str, value: IpAddr) -> Field {
        Field::new(name, FieldValue::Ip(value))
    }

    /// Build a JSON metadata field (requires the `serde_json` feature).
    ///
    /// ```
    /// # #[cfg(feature = "serde_json")]
    /// # {
    /// use masterror::{field, FieldValue};
    ///
    /// let (_, value, _) = field::json("payload", serde_json::json!({"ok": true})).into_parts();
    /// assert!(matches!(value, FieldValue::Json(payload) if payload["ok"].as_bool() == Some(true)));
    /// # }
    /// ```
    #[cfg(feature = "serde_json")]
    #[must_use]
    pub fn json(name: &'static str, value: JsonValue) -> Field {
        Field::new(name, FieldValue::Json(value))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        borrow::Cow,
        net::{IpAddr, Ipv4Addr},
        time::Duration
    };

    #[cfg(feature = "serde_json")]
    use serde_json::json;
    use uuid::Uuid;

    use super::{FieldRedaction, FieldValue, Metadata, duration_to_string, field};

    #[test]
    fn metadata_roundtrip() {
        let mut meta = Metadata::new();
        meta.insert(field::str("request_id", Cow::Borrowed("abc")));
        meta.insert(field::i64("count", 42));
        assert_eq!(
            meta.get("request_id"),
            Some(&FieldValue::Str(Cow::Borrowed("abc")))
        );
        assert_eq!(meta.get("count"), Some(&FieldValue::I64(42)));
        assert_eq!(meta.redaction("request_id"), Some(FieldRedaction::None));
    }

    #[test]
    fn metadata_from_fields_is_deterministic() {
        let uuid = Uuid::nil();
        let meta =
            Metadata::from_fields([field::uuid("trace_id", uuid), field::bool("cached", true)]);
        let collected: Vec<_> = meta.iter().collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].0, "cached");
        assert_eq!(collected[1].0, "trace_id");
    }

    #[test]
    fn metadata_supports_extended_field_types() {
        let meta = Metadata::from_fields([
            field::f64("ratio", 0.25),
            field::duration("elapsed", Duration::from_millis(1500)),
            field::ip("peer", IpAddr::from(Ipv4Addr::new(192, 168, 0, 1)))
        ]);
        assert!(meta.get("ratio").is_some_and(
            |value| matches!(value, FieldValue::F64(ratio) if ratio.to_bits() == 0.25f64.to_bits())
        ));
        assert_eq!(
            meta.get("elapsed"),
            Some(&FieldValue::Duration(Duration::from_millis(1500)))
        );
        assert_eq!(
            meta.get("peer"),
            Some(&FieldValue::Ip(IpAddr::from(Ipv4Addr::new(192, 168, 0, 1))))
        );
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn metadata_supports_json_fields() {
        let meta = Metadata::from_fields([field::json("payload", json!({ "status": "ok" }))]);
        assert!(meta.get("payload").is_some_and(|value| matches!(
            value,
            FieldValue::Json(payload) if payload["status"] == "ok"
        )));
    }

    #[test]
    fn inserting_field_replaces_previous_value() {
        let mut meta = Metadata::from_fields([field::i64("count", 1)]);
        let replaced = meta.insert(field::i64("count", 2));
        assert_eq!(replaced, Some(FieldValue::I64(1)));
        assert_eq!(meta.get("count"), Some(&FieldValue::I64(2)));
    }

    #[test]
    fn default_redaction_applies_to_common_keys() {
        let password = field::str("password", Cow::Borrowed("secret"));
        assert!(matches!(password.redaction(), FieldRedaction::Redact));
        let token = field::str("api_token", Cow::Borrowed("abcdef"));
        assert!(matches!(token.redaction(), FieldRedaction::Hash));
        let card = field::str("card_number", Cow::Borrowed("4111111111111111"));
        assert!(matches!(card.redaction(), FieldRedaction::Last4));
    }

    #[test]
    fn default_redaction_remains_case_insensitive() {
        let cases = [
            ("Password", FieldRedaction::Redact),
            ("SESSION_ID", FieldRedaction::Redact),
            ("X-API-Token", FieldRedaction::Hash),
            ("RefreshToken", FieldRedaction::Hash),
            ("CARD_NUMBER", FieldRedaction::Last4)
        ];
        for (name, expected) in cases {
            let field = field::str(name, Cow::Borrowed("value"));
            assert!(
                matches!(field.redaction(), policy if policy == expected),
                "expected {:?} for {name}",
                expected
            );
        }
    }

    #[test]
    fn field_into_parts_returns_components() {
        let field = field::u64("elapsed_ms", 30);
        let clone = field.clone();
        assert_eq!(clone.name(), field.name());
        assert_eq!(clone.value(), field.value());
        let (owned_name, owned_value, redaction) = clone.into_parts();
        assert_eq!(owned_name, field.name());
        assert_eq!(owned_value, field.value().clone());
        assert_eq!(redaction, field.redaction());
    }

    #[test]
    fn duration_to_string_trims_trailing_zeroes() {
        let text = duration_to_string(Duration::from_micros(1500));
        assert_eq!(text, "0.0015s");
    }

    #[test]
    fn duration_to_string_whole_seconds() {
        let text = duration_to_string(Duration::from_secs(5));
        assert_eq!(text, "5s");
    }

    #[test]
    fn duration_to_string_with_nanos() {
        let text = duration_to_string(Duration::new(1, 500_000_000));
        assert_eq!(text, "1.5s");
    }

    #[test]
    fn field_value_display_all_types() {
        assert_eq!(FieldValue::Str(Cow::Borrowed("hello")).to_string(), "hello");
        assert_eq!(FieldValue::I64(-42).to_string(), "-42");
        assert_eq!(FieldValue::U64(100).to_string(), "100");
        assert_eq!(FieldValue::F64(1.5).to_string(), "1.5");
        assert_eq!(FieldValue::Bool(true).to_string(), "true");
        assert_eq!(FieldValue::Bool(false).to_string(), "false");

        let uuid = Uuid::nil();
        assert_eq!(
            FieldValue::Uuid(uuid).to_string(),
            "00000000-0000-0000-0000-000000000000"
        );

        let dur = Duration::from_millis(1500);
        assert_eq!(FieldValue::Duration(dur).to_string(), "1.5s");

        let ip = IpAddr::from(Ipv4Addr::LOCALHOST);
        assert_eq!(FieldValue::Ip(ip).to_string(), "127.0.0.1");
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn field_value_display_json() {
        let value = FieldValue::Json(json!({"key": "value"}));
        assert!(value.to_string().contains("key"));
    }

    #[test]
    fn metadata_extend() {
        let mut meta = Metadata::new();
        meta.extend([field::str("a", "1"), field::str("b", "2")]);
        assert_eq!(meta.len(), 2);
        assert_eq!(meta.get("a"), Some(&FieldValue::Str(Cow::Borrowed("1"))));
        assert_eq!(meta.get("b"), Some(&FieldValue::Str(Cow::Borrowed("2"))));
    }

    #[test]
    fn metadata_get_field() {
        let meta = Metadata::from_fields([field::i64("count", 42)]);
        let field = meta.get_field("count");
        assert!(field.is_some());
        let field = field.unwrap();
        assert_eq!(field.name(), "count");
        assert_eq!(field.value(), &FieldValue::I64(42));
    }

    #[test]
    fn metadata_get_nonexistent() {
        let meta = Metadata::new();
        assert!(meta.get("missing").is_none());
        assert!(meta.get_field("missing").is_none());
        assert!(meta.redaction("missing").is_none());
    }

    #[test]
    fn metadata_set_redaction() {
        let mut meta = Metadata::from_fields([field::str("secret", "value")]);
        meta.set_redaction("secret", FieldRedaction::Redact);
        assert_eq!(meta.redaction("secret"), Some(FieldRedaction::Redact));
    }

    #[test]
    fn metadata_set_redaction_nonexistent() {
        let mut meta = Metadata::new();
        meta.set_redaction("missing", FieldRedaction::Redact);
        assert!(meta.redaction("missing").is_none());
    }

    #[test]
    fn metadata_iter_with_redaction() {
        let meta = Metadata::from_fields([
            field::str("public", "value1"),
            field::str("password", "secret")
        ]);
        let items: Vec<_> = meta.iter_with_redaction().collect();
        assert_eq!(items.len(), 2);
        let password_item = items.iter().find(|(n, _, _)| *n == "password").unwrap();
        assert_eq!(password_item.2, FieldRedaction::Redact);
    }

    #[test]
    fn metadata_into_iter() {
        let meta = Metadata::from_fields([field::i64("a", 1), field::i64("b", 2)]);
        let fields: Vec<_> = meta.into_iter().collect();
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn field_default_redaction_non_sensitive() {
        let field = field::str("user_id", "abc123");
        assert_eq!(field.redaction(), FieldRedaction::None);
    }

    #[test]
    fn field_with_redaction() {
        let field = field::str("public", "value").with_redaction(FieldRedaction::Hash);
        assert_eq!(field.redaction(), FieldRedaction::Hash);
    }

    #[test]
    fn field_redaction_default() {
        assert_eq!(FieldRedaction::default(), FieldRedaction::None);
    }

    #[test]
    fn field_value_partial_eq() {
        let v1 = FieldValue::I64(42);
        let v2 = FieldValue::I64(42);
        let v3 = FieldValue::I64(43);
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }

    #[test]
    fn metadata_len_and_is_empty() {
        let empty = Metadata::new();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);

        let meta = Metadata::from_fields([field::i64("x", 1)]);
        assert!(!meta.is_empty());
        assert_eq!(meta.len(), 1);
    }

    #[test]
    fn field_into_value() {
        let field = field::u64("count", 100);
        let value = field.into_value();
        assert_eq!(value, FieldValue::U64(100));
    }
}
