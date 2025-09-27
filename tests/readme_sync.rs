#[path = "../build/readme.rs"]
mod readme;

use std::{error::Error, fs, io, path::PathBuf};

use tempfile::tempdir;

const MINIMAL_MANIFEST: &str = r#"[package]
name = "demo"
version = "1.2.3"
rust-version = "1.89"
edition = "2024"

[features]
default = []

[package.metadata.masterror.readme]
feature_order = []
conversion_lines = []
feature_snippet_group = 2

[package.metadata.masterror.readme.features]
"#;

const MINIMAL_TEMPLATE: &str = "# Demo\\n\\nVersion {{CRATE_VERSION}}\\nMSRV {{MSRV}}\\n\\nFeatures\\n{{FEATURE_BULLETS}}\\n\\nSnippet\\n{{FEATURE_SNIPPET}}\\n\\nConversions\\n{{CONVERSION_BULLETS}}\\n";

#[test]
fn readme_is_in_sync() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = manifest_dir.join("Cargo.toml");
    let template_path = manifest_dir.join("README.template.md");
    let readme_path = manifest_dir.join("README.md");

    let generated = readme::generate_readme(&manifest_path, &template_path)?;
    let actual = fs::read_to_string(&readme_path)?;

    if actual != generated {
        // Use std::io::Error::other to satisfy clippy::io-other-error
        let msg = "README.md is out of date; run `cargo build` to regenerate";
        return Err(io::Error::other(msg).into());
    }

    Ok(())
}

#[test]
fn verify_readme_succeeds_when_in_sync() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir()?;
    let manifest_path = tmp.path().join("Cargo.toml");
    let template_path = tmp.path().join("README.template.md");
    let readme_path = tmp.path().join("README.md");

    fs::write(&manifest_path, MINIMAL_MANIFEST)?;
    fs::write(&template_path, MINIMAL_TEMPLATE)?;
    let generated = readme::generate_readme(&manifest_path, &template_path)?;
    fs::write(&readme_path, generated)?;

    readme::verify_readme(tmp.path()).map_err(|err| io::Error::other(err.to_string()))?;
    Ok(())
}

#[test]
fn verify_readme_detects_out_of_sync() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir()?;
    let manifest_path = tmp.path().join("Cargo.toml");
    let template_path = tmp.path().join("README.template.md");
    let readme_path = tmp.path().join("README.md");

    fs::write(&manifest_path, MINIMAL_MANIFEST)?;
    fs::write(&template_path, MINIMAL_TEMPLATE)?;
    fs::write(&readme_path, "outdated")?;

    match readme::verify_readme(tmp.path()) {
        Err(readme::ReadmeError::OutOfSync {
            path
        }) => {
            assert_eq!(path, readme_path);
            Ok(())
        }
        Err(err) => Err(io::Error::other(format!("unexpected error: {err}")).into()),
        Ok(_) => Err(io::Error::other("expected mismatch error").into())
    }
}
