// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Data structures for README metadata and configuration.
//!
//! Defines all types used for parsing Cargo.toml metadata and
//! representing feature documentation.

use std::collections::BTreeMap;

use serde::Deserialize;

/// Cargo.toml manifest structure.
#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub(crate) package:   Package,
    #[serde(default)]
    pub(crate) features:  BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub(crate) workspace: Option<Workspace>
}

/// Package section of Cargo.toml.
#[derive(Debug, Deserialize)]
pub struct Package {
    pub(crate) version:      String,
    #[serde(rename = "rust-version")]
    pub(crate) rust_version: Option<RustVersion>,
    #[serde(default)]
    pub(crate) metadata:     Option<PackageMetadata>
}

/// The `rust-version` field of a package.
///
/// The field holds either a literal version or a workspace inheritance.
///
/// # Examples
///
/// ```toml
/// rust-version = "1.99"
/// rust-version.workspace = true
/// ```
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RustVersion {
    Value(String),
    Inherited { workspace: bool }
}

/// Workspace section of Cargo.toml.
#[derive(Debug, Deserialize)]
pub struct Workspace {
    #[serde(default)]
    pub(crate) package: Option<WorkspacePackage>
}

/// Workspace package defaults.
#[derive(Debug, Deserialize)]
pub struct WorkspacePackage {
    #[serde(rename = "rust-version")]
    pub(crate) rust_version: Option<String>
}

/// Package metadata section.
#[derive(Debug, Deserialize)]
pub struct PackageMetadata {
    #[serde(default)]
    pub(crate) masterror: Option<MasterrorMetadata>
}

/// Masterror-specific metadata.
#[derive(Debug, Deserialize)]
pub struct MasterrorMetadata {
    #[serde(default)]
    pub(crate) readme: Option<ReadmeMetadata>
}

/// README generation metadata configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct ReadmeMetadata {
    #[serde(default)]
    pub(crate) feature_order:         Vec<String>,
    #[serde(default)]
    pub(crate) feature_snippet_group: Option<usize>,
    #[serde(default)]
    pub(crate) conversion_lines:      Vec<String>,
    #[serde(default)]
    pub(crate) features:              BTreeMap<String, FeatureMetadata>
}

/// Metadata for a single feature.
#[derive(Clone, Debug, Deserialize)]
pub struct FeatureMetadata {
    pub(crate) description: String,
    #[serde(default)]
    pub(crate) extra:       Vec<String>
}

/// Processed feature documentation.
#[derive(Clone, Debug)]
pub struct FeatureDoc {
    pub(crate) name:        String,
    pub(crate) description: String,
    pub(crate) extra:       Vec<String>
}
