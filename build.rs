use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio}
};

use crate::readme::{sync_readme, verify_readme_relaxed};

#[path = "build/readme.rs"]
mod readme;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rustc-check-cfg=cfg(masterror_has_error_generic_member_access)");
    println!("cargo:rustc-check-cfg=cfg(masterror_requires_error_generic_feature)");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=README.template.md");
    println!("cargo:rerun-if-changed=build/readme.rs");
    println!("cargo:rerun-if-env-changed=MASTERROR_DISABLE_ERROR_GENERIC_MEMBER_ACCESS");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    // Явный флаг, чтобы где угодно ослабить проверку (ремень безопасности для
    // CI/verify)
    if allow_readme_drift() {
        return Ok(());
    }

    // В tarball-е (cargo package --verify) или вообще без .git — проверяем мягко и
    // НЕ валимся.
    if is_packaged_manifest(&manifest_dir) || !has_git_anywhere(&manifest_dir) {
        if let Err(err) = verify_readme_relaxed(&manifest_dir) {
            println!("cargo:warning={err}");
        }
        return Ok(());
    }

    // В нормальном git-рабочем дереве — синхронизируем (жёсткий режим).
    sync_readme(&manifest_dir)?;

    if let Some(support) = detect_error_generic_member_access()? {
        if support.requires_feature_attr {
            println!("cargo:rustc-cfg=masterror_requires_error_generic_feature");
        }
        println!("cargo:rustc-cfg=masterror_has_error_generic_member_access");
    }

    Ok(())
}

struct ErrorGenericSupport {
    requires_feature_attr: bool
}

// Твоя прежняя эвристика: target/package/... => packaged
fn is_packaged_manifest(manifest_dir: &Path) -> bool {
    let mut seen_target = false;
    for comp in manifest_dir.components() {
        match comp {
            std::path::Component::Normal(name) => {
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

// Проверяем .git по цепочке вверх (workspace корень часто выше
// crate-директории)
fn has_git_anywhere(mut dir: &Path) -> bool {
    loop {
        if dir.join(".git").exists() {
            return true;
        }
        match dir.parent() {
            Some(p) => dir = p,
            None => return false
        }
    }
}

fn allow_readme_drift() -> bool {
    has_env("MASTERROR_ALLOW_README_DRIFT") || has_env("MASTERROR_SKIP_README_CHECK")
}

fn has_env(name: &str) -> bool {
    env::var_os(name).map(|v| !v.is_empty()).unwrap_or(false)
}

fn detect_error_generic_member_access()
-> Result<Option<ErrorGenericSupport>, Box<dyn std::error::Error>> {
    if has_env("MASTERROR_DISABLE_ERROR_GENERIC_MEMBER_ACCESS") {
        return Ok(None);
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    fs::create_dir_all(&out_dir)?;

    let stable_check = out_dir.join("check_error_generic_stable.rs");
    fs::write(&stable_check, STABLE_SNIPPET)?;
    if compile_probe(&stable_check, &out_dir)?.success() {
        return Ok(Some(ErrorGenericSupport {
            requires_feature_attr: false
        }));
    }

    let nightly_check = out_dir.join("check_error_generic_nightly.rs");
    fs::write(&nightly_check, NIGHTLY_SNIPPET)?;
    if compile_probe(&nightly_check, &out_dir)?.success() {
        return Ok(Some(ErrorGenericSupport {
            requires_feature_attr: true
        }));
    }

    Ok(None)
}

fn compile_probe(
    source: &Path,
    out_dir: &Path
) -> Result<process::ExitStatus, Box<dyn std::error::Error>> {
    let rustc = env::var("RUSTC")?;
    let mut cmd = Command::new(rustc);
    cmd.arg("--crate-type").arg("lib");
    cmd.arg("--emit").arg("metadata");
    cmd.arg(source);
    cmd.arg("-o");
    cmd.arg(out_dir.join("check_error_generic.rmeta"));
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    Ok(cmd.status()?)
}

const STABLE_SNIPPET: &str = r#"use std::error::{Error, Request};

pub fn probe(request: &mut Request<'_>, error: &(dyn Error + 'static)) {
    let _ = request;
    let _ = error;
}
"#;

const NIGHTLY_SNIPPET: &str = r#"#![feature(error_generic_member_access)]

use std::error::{Error, Request};

pub fn probe(request: &mut Request<'_>, error: &(dyn Error + 'static)) {
    request.provide_ref::<&'static str>(&"marker");
    request.provide_value::<usize>(0);
    let _ = error;
}
"#;
