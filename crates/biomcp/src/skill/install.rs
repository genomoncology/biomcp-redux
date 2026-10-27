//! Explicit destination handling; no automatic client discovery or home writes.
use super::{ASSETS, status};
use serde_json::json;
use std::{
    fs, io,
    path::{Component, Path, PathBuf},
};
pub(super) fn relative(path: &str) -> io::Result<()> {
    let value = Path::new(path);
    if path.is_empty()
        || path.contains('\\')
        || value.is_absolute()
        || value
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
        || value
            .components()
            .map(|part| part.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/")
            != path
    {
        return Err(io::Error::other("invalid managed skill path"));
    }
    Ok(())
}
pub(super) fn check(path: &Path) -> io::Result<()> {
    let mut cursor = PathBuf::new();
    for part in path.components() {
        if matches!(part, Component::ParentDir) {
            return Err(io::Error::other(
                "skill directory must not contain parent traversal",
            ));
        }
        cursor.push(part);
        match fs::symlink_metadata(&cursor) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(io::Error::other(
                    "skill path must not contain symbolic links",
                ));
            }
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}
pub(super) fn target(directory: &Path) -> io::Result<PathBuf> {
    check(directory)?;
    let target = match directory.file_name().and_then(|part| part.to_str()) {
        Some("biomcp") => directory.to_path_buf(),
        Some("skills") => directory.join("biomcp"),
        _ => directory.join("skills/biomcp"),
    };
    check(&target)?;
    Ok(target)
}
pub(super) fn run(directory: &Path, force: bool) -> io::Result<String> {
    let target = target(directory)?;
    let existed = target.exists();
    let state = status::classify(&target)?;
    if existed && !force {
        return report("unchanged", false, state);
    }
    // Preflight the whole payload before making any writes. Never take write
    // paths from an installed manifest, which may be untrusted or stale.
    for (path, _) in ASSETS {
        relative(path)?;
        check(&target.join(path))?;
    }
    check(&target.join(status::STAMP))?;
    fs::create_dir_all(directory)?;
    let staging = tempfile::tempdir_in(directory)?;
    for (path, text) in ASSETS {
        let output = staging.path().join(path);
        fs::create_dir_all(
            output
                .parent()
                .ok_or_else(|| io::Error::other("missing asset parent"))?,
        )?;
        fs::write(output, text)?;
    }
    fs::write(
        staging.path().join(status::STAMP),
        serde_json::to_vec(&status::manifest())?,
    )?;
    fs::create_dir_all(&target)?;
    for path in ASSETS.iter().map(|(path, _)| *path).chain([status::STAMP]) {
        let output = target.join(path);
        check(&output)?;
        fs::create_dir_all(
            output
                .parent()
                .ok_or_else(|| io::Error::other("missing asset parent"))?,
        )?;
        // Rename replaces a file entry rather than following its link.
        #[cfg(windows)]
        if output.exists() {
            fs::remove_file(&output)?;
        }
        fs::rename(staging.path().join(path), output)?;
    }
    report(
        if existed { "repaired" } else { "installed" },
        true,
        status::classify(&target)?,
    )
}
fn report(status: &str, changed: bool, state: &str) -> io::Result<String> {
    Ok(format!(
        "{}\n",
        json!({"kind":"skill","action":"install","status":status,"changed":changed,"skill_status":{"state":state}})
    ))
}
