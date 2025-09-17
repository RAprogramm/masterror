#![allow(dead_code)]

use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::{Path, PathBuf}
};

use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
struct Manifest {
    package:  Package,
    #[serde(default)]
    features: BTreeMap<String, Vec<String>>
}
#[derive(Debug, Deserialize)]
struct Package {
    version:      String,
    #[serde(rename = "rust-version")]
    rust_version: Option<String>,
    #[serde(default)]
    metadata:     Option<PackageMetadata>
}
#[derive(Debug, Deserialize)]
struct PackageMetadata {
    #[serde(default)]
    masterror: Option<MasterrorMetadata>
}
#[derive(Debug, Deserialize)]
struct MasterrorMetadata {
    #[serde(default)]
    readme: Option<ReadmeMetadata>
}
#[derive(Clone, Debug, Deserialize)]
struct ReadmeMetadata {
    #[serde(default)]
    feature_order:         Vec<String>,
    #[serde(default)]
    feature_snippet_group: Option<usize>,
    #[serde(default)]
    conversion_lines:      Vec<String>,
    #[serde(default)]
    features:              BTreeMap<String, FeatureMetadata>
}
#[derive(Clone, Debug, Deserialize)]
struct FeatureMetadata {
    description: String,
    #[serde(default)]
    extra:       Vec<String>
}
#[derive(Clone, Debug)]
struct FeatureDoc {
    name:        String,
    description: String,
    extra:       Vec<String>
}

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

#[cfg_attr(test, allow(dead_code))]
pub fn sync_readme(manifest_dir: &Path) -> Result<(), ReadmeError> {
    let manifest_path = manifest_dir.join("Cargo.toml");
    let template_path = manifest_dir.join("README.template.md");
    let output_path = manifest_dir.join("README.md");
    let readme = generate_readme(&manifest_path, &template_path)?;
    write_if_changed(&output_path, &readme)
}

/// Strict verify (kept for local use if нужно)
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

/// Relaxed verify: normalize line endings and single trailing newline.
/// Используем в tarball/без .git, чтобы не падать на мелочах.
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

fn normalize(s: &str) -> String {
    // 1) CRLF -> LF, 2) убираем ровно один финальный '\n'
    let mut t = s.replace("\r\n", "\n");
    if t.ends_with('\n') {
        t.pop();
    }
    t
}

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
        if !feature_names.contains(name) {
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

fn render_readme(
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

fn render_feature_bullets(features: &[FeatureDoc]) -> String {
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

fn render_conversion_bullets(conversions: &[String]) -> String {
    conversions
        .iter()
        .map(|e| format!("- {e}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_feature_snippet(features: &[FeatureDoc], group_size: usize) -> String {
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

fn find_placeholder(rendered: &str) -> Option<String> {
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
