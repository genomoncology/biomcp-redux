//! Build isolated source trees for the structural witnesses.
use super::{Check, check};
use std::{fs, path::Path};

pub(super) fn tree() -> Result<tempfile::TempDir, Box<dyn std::error::Error>> {
    let tree = tempfile::tempdir()?;
    fs::write(
        tree.path().join("Cargo.toml"),
        "[workspace]\n[workspace.dependencies]\nhttp = { package = 'ureq', version = '3' }\n",
    )?;
    put(
        tree.path(),
        "crates/biodata/Cargo.toml",
        "[package]\nname = 'biodata'\n[lints]\nworkspace = true\n",
    )?;
    put(tree.path(), "crates/biodata/src/lib.rs", "//! Values.\n")?;
    Ok(tree)
}

pub(super) fn put(root: &Path, path: &str, text: &str) -> Check {
    let path = root.join(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text)?;
    Ok(())
}

pub(super) fn rejects(path: &str, content: &str, rule: &str) -> Check {
    let tree = tree()?;
    put(tree.path(), path, content)?;
    let failure = match check(tree.path()) {
        Ok(()) => return Err("planted violation passed".into()),
        Err(error) => error.to_string(),
    };
    assert!(
        failure.contains(path) && failure.contains(rule),
        "{failure}"
    );
    Ok(())
}
