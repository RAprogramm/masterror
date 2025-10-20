#![allow(dead_code)]
// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! README generation from Cargo.toml metadata and templates.
//!
//! Provides functions to generate and validate README.md files from
//! templates with dynamic content like feature lists and version numbers.

pub mod error;
pub mod render;
pub mod types;

use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::Path
};

pub use error::ReadmeError;
use render::render_readme;
use types::{FeatureDoc, Manifest, Package, ReadmeMetadata};

/// Generates README content from manifest and template files.
///
/// Reads Cargo.toml metadata and README template, then renders
/// the complete README with substituted placeholders.
///
/// # Arguments
///
/// * `manifest_path` - Path to Cargo.toml
/// * `template_path` - Path to README.template.md
///
/// # Returns
///
/// Generated README content or error
pub fn generate_readme(manifest_path: &Path, template_path: &Path) -> Result<String, ReadmeError> {
    let manifest_raw = fs::read_to_string(manifest_path)?;
    let manifest: Manifest = toml::from_str(&manifest_raw)?;
    let Manifest {
        package,
        features
    } = manifest;
    let Package {
        version,
        rust_version,
        metadata
    } = package;

    let readme_meta = metadata
        .and_then(|m| m.masterror)
        .and_then(|m| m.readme)
        .ok_or(ReadmeError::MissingMetadata(
            "package.metadata.masterror.readme"
        ))?;

    let feature_docs = collect_feature_docs(&features, &readme_meta)?;
    let snippet_group = readme_meta.feature_snippet_group.unwrap_or(4);
    if snippet_group == 0 {
        return Err(ReadmeError::InvalidSnippetGroup);
    }

    let template_raw = fs::read_to_string(template_path)?;
    render_readme(
        &template_raw,
        &version,
        rust_version.as_deref().unwrap_or("unknown"),
        &feature_docs,
        snippet_group,
        &readme_meta.conversion_lines
    )
}

/// Writes README.md if content differs from existing file.
///
/// # Arguments
///
/// * `manifest_dir` - Repository root directory
///
/// # Returns
///
/// Success or error
#[cfg_attr(test, allow(dead_code))]
pub fn sync_readme(manifest_dir: &Path) -> Result<(), ReadmeError> {
    let manifest_path = manifest_dir.join("Cargo.toml");
    let template_path = manifest_dir.join("README.template.md");
    let output_path = manifest_dir.join("README.md");
    let readme = generate_readme(&manifest_path, &template_path)?;
    write_if_changed(&output_path, &readme)
}

/// Strictly verifies README.md matches generated content.
///
/// Used for local verification where exact byte-for-byte
/// matching is required.
///
/// # Arguments
///
/// * `manifest_dir` - Repository root directory
///
/// # Returns
///
/// Success or OutOfSync error
pub(crate) fn verify_readme(manifest_dir: &Path) -> Result<(), ReadmeError> {
    let manifest_path = manifest_dir.join("Cargo.toml");
    let template_path = manifest_dir.join("README.template.md");
    let output_path = manifest_dir.join("README.md");
    let generated = generate_readme(&manifest_path, &template_path)?;
    let actual = fs::read_to_string(&output_path)?;
    if actual == generated {
        Ok(())
    } else {
        Err(ReadmeError::OutOfSync {
            path: output_path
        })
    }
}

/// Verifies README.md with normalized line endings.
///
/// Used in tarball/release contexts where line ending differences
/// should not cause verification failures.
///
/// # Arguments
///
/// * `manifest_dir` - Repository root directory
///
/// # Returns
///
/// Success or OutOfSync error
pub(crate) fn verify_readme_relaxed(manifest_dir: &Path) -> Result<(), ReadmeError> {
    let manifest_path = manifest_dir.join("Cargo.toml");
    let template_path = manifest_dir.join("README.template.md");
    let output_path = manifest_dir.join("README.md");
    let generated = generate_readme(&manifest_path, &template_path)?;
    let actual = fs::read_to_string(&output_path)?;
    if normalize(&actual) == normalize(&generated) {
        Ok(())
    } else {
        Err(ReadmeError::OutOfSync {
            path: output_path
        })
    }
}

/// Normalizes text for comparison.
///
/// Converts CRLF to LF and removes exactly one trailing newline.
///
/// # Arguments
///
/// * `s` - Text to normalize
///
/// # Returns
///
/// Normalized text
fn normalize(s: &str) -> String {
    let mut t = s.replace("\r\n", "\n");
    if t.ends_with('\n') {
        t.pop();
    }
    t
}

/// Collects and validates feature documentation from metadata.
///
/// Ensures all features have metadata, no unknown metadata exists,
/// and feature_order references are valid.
///
/// # Arguments
///
/// * `feature_table` - Features from Cargo.toml
/// * `readme_meta` - README metadata configuration
///
/// # Returns
///
/// Ordered list of feature documentation or validation error
fn collect_feature_docs(
    feature_table: &BTreeMap<String, Vec<String>>,
    readme_meta: &ReadmeMetadata
) -> Result<Vec<FeatureDoc>, ReadmeError> {
    let feature_names: BTreeSet<String> = feature_table
        .keys()
        .filter(|n| n.as_str() != "default")
        .cloned()
        .collect();

    let mut missing_docs = Vec::new();
    let mut docs_map = BTreeMap::new();
    for name in &feature_names {
        if let Some(meta) = readme_meta.features.get(name) {
            docs_map.insert(
                name.clone(),
                FeatureDoc {
                    name:        name.clone(),
                    description: meta.description.clone(),
                    extra:       meta.extra.clone()
                }
            );
        } else {
            missing_docs.push(name.clone());
        }
    }
    if !missing_docs.is_empty() {
        return Err(ReadmeError::MissingFeatureMetadata(missing_docs));
    }

    let unknown_metadata: Vec<String> = readme_meta
        .features
        .keys()
        .filter(|n| n.as_str() != "default" && !feature_names.contains(*n))
        .cloned()
        .collect();
    if !unknown_metadata.is_empty() {
        return Err(ReadmeError::UnknownMetadataFeature(unknown_metadata));
    }

    let mut ordered = Vec::new();
    for name in &readme_meta.feature_order {
        if name == "default" {
            continue;
        }
        if !feature_names.contains(name.as_str()) {
            return Err(ReadmeError::UnknownFeatureInOrder(name.clone()));
        }
        if let Some(doc) = docs_map.remove(name) {
            ordered.push(doc);
        } else {
            return Err(ReadmeError::DuplicateFeatureInOrder(name.clone()));
        }
    }
    ordered.extend(docs_map.into_values());
    Ok(ordered)
}

#[cfg_attr(test, allow(dead_code))]
fn write_if_changed(path: &Path, contents: &str) -> Result<(), ReadmeError> {
    match fs::read_to_string(path) {
        Ok(existing) if existing == contents => return Ok(()),
        Ok(_) => {}
        Err(err) if err.kind() != io::ErrorKind::NotFound => return Err(ReadmeError::Io(err)),
        Err(_) => {}
    }
    fs::write(path, contents)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{types::FeatureMetadata, *};

    #[test]
    fn normalize_converts_crlf_to_lf() {
        let input = "line1\r\nline2\r\nline3";
        let result = normalize(input);
        assert_eq!(result, "line1\nline2\nline3");
    }

    #[test]
    fn normalize_removes_single_trailing_newline() {
        let input = "line1\nline2\n";
        let result = normalize(input);
        assert_eq!(result, "line1\nline2");
    }

    #[test]
    fn normalize_handles_empty_string() {
        let input = "";
        let result = normalize(input);
        assert_eq!(result, "");
    }

    #[test]
    fn normalize_handles_only_newline() {
        let input = "\n";
        let result = normalize(input);
        assert_eq!(result, "");
    }

    #[test]
    fn collect_feature_docs_handles_valid_features() {
        let mut features = BTreeMap::new();
        features.insert("feat1".to_string(), vec![]);
        features.insert("feat2".to_string(), vec![]);
        features.insert("default".to_string(), vec![]);

        let mut feature_meta = BTreeMap::new();
        feature_meta.insert(
            "feat1".to_string(),
            FeatureMetadata {
                description: "Feature 1".to_string(),
                extra:       vec![]
            }
        );
        feature_meta.insert(
            "feat2".to_string(),
            FeatureMetadata {
                description: "Feature 2".to_string(),
                extra:       vec!["Extra note".to_string()]
            }
        );

        let readme_meta = ReadmeMetadata {
            feature_order:         vec!["feat1".to_string()],
            feature_snippet_group: Some(4),
            conversion_lines:      vec![],
            features:              feature_meta
        };

        let result = collect_feature_docs(&features, &readme_meta);
        assert!(result.is_ok());
        let docs = result.unwrap();
        assert_eq!(docs.len(), 2);
        assert_eq!(docs[0].name, "feat1");
        assert_eq!(docs[1].name, "feat2");
    }

    #[test]
    fn collect_feature_docs_errors_on_missing_metadata() {
        let mut features = BTreeMap::new();
        features.insert("feat1".to_string(), vec![]);

        let readme_meta = ReadmeMetadata {
            feature_order:         vec![],
            feature_snippet_group: Some(4),
            conversion_lines:      vec![],
            features:              BTreeMap::new()
        };

        let result = collect_feature_docs(&features, &readme_meta);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ReadmeError::MissingFeatureMetadata(_)
        ));
    }

    #[test]
    fn collect_feature_docs_errors_on_unknown_metadata() {
        let features = BTreeMap::new();

        let mut feature_meta = BTreeMap::new();
        feature_meta.insert(
            "unknown".to_string(),
            FeatureMetadata {
                description: "Unknown".to_string(),
                extra:       vec![]
            }
        );

        let readme_meta = ReadmeMetadata {
            feature_order:         vec![],
            feature_snippet_group: Some(4),
            conversion_lines:      vec![],
            features:              feature_meta
        };

        let result = collect_feature_docs(&features, &readme_meta);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ReadmeError::UnknownMetadataFeature(_)
        ));
    }

    #[test]
    fn collect_feature_docs_errors_on_unknown_feature_in_order() {
        let mut features = BTreeMap::new();
        features.insert("feat1".to_string(), vec![]);

        let mut feature_meta = BTreeMap::new();
        feature_meta.insert(
            "feat1".to_string(),
            FeatureMetadata {
                description: "Feature 1".to_string(),
                extra:       vec![]
            }
        );

        let readme_meta = ReadmeMetadata {
            feature_order:         vec!["unknown".to_string()],
            feature_snippet_group: Some(4),
            conversion_lines:      vec![],
            features:              feature_meta
        };

        let result = collect_feature_docs(&features, &readme_meta);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ReadmeError::UnknownFeatureInOrder(_)
        ));
    }

    #[test]
    fn collect_feature_docs_errors_on_duplicate_in_order() {
        let mut features = BTreeMap::new();
        features.insert("feat1".to_string(), vec![]);

        let mut feature_meta = BTreeMap::new();
        feature_meta.insert(
            "feat1".to_string(),
            FeatureMetadata {
                description: "Feature 1".to_string(),
                extra:       vec![]
            }
        );

        let readme_meta = ReadmeMetadata {
            feature_order:         vec!["feat1".to_string(), "feat1".to_string()],
            feature_snippet_group: Some(4),
            conversion_lines:      vec![],
            features:              feature_meta
        };

        let result = collect_feature_docs(&features, &readme_meta);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ReadmeError::DuplicateFeatureInOrder(_)
        ));
    }

    #[test]
    fn collect_feature_docs_skips_default_in_order() {
        let mut features = BTreeMap::new();
        features.insert("feat1".to_string(), vec![]);
        features.insert("default".to_string(), vec![]);

        let mut feature_meta = BTreeMap::new();
        feature_meta.insert(
            "feat1".to_string(),
            FeatureMetadata {
                description: "Feature 1".to_string(),
                extra:       vec![]
            }
        );

        let readme_meta = ReadmeMetadata {
            feature_order:         vec!["default".to_string(), "feat1".to_string()],
            feature_snippet_group: Some(4),
            conversion_lines:      vec![],
            features:              feature_meta
        };

        let result = collect_feature_docs(&features, &readme_meta);
        assert!(result.is_ok());
        let docs = result.unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].name, "feat1");
    }
}
