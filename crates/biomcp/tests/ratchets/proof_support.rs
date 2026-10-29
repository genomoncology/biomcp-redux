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

pub(super) fn check_inventory_mutations() -> Check {
    use super::source::adapter_values::validate;
    let original = include_str!("adapter_value_audit.json");
    let mut current: serde_json::Value = serde_json::from_str(original)?;
    let sites = current["sites"]
        .as_array_mut()
        .ok_or("missing audit sites")?;
    sites.retain(|entry| entry["maximum"].as_u64().is_some_and(|n| n > 1));
    for entry in sites.iter_mut() {
        entry["maximum"] = 1.into();
        entry["class"] = "migrated".into();
    }
    validate(&current.to_string())?;
    validate("{\"sites\":[]}")?;
    let planted = serde_json::json!({"sites": [{
        "path": "crates/biomcp-adapters/src/example/details.rs",
        "site": "new pending identity",
        "maximum": 1,
        "class": "pending_audit",
        "owner": "example",
        "reason": "Planted new entry"
    }]});
    assert!(
        validate(&planted.to_string()).is_err(),
        "new entry in empty inventory passed"
    );
    let mut current: serde_json::Value = serde_json::from_str(original)?;
    if current["sites"].as_array().is_some_and(Vec::is_empty) {
        return Ok(());
    }
    current["sites"][0]["site"] = "new pending identity".into();
    assert!(
        validate(&current.to_string()).is_err(),
        "same-count replacement passed"
    );
    let mut current: serde_json::Value = serde_json::from_str(original)?;
    let maximum = current["sites"][0]["maximum"]
        .as_u64()
        .ok_or("missing count")?;
    current["sites"][0]["maximum"] = (maximum + 1).into();
    assert!(
        validate(&current.to_string()).is_err(),
        "increased ceiling passed"
    );
    let mut current: serde_json::Value = serde_json::from_str(original)?;
    current["sites"][0]["owner"] = "other_adapter".into();
    assert!(
        validate(&current.to_string()).is_err(),
        "wrong adapter owner passed"
    );
    let mut current: serde_json::Value = serde_json::from_str(original)?;
    let duplicate = current["sites"][0].clone();
    current["sites"]
        .as_array_mut()
        .ok_or("missing audit sites")?
        .push(duplicate);
    assert!(
        validate(&current.to_string()).is_err(),
        "duplicate pending entry passed"
    );
    Ok(())
}
