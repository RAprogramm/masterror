// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Integration tests for build script functionality

#[path = "../build/readme.rs"]
mod readme;

use std::{collections::BTreeMap, fs, io};

use readme::{ReadmeError, generate_readme};
use tempfile::TempDir;

#[test]
fn generate_readme_with_valid_input() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("Cargo.toml");
    let template_path = temp.path().join("README.template.md");

    let manifest_content = r#"
[package]
name = "test-crate"
version = "0.1.0"
rust-version = "1.70"

[package.metadata.masterror.readme]
feature_order = ["feature1", "feature2"]
feature_snippet_group = 2
conversion_lines = ["std::io::Error → Internal", "String → BadRequest"]

[package.metadata.masterror.readme.features.feature1]
description = "First feature"
extra = ["Extra note 1"]

[package.metadata.masterror.readme.features.feature2]
description = "Second feature"

[features]
default = []
feature1 = []
feature2 = []
"#;

    let template_content = r#"# Test Crate

Version: {{CRATE_VERSION}}
MSRV: {{MSRV}}

## Features

{{FEATURE_BULLETS}}

## Example

```toml
[dependencies]
test-crate = { version = "{{CRATE_VERSION}}", features = [
{{FEATURE_SNIPPET}}
] }
```

## Conversions

{{CONVERSION_BULLETS}}
"#;

    fs::write(&manifest_path, manifest_content).unwrap();
    fs::write(&template_path, template_content).unwrap();

    let result = generate_readme(&manifest_path, &template_path).unwrap();

    assert!(result.contains("Version: 0.1.0"));
    assert!(result.contains("MSRV: 1.70"));
    assert!(result.contains("- `feature1` — First feature"));
    assert!(result.contains("  - Extra note 1"));
    assert!(result.contains("- `feature2` — Second feature"));
    assert!(result.contains("\"feature1\", \"feature2\""));
    assert!(result.contains("- std::io::Error → Internal"));
    assert!(result.contains("- String → BadRequest"));
}

#[test]
fn generate_readme_fails_with_missing_metadata() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("Cargo.toml");
    let template_path = temp.path().join("README.template.md");

    let manifest_content = r#"
[package]
name = "test-crate"
version = "0.1.0"

[features]
feature1 = []
"#;

    fs::write(&manifest_path, manifest_content).unwrap();
    fs::write(&template_path, "# Template").unwrap();

    let result = generate_readme(&manifest_path, &template_path);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ReadmeError::MissingMetadata(_)
    ));
}

#[test]
fn generate_readme_fails_with_missing_feature_metadata() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("Cargo.toml");
    let template_path = temp.path().join("README.template.md");

    let manifest_content = r#"
[package]
name = "test-crate"
version = "0.1.0"

[package.metadata.masterror.readme]

[features]
default = []
feature1 = []
"#;

    fs::write(&manifest_path, manifest_content).unwrap();
    fs::write(&template_path, "# Template").unwrap();

    let result = generate_readme(&manifest_path, &template_path);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ReadmeError::MissingFeatureMetadata(_)
    ));
}

#[test]
fn generate_readme_fails_with_unknown_feature_in_order() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("Cargo.toml");
    let template_path = temp.path().join("README.template.md");

    let manifest_content = r#"
[package]
name = "test-crate"
version = "0.1.0"

[package.metadata.masterror.readme]
feature_order = ["unknown_feature"]

[package.metadata.masterror.readme.features.feature1]
description = "Feature 1"

[features]
default = []
feature1 = []
"#;

    fs::write(&manifest_path, manifest_content).unwrap();
    fs::write(&template_path, "# Template").unwrap();

    let result = generate_readme(&manifest_path, &template_path);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ReadmeError::UnknownFeatureInOrder(_)
    ));
}

#[test]
fn generate_readme_fails_with_duplicate_feature_in_order() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("Cargo.toml");
    let template_path = temp.path().join("README.template.md");

    let manifest_content = r#"
[package]
name = "test-crate"
version = "0.1.0"

[package.metadata.masterror.readme]
feature_order = ["feature1", "feature1"]

[package.metadata.masterror.readme.features.feature1]
description = "Feature 1"

[features]
default = []
feature1 = []
"#;

    fs::write(&manifest_path, manifest_content).unwrap();
    fs::write(&template_path, "# Template").unwrap();

    let result = generate_readme(&manifest_path, &template_path);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ReadmeError::DuplicateFeatureInOrder(_)
    ));
}

#[test]
fn generate_readme_fails_with_unresolved_placeholder() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("Cargo.toml");
    let template_path = temp.path().join("README.template.md");

    let manifest_content = r#"
[package]
name = "test-crate"
version = "0.1.0"

[package.metadata.masterror.readme]

[features]
default = []
"#;

    let template_content = "# Template {{UNKNOWN_PLACEHOLDER}}";

    fs::write(&manifest_path, manifest_content).unwrap();
    fs::write(&template_path, template_content).unwrap();

    let result = generate_readme(&manifest_path, &template_path);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ReadmeError::UnresolvedPlaceholder(_)
    ));
}

#[test]
fn generate_readme_fails_with_zero_snippet_group() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("Cargo.toml");
    let template_path = temp.path().join("README.template.md");

    let manifest_content = r#"
[package]
name = "test-crate"
version = "0.1.0"

[package.metadata.masterror.readme]
feature_snippet_group = 0

[features]
default = []
"#;

    fs::write(&manifest_path, manifest_content).unwrap();
    fs::write(&template_path, "{{FEATURE_SNIPPET}}").unwrap();

    let result = generate_readme(&manifest_path, &template_path);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ReadmeError::InvalidSnippetGroup
    ));
}

#[test]
fn generate_readme_handles_missing_rust_version() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("Cargo.toml");
    let template_path = temp.path().join("README.template.md");

    let manifest_content = r#"
[package]
name = "test-crate"
version = "0.1.0"

[package.metadata.masterror.readme]

[features]
default = []
"#;

    let template_content = "MSRV: {{MSRV}}";

    fs::write(&manifest_path, manifest_content).unwrap();
    fs::write(&template_path, template_content).unwrap();

    let result = generate_readme(&manifest_path, &template_path).unwrap();

    assert!(result.contains("MSRV: unknown"));
}

#[test]
fn readme_error_implements_error_trait() {
    let err = ReadmeError::InvalidSnippetGroup;
    let _: &dyn std::error::Error = &err;
}

#[test]
fn readme_error_from_io_error() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "test");
    let err: ReadmeError = io_err.into();
    assert!(matches!(err, ReadmeError::Io(_)));
}

#[test]
fn readme_error_from_toml_error() {
    let toml_err = toml::from_str::<BTreeMap<String, String>>("invalid {toml").unwrap_err();
    let err: ReadmeError = toml_err.into();
    assert!(matches!(err, ReadmeError::Toml(_)));
}
