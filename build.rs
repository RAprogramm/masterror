use std::{
    env,
    error::Error,
    path::{Component, Path, PathBuf},
    process
};

#[path = "build/readme.rs"]
mod readme;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=README.template.md");
    println!("cargo:rerun-if-changed=build/readme.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    if is_packaged_manifest(&manifest_dir) {
        readme::verify_readme(&manifest_dir)?;
    } else {
        readme::sync_readme(&manifest_dir)?;
    }
    Ok(())
}

fn is_packaged_manifest(manifest_dir: &Path) -> bool {
    let mut seen_target = false;
    for component in manifest_dir.components() {
        match component {
            Component::Normal(name) => {
                if seen_target && name == "package" {
                    return true;
                }
                seen_target = name == "target";
            }
            _ => {
                seen_target = false;
            }
        }
    }
    false
}
