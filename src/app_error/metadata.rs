use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt::{Display, Formatter, Result as FmtResult}
};

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
    /// Boolean flag.
    Bool(bool),
    /// UUID represented with the canonical binary type.
    Uuid(Uuid)
}

impl Display for FieldValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Str(value) => Display::fmt(value, f),
            Self::I64(value) => Display::fmt(value, f),
            Self::U64(value) => Display::fmt(value, f),
            Self::Bool(value) => Display::fmt(value, f),
            Self::Uuid(value) => Display::fmt(value, f)
        }
    }
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
    let lowered = name.to_ascii_lowercase();

    if lowered.contains("password")
        || lowered.contains("passphrase")
        || lowered.contains("secret")
        || lowered.contains("authorization")
        || lowered.contains("cookie")
        || lowered.contains("session")
        || lowered.contains("jwt")
        || lowered.contains("bearer")
        || lowered.contains("otp")
        || lowered.contains("pin")
    {
        return FieldRedaction::Redact;
    }

    let mut card_like = false;
    let mut number_like = false;

    for segment in lowered.split(['.', '_', '-', ':', '/']) {
        if segment.is_empty() {
            continue;
        }
        if segment.eq_ignore_ascii_case("token")
            || segment.eq_ignore_ascii_case("apikey")
            || segment.eq_ignore_ascii_case("api") && lowered.contains("key")
            || segment.ends_with("token")
            || segment.eq_ignore_ascii_case("key")
            || segment.eq_ignore_ascii_case("access") && lowered.contains("token")
            || segment.eq_ignore_ascii_case("refresh") && lowered.contains("token")
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

/// Structured metadata attached to [`crate::AppError`].
///
/// Internally backed by a deterministic [`BTreeMap`] keyed by `'static` field
/// names. Use the helpers in [`field`] to build [`Field`] values without manual
/// enum construction.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Metadata {
    fields: BTreeMap<&'static str, Field>
}

impl Metadata {
    /// Create an empty metadata container.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Build metadata from an iterator of [`Field`] values.
    #[must_use]
    pub fn from_fields(fields: impl IntoIterator<Item = Field>) -> Self {
        let mut map = BTreeMap::new();
        for field in fields {
            map.insert(field.name, field);
        }
        Self {
            fields: map
        }
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
    pub fn insert(&mut self, field: Field) -> Option<FieldValue> {
        self.fields
            .insert(field.name, field)
            .map(|previous| previous.into_value())
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
        self.fields.get(name).map(|field| field.value())
    }

    /// Borrow the full field entry by name.
    #[must_use]
    pub fn get_field(&self, name: &'static str) -> Option<&Field> {
        self.fields.get(name)
    }

    /// Override the redaction policy for a specific field.
    pub fn set_redaction(&mut self, name: &'static str, redaction: FieldRedaction) {
        if let Some(field) = self.fields.get_mut(name) {
            field.set_redaction(redaction);
        }
    }

    /// Retrieve the redaction policy for a field if present.
    #[must_use]
    pub fn redaction(&self, name: &'static str) -> Option<FieldRedaction> {
        self.fields.get(name).map(|field| field.redaction())
    }

    /// Iterator over metadata fields in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &FieldValue)> {
        self.fields.iter().map(|(k, v)| (*k, v.value()))
    }

    /// Iterator over metadata entries including the redaction policy.
    pub fn iter_with_redaction(
        &self
    ) -> impl Iterator<Item = (&'static str, &FieldValue, FieldRedaction)> {
        self.fields
            .iter()
            .map(|(name, field)| (*name, field.value(), field.redaction()))
    }
}

impl IntoIterator for Metadata {
    type Item = Field;
    type IntoIter = std::iter::Map<
        std::collections::btree_map::IntoIter<&'static str, Field>,
        fn((&'static str, Field)) -> Field
    >;

    fn into_iter(self) -> Self::IntoIter {
        fn into_field(entry: (&'static str, Field)) -> Field {
            entry.1
        }
        self.fields
            .into_iter()
            .map(into_field as fn((&'static str, Field)) -> Field)
    }
}

/// Factories for [`Field`] values.
pub mod field {
    use std::borrow::Cow;

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
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use uuid::Uuid;

    use super::{FieldRedaction, FieldValue, Metadata, field};

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
}
