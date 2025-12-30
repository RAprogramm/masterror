// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Template rendering functions for README generation.
//!
//! Handles substitution of placeholders in README templates with
//! generated content like feature lists, version numbers, and code snippets.

use super::{error::ReadmeError, types::FeatureDoc};

/// Renders the complete README from template and data.
///
/// Substitutes all standard placeholders and validates that no
/// unresolved placeholders remain.
///
/// # Arguments
///
/// * `template` - The README template with {{PLACEHOLDER}} markers
/// * `version` - Crate version string
/// * `rust_version` - Minimum supported Rust version
/// * `features` - Feature documentation list
/// * `snippet_group` - Number of features per line in snippet
/// * `conversions` - Conversion documentation lines
///
/// # Returns
///
/// Rendered README content or error if placeholders unresolved
pub(crate) fn render_readme(
    template: &str,
    version: &str,
    rust_version: &str,
    features: &[FeatureDoc],
    snippet_group: usize,
    conversions: &[String]
) -> Result<String, ReadmeError> {
    let feature_bullets = render_feature_bullets(features);
    let feature_snippet = render_feature_snippet(features, snippet_group);
    let conversion_bullets = render_conversion_bullets(conversions);
    let mut rendered = template.replace("{{CRATE_VERSION}}", version);
    rendered = rendered.replace("{{MSRV}}", rust_version);
    rendered = rendered.replace("{{FEATURE_BULLETS}}", &feature_bullets);
    rendered = rendered.replace("{{FEATURE_SNIPPET}}", &feature_snippet);
    rendered = rendered.replace("{{CONVERSION_BULLETS}}", &conversion_bullets);
    if let Some(name) = find_placeholder(&rendered) {
        return Err(ReadmeError::UnresolvedPlaceholder(name));
    }
    Ok(rendered)
}

/// Renders feature documentation as markdown bullet list.
///
/// Each feature becomes a bullet point with its description.
/// Extra notes are rendered as nested bullets.
///
/// # Arguments
///
/// * `features` - Feature documentation list
///
/// # Returns
///
/// Markdown formatted bullet list
pub(crate) fn render_feature_bullets(features: &[FeatureDoc]) -> String {
    let mut lines = Vec::new();
    for feature in features {
        lines.push(format!("- `{}` — {}", feature.name, feature.description));
        if !feature.extra.is_empty() {
            for note in &feature.extra {
                lines.push(format!("  - {note}"));
            }
        }
    }
    lines.join("\n")
}

/// Renders conversion documentation as markdown bullet list.
///
/// # Arguments
///
/// * `conversions` - List of conversion descriptions
///
/// # Returns
///
/// Markdown formatted bullet list
pub(crate) fn render_conversion_bullets(conversions: &[String]) -> String {
    conversions
        .iter()
        .map(|e| format!("- {e}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Renders features as TOML snippet for Cargo.toml dependencies.
///
/// Groups features into lines according to group_size parameter.
///
/// # Arguments
///
/// * `features` - Feature documentation list
/// * `group_size` - Number of features per line
///
/// # Returns
///
/// Formatted TOML snippet
pub(crate) fn render_feature_snippet(features: &[FeatureDoc], group_size: usize) -> String {
    if features.is_empty() {
        return String::new();
    }
    let mut items = Vec::with_capacity(features.len());
    for f in features {
        items.push(format!("\"{}\"", f.name));
    }
    let chunk = group_size;
    let chunks = items.len().div_ceil(chunk);
    let mut lines = Vec::with_capacity(chunks);
    for (i, part) in items.chunks(chunk).enumerate() {
        let mut line = String::from("#   ");
        line.push_str(&part.join(", "));
        if i + 1 != chunks {
            line.push(',');
        }
        lines.push(line);
    }
    lines.join("\n")
}

/// Finds first unresolved placeholder in rendered text.
///
/// Returns placeholder name if found, None otherwise.
///
/// # Arguments
///
/// * `rendered` - The rendered text to check
///
/// # Returns
///
/// Placeholder name or None
pub(crate) fn find_placeholder(rendered: &str) -> Option<String> {
    let start = rendered.find("{{")?;
    let after = &rendered[start + 2..];
    if let Some(end) = after.find("}}") {
        let name = after[..end].trim();
        Some(name.to_string())
    } else {
        let snippet: String = after.chars().take(32).collect();
        Some(snippet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_placeholder_detects_placeholder() {
        let text = "Some text {{PLACEHOLDER}} more text";
        let result = find_placeholder(text);
        assert_eq!(result, Some("PLACEHOLDER".to_string()));
    }

    #[test]
    fn find_placeholder_returns_none_when_no_placeholder() {
        let text = "No placeholders here";
        let result = find_placeholder(text);
        assert_eq!(result, None);
    }

    #[test]
    fn find_placeholder_handles_unclosed_braces() {
        let text = "{{INCOMPLETE";
        let result = find_placeholder(text);
        assert!(result.is_some());
        assert!(result.unwrap().starts_with("INCOMPLETE"));
    }

    #[test]
    fn render_feature_bullets_creates_list() {
        let features = vec![
            FeatureDoc {
                name:        "actix".to_string(),
                description: "Actix-web integration".to_string(),
                extra:       vec![]
            },
            FeatureDoc {
                name:        "axum".to_string(),
                description: "Axum integration".to_string(),
                extra:       vec!["Requires Tokio runtime".to_string()]
            },
        ];
        let result = render_feature_bullets(&features);
        assert!(result.contains("- `actix` — Actix-web integration"));
        assert!(result.contains("- `axum` — Axum integration"));
        assert!(result.contains("  - Requires Tokio runtime"));
    }

    #[test]
    fn render_feature_bullets_handles_empty_list() {
        let features = vec![];
        let result = render_feature_bullets(&features);
        assert_eq!(result, "");
    }

    #[test]
    fn render_conversion_bullets_creates_list() {
        let conversions = vec![
            "std::io::Error → AppError::Internal".to_string(),
            "String → AppError::BadRequest".to_string(),
        ];
        let result = render_conversion_bullets(&conversions);
        assert_eq!(
            result,
            "- std::io::Error → AppError::Internal\n- String → AppError::BadRequest"
        );
    }

    #[test]
    fn render_conversion_bullets_handles_empty_list() {
        let conversions = vec![];
        let result = render_conversion_bullets(&conversions);
        assert_eq!(result, "");
    }

    #[test]
    fn render_feature_snippet_groups_features() {
        let features = vec![
            FeatureDoc {
                name:        "feat1".to_string(),
                description: "desc1".to_string(),
                extra:       vec![]
            },
            FeatureDoc {
                name:        "feat2".to_string(),
                description: "desc2".to_string(),
                extra:       vec![]
            },
            FeatureDoc {
                name:        "feat3".to_string(),
                description: "desc3".to_string(),
                extra:       vec![]
            },
        ];
        let result = render_feature_snippet(&features, 2);
        assert!(result.contains("\"feat1\", \"feat2\","));
        assert!(result.contains("\"feat3\""));
    }

    #[test]
    fn render_feature_snippet_handles_empty_list() {
        let features = vec![];
        let result = render_feature_snippet(&features, 4);
        assert_eq!(result, "");
    }

    #[test]
    fn render_readme_substitutes_placeholders() {
        let template = "Version: {{CRATE_VERSION}}\nMSRV: {{MSRV}}\nFeatures:\n{{FEATURE_BULLETS}}\nSnippet:\n{{FEATURE_SNIPPET}}\nConversions:\n{{CONVERSION_BULLETS}}";
        let features = vec![FeatureDoc {
            name:        "test".to_string(),
            description: "Test feature".to_string(),
            extra:       vec![]
        }];
        let conversions = vec!["Error → AppError".to_string()];
        let result = render_readme(template, "1.0.0", "1.70", &features, 4, &conversions);
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("Version: 1.0.0"));
        assert!(rendered.contains("MSRV: 1.70"));
        assert!(rendered.contains("`test` — Test feature"));
        assert!(rendered.contains("\"test\""));
        assert!(rendered.contains("- Error → AppError"));
    }

    #[test]
    fn render_readme_errors_on_unresolved_placeholder() {
        let template = "{{CRATE_VERSION}} {{UNKNOWN}}";
        let features = vec![];
        let conversions = vec![];
        let result = render_readme(template, "1.0.0", "1.70", &features, 4, &conversions);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ReadmeError::UnresolvedPlaceholder(_)
        ));
    }
}
