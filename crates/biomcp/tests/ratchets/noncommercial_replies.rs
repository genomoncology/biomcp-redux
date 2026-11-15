//! Replies from noncommercial sources are synthetic, and a README beside them says so (0695, 0920).
use super::Check;
use biomcp_types::{Source, Use};
use serde_json::Value;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

pub(super) fn check(root: &Path) -> Check {
    let replies = root.join("tests/replies");
    if !replies.exists() {
        return Ok(());
    }
    // The registry's terms decide which sources and carried fields are noncommercial.
    let sources: BTreeSet<String> = Source::ALL
        .iter()
        .filter(|&&source| {
            biomcp_adapters::registry::terms(source)
                .is_some_and(|terms| terms.usage == Use::Noncommercial)
        })
        .map(ToString::to_string)
        .collect();
    let components: Vec<(String, &str)> = biomcp_adapters::registry::components()
        .filter(|component| component.terms.usage == Use::Noncommercial)
        .map(|component| (component.source.to_string(), component.name))
        .collect();
    let mut unmarked = Vec::new();
    for path in json_files(&replies)? {
        // Other JSON under tests/replies (license lists, projections) is not a recording.
        let Ok(record) = serde_json::from_slice::<Value>(&fs::read(&path)?) else {
            continue;
        };
        let Some(source) = record.get("source").and_then(Value::as_str) else {
            continue;
        };
        let carried = components.iter().any(|(owner, name)| {
            owner == source
                && record
                    .get("exchanges")
                    .is_some_and(|value| has_key(value, name))
        });
        if (sources.contains(source) || carried) && !marked(&replies, &path)? {
            unmarked.push(format!(
                "{} ({source})",
                path.strip_prefix(root)?.to_string_lossy()
            ));
        }
    }
    if unmarked.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "replies from noncommercial sources need a nearby README.md saying they are synthetic:\n{}",
            unmarked.join("\n")
        )
        .into())
    }
}

/// The nearest README.md above the recording, inside tests/replies, must say "synthetic".
fn marked(replies: &Path, path: &Path) -> Result<bool, std::io::Error> {
    let mut folder = path.parent();
    while let Some(current) = folder.filter(|current| *current != replies) {
        let readme = current.join("README.md");
        if readme.exists() {
            return Ok(fs::read_to_string(readme)?
                .to_lowercase()
                .contains("synthetic"));
        }
        folder = current.parent();
    }
    Ok(false)
}

fn has_key(value: &Value, name: &str) -> bool {
    match value {
        Value::Object(map) => map
            .iter()
            .any(|(key, item)| key == name || has_key(item, name)),
        Value::Array(items) => items.iter().any(|item| has_key(item, name)),
        _ => false,
    }
}

fn json_files(path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            files.extend(json_files(&path)?);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

#[test]
fn unmarked_noncommercial_replies_fail() -> Check {
    let root = tempfile::tempdir()?;
    let case = root.path().join("tests/replies/area/case");
    for (source, body) in [
        ("semantic-scholar", serde_json::json!({"data": []})),
        ("myvariant", serde_json::json!({"hits": [{"cadd": {}}]})),
    ] {
        fs::create_dir_all(case.join(source))?;
        let record = serde_json::json!({
            "source": source,
            "exchanges": [{"body": {"kind": "json", "value": body}}]
        });
        fs::write(case.join(source).join("a.json"), record.to_string())?;
    }
    let open = serde_json::json!({"source": "pubmed", "exchanges": []});
    fs::write(case.join("open.json"), open.to_string())?;
    let error = check(root.path()).err().ok_or("unmarked replies passed")?;
    for source in ["(semantic-scholar)", "(myvariant)"] {
        if !error.to_string().contains(source) {
            return Err(format!("{source} was not reported: {error}").into());
        }
    }
    if error.to_string().contains("(pubmed)") {
        return Err("an open source was reported".into());
    }
    fs::write(
        root.path().join("tests/replies/area/README.md"),
        "Real rows.",
    )?;
    if check(root.path()).is_ok() {
        return Err("a README that does not say synthetic passed".into());
    }
    fs::write(case.join("README.md"), "These rows are Synthetic.")?;
    check(root.path())
}
