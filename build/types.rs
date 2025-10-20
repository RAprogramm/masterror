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
pub(crate) struct Manifest {
    pub(crate) package:  Package,
    #[serde(default)]
    pub(crate) features: BTreeMap<String, Vec<String>>
}

/// Package section of Cargo.toml.
#[derive(Debug, Deserialize)]
pub(crate) struct Package {
    pub(crate) version:      String,
    #[serde(rename = "rust-version")]
    pub(crate) rust_version: Option<String>,
    #[serde(default)]
    pub(crate) metadata:     Option<PackageMetadata>
}

/// Package metadata section.
#[derive(Debug, Deserialize)]
pub(crate) struct PackageMetadata {
    #[serde(default)]
    pub(crate) masterror: Option<MasterrorMetadata>
}

/// Masterror-specific metadata.
#[derive(Debug, Deserialize)]
pub(crate) struct MasterrorMetadata {
    #[serde(default)]
    pub(crate) readme: Option<ReadmeMetadata>
}

/// README generation metadata configuration.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ReadmeMetadata {
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
pub(crate) struct FeatureMetadata {
    pub(crate) description: String,
    #[serde(default)]
    pub(crate) extra:       Vec<String>
}

/// Processed feature documentation.
#[derive(Clone, Debug)]
pub(crate) struct FeatureDoc {
    pub(crate) name:        String,
    pub(crate) description: String,
    pub(crate) extra:       Vec<String>
}
