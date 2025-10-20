// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Error types for README generation.
//!
//! Defines all error conditions that can occur during README generation,
//! including IO errors, TOML parsing errors, metadata validation errors,
//! and template processing errors.

use std::{io, path::PathBuf};

/// Error type describing issues while generating the README file.
#[derive(Debug)]
pub enum ReadmeError {
    Io(io::Error),
    Toml(toml::de::Error),
    MissingMetadata(&'static str),
    MissingFeatureMetadata(Vec<String>),
    UnknownFeatureInOrder(String),
    DuplicateFeatureInOrder(String),
    UnknownMetadataFeature(Vec<String>),
    InvalidSnippetGroup,
    UnresolvedPlaceholder(String),
    OutOfSync { path: PathBuf }
}

impl std::fmt::Display for ReadmeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "IO error: {err}"),
            Self::Toml(err) => write!(f, "Failed to parse Cargo.toml: {err}"),
            Self::MissingMetadata(path) => write!(f, "Missing metadata section {path}"),
            Self::MissingFeatureMetadata(features) => {
                write!(f, "Missing metadata for features: {}", features.join(", "))
            }
            Self::UnknownFeatureInOrder(feature) => {
                write!(f, "Feature order references unknown feature '{feature}'")
            }
            Self::DuplicateFeatureInOrder(feature) => {
                write!(
                    f,
                    "Feature '{feature}' listed multiple times in feature_order"
                )
            }
            Self::UnknownMetadataFeature(features) => {
                write!(
                    f,
                    "Metadata defined for unknown features: {}",
                    features.join(", ")
                )
            }
            Self::InvalidSnippetGroup => {
                write!(f, "feature_snippet_group must be greater than zero")
            }
            Self::UnresolvedPlaceholder(name) => {
                write!(
                    f,
                    "Template placeholder '{{{{{name}}}}}' was not substituted"
                )
            }
            Self::OutOfSync {
                path
            } => {
                write!(
                    f,
                    "README at {} is out of sync; run `cargo build` in the repository root to refresh it",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for ReadmeError {}

impl From<io::Error> for ReadmeError {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}

impl From<toml::de::Error> for ReadmeError {
    fn from(v: toml::de::Error) -> Self {
        Self::Toml(v)
    }
}

#[cfg(test)]
mod tests {
    use std::{io, path::PathBuf};

    use super::{super::types::Manifest, *};

    #[test]
    fn readme_error_display_formats_io_error() {
        let err = ReadmeError::Io(io::Error::new(io::ErrorKind::NotFound, "file not found"));
        let formatted = format!("{}", err);
        assert!(formatted.contains("IO error"));
        assert!(formatted.contains("file not found"));
    }

    #[test]
    fn readme_error_display_formats_missing_metadata() {
        let err = ReadmeError::MissingMetadata("package.metadata.masterror");
        let formatted = format!("{}", err);
        assert!(formatted.contains("Missing metadata section"));
        assert!(formatted.contains("package.metadata.masterror"));
    }

    #[test]
    fn readme_error_display_formats_unknown_feature() {
        let err = ReadmeError::UnknownFeatureInOrder("unknown_feat".to_string());
        let formatted = format!("{}", err);
        assert!(formatted.contains("Feature order references unknown feature"));
        assert!(formatted.contains("unknown_feat"));
    }

    #[test]
    fn readme_error_display_formats_duplicate_feature() {
        let err = ReadmeError::DuplicateFeatureInOrder("duplicate_feat".to_string());
        let formatted = format!("{}", err);
        assert!(formatted.contains("listed multiple times"));
        assert!(formatted.contains("duplicate_feat"));
    }

    #[test]
    fn readme_error_display_formats_unresolved_placeholder() {
        let err = ReadmeError::UnresolvedPlaceholder("PLACEHOLDER".to_string());
        let formatted = format!("{}", err);
        assert!(formatted.contains("{{PLACEHOLDER}}"));
        assert!(formatted.contains("was not substituted"));
    }

    #[test]
    fn readme_error_from_io_error_converts() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let err: ReadmeError = io_err.into();
        assert!(matches!(err, ReadmeError::Io(_)));
    }

    #[test]
    fn readme_error_from_toml_error_converts() {
        let toml_err = toml::from_str::<Manifest>("invalid toml {]").unwrap_err();
        let err: ReadmeError = toml_err.into();
        assert!(matches!(err, ReadmeError::Toml(_)));
    }

    #[test]
    fn readme_error_display_formats_toml_error() {
        let toml_err = toml::from_str::<Manifest>("invalid").unwrap_err();
        let err = ReadmeError::Toml(toml_err);
        let formatted = format!("{}", err);
        assert!(formatted.contains("Failed to parse Cargo.toml"));
    }

    #[test]
    fn readme_error_display_formats_missing_feature_metadata() {
        let err =
            ReadmeError::MissingFeatureMetadata(vec!["feat1".to_string(), "feat2".to_string()]);
        let formatted = format!("{}", err);
        assert!(formatted.contains("Missing metadata for features"));
        assert!(formatted.contains("feat1"));
        assert!(formatted.contains("feat2"));
    }

    #[test]
    fn readme_error_display_formats_unknown_metadata_feature() {
        let err = ReadmeError::UnknownMetadataFeature(vec!["unknown1".to_string()]);
        let formatted = format!("{}", err);
        assert!(formatted.contains("Metadata defined for unknown features"));
        assert!(formatted.contains("unknown1"));
    }

    #[test]
    fn readme_error_display_formats_invalid_snippet_group() {
        let err = ReadmeError::InvalidSnippetGroup;
        let formatted = format!("{}", err);
        assert!(formatted.contains("feature_snippet_group must be greater than zero"));
    }

    #[test]
    fn readme_error_display_formats_out_of_sync() {
        let path = PathBuf::from("/path/to/README.md");
        let err = ReadmeError::OutOfSync {
            path
        };
        let formatted = format!("{}", err);
        assert!(formatted.contains("README at"));
        assert!(formatted.contains("is out of sync"));
        assert!(formatted.contains("cargo build"));
    }
}
