use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt::{Display, Formatter, Result as FmtResult}
};

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
    name:  &'static str,
    value: FieldValue
}

impl Field {
    /// Create a new [`Field`].
    #[must_use]
    pub const fn new(name: &'static str, value: FieldValue) -> Self {
        Self {
            name,
            value
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

    /// Consume the field and return owned components.
    #[must_use]
    pub fn into_parts(self) -> (&'static str, FieldValue) {
        (self.name, self.value)
    }
}

/// Structured metadata attached to [`crate::AppError`].
///
/// Internally backed by a deterministic [`BTreeMap`] keyed by `'static` field
/// names. Use the helpers in [`field`] to build [`Field`] values without manual
/// enum construction.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Metadata {
    fields: BTreeMap<&'static str, FieldValue>
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
        for Field {
            name,
            value
        } in fields
        {
            map.insert(name, value);
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
        let (name, value) = field.into_parts();
        self.fields.insert(name, value)
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
        self.fields.get(name)
    }

    /// Iterator over metadata fields in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &FieldValue)> {
        self.fields.iter().map(|(k, v)| (*k, v))
    }
}

impl IntoIterator for Metadata {
    type Item = Field;
    type IntoIter = std::iter::Map<
        std::collections::btree_map::IntoIter<&'static str, FieldValue>,
        fn((&'static str, FieldValue)) -> Field
    >;

    fn into_iter(self) -> Self::IntoIter {
        fn into_field(entry: (&'static str, FieldValue)) -> Field {
            Field::new(entry.0, entry.1)
        }
        self.fields
            .into_iter()
            .map(into_field as fn((&'static str, FieldValue)) -> Field)
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

    use super::{FieldValue, Metadata, field};

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
    fn field_into_parts_returns_components() {
        let field = field::u64("elapsed_ms", 30);
        let clone = field.clone();
        assert_eq!(clone.name(), field.name());
        assert_eq!(clone.value(), field.value());
        let (owned_name, owned_value) = clone.into_parts();
        assert_eq!(owned_name, field.name());
        assert_eq!(owned_value, field.value().clone());
    }
}
