// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Application error types for masterror-cli.

use std::{fmt, io, path::PathBuf};

/// Application-wide error type.
#[derive(Debug)]
pub enum AppError {
    /// I/O error (file operations, process spawning).
    Io(io::Error),
    /// JSON parsing error from cargo output.
    Json(serde_json::Error),
    /// Cargo check process failed with exit code.
    CargoFailed {
        /// Exit code from cargo process.
        code: i32
    },
    /// Cargo check process was terminated by signal.
    CargoSignaled,
    /// Unknown error code requested.
    UnknownErrorCode {
        /// The requested error code.
        code: String
    },
    /// Unknown practice code requested.
    UnknownPracticeCode {
        /// The requested practice code.
        code: String
    },
    /// Invalid category name.
    InvalidCategory {
        /// The invalid category name.
        name: String
    },
    /// Invalid command-line argument.
    InvalidArgument {
        /// The invalid argument.
        arg: String
    },
    /// Config file parse error.
    ConfigParse {
        /// Path to config file.
        path:    PathBuf,
        /// Error message.
        message: String
    },
    /// Error with additional context.
    #[allow(dead_code)]
    WithContext {
        /// Context message.
        context: String,
        /// Original error.
        source:  Box<AppError>
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Json(e) => write!(f, "JSON parse error: {e}"),
            Self::CargoFailed {
                code
            } => write!(f, "cargo check failed with exit code {code}"),
            Self::CargoSignaled => write!(f, "cargo check was terminated by signal"),
            Self::UnknownErrorCode {
                code
            } => write!(f, "unknown error code: {code}"),
            Self::UnknownPracticeCode {
                code
            } => write!(f, "unknown practice code: {code}"),
            Self::InvalidCategory {
                name
            } => write!(f, "invalid category: {name}"),
            Self::InvalidArgument {
                arg
            } => write!(f, "invalid argument: {arg}"),
            Self::ConfigParse {
                path,
                message
            } => write!(f, "config error in {}: {message}", path.display()),
            Self::WithContext {
                context,
                source
            } => write!(f, "{context}: {source}")
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Json(e) => Some(e),
            Self::WithContext {
                source, ..
            } => Some(source.as_ref()),
            _ => None
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

/// Result type alias for AppError.
pub type Result<T> = std::result::Result<T, AppError>;

/// Extension trait for adding context to Results.
///
/// # Example
///
/// ```ignore
/// use masterror::error::ResultExt;
///
/// fs::read_to_string(path).context("reading config")?;
/// ```
#[allow(dead_code)]
pub trait ResultExt<T> {
    /// Add static context to an error.
    fn context(self, ctx: &'static str) -> Result<T>;

    /// Add dynamic context to an error.
    fn with_context<F, S>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> S,
        S: Into<String>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: Into<AppError>
{
    fn context(self, ctx: &'static str) -> Result<T> {
        self.map_err(|e| {
            let inner = e.into();
            AppError::WithContext {
                context: ctx.to_string(),
                source:  Box::new(inner)
            }
        })
    }

    fn with_context<F, S>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> S,
        S: Into<String>
    {
        self.map_err(|e| {
            let inner = e.into();
            AppError::WithContext {
                context: f().into(),
                source:  Box::new(inner)
            }
        })
    }
}
