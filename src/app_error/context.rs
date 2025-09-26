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
    #[must_use]
    pub fn category(mut self, category: AppErrorKind) -> Self {
        self.category = category;
        if !self.code_overridden {
            self.code = AppCode::from(category);
        }
        self
    }

    /// Attach a metadata [`Field`].
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
        }
        for &(name, redaction) in &field_policies {
            error = error.redact_field(name, redaction);
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
