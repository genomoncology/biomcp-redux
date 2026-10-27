//! Repository structure is part of the build contract.
#[path = "ratchets/dependencies.rs"]
mod dependencies;
#[path = "ratchets/placement.rs"]
mod placement;
#[path = "ratchets/proof.rs"]
mod proof;
#[path = "ratchets/reasons.rs"]
mod reasons;
#[path = "ratchets/source.rs"]
mod source;

use std::{fs, path::Path};

type Check = Result<(), Box<dyn std::error::Error>>;

fn check(root: &Path) -> Check {
    dependencies::check(root)?;
    reasons::check(root)?;
    source::check(root)
}

fn rust_files(path: &Path) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let mut files = Vec::new();
    if path.exists() {
        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            if path.is_dir() {
                files.extend(rust_files(&path)?);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

#[test]
fn repository_obeys_structural_contract() -> Check {
    check(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .as_path(),
    )
}
