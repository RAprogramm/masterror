use core::fmt::{self, Debug, Display, Formatter};

use super::{core::ErrorResponse, problem_json::ProblemJson};

/// Formatter exposing response internals for opt-in diagnostics.
#[derive(Clone, Copy)]
pub struct ErrorResponseFormatter<'a> {
    inner: &'a ErrorResponse
}

impl<'a> ErrorResponseFormatter<'a> {
    pub(crate) fn new(inner: &'a ErrorResponse) -> Self {
        Self {
            inner
        }
    }
}

impl Debug for ErrorResponseFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorResponse")
            .field("status", &self.inner.status)
            .field("code", &self.inner.code)
            .field("message", &self.inner.message)
            .field("details", &self.inner.details)
            .field("retry", &self.inner.retry)
            .field("www_authenticate", &self.inner.www_authenticate)
            .finish()
    }
}

impl Display for ErrorResponseFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.inner, f)
    }
}

/// Formatter exposing problem-json internals for opt-in diagnostics.
#[derive(Clone, Copy)]
pub struct ProblemJsonFormatter<'a> {
    inner: &'a ProblemJson
}

impl<'a> ProblemJsonFormatter<'a> {
    pub(crate) fn new(inner: &'a ProblemJson) -> Self {
        Self {
            inner
        }
    }
}

impl Debug for ProblemJsonFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProblemJson")
            .field("type", &self.inner.type_uri)
            .field("title", &self.inner.title)
            .field("status", &self.inner.status)
            .field("detail", &self.inner.detail)
            .field("code", &self.inner.code)
            .field("grpc", &self.inner.grpc)
            .field("metadata", &self.inner.metadata)
            .finish()
    }
}

impl Display for ProblemJsonFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {:?}",
            self.inner.status, self.inner.code, self.inner.detail
        )
    }
}
