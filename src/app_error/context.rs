// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;
use core::{error::Error as CoreError, panic::Location};

use super::{
    core::{AppError, Error, MessageEditPolicy},
    metadata::{Field, FieldRedaction, FieldValue}
};
use crate::{AppCode, AppErrorKind};

/// Builder describing how to convert an external error into [`AppError`].
///
/// The context captures the target [`AppCode`], [`AppErrorKind`], optional
/// metadata fields and redaction policy. It is primarily consumed by
/// [`ResultExt`](crate::ResultExt) when promoting `Result<T, E>` values into
/// [`AppError`].
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "std")] {
/// use std::io::{Error as IoError, ErrorKind};
///
/// use masterror::{AppErrorKind, Context, ResultExt, field};
///
/// fn failing_io() -> Result<(), IoError> {
///     Err(IoError::from(ErrorKind::Other))
/// }
///
/// let err = failing_io()
///     .ctx(|| {
///         Context::new(AppErrorKind::Service)
///             .with(field::str("operation", "sync"))
///             .redact(true)
///             .track_caller()
///     })
///     .unwrap_err();
///
/// assert_eq!(err.kind, AppErrorKind::Service);
/// assert!(err.metadata().get("operation").is_some());
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Context {
    code:            AppCode,
    category:        AppErrorKind,
    fields:          Vec<Field>,
    field_policies:  Vec<(&'static str, FieldRedaction)>,
    edit_policy:     MessageEditPolicy,
    caller_location: Option<&'static Location<'static>>,
    code_overridden: bool
}

impl Context {
    /// Create a new [`Context`] targeting the provided [`AppErrorKind`].
    ///
    /// The initial [`AppCode`] defaults to the canonical mapping for the
    /// supplied kind. Use [`Context::code`] to override it.
    #[must_use]
    pub fn new(category: AppErrorKind) -> Self {
        Self {
            code: AppCode::from(category),
            category,
            fields: Vec::new(),
            field_policies: Vec::new(),
            edit_policy: MessageEditPolicy::Preserve,
            caller_location: None,
            code_overridden: false
        }
    }

    /// Override the public [`AppCode`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use masterror::{AppCode, AppErrorKind, Context};
    ///
    /// let ctx = Context::new(AppErrorKind::Service).code(AppCode::Internal);
    /// # }
    /// ```
    #[must_use]
    pub fn code(mut self, code: AppCode) -> Self {
        self.code = code;
        self.code_overridden = true;
        self
    }

    /// Update the [`AppErrorKind`].
    ///
    /// When the code has not been overridden explicitly, it is kept in sync
    /// with the new kind.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use masterror::{AppErrorKind, Context};
    ///
    /// let ctx = Context::new(AppErrorKind::BadRequest).category(AppErrorKind::Service);
    /// # }
    /// ```
    #[must_use]
    pub fn category(mut self, category: AppErrorKind) -> Self {
        self.category = category;
        if !self.code_overridden {
            self.code = AppCode::from(category);
        }
        self
    }

    /// Attach a metadata [`Field`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use masterror::{AppErrorKind, Context, field};
    ///
    /// let ctx = Context::new(AppErrorKind::Service)
    ///     .with(field::str("operation", "sync"))
    ///     .with(field::u64("retry_count", 3));
    /// # }
    /// ```
    #[must_use]
    pub fn with(mut self, mut field: Field) -> Self {
        if let Some((_, policy)) = self
            .field_policies
            .iter()
            .rev()
            .find(|(name, _)| *name == field.name())
        {
            field.set_redaction(*policy);
        }
        self.fields.push(field);
        self
    }

    /// Override the redaction policy for a metadata field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use masterror::{AppErrorKind, Context, FieldRedaction, field};
    ///
    /// let ctx = Context::new(AppErrorKind::Service)
    ///     .with(field::str("password", "secret"))
    ///     .redact_field("password", FieldRedaction::Redact);
    /// # }
    /// ```
    #[must_use]
    pub fn redact_field(mut self, name: &'static str, redaction: FieldRedaction) -> Self {
        self.set_field_policy(name, redaction);
        self
    }

    /// Override the redaction policy for a metadata field in place.
    #[must_use]
    pub fn redact_field_mut(
        &mut self,
        name: &'static str,
        redaction: FieldRedaction
    ) -> &mut Self {
        self.set_field_policy(name, redaction);
        self
    }

    /// Toggle message redaction policy.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] {
    /// use masterror::{AppErrorKind, Context};
    ///
    /// let ctx = Context::new(AppErrorKind::Service).redact(true);
    /// # }
    /// ```
    #[must_use]
    pub fn redact(mut self, redact: bool) -> Self {
        self.edit_policy = if redact {
            MessageEditPolicy::Redact
        } else {
            MessageEditPolicy::Preserve
        };
        self
    }

    /// Capture caller location and store it as metadata.
    #[must_use]
    #[track_caller]
    pub fn track_caller(mut self) -> Self {
        self.caller_location = Some(Location::caller());
        self
    }

    pub(crate) fn into_error<E>(self, source: E) -> Error
    where
        E: CoreError + Send + Sync + 'static
    {
        let Context {
            mut fields,
            field_policies,
            edit_policy,
            caller_location,
            code,
            category,
            ..
        } = self;

        if let Some(location) = caller_location {
            fields.push(Field::new(
                "caller.file",
                FieldValue::Str(location.file().into())
            ));
            fields.push(Field::new(
                "caller.line",
                FieldValue::U64(u64::from(location.line()))
            ));
            fields.push(Field::new(
                "caller.column",
                FieldValue::U64(u64::from(location.column()))
            ));
        }

        let mut error = AppError::new_raw(category, None);
        error.code = code;
        if !fields.is_empty() {
            Self::apply_field_redactions(&mut fields, &field_policies);
            error.metadata.extend(fields);
        } else if !field_policies.is_empty() {
            for &(name, redaction) in &field_policies {
                error = error.redact_field(name, redaction);
            }
        }
        if matches!(edit_policy, MessageEditPolicy::Redact) {
            error.edit_policy = MessageEditPolicy::Redact;
        }
        let error = error.with_context(source);
        error.emit_telemetry();
        error
    }
}

impl Context {
    fn apply_field_redactions(
        fields: &mut Vec<Field>,
        policies: &[(&'static str, FieldRedaction)]
    ) {
        if policies.is_empty() {
            return;
        }
        for field in fields {
            if let Some((_, policy)) = policies
                .iter()
                .rev()
                .find(|(name, _)| *name == field.name())
            {
                field.set_redaction(*policy);
            }
        }
    }

    fn set_field_policy(&mut self, name: &'static str, redaction: FieldRedaction) {
        self.field_policies
            .retain(|(existing, _)| *existing != name);
        self.field_policies.push((name, redaction));
        for field in &mut self.fields {
            if field.name() == name {
                field.set_redaction(redaction);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field;

    #[test]
    fn context_new_creates_with_kind_and_default_code() {
        let ctx = Context::new(AppErrorKind::BadRequest);
        assert_eq!(ctx.category, AppErrorKind::BadRequest);
        assert_eq!(ctx.code, AppCode::from(AppErrorKind::BadRequest));
        assert!(!ctx.code_overridden);
        assert!(ctx.fields.is_empty());
        assert!(ctx.field_policies.is_empty());
    }

    #[test]
    fn context_code_override_sets_custom_code() {
        let ctx = Context::new(AppErrorKind::Service).code(AppCode::Internal);
        assert_eq!(ctx.code, AppCode::Internal);
        assert!(ctx.code_overridden);
    }

    #[test]
    fn context_category_updates_kind_and_syncs_code_when_not_overridden() {
        let ctx = Context::new(AppErrorKind::BadRequest).category(AppErrorKind::Service);
        assert_eq!(ctx.category, AppErrorKind::Service);
        assert_eq!(ctx.code, AppCode::from(AppErrorKind::Service));
        assert!(!ctx.code_overridden);
    }

    #[test]
    fn context_category_preserves_code_when_overridden() {
        let ctx = Context::new(AppErrorKind::BadRequest)
            .code(AppCode::Internal)
            .category(AppErrorKind::Service);
        assert_eq!(ctx.category, AppErrorKind::Service);
        assert_eq!(ctx.code, AppCode::Internal);
        assert!(ctx.code_overridden);
    }

    #[test]
    fn context_with_adds_metadata_field() {
        let ctx = Context::new(AppErrorKind::Service).with(field::str("operation", "sync"));
        assert_eq!(ctx.fields.len(), 1);
        assert_eq!(ctx.fields[0].name(), "operation");
    }

    #[test]
    fn context_with_adds_multiple_fields() {
        let ctx = Context::new(AppErrorKind::Service)
            .with(field::str("operation", "sync"))
            .with(field::u64("retry_count", 3))
            .with(field::bool("is_critical", true));
        assert_eq!(ctx.fields.len(), 3);
        assert_eq!(ctx.fields[0].name(), "operation");
        assert_eq!(ctx.fields[1].name(), "retry_count");
        assert_eq!(ctx.fields[2].name(), "is_critical");
    }

    #[test]
    fn context_redact_field_sets_policy() {
        let ctx =
            Context::new(AppErrorKind::Service).redact_field("secret", FieldRedaction::Redact);
        assert_eq!(ctx.field_policies.len(), 1);
        assert_eq!(ctx.field_policies[0].0, "secret");
        assert_eq!(ctx.field_policies[0].1, FieldRedaction::Redact);
    }

    #[test]
    fn context_redact_field_mut_sets_policy_in_place() {
        let mut ctx = Context::new(AppErrorKind::Service);
        let _ = ctx.redact_field_mut("secret", FieldRedaction::Redact);
        assert_eq!(ctx.field_policies.len(), 1);
        assert_eq!(ctx.field_policies[0].0, "secret");
    }

    #[test]
    fn context_redact_field_updates_existing_policy() {
        let ctx = Context::new(AppErrorKind::Service)
            .redact_field("secret", FieldRedaction::Redact)
            .redact_field("secret", FieldRedaction::Hash);
        assert_eq!(ctx.field_policies.len(), 1);
        assert_eq!(ctx.field_policies[0].1, FieldRedaction::Hash);
    }

    #[test]
    fn context_redact_field_applies_to_existing_fields() {
        let ctx = Context::new(AppErrorKind::Service)
            .with(field::str("secret", "value"))
            .redact_field("secret", FieldRedaction::Redact);
        assert_eq!(ctx.fields[0].redaction(), FieldRedaction::Redact);
    }

    #[test]
    fn context_with_applies_field_policy_when_added_after_policy() {
        let ctx = Context::new(AppErrorKind::Service)
            .redact_field("secret", FieldRedaction::Redact)
            .with(field::str("secret", "value"));
        assert_eq!(ctx.fields[0].redaction(), FieldRedaction::Redact);
    }

    #[test]
    fn context_redact_sets_message_policy_to_redact() {
        let ctx = Context::new(AppErrorKind::Service).redact(true);
        assert!(matches!(ctx.edit_policy, MessageEditPolicy::Redact));
    }

    #[test]
    fn context_redact_sets_message_policy_to_preserve() {
        let ctx = Context::new(AppErrorKind::Service).redact(false);
        assert!(matches!(ctx.edit_policy, MessageEditPolicy::Preserve));
    }

    #[test]
    #[track_caller]
    fn context_track_caller_captures_location() {
        let ctx = Context::new(AppErrorKind::Service).track_caller();
        assert!(ctx.caller_location.is_some());
        let location = ctx.caller_location.unwrap();
        assert!(location.file().contains("context.rs"));
    }

    #[cfg(feature = "std")]
    #[test]
    fn context_into_error_creates_error_with_kind_and_code() {
        use std::io::{Error as IoError, ErrorKind};

        let io_err = IoError::from(ErrorKind::Other);
        let ctx = Context::new(AppErrorKind::Service);
        let err = ctx.into_error(io_err);
        assert_eq!(err.kind, AppErrorKind::Service);
        assert_eq!(err.code, AppCode::from(AppErrorKind::Service));
    }

    #[cfg(feature = "std")]
    #[test]
    fn context_into_error_applies_metadata_fields() {
        use std::io::{Error as IoError, ErrorKind};

        let io_err = IoError::from(ErrorKind::Other);
        let ctx = Context::new(AppErrorKind::Service)
            .with(field::str("operation", "sync"))
            .with(field::u64("retry_count", 3));
        let err = ctx.into_error(io_err);
        let metadata = err.metadata();
        assert_eq!(
            metadata.get("operation"),
            Some(&FieldValue::Str("sync".into()))
        );
        assert_eq!(metadata.get("retry_count"), Some(&FieldValue::U64(3)));
    }

    #[cfg(feature = "std")]
    #[test]
    fn context_into_error_applies_field_redactions() {
        use std::io::{Error as IoError, ErrorKind};

        let io_err = IoError::from(ErrorKind::Other);
        let ctx = Context::new(AppErrorKind::Service)
            .with(field::str("secret", "password"))
            .redact_field("secret", FieldRedaction::Redact);
        let err = ctx.into_error(io_err);
        assert_eq!(
            err.metadata().redaction("secret"),
            Some(FieldRedaction::Redact)
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn context_into_error_applies_message_redaction() {
        use std::io::{Error as IoError, ErrorKind};

        let io_err = IoError::from(ErrorKind::Other);
        let ctx = Context::new(AppErrorKind::Service).redact(true);
        let err = ctx.into_error(io_err);
        assert!(matches!(err.edit_policy, MessageEditPolicy::Redact));
    }

    #[cfg(feature = "std")]
    #[test]
    #[track_caller]
    fn context_into_error_captures_caller_location() {
        use std::io::{Error as IoError, ErrorKind};

        let io_err = IoError::from(ErrorKind::Other);
        let ctx = Context::new(AppErrorKind::Service).track_caller();
        let err = ctx.into_error(io_err);
        let metadata = err.metadata();
        assert!(metadata.get("caller.file").is_some());
        assert!(metadata.get("caller.line").is_some());
        assert!(metadata.get("caller.column").is_some());
    }

    #[cfg(feature = "std")]
    #[test]
    fn context_into_error_with_custom_code() {
        use std::io::{Error as IoError, ErrorKind};

        let io_err = IoError::from(ErrorKind::Other);
        let ctx = Context::new(AppErrorKind::Service).code(AppCode::Validation);
        let err = ctx.into_error(io_err);
        assert_eq!(err.code, AppCode::Validation);
        assert_eq!(err.kind, AppErrorKind::Service);
    }

    #[cfg(feature = "std")]
    #[test]
    fn context_apply_field_redactions_updates_all_matching_fields() {
        let mut fields = vec![
            field::str("secret", "value1"),
            field::str("public", "value2"),
            field::str("secret", "value3"),
        ];
        let policies = vec![("secret", FieldRedaction::Redact)];
        Context::apply_field_redactions(&mut fields, &policies);
        assert_eq!(fields[0].redaction(), FieldRedaction::Redact);
        assert_eq!(fields[1].redaction(), FieldRedaction::None);
        assert_eq!(fields[2].redaction(), FieldRedaction::Redact);
    }

    #[cfg(feature = "std")]
    #[test]
    fn context_apply_field_redactions_with_empty_policies() {
        let mut fields = vec![field::str("key", "value")];
        let original_redaction = fields[0].redaction();
        Context::apply_field_redactions(&mut fields, &[]);
        assert_eq!(fields[0].redaction(), original_redaction);
    }

    #[cfg(feature = "std")]
    #[test]
    fn context_apply_field_redactions_uses_last_policy() {
        let mut fields = vec![field::str("secret", "value")];
        let policies = vec![
            ("secret", FieldRedaction::Redact),
            ("secret", FieldRedaction::Hash),
        ];
        Context::apply_field_redactions(&mut fields, &policies);
        assert_eq!(fields[0].redaction(), FieldRedaction::Hash);
    }

    #[test]
    fn context_set_field_policy_removes_duplicate_policies() {
        let mut ctx = Context::new(AppErrorKind::Service);
        ctx.set_field_policy("secret", FieldRedaction::Redact);
        ctx.set_field_policy("secret", FieldRedaction::Hash);
        assert_eq!(ctx.field_policies.len(), 1);
        assert_eq!(ctx.field_policies[0].1, FieldRedaction::Hash);
    }

    #[test]
    fn context_builder_chain_preserves_all_settings() {
        let ctx = Context::new(AppErrorKind::BadRequest)
            .code(AppCode::Validation)
            .category(AppErrorKind::Service)
            .with(field::str("operation", "sync"))
            .with(field::u64("retry", 3))
            .redact_field("secret", FieldRedaction::Redact)
            .redact(true)
            .track_caller();

        assert_eq!(ctx.category, AppErrorKind::Service);
        assert_eq!(ctx.code, AppCode::Validation);
        assert!(ctx.code_overridden);
        assert_eq!(ctx.fields.len(), 2);
        assert_eq!(ctx.field_policies.len(), 1);
        assert!(matches!(ctx.edit_policy, MessageEditPolicy::Redact));
        assert!(ctx.caller_location.is_some());
    }
}
