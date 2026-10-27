//! Managed payload status never follows installed links or manifest-supplied paths.
use super::{ASSETS, install};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io, path::Path};
pub(super) const STAMP: &str = ".biomcp-skill.json";
pub(super) fn digest(text: &[u8]) -> String {
    Sha256::digest(text)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
pub(super) fn manifest() -> Value {
    let files: BTreeMap<_, _> = ASSETS
        .iter()
        .map(|(path, text)| (*path, digest(text.as_bytes())))
        .collect();
    json!({"schema_version":1,"biomcp_version":env!("CARGO_PKG_VERSION"),"managed_files":files})
}
pub(super) fn classify(target: &Path) -> io::Result<&'static str> {
    install::check(target)?;
    if !target.join("SKILL.md").exists() {
        return Ok("missing");
    }
    let stamp = target.join(STAMP);
    install::check(&stamp)?;
    let recorded: Value = match fs::read(&stamp) {
        Ok(bytes) => match serde_json::from_slice(&bytes) {
            Ok(value) => value,
            Err(_) => return Ok("unmanaged"),
        },
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok("unmanaged"),
        Err(error) => return Err(error),
    };
    let Some(files) = recorded.get("managed_files").and_then(Value::as_object) else {
        return Ok("unmanaged");
    };
    if recorded.get("schema_version") != Some(&json!(1)) || files.is_empty() {
        return Ok("unmanaged");
    }
    // Validate every path before reading any of the manifest's files.
    for (path, hash) in files {
        install::relative(path)?;
        install::check(&target.join(path))?;
        if hash.as_str().is_none_or(|value| {
            value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit())
        }) {
            return Ok("unmanaged");
        }
    }
    for (path, hash) in files {
        match fs::read(target.join(path)) {
            Ok(bytes) if Some(digest(&bytes).as_str()) == hash.as_str() => {}
            Ok(_) => return Ok("locally_modified"),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok("locally_modified"),
            Err(error) => return Err(error),
        }
    }
    Ok(if recorded == manifest() {
        "current"
    } else {
        "stale"
    })
}
pub(super) fn render(directory: &Path) -> io::Result<String> {
    let target = install::target(directory)?;
    Ok(format!(
        "{}\n",
        json!({"state": classify(&target)?, "current_version":env!("CARGO_PKG_VERSION")})
    ))
}
