#[path = "../build/readme.rs"]
mod readme;

use std::{error::Error, fs, io, path::PathBuf};

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
